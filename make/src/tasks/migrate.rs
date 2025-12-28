use crate::config::Config;
use crate::task::{command_exists, run_tasks, verify_directory, Task};
use anyhow::{bail, Context, Result};
use std::fs;

const LIQUIBASE_VERSION: &str = "4.29.2";
const LIQUIBASE_URL: &str =
    "https://github.com/liquibase/liquibase/releases/download/v4.29.2/liquibase-4.29.2.tar.gz";

/// Check if liquibase is installed
fn check_liquibase() -> Result<bool> {
    Ok(command_exists("liquibase"))
}

/// Get instructions for installing liquibase
fn liquibase_install_instructions() -> String {
    format!(
        r#"
Liquibase is not installed. Please install it:

Option 1 - Download from official site:
  wget {url}
  tar -xzf liquibase-{version}.tar.gz
  sudo mv liquibase /usr/local/bin/
  
Option 2 - Use package manager:
  # On macOS
  brew install liquibase
  
  # On Ubuntu/Debian
  sudo apt-get install liquibase
  
  # On Fedora
  sudo dnf install liquibase

Option 3 - Use SDKMAN:
  sdk install liquibase

After installation, run this command again.
"#,
        url = LIQUIBASE_URL,
        version = LIQUIBASE_VERSION
    )
}

/// Generate migration diff from schema files
pub async fn diff(config: &Config) -> Result<()> {
    println!("🔍 Generating migration diff from schema files\n");

    // Check liquibase
    if !check_liquibase()? {
        bail!("{}", liquibase_install_instructions());
    }

    // Verify db directory exists
    let db_path = config.db_path();
    verify_directory(&db_path)?;

    let schema_path = db_path.join("schema");
    verify_directory(&schema_path)?;

    println!("Database path: {}", db_path.display());
    println!("Schema path: {}\n", schema_path.display());

    // Check if there are any schema files
    let schema_files: Vec<_> = fs::read_dir(&schema_path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "sql")
                .unwrap_or(false)
        })
        .collect();

    if schema_files.is_empty() {
        bail!("No schema files found in {}", schema_path.display());
    }

    println!("Found {} schema file(s):", schema_files.len());
    for file in &schema_files {
        println!("  - {}", file.file_name().to_string_lossy());
    }
    println!();

    // Create timestamp for changeset
    let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let changeset_file = format!("{}-generated.xml", timestamp);

    println!("⚠️  Note: This command requires a reference database.");
    println!("   You'll need to:");
    println!("   1. Have a PostgreSQL database running");
    println!("   2. Configure connection in db/liquibase.properties");
    println!("   3. Run this command to generate diff\n");

    // For now, we'll create a task that runs liquibase diffChangeLog
    // This will compare the actual database with the schema files
    let tasks = vec![Task::new("liquibase diffChangeLog", "liquibase")
        .args([
            "diffChangeLog",
            "--changeLogFile",
            &format!("migrations/{}", changeset_file),
        ])
        .working_dir(db_path.to_string_lossy().to_string())];

    match run_tasks(tasks).await {
        Ok(_) => {
            println!("✅ Generated migration: migrations/{}", changeset_file);
            println!("\nNext steps:");
            println!("  1. Review the generated changeset:");
            println!("     cat db/migrations/{}", changeset_file);
            println!("  2. Apply the migration:");
            println!("     cwrdd-make migrate");
            println!("  3. Commit both schema and migration:");
            println!("     git add db/schema/ db/migrations/");
            Ok(())
        }
        Err(e) => {
            eprintln!("\n❌ Error generating diff:");
            eprintln!("   {}", e);
            eprintln!("\nCommon issues:");
            eprintln!("  - Database not running or not accessible");
            eprintln!("  - Connection settings incorrect in liquibase.properties");
            eprintln!("  - JDBC driver not available");
            bail!("Migration diff failed")
        }
    }
}

/// Apply pending migrations
pub async fn apply(config: &Config) -> Result<()> {
    println!("🚀 Applying database migrations\n");

    // Check liquibase
    if !check_liquibase()? {
        bail!("{}", liquibase_install_instructions());
    }

    // Verify db directory exists
    let db_path = config.db_path();
    verify_directory(&db_path)?;

    println!("Database path: {}\n", db_path.display());

    // Run liquibase update
    let tasks = vec![Task::new("liquibase update", "liquibase")
        .args(["update"])
        .working_dir(db_path.to_string_lossy().to_string())];

    run_tasks(tasks).await?;

    println!("✅ Migrations applied successfully!");
    Ok(())
}

