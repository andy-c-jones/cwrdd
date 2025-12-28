use crate::config::Config;
use crate::task::{command_exists, run_tasks, Task};
use anyhow::{bail, Result};

/// Install development tools (Podman, Liquibase, PostgreSQL client, cargo-nextest)
pub async fn run(_config: &Config) -> Result<()> {
    println!("🔧 Installing development tools\n");

    // Detect OS
    let os = detect_os()?;
    println!("Detected OS: {}\n", os);

    match os.as_str() {
        "ubuntu" | "debian" => install_ubuntu().await?,
        "macos" => install_macos().await?,
        _ => {
            bail!("Unsupported OS: {}. Please install tools manually:\n\
                   - Podman: https://podman.io/getting-started/installation\n\
                   - podman-compose: https://github.com/containers/podman-compose\n\
                   - Liquibase: https://www.liquibase.org/download\n\
                   - PostgreSQL client: psql command\n\
                   - cargo-nextest: cargo install cargo-nextest --locked", os);
        }
    }

    Ok(())
}

/// Detect operating system
fn detect_os() -> Result<String> {
    if cfg!(target_os = "linux") {
        // Check if it's Ubuntu/Debian
        if std::path::Path::new("/etc/os-release").exists() {
            let content = std::fs::read_to_string("/etc/os-release")?;
            if content.contains("ubuntu") || content.contains("Ubuntu") {
                return Ok("ubuntu".to_string());
            }
            if content.contains("debian") || content.contains("Debian") {
                return Ok("debian".to_string());
            }
        }
        Ok("linux".to_string())
    } else if cfg!(target_os = "macos") {
        Ok("macos".to_string())
    } else {
        Ok("unknown".to_string())
    }
}

/// Install tools on Ubuntu/Debian
async fn install_ubuntu() -> Result<()> {
    println!("Installing tools for Ubuntu/Debian...\n");

    let mut tasks = vec![];

    // Check what needs to be installed
    let needs_podman = !command_exists("podman");
    let needs_podman_compose = !command_exists("podman-compose");
    let needs_liquibase = !command_exists("liquibase");
    let needs_psql = !command_exists("psql");
    let needs_nextest = !command_exists("cargo-nextest");

    if !needs_podman && !needs_podman_compose && !needs_liquibase && !needs_psql && !needs_nextest {
        println!("✅ All tools already installed!");
        return Ok(());
    }

    // Update apt if we need to install anything
    if needs_podman || needs_podman_compose || needs_liquibase || needs_psql {
        println!("📦 Updating package lists...");
        tasks.push(
            Task::new("apt update", "sudo")
                .args(["apt", "update"])
        );
    }

    // Install Podman
    if needs_podman {
        println!("🐳 Installing Podman...");
        tasks.push(
            Task::new("install podman", "sudo")
                .args(["apt", "install", "-y", "podman"])
        );
    } else {
        println!("✓ Podman already installed");
    }

    // Install podman-compose
    if needs_podman_compose {
        println!("🐳 Installing podman-compose...");
        tasks.push(
            Task::new("install podman-compose", "sudo")
                .args(["apt", "install", "-y", "podman-compose"])
        );
    } else {
        println!("✓ podman-compose already installed");
    }

    // Install PostgreSQL client
    if needs_psql {
        println!("🐘 Installing PostgreSQL client...");
        tasks.push(
            Task::new("install postgresql-client", "sudo")
                .args(["apt", "install", "-y", "postgresql-client"])
        );
    } else {
        println!("✓ PostgreSQL client already installed");
    }

    // Install Liquibase
    if needs_liquibase {
        println!("💧 Installing Liquibase...");
        
        // Install Java if needed
        if !command_exists("java") {
            tasks.push(
                Task::new("install java", "sudo")
                    .args(["apt", "install", "-y", "default-jre"])
            );
        }

        // Download and install Liquibase
        let liquibase_version = "4.29.2";
        let liquibase_url = format!(
            "https://github.com/liquibase/liquibase/releases/download/v{}/liquibase-{}.tar.gz",
            liquibase_version, liquibase_version
        );

        tasks.push(
            Task::new("download liquibase", "wget")
                .args(["-q", &liquibase_url, "-O", "/tmp/liquibase.tar.gz"])
        );

        tasks.push(
            Task::new("create liquibase dir", "sudo")
                .args(["mkdir", "-p", "/opt/liquibase"])
        );

        tasks.push(
            Task::new("extract liquibase", "sudo")
                .args(["tar", "-xzf", "/tmp/liquibase.tar.gz", "-C", "/opt/liquibase"])
        );

        tasks.push(
            Task::new("symlink liquibase", "sudo")
                .args(["ln", "-sf", "/opt/liquibase/liquibase", "/usr/local/bin/liquibase"])
        );

        tasks.push(
            Task::new("chmod liquibase", "sudo")
                .args(["chmod", "+x", "/usr/local/bin/liquibase"])
        );

        tasks.push(
            Task::new("cleanup", "rm")
                .args(["/tmp/liquibase.tar.gz"])
        );
    } else {
        println!("✓ Liquibase already installed");
    }

    // Run apt tasks
    if !tasks.is_empty() {
        run_tasks(tasks).await?;
    }

    // Install cargo-nextest (doesn't need sudo)
    if needs_nextest {
        println!("\n🧪 Installing cargo-nextest...");
        let nextest_tasks = vec![
            Task::new("install cargo-nextest", "cargo")
                .args(["install", "cargo-nextest", "--locked"])
        ];
        run_tasks(nextest_tasks).await?;
    } else {
        println!("✓ cargo-nextest already installed");
    }

    // Enable podman socket for current user
    if needs_podman {
        println!("\n🔌 Configuring Podman socket...");
        let socket_tasks = vec![
            Task::new("enable podman socket", "systemctl")
                .args(["--user", "enable", "podman.socket"]),
            Task::new("start podman socket", "systemctl")
                .args(["--user", "start", "podman.socket"]),
        ];
        
        // These might fail if not in a systemd environment, that's ok
        let _ = run_tasks(socket_tasks).await;
    }

    println!("\n✅ All tools installed successfully!\n");
    print_versions();

    Ok(())
}

