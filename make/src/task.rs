use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Represents a task that can be executed
pub struct Task {
    name: String,
    command: String,
    args: Vec<String>,
    working_dir: Option<String>,
}

impl Task {
    /// Create a new task
    pub fn new(name: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            args: Vec::new(),
            working_dir: None,
        }
    }

    /// Add arguments to the task
    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.args = args.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set the working directory for the task
    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }

    /// Execute the task
    pub async fn execute(&self) -> Result<()> {
        println!("🔧 Running: {}", self.name);
        println!("   Command: {} {}", self.command, self.args.join(" "));

        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);

        if let Some(ref dir) = self.working_dir {
            cmd.current_dir(dir);
            println!("   Working directory: {}", dir);
        }

        // Capture stdout and stderr
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd
            .spawn()
            .context(format!("Failed to spawn command: {}", self.command))?;

        // Stream stdout
        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();

            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    println!("   {}", line);
                }
            });
        }

        // Stream stderr
        if let Some(stderr) = child.stderr.take() {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();

            tokio::spawn(async move {
                while let Ok(Some(line)) = lines.next_line().await {
                    eprintln!("   {}", line);
                }
            });
        }

        // Wait for command to complete
        let status = child
            .wait()
            .await
            .context("Failed to wait for command")?;

        if !status.success() {
            bail!(
                "Task '{}' failed with exit code: {}",
                self.name,
                status.code().unwrap_or(-1)
            );
        }

        println!("✅ {}: completed successfully\n", self.name);
        Ok(())
    }
}

/// Run a sequence of tasks
pub async fn run_tasks(tasks: Vec<Task>) -> Result<()> {
    for task in tasks {
        task.execute().await?;
    }
    Ok(())
}

/// Check if a command exists on the system
pub fn command_exists(command: &str) -> bool {
    std::process::Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Verify working directory exists
pub fn verify_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("Directory does not exist: {}", path.display());
    }
    if !path.is_dir() {
        bail!("Path is not a directory: {}", path.display());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_task() {
        let task = Task::new("test echo", "echo").args(["hello", "world"]);
        let result = task.execute().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_command_exists() {
        assert!(command_exists("echo"));
        assert!(command_exists("ls"));
        assert!(!command_exists("nonexistent-command-12345"));
    }

    #[test]
    fn test_verify_directory() {
        // Test with /tmp which should exist
        let result = verify_directory(Path::new("/tmp"));
        assert!(result.is_ok());

        // Test with non-existent directory
        let result = verify_directory(Path::new("/nonexistent-dir-12345"));
        assert!(result.is_err());
    }
}
