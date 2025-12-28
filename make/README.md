# cwrdd-make

Build tool for cwrdd development.

## Installation

### From Source (Recommended for Development)

```bash
cd make
cargo build --release
cargo run --release -- install
```

This will:
1. Build cwrdd-make in release mode
2. Install it to `~/.local/bin/cwrdd-make`
3. Show instructions for adding to PATH if needed

### Verify Installation

```bash
cwrdd-make --help
```

## Usage

### Build and Test

```bash
# Build the cwrdd application
cwrdd-make build

# Run tests
cwrdd-make test
```

### Database Migrations

```bash
# Generate migration from schema diff
cwrdd-make migrate-diff

# Apply pending migrations
cwrdd-make migrate

# Check migration status
cwrdd-make migrate-status

# Rollback last migration
cwrdd-make rollback

# Seed database with development data
cwrdd-make seed
```

### Local Environment

```bash
# Start local services (PostgreSQL, Redis, Observability stack)
cwrdd-make up

# Stop local services
cwrdd-make down

# View logs (all services)
cwrdd-make logs

# View logs for specific service
cwrdd-make logs postgres

# Follow logs in real-time
cwrdd-make logs --follow
cwrdd-make logs -f postgres
```

## Available Commands

Run `cwrdd-make --help` to see all commands:

```
Commands:
  build           Build the application
  test            Run tests
  up              Start local development environment
  down            Stop local development environment
  logs            Show logs from development environment
  migrate-diff    Generate migration from schema diff
  migrate         Apply pending migrations
  migrate-status  Show migration status
  rollback        Rollback last migration
  seed            Seed database with development data
  get-tools       Install development tools (Podman, Liquibase, etc.)
  install         Install cwrdd-make to user's PATH
  help            Print this message or the help of the given subcommand(s)
```

## Configuration

cwrdd-make uses these default paths:
- Repository root: `~/Projects/cwrdd`
- Application: `~/Projects/cwrdd/app`
- Database: `~/Projects/cwrdd/db`

### Custom Repository Path

If you cloned cwrdd to a different location, create a config file:

```bash
# Create config file next to cwrdd-make binary
cat > ~/.local/bin/cwrdd-make-config.toml << EOF
repo_path = "/path/to/your/cwrdd"
EOF
```

## Development

### Building cwrdd-make

```bash
cd make
cargo build
```

### Running without Installing

```bash
cd make
cargo run -- <command>

# Examples:
cargo run -- build
cargo run -- test
cargo run -- migrate --help
```

### Running Tests

```bash
cd make
cargo test
```

## Prerequisites

- **Rust** 1.70+ - Required for building cwrdd-make

### Install All Development Tools

```bash
cd make
cargo run --release -- get-tools
```

This will automatically install all required tools for your operating system:
- **Podman** - Container runtime
- **podman-compose** - Compose support for local environment
- **PostgreSQL** client - For database operations
- **Liquibase** 4.20+ - For migrations
- **cargo-nextest** - For faster test execution

## Troubleshooting

### "cwrdd-make: command not found"

Make sure `~/.local/bin` is in your PATH:

```bash
# Check if it's in PATH
echo $PATH | grep .local/bin

# Add to PATH (bash)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Add to PATH (zsh)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Add to PATH (fish)
fish_add_path ~/.local/bin
```

### "Liquibase not found"

Install Liquibase:

```bash
# macOS
brew install liquibase

# Ubuntu/Debian
sudo apt-get install liquibase

# Or download from https://www.liquibase.org/download
```

### "Repository directory not found"

If you cloned cwrdd to a non-default location, create a config file:

```bash
cat > ~/.local/bin/cwrdd-make-config.toml << EOF
repo_path = "/your/custom/path/to/cwrdd"
EOF
```

## Architecture

cwrdd-make is built with:
- **Rust** - High-performance, safe systems language
- **Tokio** - Async runtime for concurrent task execution
- **Clap** - Command-line argument parsing
- **Liquibase** - Database migration management

See the main [cwrdd README](../README.md) for overall project architecture.