/// Install tools on macOS
async fn install_macos() -> Result<()> {
    println!("Installing tools for macOS...\n");

    if !command_exists("brew") {
        bail!("Homebrew is required. Install it from: https://brew.sh");
    }

    let mut tasks = vec![];

    // Check what needs to be installed
    let needs_podman = !command_exists("podman");
    let needs_podman_compose = !command_exists("podman-compose");
    let needs_liquibase = !command_exists("liquibase");
    let needs_psql = !command_exists("psql");
    let needs_nextest = !command_exists("cargo-nextest");

    if !needs_podman && !needs_podman_compose && !needs_liquibase && !needs_psql && !needs_nextest {
        println!("✅ All tools already installed!");
        return Ok(());
    }

    // Install via Homebrew
    if needs_podman {
        println!("🐳 Installing Podman...");
        tasks.push(Task::new("brew install podman", "brew").args(["install", "podman"]));
    } else {
        println!("✓ Podman already installed");
    }

    if needs_podman_compose {
        println!("🐳 Installing podman-compose...");
        tasks.push(Task::new("brew install podman-compose", "brew").args(["install", "podman-compose"]));
    } else {
        println!("✓ podman-compose already installed");
    }

    if needs_liquibase {
        println!("💧 Installing Liquibase...");
        tasks.push(Task::new("brew install liquibase", "brew").args(["install", "liquibase"]));
    } else {
        println!("✓ Liquibase already installed");
    }

    if needs_psql {
        println!("🐘 Installing PostgreSQL client...");
        tasks.push(
            Task::new("brew install postgresql", "brew").args(["install", "postgresql@16"])
        );
    } else {
        println!("✓ PostgreSQL client already installed");
    }

    if !tasks.is_empty() {
        run_tasks(tasks).await?;
    }

    // Install cargo-nextest
    if needs_nextest {
        println!("\n🧪 Installing cargo-nextest...");
        let nextest_tasks = vec![
            Task::new("install cargo-nextest", "cargo")
                .args(["install", "cargo-nextest", "--locked"])
        ];
        run_tasks(nextest_tasks).await?;
    } else {
        println!("✓ cargo-nextest already installed");
    }

    println!("\n✅ All tools installed successfully!\n");
    print_versions();

    Ok(())
}

/// Print versions of installed tools
fn print_versions() {
    println!("Installed versions:");
    
    if command_exists("podman") {
        if let Ok(output) = std::process::Command::new("podman").arg("--version").output() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  Podman: {}", version.trim());
        }
    }
    
    if command_exists("podman-compose") {
        if let Ok(output) = std::process::Command::new("podman-compose").arg("--version").output() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  podman-compose: {}", version.trim());
        }
    }
    
    if command_exists("liquibase") {
        if let Ok(output) = std::process::Command::new("liquibase").arg("--version").output() {
            let version = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = version.lines().next() {
                println!("  Liquibase: {}", line.trim());
            }
        }
    }
    
    if command_exists("psql") {
        if let Ok(output) = std::process::Command::new("psql").arg("--version").output() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  PostgreSQL: {}", version.trim());
        }
    }
    
    if command_exists("cargo-nextest") {
        if let Ok(output) = std::process::Command::new("cargo-nextest").arg("--version").output() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("  cargo-nextest: {}", version.trim());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_os() {
        let result = detect_os();
        assert!(result.is_ok());
        let os = result.unwrap();
        assert!(
            os == "ubuntu" || os == "debian" || os == "macos" || os == "linux" || os == "unknown"
        );
    }
}
