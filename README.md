# cwrdd

**Pronunciation:** *koor-th* (for English speakers)

## Project Philosophy

cwrdd is built on the fundamental belief that human connection and democratic participation require safe, secure spaces for people to share ideas and arrange in-person meetings.

### Core Principles

#### Privacy and Security

- **People deserve a safe and secure place to share ideas and arrange in-person meetings.** Privacy and security are not optional features—they are fundamental rights that enable free expression and democratic participation.

- **People should be allowed complete privacy in private channels but should not be able to hide behind anonymity in public channels and groups.** Privacy and accountability serve different purposes: private conversations deserve absolute privacy, while public discourse benefits from accountability.

- **It should be exceptionally clear to users what is being done 'publicly' vs 'privately'.** Users must always understand the context of their communications and the implications of their actions.

- **cwrdd will never provide a mechanism to decrypt private user data.** We will not launch in jurisdictions that have laws requiring backdoors or access to encrypted communications. This is a non-negotiable technical and ethical stance.

#### Human Connection

- **Human interaction is more important than ever in an era dominated by technology.** While technology connects us digitally, it should facilitate and enhance real-world human connection, not replace it.

- **Real in-person interactions are critical for democracy.** Democratic participation requires the ability to organize, meet, and discuss ideas face-to-face. Technology should empower these interactions, not hinder them.

#### Trust and Safety

- **Users must be at least 16 years old to create an account.** We protect young people by enforcing age requirements and will comply with jurisdiction-specific age regulations.

- **Users deserve protection from spam and harmful content.** We employ machine learning, automated filtering, and user reporting to detect and prevent spam while respecting privacy in encrypted communications.

- **Illegal content will not be tolerated.** We use automated scanning, hash-matching (PhotoDNA), and machine learning to detect and remove illegal content from public channels. For private encrypted messages, we rely on user reports and blind pattern detection to identify bad actors without accessing message content.

- **Bad actors will be restricted and removed.** We implement graduated enforcement: accounts flagged for spam or harmful content face restrictions on messaging non-contacts, and repeated violations result in permanent bans.

## Architectural Philosophy

### Technology Choices

**Rust** is our primary language because performance and error rates are critical to user experience. People deserve high-performance, bug-free technology. By striving for performant software with extremely low error rates and high levels of automation, we reduce cost to serve, allowing us to provide this service to as many people as possible.

**End-to-end encryption** will be used for all private communications, ensuring that privacy is protected at the technical level.

### Architecture

cwrdd is designed as a **modular monolith** that can be deployed as a single container application and horizontally scaled to meet demand.

Each module follows an **N-tier architecture**:

- **Application Layer**: Provides services that form the interface for user interactions
- **Domain Layer**: Contains value objects, domain services, and traits that define the core business logic
- **Infrastructure Layer**: Implements traits from the domain layer for external integrations (HTTP APIs, databases, caches, etc.)

### Technology Stack

