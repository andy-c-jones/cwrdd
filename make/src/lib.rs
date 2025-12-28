//! # cwrdd-make
//!
//! A build and development automation tool for the cwrdd project.
//!
//! ## Overview
//!
//! `cwrdd-make` provides a unified command-line interface for common development tasks:
//!
//! - **Building**: Compile the Rust application
//! - **Testing**: Run tests with cargo-nextest or cargo test
//! - **Local Development**: Manage Docker/Podman containers for the full stack
//! - **Database Migrations**: Apply, rollback, and manage database schema changes
//! - **TLS Certificates**: Generate and trust self-signed certificates for local HTTPS
//! - **Tool Installation**: Install required development dependencies
//!
//! ## Quick Start
//!
//! ```bash
//! # Install cwrdd-make to your PATH
//! cargo build --release -p cwrdd-make
//! ./target/release/cwrdd-make install
//!
//! # Start the full development environment
//! cwrdd-make up
//!
//! # Trust the self-signed certificate (optional, requires sudo)
//! cwrdd-make trust-cert
//!
//! # View available commands
//! cwrdd-make --help
//! ```
//!
//! ## Architecture
//!
//! The tool is organized into several modules:
//!
//! - [`config`]: Configuration loading and management
//! - [`task`]: Task execution primitives for running shell commands
//! - [`tasks`]: Individual task implementations (build, test, compose, etc.)
//!
//! ## Configuration
//!
//! `cwrdd-make` looks for a `cwrdd-make-config.toml` file next to the binary.
//! If not found, it uses sensible defaults pointing to `~/Projects/cwrdd`.
//!
//! Example configuration:
//!
//! ```toml
//! repo_path = "/home/user/Projects/cwrdd"
//! ```

pub mod config;
pub mod task;
pub mod tasks;
