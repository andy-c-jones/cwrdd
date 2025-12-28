use std::process::Command;

#[test]
fn test_cwrdd_make_build_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "build", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Build the application"));
}

#[test]
fn test_cwrdd_make_test_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "test", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Run tests"));
}

#[test]
fn test_cwrdd_make_general_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Build tool for cwrdd development"));
    assert!(stdout.contains("build"));
    assert!(stdout.contains("test"));
}

#[test]
fn test_cwrdd_make_migrate_commands() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("migrate-diff"));
    assert!(stdout.contains("migrate"));
    assert!(stdout.contains("migrate-status"));
    assert!(stdout.contains("rollback"));
    assert!(stdout.contains("seed"));
}
