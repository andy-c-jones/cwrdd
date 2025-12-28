#!/bin/bash
# Minimal setup script for cwrdd development on Ubuntu-based distributions
# This installs just enough to build cwrdd-make
# Run 'cwrdd-make get-tools' after this to install the rest

set -e

echo "🚀 Setting up cwrdd development environment (Ubuntu)"
echo ""
echo "This will install:"
echo "  - build-essential (gcc, make, etc.)"
echo "  - curl, wget, git"
echo "  - rustup (Rust toolchain installer)"
echo ""
read -p "Continue? (y/N) " -n 1 -r
echo
[[ ! $REPLY =~ ^[Yy]$ ]] && exit 0

# Update and install build essentials and rustup
echo "📦 Installing build essentials and rustup..."
sudo apt update
sudo apt install -y build-essential curl wget git ca-certificates rustup

# Set up Rust stable toolchain
echo "🦀 Installing Rust stable toolchain..."
rustup default stable

# Add cargo to PATH in shell config
SHELL_RC="$HOME/.bashrc"
[ -f "$HOME/.zshrc" ] && SHELL_RC="$HOME/.zshrc"

if ! grep -q ".cargo/env" "$SHELL_RC" 2>/dev/null; then
    echo '' >> "$SHELL_RC"
    echo '# Rust (cargo)' >> "$SHELL_RC"
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$SHELL_RC"
fi

if ! grep -q ".local/bin" "$SHELL_RC" 2>/dev/null; then
    echo '# Local binaries' >> "$SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
fi

echo ""
echo "✅ Basic setup complete!"
echo ""
echo "Installed:"
echo "  Rust: $(rustc --version)"
echo ""
echo "Next steps:"
echo "  1. source ~/.bashrc  (or open new terminal)"
echo "  2. cd make"
echo "  3. cargo build --release"
echo "  4. cargo run --release -- install"
echo "  5. cwrdd-make get-tools  (installs Podman, Liquibase, etc.)"
echo ""
