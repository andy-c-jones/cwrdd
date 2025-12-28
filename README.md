# cwrdd

**Pronunciation:** *koor-th* (for English speakers)

## Quick Start

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
./target/release/cwrdd-make install

# 5. Install development tools (Podman, Liquibase, etc.)
cwrdd-make get-tools

# 6. Start the development environment
cwrdd-make up

# 7. (Optional) Trust the self-signed certificate for HTTPS
cwrdd-make trust-cert

# You're ready to develop! 🎉
# Access the app at https://localhost:8443
```

### Available Commands

```bash
cwrdd-make --help          # Show all commands

# Development
cwrdd-make up              # Build app, Docker image, start all services
cwrdd-make down            # Stop all services
cwrdd-make logs [service]  # View logs
cwrdd-make build           # Build the Rust application
cwrdd-make test            # Run tests

# Database
cwrdd-make migrate         # Apply pending migrations
cwrdd-make migrate-status  # Show migration status
cwrdd-make migrate-diff    # Generate migration from schema diff
cwrdd-make rollback        # Rollback last migration
cwrdd-make seed            # Seed development data

# TLS Certificates
cwrdd-make trust-cert      # Trust self-signed cert (requires sudo)
cwrdd-make untrust-cert    # Remove certificate trust

# Tools & Documentation
cwrdd-make get-tools       # Install Podman, Liquibase, etc.
cwrdd-make install         # Install cwrdd-make to PATH
cwrdd-make doc --open      # Build and view rustdoc documentation
```

### Services Started by `cwrdd-make up`

| Service | URL/Port |
|---------|----------|
| cwrdd App (HTTPS) | https://localhost:8443 |
| PostgreSQL | localhost:5432 |
| Redis | localhost:6379 |
| Grafana | http://localhost:3000 |
| Prometheus | http://localhost:9090 |
| Alloy (OTEL) | http://localhost:12345 |

---

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
- **Templating**: [askama](https://github.com/djc/askama) (compile-time templates)
- **Frontend**: [htmx](https://htmx.org/) (hypermedia-driven)
- **Database**: PostgreSQL
- **Database Driver**: tokio-postgres (no ORM)
- **Schema Management**: Liquibase Community Edition with `diffChangeLog`
- **Cache**: Redis (distributed cache)
- **Containerization**: Podman with Dockerfiles
- **Build Tool**: Custom Rust-based build tool (`cwrdd-make`)

### Development Experience

A first-class development experience is critical. The `cwrdd-make` tool (located in the `make/` directory) is a high-performance Rust application that manages the entire development lifecycle:

```bash
# Stand up local environment with HTTPS
cwrdd-make up

# Tear down the entire environment
cwrdd-make down

# View logs
cwrdd-make logs -f
```

The build tool uses the same technology stack as the main application (Rust, tokio), making it easy to move between codebases.

## Repository Structure

```
cwrdd/
├── app/                       # Main application (warp + askama + htmx)
│   ├── src/                   # Rust source code
│   ├── templates/             # Askama HTML templates
│   └── Dockerfile             # Container build
├── config/                    # Configuration files
│   ├── certs/                 # Generated TLS certificates (gitignored)
│   ├── grafana/               # Grafana provisioning
│   ├── prometheus/            # Prometheus config
│   └── ...
├── db/                        # Database schema and migrations
├── docs/                      # Design and architectural documentation
│   ├── adr/                   # Architectural Decision Records
│   └── features/              # Feature requirements
├── make/                      # cwrdd-make build tool (Rust)
├── scripts/                   # Setup scripts
└── compose.yaml               # Docker/Podman compose for local dev
```

## Documentation

All design documentation is maintained in this repository to serve as living documentation for the project. See:

- [ADR Directory](docs/adr/) - Architectural Decision Records documenting key architectural choices
- [Feature Documentation](docs/features/) - Feature requirements organized by functional area

### Rustdoc

Build and browse API documentation:

```bash
cwrdd-make doc --open
```

## Troubleshooting

**cwrdd-make not found after installation:**
- Verify ~/.local/bin is in your PATH: `echo $PATH | grep .local/bin`
- Re-run the install: `cd make && cargo run --release -- install`
- Reload your shell: `source ~/.bashrc` or open a new terminal

**HTTPS certificate warnings:**
- Run `cwrdd-make trust-cert` to trust the self-signed certificate
- For Firefox: manually import from `config/certs/cert.pem`

**Container issues:**
- Check logs: `cwrdd-make logs app`
- Recreate containers: `cwrdd-make up --recreate`

## License

*To be determined*
