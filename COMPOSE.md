# cwrdd Local Development Environment

This directory contains the Podman Compose configuration for running the cwrdd local development stack.

## Stack Components

### Core Services
- **PostgreSQL 16** - Primary database (port 5432)
- **Redis 7** - Cache layer (port 6379)

### Observability Stack (LGTM + Pyroscope)
- **Grafana Tempo** - Distributed tracing backend (port 3200, OTLP 4317/4318)
- **Grafana Loki** - Log aggregation (port 3100)
- **Prometheus** - Metrics storage (port 9090)
- **Grafana Pyroscope** - Continuous profiling (port 4040)
- **Grafana Alloy** - Unified telemetry collector (port 12345, OTLP 4319/4320)
- **Grafana** - Visualization and dashboards (port 3000)

All observability components run in **monolithic mode** for simplified local development.

## Architecture

```
┌─────────────────┐
│   cwrdd app     │
│ (OpenTelemetry) │
└────────┬────────┘
         │ OTLP (4319:gRPC / 4320:HTTP)
         ▼
┌─────────────────┐
│  Grafana Alloy  │  ← Collector/Router
└────────┬────────┘
         │
         ├─► Loki (Logs)        :3100
         ├─► Tempo (Traces)     :3200
         ├─► Prometheus (Metrics):9090
         └─► Pyroscope (Profiles):4040
               │
               ▼
          ┌──────────┐
          │ Grafana  │  ← Visualization :3000
          └──────────┘
```

## Prerequisites

### Install Development Tools

Use cwrdd-make to install all required tools including podman-compose:

```bash
# From the make directory
cd make
cargo run --release -- get-tools
```

This will install:
- Podman
- podman-compose
- Liquibase
- PostgreSQL client
- cargo-nextest

## Quick Start

### Start All Services

```bash
cwrdd-make up
```

This will:
1. Start all containers with podman-compose
2. Wait for PostgreSQL to be ready
3. Run database migrations if needed
4. Seed database with development data if empty
5. Display access information for all services

### Start Core Services Only (no observability)

If you want to save resources during development:

```bash
podman-compose up -d postgres redis
```

### Stop All Services

```bash
cwrdd-make down
```

### View Logs

```bash
# All services
cwrdd-make logs

# Specific service
cwrdd-make logs postgres
cwrdd-make logs grafana

# Follow logs in real-time
cwrdd-make logs --follow
cwrdd-make logs -f postgres
```

### Manual Compose Commands

You can also use podman-compose directly:

```bash
# Start services manually
podman-compose up -d

# Stop services
podman-compose down
```

### Stop and Remove Volumes

**Warning:** This will delete all data!

```bash
podman-compose down -v
```

## Accessing Services

### Core Services
- **PostgreSQL**: `localhost:5432`
  - Database: `cwrdd_dev`
  - User: `cwrdd_user`
  - Password: `cwrdd_password`
- **Redis**: `localhost:6379`

### Observability UIs
- **Grafana**: http://localhost:3000 (anonymous admin access enabled)
- **Prometheus**: http://localhost:9090
- **Alloy**: http://localhost:12345

### Application Telemetry Endpoints
Configure your cwrdd application to send telemetry to:
- **OTLP gRPC**: `http://localhost:4319`
- **OTLP HTTP**: `http://localhost:4320`

## Configuration

All configuration files are in the `config/` directory:

```
config/
├── alloy/
│   └── config.alloy          # Alloy telemetry pipeline
├── tempo/
│   └── tempo.yaml            # Tempo tracing config
├── loki/
│   └── loki.yaml             # Loki logs config
├── prometheus/
│   └── prometheus.yml        # Prometheus scrape config
└── grafana/
    └── provisioning/
        ├── datasources/      # Pre-configured data sources
        └── dashboards/       # Dashboard provisioning
```

### Modifying Configuration

1. Edit the relevant config file in `config/`
2. Restart the affected service:
   ```bash
   podman-compose restart <service-name>
   ```

## Database Migrations

The compose file does NOT automatically run migrations. Use `cwrdd-make` to manage migrations:

```bash
# Generate migration from schema
cwrdd-make migrate-diff

# Apply migrations
cwrdd-make migrate

# Check migration status
cwrdd-make migrate-status
```

Or run Liquibase manually with the migrate profile:

```bash
podman-compose --profile migrate up liquibase
```

## Volumes

Persistent data is stored in named volumes:

- `cwrdd-postgres-data` - PostgreSQL database
- `cwrdd-redis-data` - Redis persistence
- `cwrdd-tempo-data` - Tempo traces
- `cwrdd-loki-data` - Loki logs
- `cwrdd-prometheus-data` - Prometheus metrics
- `cwrdd-pyroscope-data` - Pyroscope profiles
- `cwrdd-alloy-data` - Alloy state
- `cwrdd-grafana-data` - Grafana dashboards and settings

### Inspecting Volumes

```bash
# List volumes
podman volume ls | grep cwrdd

# Inspect a volume
podman volume inspect cwrdd-postgres-data

# Remove all cwrdd volumes (WARNING: deletes all data)
podman volume ls -q | grep cwrdd | xargs podman volume rm
```

## Networking

All services are connected via the `cwrdd-network` bridge network.

Service discovery works by container name:
- `postgres:5432`
- `redis:6379`
- `tempo:3200`
- `loki:3100`
- `prometheus:9090`
- `pyroscope:4040`
- `alloy:12345`

## Health Checks

PostgreSQL and Redis include health checks. View status:

```bash
podman-compose ps
```

## Troubleshooting

### Check Service Logs

```bash
# All services
podman-compose logs -f

# Specific service
podman-compose logs -f postgres
podman-compose logs -f alloy
```

### Verify Network Connectivity

```bash
# From inside a container
podman exec cwrdd-alloy ping -c 3 tempo
```

### Reset Everything

```bash
# Stop and remove containers, networks, and volumes
podman-compose down -v

# Start fresh
podman-compose up -d
```

### Resource Usage

Check resource consumption:

```bash
podman stats
```

For minimal resource usage, run only core services:
```bash
podman-compose up -d postgres redis
```

## Integration with cwrdd-make

The `cwrdd-make` build tool will integrate with this compose file:

```bash
# Start development environment
cwrdd-make up

# Stop development environment
cwrdd-make down

# View logs
cwrdd-make logs

# Run migrations
cwrdd-make migrate
```

## Production Differences

This configuration is optimized for **local development only**:

- Anonymous admin access in Grafana
- No authentication between services
- Local filesystem storage (not object storage)
- Single-node, monolithic deployments
- No replication or high availability

For production, use:
- Proper authentication and TLS
- Object storage backends (S3, GCS, etc.)
- Distributed, replicated deployments
- External secret management
- Resource limits and quotas

## Related Documentation

- [ADR-001: Modular Monolith Architecture](../docs/adr/001-modular-monolith-architecture.md)
- [ADR-003: OpenTelemetry Observability](../docs/adr/003-opentelemetry-observability.md)
- [Database README](../db/README.md)
