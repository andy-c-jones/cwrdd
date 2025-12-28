mod config;
mod task;
mod tasks;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cwrdd-make")]
#[command(about = "Build tool for cwrdd development", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the application
    Build,
    /// Run tests
    Test,
    /// Start local development environment
    Up {
        /// Force recreate containers even if already running
        #[arg(long)]
        recreate: bool,
    },
    /// Stop local development environment
    Down,
    /// Show logs from development environment
    Logs {
        /// Specific service to show logs for
        service: Option<String>,
        /// Follow log output
        #[arg(short, long)]
        follow: bool,
    },
    /// Generate migration from schema diff
    MigrateDiff,
    /// Apply pending migrations
    Migrate,
    /// Show migration status
    MigrateStatus,
    /// Rollback last migration
    Rollback,
    /// Seed database with development data
    Seed,
    /// Install cwrdd-make to user's PATH
    Install,
    /// Install development tools (Podman, Liquibase, etc.)
    GetTools,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = config::Config::load()?;

    // Execute command
    match cli.command {
        Commands::Build => tasks::build::run(&config).await?,
        Commands::Test => tasks::test::run(&config).await?,
        Commands::Up { recreate } => tasks::compose::up(&config, recreate).await?,
        Commands::Down => tasks::compose::down(&config).await?,
        Commands::Logs { service, follow } => tasks::compose::logs(&config, service, follow).await?,
        Commands::MigrateDiff => tasks::migrate::diff(&config).await?,
        Commands::Migrate => tasks::migrate::apply(&config).await?,
        Commands::MigrateStatus => tasks::migrate::status(&config).await?,
        Commands::Rollback => tasks::migrate::rollback(&config).await?,
        Commands::Seed => tasks::migrate::seed(&config).await?,
        Commands::Install => tasks::install::run(&config).await?,
        Commands::GetTools => tasks::get_tools::run(&config).await?,
    }

    Ok(())
}