- **Web Framework**: [warp](https://github.com/seanmonstar/warp) (built on tokio)
- **Database**: PostgreSQL
- **Database Driver**: tokio-postgres (no ORM)
- **Schema Management**: Liquibase Community Edition with `diffChangeLog`
- **Cache**: Redis (distributed cache)
- **Containerization**: Podman with Dockerfiles
- **Build Tool**: Custom Rust-based build tool (`cwrdd-make`)

### Development Experience

A first-class development experience is critical. The `cwrdd-make` tool (located in the `make/` directory) is a high-performance Rust application that manages the entire development lifecycle:

```bash
# Stand up local environment, run migrations, and start the application
cwrdd-make up

# Tear down the entire environment
cwrdd-make down

# Fresh start with a clean database
cwrdd-make refresh
```

The build tool uses the same technology stack as the main application (Rust, tokio, tokio-postgres), making it easy to move between codebases.

## Repository Structure

```
cwrdd/
├── docs/                      # All design and architectural documentation
│   ├── adr/                   # Architectural Decision Records
│   ├── features/              # Feature requirements organized by functional area
│   └── ...
├── db/                        # Database schema and migrations
├── make/                      # cwrdd-make build tool (Rust)
└── ...
```

## Documentation

All design documentation is maintained in this repository to serve as living documentation for the project. See:

- [ADR Directory](docs/adr/) - Architectural Decision Records documenting key architectural choices
- [Feature Documentation](docs/features/) - Feature requirements organized by functional area
- [Feature Documentation Guide](docs/features/README.md) - How feature documentation is organized

## Getting Started

### Quick Start (Ubuntu)

For Ubuntu-based distributions, use our automated setup script:

```bash
# 1. Clone the repository
git clone https://github.com/andy-c-jones/cwrdd.git
cd cwrdd

# 2. Run the setup script (installs Rust and build tools)
./scripts/setup-ubuntu.sh

# 3. Reload your shell
source ~/.bashrc  # or open a new terminal

# 4. Build and install cwrdd-make
cd make
cargo build --release
cargo run --release -- install

# 5. Install additional development tools
cwrdd-make get-tools  # Installs Podman, Liquibase, PostgreSQL client, cargo-nextest

# 6. Set up development environment
cwrdd-make up        # Start containers (PostgreSQL, Redis, etc.) - coming soon
cwrdd-make migrate   # Apply database migrations
cwrdd-make seed      # Load test data
cwrdd-make build     # Build application
cwrdd-make test      # Run tests

# You're ready to develop! 🎉
```

### What Gets Installed

**setup-ubuntu.sh script:**
- build-essential (gcc, make, etc.)
- git, curl, wget
- rustup (from apt)
- Rust stable toolchain

**cwrdd-make get-tools:**
- Podman (container runtime)
- Liquibase (database migrations)
- PostgreSQL client (psql command)
- cargo-nextest (fast test runner)

### Manual Setup (Other Distributions)

If you're not on Ubuntu:

**1. Install Rust and build tools:**
```bash
# Arch Linux
sudo pacman -S rustup base-devel git
rustup default stable

# Fedora
sudo dnf install rustup
sudo dnf groupinstall "Development Tools"
rustup default stable

# macOS
brew install rustup-init
rustup-init
xcode-select --install
```

**2. Build and install cwrdd-make:**
```bash
cd make
cargo build --release
cargo run --release -- install
```

**3. Install other tools:**
```bash
cwrdd-make get-tools  # Auto-installs for Ubuntu/Debian/macOS
```

If `get-tools` doesn't support your OS, install manually:
- **Podman**: https://podman.io/getting-started/installation
- **Liquibase**: https://www.liquibase.org/download (requires Java)
- **PostgreSQL client**: Your distro's postgresql-client package
- **cargo-nextest**: `cargo install cargo-nextest --locked`

### Prerequisites

- **Operating System**: Linux or macOS (Windows via WSL2)
- **Rust** 1.70+ via rustup
- **Build tools** (gcc, make, git)
- **Podman** 4.0+ (container runtime - installed via `cwrdd-make get-tools`)
- **Liquibase** 4.20+ (migrations - installed via `cwrdd-make get-tools`)
- **PostgreSQL client** (installed via `cwrdd-make get-tools`)

### Initial Setup

**1. Clone the repository:**
```bash
git clone https://github.com/andy-c-jones/cwrdd.git
cd cwrdd
```

**2. Bootstrap cwrdd-make (build tool):**
```bash
cd make
cargo build --release
cargo run --release -- install
```

This builds and installs `cwrdd-make` to your PATH (~/.local/bin by default).

**3. Add to PATH if needed:**

If the installer indicates `~/.local/bin` is not in your PATH, add it:

```bash
# For bash (add to ~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"

# For zsh (add to ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

# For fish
fish_add_path ~/.local/bin

# Reload your shell
source ~/.bashrc  # or ~/.zshrc
```

**4. Verify installation:**
```bash
cwrdd-make --help
```

You should see the cwrdd-make help output with all available commands.

```bash
# For bash (add to ~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"

# For zsh (add to ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

# For fish
fish_add_path ~/.local/bin

# Reload your shell
source ~/.bashrc  # or ~/.zshrc
```

**4. Verify installation:**
```bash
cwrdd-make --help
```

You should see the cwrdd-make help output with all available commands.

### Development Workflow

Once cwrdd-make is installed:

```bash
# Start local development environment (PostgreSQL, Redis, etc)
cwrdd-make up

# Generate and apply database migrations
cwrdd-make migrate-diff  # Generate migration from schema
cwrdd-make migrate       # Apply migrations

# Seed development data
cwrdd-make seed

# Build the application
cwrdd-make build

# Run tests
cwrdd-make test

# Stop local environment
cwrdd-make down
```

### First Time Setup

After installing cwrdd-make, run these commands to set up your development environment:

```bash
# 1. Start local services (PostgreSQL, Redis, etc.)
cwrdd-make up

# 2. Apply database migrations
cwrdd-make migrate

# 3. Seed development data
cwrdd-make seed

# 4. Build the application
cwrdd-make build

# 5. Run tests to verify everything works
cwrdd-make test
```

### Available Commands

Run `cwrdd-make --help` to see all available commands:

- `build` - Build the application
- `test` - Run tests
- `migrate-diff` - Generate migration from schema diff
- `migrate` - Apply pending migrations
- `migrate-status` - Show migration status
- `rollback` - Rollback last migration
- `seed` - Seed database with development data
- `up` - Start local development environment (coming soon)
- `down` - Stop local environment (coming soon)
- `refresh` - Fresh start with clean database (coming soon)

### Troubleshooting

**cwrdd-make not found after installation:**
- Verify ~/.local/bin is in your PATH: `echo $PATH | grep .local/bin`
- Re-run the install: `cd make && cargo run --release -- install`
- Reload your shell: `source ~/.bashrc` or open a new terminal

**Liquibase not found:**
- Install Liquibase: See [installation guide](https://www.liquibase.org/download)
- Verify installation: `liquibase --version`

**PostgreSQL connection errors:**
- Ensure PostgreSQL is running
- Check connection settings in `db/liquibase.properties`
- Verify database exists: `psql -l`

## License

*To be determined*