/// Show migration status
pub async fn status(config: &Config) -> Result<()> {
    println!("📊 Checking migration status\n");

    // Check liquibase
    if !check_liquibase()? {
        bail!("{}", liquibase_install_instructions());
    }

    // Verify db directory exists
    let db_path = config.db_path();
    verify_directory(&db_path)?;

    println!("Database path: {}\n", db_path.display());

    // Run liquibase status
    let tasks = vec![Task::new("liquibase status", "liquibase")
        .args(["status", "--verbose"])
        .working_dir(db_path.to_string_lossy().to_string())];

    run_tasks(tasks).await?;

    Ok(())
}

/// Rollback last migration
pub async fn rollback(config: &Config) -> Result<()> {
    println!("⏪ Rolling back last migration\n");

    // Check liquibase
    if !check_liquibase()? {
        bail!("{}", liquibase_install_instructions());
    }

    // Verify db directory exists
    let db_path = config.db_path();
    verify_directory(&db_path)?;

    println!("Database path: {}\n", db_path.display());
    println!("⚠️  Warning: This will rollback the most recent changeset.");
    println!("   Make sure this is what you want to do!\n");

    // Run liquibase rollback
    let tasks = vec![Task::new("liquibase rollback", "liquibase")
        .args(["rollbackCount", "1"])
        .working_dir(db_path.to_string_lossy().to_string())];

    run_tasks(tasks).await?;

    println!("✅ Rollback completed successfully!");
    Ok(())
}

/// Seed database with development data
pub async fn seed(config: &Config) -> Result<()> {
    println!("🌱 Seeding database with development data\n");

    // Verify db directory exists
    let db_path = config.db_path();
    verify_directory(&db_path)?;

    let seed_file = db_path.join("scripts/seed-dev-data.sql");
    if !seed_file.exists() {
        bail!("Seed file not found: {}", seed_file.display());
    }

    println!("Database path: {}", db_path.display());
    println!("Seed file: {}\n", seed_file.display());

    // Read database connection from liquibase.properties
    let props_file = db_path.join("liquibase.properties");
    let props_content =
        fs::read_to_string(&props_file).context("Failed to read liquibase.properties")?;

    // Parse connection details (simple parsing)
    let url = extract_property(&props_content, "url")?;
    let username = extract_property(&props_content, "username")?;
    let password = extract_property(&props_content, "password")?;

    // Extract database name from JDBC URL
    let db_name = extract_db_name(&url)?;

    println!("Connecting to database: {}", db_name);
    println!("User: {}\n", username);

    // Use psql to run the seed file
    let tasks = vec![Task::new("psql seed", "psql")
        .args([
            "-h",
            "localhost",
            "-U",
            &username,
            "-d",
            &db_name,
            "-f",
            seed_file.to_str().unwrap(),
        ])
        .env("PGPASSWORD", &password)
        .working_dir(db_path.to_string_lossy().to_string())];

    run_tasks(tasks).await?;

    println!("✅ Database seeded successfully!");
    Ok(())
}

/// Extract property value from liquibase.properties
fn extract_property(content: &str, key: &str) -> Result<String> {
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with(key) {
            // Split on first : and take everything after
            if let Some(colon_pos) = line.find(':') {
                let value = &line[colon_pos + 1..];
                return Ok(value.trim().to_string());
            }
        }
    }
    bail!("Property '{}' not found in liquibase.properties", key)
}

/// Extract database name from JDBC URL
fn extract_db_name(url: &str) -> Result<String> {
    // JDBC URL format: jdbc:postgresql://localhost:5432/dbname
    if let Some(last_part) = url.split('/').last() {
        let db_name = last_part.split('?').next().unwrap_or(last_part);
        return Ok(db_name.to_string());
    }
    bail!("Could not extract database name from URL: {}", url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_db_name() {
        let url = "jdbc:postgresql://localhost:5432/cwrdd_dev";
        let result = extract_db_name(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "cwrdd_dev");
    }

    #[test]
    fn test_extract_db_name_with_params() {
        let url = "jdbc:postgresql://localhost:5432/cwrdd_dev?ssl=true";
        let result = extract_db_name(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "cwrdd_dev");
    }

    #[test]
    fn test_extract_property() {
        let content = "url: jdbc:postgresql://localhost:5432/cwrdd_dev\nusername: cwrdd_user\npassword: cwrdd_password";

        let url = extract_property(content, "url");
        assert!(url.is_ok(), "Failed to extract url: {:?}", url);
        let url_value = url.unwrap();
        assert!(
            url_value.contains("cwrdd_dev"),
            "URL '{}' doesn't contain 'cwrdd_dev'",
            url_value
        );

        let username = extract_property(content, "username");
        assert!(username.is_ok());
        assert_eq!(username.unwrap(), "cwrdd_user");
    }
}
