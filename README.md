# cwrdd

**Pronunciation:** *koor-th* (for English speakers)

## Project Philosophy

cwrdd is built on the fundamental belief that human connection and democratic participation require safe, secure spaces for people to share ideas and arrange in-person meetings.

### Core Principles

- **People deserve a safe and secure place to share ideas and arrange in-person meetings.** Privacy and security are not optional features—they are fundamental rights that enable free expression and democratic participation.

- **Human interaction is more important than ever in an era dominated by technology.** While technology connects us digitally, it should facilitate and enhance real-world human connection, not replace it.

- **Real in-person interactions are critical for democracy.** Democratic participation requires the ability to organize, meet, and discuss ideas face-to-face. Technology should empower these interactions, not hinder them.

- **People should be allowed complete privacy in private channels but should not be able to hide behind anonymity in public channels and groups.** Privacy and accountability serve different purposes: private conversations deserve absolute privacy, while public discourse benefits from accountability.

- **It should be exceptionally clear to users what is being done 'publicly' vs 'privately'.** Users must always understand the context of their communications and the implications of their actions.

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

*Coming soon: Setup instructions and development guide*

## License

*To be determined*
