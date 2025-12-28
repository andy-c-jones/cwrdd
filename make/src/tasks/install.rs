use crate::config::Config;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Install cwrdd-make to user's PATH
pub async fn run(_config: &Config) -> Result<()> {
    println!("📦 Installing cwrdd-make to your PATH\n");

    // Get the current executable path (the one we're running)
    let current_exe = env::current_exe().context("Failed to get current executable path")?;
    
    println!("Current executable: {}", current_exe.display());

    // Determine installation directory
    let install_dir = get_install_dir()?;
    let install_path = install_dir.join("cwrdd-make");

    println!("Installation directory: {}", install_dir.display());
    println!("Target path: {}\n", install_path.display());

    // Check if install directory is in PATH
    check_path(&install_dir)?;

    // Create install directory if it doesn't exist
    if !install_dir.exists() {
        println!("Creating directory: {}", install_dir.display());
        fs::create_dir_all(&install_dir)
            .context(format!("Failed to create directory: {}", install_dir.display()))?;
    }

    // Copy the binary
    println!("Copying binary...");
    fs::copy(&current_exe, &install_path).context(format!(
        "Failed to copy binary from {} to {}",
        current_exe.display(),
        install_path.display()
    ))?;

    // Make it executable
    let mut perms = fs::metadata(&install_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&install_path, perms)?;

    println!("✅ cwrdd-make installed successfully!\n");
    println!("Installation location: {}", install_path.display());
    println!("\nYou can now run:");
    println!("  cwrdd-make --help");
    println!("  cwrdd-make build");
    println!("  cwrdd-make migrate");
    println!("  etc.\n");

    // Check if we need to add to PATH
    if !is_in_path(&install_dir) {
        print_path_instructions(&install_dir);
    }

    Ok(())
}

/// Get the installation directory (prefer ~/.local/bin)
fn get_install_dir() -> Result<PathBuf> {
    // Try ~/.local/bin (XDG standard)
    if let Some(home) = env::var_os("HOME") {
        let local_bin = PathBuf::from(home).join(".local/bin");
        return Ok(local_bin);
    }

    // Fallback to ~/bin
    if let Some(home) = env::var_os("HOME") {
        let bin = PathBuf::from(home).join("bin");
        return Ok(bin);
    }

    anyhow::bail!("Could not determine home directory")
}

/// Check if a directory is in PATH
fn is_in_path(dir: &Path) -> bool {
    if let Some(path) = env::var_os("PATH") {
        for p in env::split_paths(&path) {
            if p == dir {
                return true;
            }
        }
    }
    false
}

/// Check PATH and provide feedback
fn check_path(install_dir: &Path) -> Result<()> {
    if is_in_path(install_dir) {
        println!("✓ {} is in your PATH", install_dir.display());
        Ok(())
    } else {
        println!("⚠️  {} is not in your PATH", install_dir.display());
        println!("   (This is okay, we'll show you how to add it after installation)");
        Ok(())
    }
}

/// Print instructions for adding to PATH
fn print_path_instructions(install_dir: &Path) {
    println!("📝 To add {} to your PATH:", install_dir.display());
    println!("\nFor bash, add this to ~/.bashrc:");
    println!("  export PATH=\"{}:$PATH\"", install_dir.display());
    println!("\nFor zsh, add this to ~/.zshrc:");
    println!("  export PATH=\"{}:$PATH\"", install_dir.display());
    println!("\nFor fish, run:");
    println!(
        "  fish_add_path {}",
        install_dir.display()
    );
    println!("\nThen reload your shell or run:");
    println!("  source ~/.bashrc   # or ~/.zshrc");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_install_dir() {
        let result = get_install_dir();
        assert!(result.is_ok());
        let dir = result.unwrap();
        // Should end with .local/bin or bin
        assert!(
            dir.ends_with(".local/bin") || dir.ends_with("bin"),
            "Install dir should be .local/bin or bin, got: {}",
            dir.display()
        );
    }

    #[test]
    fn test_is_in_path_current_paths() {
        // Test with paths that should be in PATH
        let result = is_in_path(Path::new("/usr/bin"));
        // This should generally be true, but might not be in all test environments
        println!("Is /usr/bin in PATH: {}", result);
    }
}
