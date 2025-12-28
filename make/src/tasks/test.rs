use crate::config::Config;
use crate::task::{command_exists, run_tasks, verify_directory, Task};
use anyhow::{bail, Result};

/// Run the test task
pub async fn run(config: &Config) -> Result<()> {
    println!("🧪 Running tests for cwrdd application\n");

    // Verify cargo exists
    if !command_exists("cargo") {
        bail!("cargo is not installed. Please install Rust: https://rustup.rs/");
    }

    // Check if nextest is available
    let use_nextest = command_exists("cargo-nextest");

    if !use_nextest {
        println!("⚠️  cargo-nextest not found. Using cargo test instead.");
        println!("   Install nextest for better performance: cargo install cargo-nextest --locked\n");
    }

    // Verify app directory exists
    let app_path = config.app_path();
    verify_directory(&app_path)?;

    println!("Repository path: {}", config.repo_path.display());
    println!("App path: {}\n", app_path.display());

    // Create test tasks
    let tasks = if use_nextest {
        vec![
            Task::new("cargo nextest run", "cargo")
                .args(["nextest", "run", "--workspace"])
                .working_dir(app_path.to_string_lossy().to_string()),
        ]
    } else {
        vec![
            Task::new("cargo test", "cargo")
                .args(["test", "--workspace"])
                .working_dir(app_path.to_string_lossy().to_string()),
        ]
    };

    // Run tasks
    run_tasks(tasks).await?;

    println!("🎉 All tests passed!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_cargo_exists() {
        assert!(command_exists("cargo"));
    }

    #[tokio::test]
    async fn test_with_nonexistent_directory() {
        let config = Config {
            repo_path: PathBuf::from("/nonexistent-directory-12345"),
        };

        let result = run(&config).await;
        assert!(result.is_err());
    }
}
