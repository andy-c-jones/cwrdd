# Development Scripts

This directory contains scripts to help set up the cwrdd development environment.

## setup-ubuntu.sh

Minimal setup script for Ubuntu-based distributions.

**What it installs:**
- Build essentials (gcc, make, git, curl, wget)
- Rust (via rustup)
- Shell configuration

**Usage:**

```bash
./scripts/setup-ubuntu.sh
```

**After running:**

```bash
# Reload your shell
source ~/.bashrc

# Build and install cwrdd-make
cd make
cargo build --release
cargo run --release -- install

# Install additional tools (Podman, Liquibase, etc.)
cwrdd-make get-tools
```

## Why Two Steps?

1. **Minimal setup script** - Just enough to build cwrdd-make (Rust + build tools)
2. **cwrdd-make get-tools** - Installs everything else (Podman, Liquibase, PostgreSQL client, etc.)

This approach:
- Keeps the shell script simple and fast
- Uses Rust for the complex installation logic
- Makes it easier to maintain and test
- Provides better error handling and cross-platform support

## Manual Installation

If you prefer not to use the script:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install build tools
sudo apt install build-essential git

# Build cwrdd-make
cd make && cargo build --release
cargo run --release -- install

# Get other tools
cwrdd-make get-tools
```
