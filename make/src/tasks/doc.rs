//! Documentation generation for the cwrdd project.
//!
//! Builds rustdoc documentation for all crates in the workspace and optionally
//! opens the documentation in a browser.

use crate::config::Config;
use crate::task::{command_exists, Task};
use anyhow::{bail, Result};

/// Build rustdoc documentation for the entire workspace.
///
/// Generates documentation for:
/// - `cwrdd-make`: The build tool
/// - `cwrdd-app`: The main application
///
/// # Arguments
///
/// * `config` - The cwrdd-make configuration
/// * `open` - Whether to open the documentation in a browser after building
pub async fn run(config: &Config, open: bool) -> Result<()> {
    println!("📚 Building documentation\n");

    if !command_exists("cargo") {
        bail!("cargo is not installed. Please install Rust: https://rustup.rs/");
    }

    // Build docs for cwrdd-make
    println!("Building cwrdd-make documentation...");
    let make_path = config.repo_path.join("make");
    let make_doc = Task::new("cargo doc (make)", "cargo")
        .args(["doc", "--no-deps", "--document-private-items"])
        .working_dir(make_path.to_string_lossy().to_string());
    make_doc.execute().await?;

    // Build docs for the app
    println!("Building cwrdd-app documentation...");
    let app_path = config.app_path();
    let app_doc = Task::new("cargo doc (app)", "cargo")
        .args(["doc", "--no-deps"])
        .working_dir(app_path.to_string_lossy().to_string());
    app_doc.execute().await?;

    let make_doc_path = make_path.join("target/doc/cwrdd_make/index.html");
    let app_doc_path = app_path.join("target/doc/cwrdd_app/index.html");

    println!("✅ Documentation built successfully!\n");
    println!("📖 Documentation locations:");
    println!("   cwrdd-make: file://{}", make_doc_path.display());
    println!("   cwrdd-app:  file://{}", app_doc_path.display());

    if open {
        println!("\n🌐 Opening documentation in browser...");
        let _ = std::process::Command::new("xdg-open")
            .arg(&make_doc_path)
            .spawn();
    }

    Ok(())
}
