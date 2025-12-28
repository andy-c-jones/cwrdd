use crate::config::Config;
use crate::task::Task;
use anyhow::{bail, Result};

/// Trust the self-signed certificate locally (Linux only)
pub async fn trust(config: &Config) -> Result<()> {
    println!("🔐 Trusting self-signed certificate locally\n");

    let cert_path = config.repo_path.join("config/certs/cert.pem");

    if !cert_path.exists() {
        bail!(
            "Certificate not found at {:?}\n\
             Run 'cwrdd-make up' first to generate certificates.",
            cert_path
        );
    }

    let dest_path = "/usr/local/share/ca-certificates/cwrdd-dev.crt";

    println!("📋 Installing certificate to system trust store...");
    println!("   This requires sudo access.\n");

    // Copy cert to ca-certificates directory
    let copy_task = Task::new("copy cert", "sudo")
        .args(["cp", cert_path.to_str().unwrap(), dest_path]);
    copy_task.execute().await?;

    // Update ca-certificates
    let update_task = Task::new("update ca-certificates", "sudo")
        .args(["update-ca-certificates"]);
    update_task.execute().await?;

    println!("\n✅ Certificate trusted system-wide!");
    println!("\n⚠️  Note: Some browsers maintain their own certificate stores.");
    println!("   For Firefox: Settings > Privacy & Security > Certificates > View Certificates > Import");
    println!("   For Chrome/Chromium: The system store should work, restart browser if needed.");
    println!("\n   Certificate location: {:?}", cert_path);

    Ok(())
}

/// Remove trust for the self-signed certificate
pub async fn untrust(_config: &Config) -> Result<()> {
    println!("🔓 Removing trust for self-signed certificate\n");

    let dest_path = "/usr/local/share/ca-certificates/cwrdd-dev.crt";

    if std::path::Path::new(dest_path).exists() {
        let remove_task = Task::new("remove cert", "sudo")
            .args(["rm", dest_path]);
        remove_task.execute().await?;

        let update_task = Task::new("update ca-certificates", "sudo")
            .args(["update-ca-certificates", "--fresh"]);
        update_task.execute().await?;

        println!("✅ Certificate removed from system trust store");
    } else {
        println!("ℹ️  Certificate was not installed in system trust store");
    }

    Ok(())
}
