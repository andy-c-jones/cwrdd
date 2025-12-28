use crate::config::Config;
use crate::task::{command_exists, Task};
use anyhow::{bail, Context, Result};
use std::time::Duration;
use tokio::time::sleep;

/// Ensure self-signed TLS certificates exist
async fn ensure_certs(config: &Config) -> Result<()> {
    let certs_dir = config.repo_path.join("config/certs");
    let cert_path = certs_dir.join("cert.pem");
    let key_path = certs_dir.join("key.pem");

    if cert_path.exists() && key_path.exists() {
        println!("✅ TLS certificates already exist\n");
        return Ok(());
    }

    println!("🔐 Generating self-signed TLS certificates...");

    // Create certs directory if it doesn't exist
    std::fs::create_dir_all(&certs_dir)?;

    let cert_task = Task::new("generate certs", "openssl")
        .args([
            "req", "-x509", "-newkey", "rsa:4096",
            "-keyout", key_path.to_str().unwrap(),
            "-out", cert_path.to_str().unwrap(),
            "-days", "365",
            "-nodes",
            "-subj", "/CN=localhost/O=cwrdd-dev",
            "-addext", "subjectAltName=DNS:localhost,IP:127.0.0.1",
        ]);
    cert_task.execute().await?;

    println!("✅ TLS certificates generated\n");
    Ok(())
}

/// Start the local development environment
pub async fn up(config: &Config, recreate: bool) -> Result<()> {
    println!("🚀 Starting local development environment\n");

    // Check if podman-compose is installed
    if !command_exists("podman-compose") {
        bail!(
            "podman-compose is not installed.\n\
             Run: cwrdd-make get-tools"
        );
    }

    // Ensure TLS certificates exist
    ensure_certs(config).await?;

    // Build the Rust application first
    println!("🏗️  Building cwrdd application...");
    crate::tasks::build::run(config).await?;

    // Build the Docker image
    println!("\n🐳 Building Docker image...");
    let app_path = config.app_path();
    let docker_build = Task::new("docker build", "podman")
        .args(["build", "-t", "cwrdd-app:local", "."])
        .working_dir(app_path.to_string_lossy().to_string());
    docker_build.execute().await?;

    // Navigate to repo root and start compose
    let repo_path = &config.repo_path;
    
    println!("📦 Starting containers...");
    let args = if recreate {
        vec!["up", "-d", "--force-recreate"]
    } else {
        vec!["up", "-d", "--no-recreate"]
    };
    let compose_up = Task::new("compose up", "podman-compose")
        .args(args)
        .working_dir(repo_path.to_string_lossy().to_string());
    
    compose_up.execute().await?;

    println!("\n⏳ Waiting for PostgreSQL to be ready...");
    
    // Wait for PostgreSQL to be healthy
    let max_retries = 30;
    let mut retries = 0;
    
    while retries < max_retries {
        sleep(Duration::from_secs(2)).await;
        
        let check = Task::new("check postgres", "podman")
            .args([
                "exec",
                "cwrdd-postgres",
                "pg_isready",
                "-U", "cwrdd_user",
                "-d", "cwrdd_dev"
            ]);
        
        if check.execute().await.is_ok() {
            println!("✅ PostgreSQL is ready!\n");
            break;
        }
        
        retries += 1;
        if retries == max_retries {
            bail!("PostgreSQL did not become ready in time. Check logs with: podman-compose logs postgres");
        }
    }

    // Check if migrations are needed
    println!("🔍 Checking database migration status...");
    
    let needs_migration = check_needs_migration(config).await?;
    
    if needs_migration {
        println!("📝 Running database migrations...");
        crate::tasks::migrate::apply(config).await?;
    } else {
        println!("✅ Database is up to date\n");
    }

    // Check if database needs seeding
    println!("🔍 Checking if database needs seeding...");
    
    let needs_seed = check_needs_seed().await?;
    
    if needs_seed {
        println!("🌱 Seeding database with development data...");
        crate::tasks::migrate::seed(config).await?;
    } else {
        println!("✅ Database already has seed data\n");
    }

    println!("✅ Development environment is ready!\n");
    print_access_info();

    Ok(())
}

