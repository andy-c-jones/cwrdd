# ADR-001: Modular Monolith Architecture

## Status

**ACCEPTED** - 2025-12-28

## Context

cwrdd aims to provide a safe, secure platform for people to share ideas and arrange in-person meetings. The system must support both private communications (with end-to-end encryption) and public channels (with identity verification). It needs to be:

- **Performant**: Users expect fast, responsive experiences
- **Reliable**: Low error rates are critical for user trust and experience
- **Scalable**: The system must handle growth in users and usage
- **Cost-effective**: Lower operational costs allow us to serve more people
- **Maintainable**: The codebase should be understandable and evolvable over time

We must choose an architectural approach that balances these needs while acknowledging that we're building a new system with an initially small team.

## Decision

We will implement cwrdd as a **modular monolith** using Rust, deployable as a single container that can be horizontally scaled.

### Modular Structure

The monolith consists of several types of modules:

#### 1. Business Domain Modules

Each business domain module follows an **N-tier architecture** with clear separation of concerns:

```
module/
├── application/      # Application services (use case orchestration)
├── domain/          # Domain models, value objects, domain services, traits
└── infrastructure/  # External integrations (databases, caches, HTTP APIs)
```

**Layer responsibilities:**

- **Application Layer**: Orchestrates use cases, provides the interface for business logic execution. This is where the domain logic is exposed to the rest of the system.

- **Domain Layer**: Contains the core business logic, including value objects, entities, domain services, and traits (ports) that define contracts for dependencies. This layer has no knowledge of external systems.

- **Infrastructure Layer**: Implements traits defined in the domain layer. Handles interactions with PostgreSQL, Redis, external HTTP APIs, and other third-party services.

**Dependency rule**: Dependencies flow inward. Infrastructure and Application layers depend on Domain. Domain has no external dependencies.

#### 2. Adapter Modules

Following the **Ports and Adapters pattern**, adapter modules provide implementations that multiple business modules can depend on. These implement traits (ports) defined in business domain modules.

```
adapter/
└── implementations/  # Shared adapter implementations
```

Examples: Database connection pooling, Redis client wrappers, external API clients, message queue adapters.

**Key principle**: Multiple business modules' domain or infrastructure layers may depend on the same ports (traits). Adapters provide reusable implementations of these ports that can be shared across the codebase.

#### 3. Utility Modules

Utility modules contain **generic, reusable functions with no business logic**. These are purely technical utilities.

```
utility/
└── functions/       # Generic utility functions
```

Examples: Date/time formatting, string manipulation, cryptographic helpers, configuration parsing.

**Critical constraint**: Utility modules must never contain business logic. They should only provide generic, domain-agnostic functionality.

#### 4. Web Application

The **web** application is the entry point that composes all modules together. It has two primary responsibilities:

1. **MVC-style web frontend**: Server-side rendered HTML with Askama templates and HTMX for progressive enhancement
2. **RESTful HTTP API**: JSON API for programmatic access

```
web/
├── handlers/        # HTTP request handlers
├── middleware/      # Request/response middleware
├── templates/       # Askama HTML templates
└── main.rs         # Application composition and dependency injection
```

**Key responsibility**: The web application is where **dependency injection** occurs and the entire application is constructed. It wires together all business modules, adapters, and utilities, creating the running application.

See [ADR-002: Server-Side Rendering with HTMX](./002-server-side-rendering-htmx.md) for details on the web frontend architecture.

### Technology Stack

- **Language**: Rust
- **Web Framework**: warp (built on tokio)
- **Database**: PostgreSQL with tokio-postgres (no ORM)
- **Cache**: Redis
- **Schema Management**: Liquibase Community Edition
- **Deployment**: Podman/Docker containers

### Rationale for Rust

Rust provides:
- **Memory safety without garbage collection**: Eliminates entire classes of bugs while maintaining performance
- **Fearless concurrency**: tokio enables highly concurrent, efficient I/O operations
- **Zero-cost abstractions**: High-level code that compiles to performant machine code
- **Excellent error handling**: Result types force explicit error handling, reducing runtime errors

These characteristics directly support our core principle: **people deserve high-performance, bug-free technology**.

### Rationale for No ORM

We're avoiding ORMs in favor of tokio-postgres for several reasons:
- **Performance**: Direct SQL queries are more efficient and give us fine-grained control
- **Explicitness**: SQL queries are visible and reviewable, avoiding hidden N+1 problems
- **Learning curve**: Team members can leverage SQL knowledge directly
- **Flexibility**: Complex queries don't require fighting ORM abstractions

## Consequences

### Positive

1. **Simple deployment model**: Single container simplifies operations and reduces infrastructure complexity
2. **Horizontal scalability**: Stateless containers can be scaled out to meet demand
3. **Performance**: Rust + tokio + direct database access = excellent performance characteristics
4. **Cost efficiency**: Better performance per resource = lower operational costs = ability to serve more users
5. **Strong boundaries**: Modular structure prevents tangled dependencies while maintaining simplicity
6. **Rapid development initially**: No need for distributed systems complexity (service meshes, API gateways, distributed transactions)
7. **Database transactions**: Operations spanning multiple modules can use ACID transactions
8. **Easy local development**: Entire system runs in one process
9. **Adapter reusability**: Shared adapter implementations reduce code duplication across business modules
10. **Clear separation**: Utility modules, adapters, business modules, and web application have distinct, well-defined purposes

### Negative

1. **Rust learning curve**: Team members must be proficient in Rust, which has a steeper learning curve than some alternatives
2. **Deployment coupling**: All modules deploy together; a bug in one module requires redeploying the entire application
3. **Language lock-in**: All modules must be written in Rust
4. **Scaling granularity**: Cannot scale individual modules independently; must scale the entire application
5. **No ORM safety net**: SQL queries must be carefully written and tested; no compile-time query validation

### Mitigation Strategies

- **Learning**: Invest in Rust training and documentation; the long-term benefits justify the upfront learning cost
- **Testing**: Comprehensive automated testing (unit, integration, end-to-end) to catch bugs before deployment
- **CI/CD**: Fast, reliable continuous integration pipeline to make deployments low-risk and frequent
- **Monitoring**: Robust observability (logging, metrics, tracing) to quickly identify and diagnose issues
- **Module boundaries**: Maintain strict module boundaries to enable future extraction to microservices if needed
- **Database testing**: Use test databases and migration testing to validate SQL queries

## Migration Path

This architecture supports future evolution:

1. **Phase 1 (current)**: Modular monolith with clear module boundaries
2. **Phase 2 (if needed)**: Extract specific modules to separate services while maintaining monolith for remaining modules
3. **Phase 3 (if needed)**: Full microservices architecture

Well-defined module boundaries and dependency rules mean extraction is possible without complete rewrites.

## References

- [Modular Monoliths by Simon Brown](https://www.youtube.com/watch?v=5OjqD-ow8GE)
- [Hexagonal Architecture (Ports and Adapters) by Alistair Cockburn](https://alistair.cockburn.us/hexagonal-architecture/)
- [Rust Async Book](https://rust-lang.github.io/async-book/)
- [Clean Architecture by Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)

## Notes

This ADR establishes the foundation for cwrdd's architecture. Specific module designs, API contracts, and implementation details will be documented separately as the system evolves.
