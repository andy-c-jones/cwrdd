use crate::config::Config;
use crate::task::{command_exists, run_tasks, verify_directory, Task};
use anyhow::{bail, Result};

/// Run the build task
pub async fn run(config: &Config) -> Result<()> {
    println!("🏗️  Building cwrdd application\n");

    // Verify cargo exists
    if !command_exists("cargo") {
        bail!("cargo is not installed. Please install Rust: https://rustup.rs/");
    }

    // Verify app directory exists
    let app_path = config.app_path();
    verify_directory(&app_path)?;

    println!("Repository path: {}", config.repo_path.display());
    println!("App path: {}\n", app_path.display());

    // Create build tasks
    let tasks = vec![
        Task::new("cargo build", "cargo")
            .args(["build", "--workspace"])
            .working_dir(app_path.to_string_lossy().to_string()),
    ];

    // Run tasks
    run_tasks(tasks).await?;

    println!("🎉 Build completed successfully!");
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
    async fn test_build_with_nonexistent_directory() {
        let config = Config {
            repo_path: PathBuf::from("/nonexistent-directory-12345"),
        };

        let result = run(&config).await;
        assert!(result.is_err());
    }
}