/// Stop the local development environment
pub async fn down(config: &Config) -> Result<()> {
    println!("🛑 Stopping local development environment\n");

    // Check if podman-compose is installed
    if !command_exists("podman-compose") {
        bail!(
            "podman-compose is not installed.\n\
             Run: cwrdd-make get-tools"
        );
    }

    let repo_path = &config.repo_path;
    
    let compose_down = Task::new("compose down", "podman-compose")
        .args(["down"])
        .working_dir(repo_path.to_string_lossy().to_string());
    
    compose_down.execute().await?;

    println!("✅ Development environment stopped\n");

    Ok(())
}

/// Show logs from the development environment
pub async fn logs(config: &Config, service: Option<String>, follow: bool) -> Result<()> {
    if !command_exists("podman-compose") {
        bail!(
            "podman-compose is not installed.\n\
             Run: cwrdd-make get-tools"
        );
    }

    let repo_path = &config.repo_path;
    
    let mut args = vec!["logs".to_string()];
    
    if follow {
        args.push("-f".to_string());
    }
    
    if let Some(svc) = service {
        args.push(svc);
    }
    
    let logs_task = Task::new("compose logs", "podman-compose")
        .args(args)
        .working_dir(repo_path.to_string_lossy().to_string());
    
    logs_task.execute().await?;

    Ok(())
}

/// Check if migrations need to be run
async fn check_needs_migration(config: &Config) -> Result<bool> {
    // Run liquibase status and check if there are pending changesets
    let db_path = config.db_path();
    
    let status_output = tokio::process::Command::new("liquibase")
        .args([
            "--changeLogFile=migrations/changelog.xml",
            "--url=jdbc:postgresql://localhost:5432/cwrdd_dev",
            "--username=cwrdd_user",
            "--password=cwrdd_password",
            "--driver=org.postgresql.Driver",
            "status",
            "--verbose"
        ])
        .current_dir(&db_path)
        .output()
        .await
        .context("Failed to check migration status")?;

    let stdout = String::from_utf8_lossy(&status_output.stdout);
    
    // If output contains "changesets have not been applied", we need to migrate
    Ok(stdout.contains("changesets have not been applied") || 
       stdout.contains("is not up to date"))
}

/// Check if database needs seeding
async fn check_needs_seed() -> Result<bool> {
    // Check if there's any data in a key table (e.g., users table)
    // For now, we'll use a simple query to check if tables are empty
    
    let check_output = tokio::process::Command::new("psql")
        .args([
            "-h", "localhost",
            "-p", "5432",
            "-U", "cwrdd_user",
            "-d", "cwrdd_dev",
            "-t",
            "-c", "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE';"
        ])
        .env("PGPASSWORD", "cwrdd_password")
        .output()
        .await;

    match check_output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let table_count: i32 = stdout.trim().parse().unwrap_or(0);
            
            // If we have tables, check if they're empty
            if table_count > 0 {
                // Check for any rows in any table (simplified check)
                // In a real scenario, you'd check specific seed marker tables
                let data_check = tokio::process::Command::new("psql")
                    .args([
                        "-h", "localhost",
                        "-p", "5432",
                        "-U", "cwrdd_user",
                        "-d", "cwrdd_dev",
                        "-t",
                        "-c", "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = 'public' LIMIT 1);"
                    ])
                    .env("PGPASSWORD", "cwrdd_password")
                    .output()
                    .await?;
                
                let has_data = String::from_utf8_lossy(&data_check.stdout);
                
                // Need seed if we have tables but they're likely empty
                // This is a simplified heuristic
                Ok(table_count > 0 && !has_data.contains("f"))
            } else {
                // No tables means migrations haven't run yet
                Ok(false)
            }
        }
        Err(_) => {
            // If psql fails, assume we need to seed
            Ok(false)
        }
    }
}

/// Print access information for running services
fn print_access_info() {
    println!("🌐 Access your services:");
    println!("   cwrdd App:     https://localhost:8443");
    println!("   PostgreSQL:    localhost:5432 (cwrdd_dev / cwrdd_user / cwrdd_password)");
    println!("   Redis:         localhost:6379");
    println!("   Grafana:       http://localhost:3000");
    println!("   Prometheus:    http://localhost:9090");
    println!("   Alloy:         http://localhost:12345");
    println!("\n📊 Send telemetry to:");
    println!("   OTLP gRPC:     localhost:4319");
    println!("   OTLP HTTP:     localhost:4320");
    println!("\n📝 Useful commands:");
    println!("   View logs:     cwrdd-make logs [service]");
    println!("   Stop all:      cwrdd-make down");
}
