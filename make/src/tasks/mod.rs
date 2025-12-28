//! Task implementations for cwrdd-make commands.
//!
//! Each submodule implements one or more cwrdd-make commands:
//!
//! - [`build`]: Build the Rust application
//! - [`certs`]: TLS certificate management (trust/untrust)
//! - [`compose`]: Docker/Podman compose management (up/down/logs)
//! - [`doc`]: Build and view rustdoc documentation
//! - [`get_tools`]: Install development dependencies
//! - [`install`]: Install cwrdd-make to user's PATH
//! - [`migrate`]: Database migration commands
//! - [`test`]: Run tests

pub mod build;
pub mod certs;
pub mod compose;
pub mod doc;
pub mod get_tools;
pub mod install;
pub mod migrate;
pub mod test;
