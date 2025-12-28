# ADR-003: OpenTelemetry for Observability

## Status

**ACCEPTED** - 2025-12-28

## Context

cwrdd is built on the principle that people deserve high-performance, bug-free technology. To deliver on this promise, we need comprehensive observability into our system's behavior in both development and production environments.

Observability requirements:

- **Logs**: Structured logging for debugging and audit trails
- **Metrics**: Performance indicators, resource utilization, business metrics
- **Traces**: Distributed tracing to understand request flows and identify bottlenecks
- **Profiles**: Continuous profiling to identify performance optimization opportunities
- **Developer experience**: Local observability must be first-class so developers can debug effectively
- **Production ready**: Same tooling and instrumentation in development and production
- **Performance**: Observability overhead must be minimal and predictable
- **Standards-based**: Use open standards to avoid vendor lock-in

The Rust ecosystem has several observability options:

1. **tracing** crate ecosystem - Popular Rust-native logging/tracing framework
2. **log** crate - Traditional logging interface
3. **OpenTelemetry** - Vendor-neutral, open standard for telemetry data
4. **Prometheus client** - Direct Prometheus integration
5. **Custom solutions** - Build our own observability stack

We must choose an approach that provides comprehensive observability while remaining maintainable and performant.

## Decision

We will use **OpenTelemetry Rust** (opentelemetry-rust) as our observability framework, with **Grafana Alloy** as the local collector and **Grafana** stack for visualization.

### Observability Stack

#### Application Layer

**OpenTelemetry Rust SDK:**
- `opentelemetry` - Core SDK
- `opentelemetry_sdk` - SDK implementation
- `opentelemetry-otlp` - OTLP exporter for logs, metrics, traces
- `tracing-opentelemetry` - Bridge between `tracing` crate and OpenTelemetry
- `tracing-subscriber` - Composable tracing subscriber

**Instrumentation:**
- Automatic instrumentation for HTTP (warp), database (tokio-postgres), Redis
- Custom instrumentation for business logic spans
- Structured logging with tracing macros
- Metrics for request rates, latencies, errors, resource usage
- Continuous profiling integration (pprof format)

#### Local Development Environment

The local environment includes full observability stack via containers:

```
┌─────────────────┐
│   cwrdd app     │
│ (OpenTelemetry) │
└────────┬────────┘
         │ OTLP
         ▼
┌─────────────────┐
│  Grafana Alloy  │  ← Collector/Processor
│   (Container)   │
└────────┬────────┘
         │
         ├─► Loki (Logs)
         ├─► Tempo (Traces)
         ├─► Mimir/Prometheus (Metrics)
         └─► Pyroscope (Profiles)
              │
              ▼
         ┌──────────┐
         │ Grafana  │  ← Visualization
         │ (Container)│
         └──────────┘
```

**Container Setup (managed by cwrdd-make):**

```yaml
services:
  grafana-alloy:
    image: grafana/alloy:latest
    ports:
      - "4317:4317"  # OTLP gRPC
      - "4318:4318"  # OTLP HTTP
    volumes:
      - ./config/alloy.yaml:/etc/alloy/config.yaml

  tempo:
    image: grafana/tempo:latest
    ports:
      - "3200:3200"

  loki:
    image: grafana/loki:latest
    ports:
      - "3100:3100"

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"

  pyroscope:
    image: grafana/pyroscope:latest
    ports:
      - "4040:4040"

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_AUTH_ANONYMOUS_ENABLED=true
      - GF_AUTH_ANONYMOUS_ORG_ROLE=Admin
    volumes:
      - ./config/grafana/datasources.yaml:/etc/grafana/provisioning/datasources/datasources.yaml
      - ./config/grafana/dashboards.yaml:/etc/grafana/provisioning/dashboards/dashboards.yaml
```

#### Production Environment

In production, the same OTLP protocol is used, but data is sent to production-grade backends:

- Option 1: Self-hosted Grafana stack (Loki, Tempo, Mimir, Pyroscope)
- Option 2: Grafana Cloud (managed service)
- Option 3: Other OTLP-compatible backends (Honeycomb, Datadog, New Relic, etc.)

The application code remains unchanged - only the OTLP endpoint configuration differs.

### Implementation Approach

#### Instrumentation in Application Code

**Tracing:**

```rust
use tracing::{info, instrument, span};

#[instrument(skip(db_pool))]
async fn create_user(db_pool: &DbPool, username: String) -> Result<User> {
    let span = span!(Level::INFO, "db.query", table = "users");
    let _enter = span.enter();
    
    info!(username = %username, "Creating new user");
    // ... database logic
    
    Ok(user)
}
```

**Metrics:**

```rust
use opentelemetry::metrics::{Counter, Histogram};

lazy_static! {
    static ref REQUEST_COUNTER: Counter<u64> = 
        meter().u64_counter("http.requests.total").init();
    
    static ref REQUEST_DURATION: Histogram<f64> = 
        meter().f64_histogram("http.request.duration").init();
}

REQUEST_COUNTER.add(1, &[KeyValue::new("method", "GET")]);
REQUEST_DURATION.record(duration.as_secs_f64(), &[]);
```

**Structured Logging:**

```rust
use tracing::{info, error};

info!(
    user_id = %user.id,
    action = "login",
    ip_address = %client_ip,
    "User logged in successfully"
);

error!(
    error = %e,
    user_id = %user.id,
    "Failed to process payment"
);
```

#### cwrdd-make Integration

The build tool manages the observability stack:

```bash
# Start everything including observability
cwrdd-make up

# Access Grafana at http://localhost:3000
# Pre-configured with all data sources
# Pre-loaded with dashboards for cwrdd
```

**What cwrdd-make handles:**
1. Start Grafana Alloy, Tempo, Loki, Prometheus, Pyroscope, Grafana containers
2. Configure data sources automatically
3. Load pre-built dashboards
4. Configure cwrdd application to send OTLP to Alloy
5. Health checks to ensure stack is ready

### Configuration

**Development (local):**
```rust
// In main.rs
let otlp_endpoint = env::var("OTLP_ENDPOINT")
    .unwrap_or_else(|_| "http://localhost:4317".to_string());

init_telemetry(&otlp_endpoint)?;
```

**Production:**
```bash
# Environment variable points to production collector
OTLP_ENDPOINT=https://otlp.production.cwrdd.com:4317
```

### Observability Dashboards

Pre-built Grafana dashboards for:

1. **Application Overview**: Request rates, errors, latencies (p50, p95, p99)
2. **Database Performance**: Query performance, connection pool usage
3. **Business Metrics**: User registrations, messages sent, active users
4. **Infrastructure**: CPU, memory, network, container health
5. **Traces**: Distributed trace visualization with Tempo
6. **Logs**: Structured log exploration with Loki
7. **Profiles**: Continuous profiling with Pyroscope

## Rationale

### Why OpenTelemetry?

**Vendor Neutrality:**
- Open standard backed by CNCF
- Not locked into any specific vendor
- Can change backends without changing application code
- Reduces operational risk

**Comprehensive Coverage:**
- Single framework for logs, metrics, traces, and profiles
- Unified semantic conventions
- Correlation between signals (trace ID in logs, etc.)
- Future-proof as standard evolves

**Ecosystem Maturity:**
- Broad adoption across industry
- Strong Rust support via opentelemetry-rust
- Integration with tracing crate (Rust standard)
- Automatic instrumentation for common libraries

**Production Ready:**
- Battle-tested in large-scale systems
- Low overhead and high performance
- Configurable sampling and batching
- Reliable OTLP protocol

### Why Grafana Stack?

**Developer Experience:**
- Single pane of glass for all observability data
- Excellent query languages (LogQL, TraceQL, PromQL)
- Powerful visualization capabilities
- Easy to run locally in containers

**Cost Effective:**
- Open source - free to self-host
- Grafana Cloud available if we want managed service
- No per-seat licensing for self-hosted
- Scales with our growth

**Production Ready:**
- Industry standard for observability
- Proven at scale
- Active development and community
- Commercial support available if needed

**Integration:**
- Native support for OpenTelemetry
- Tempo for traces, Loki for logs, Mimir for metrics, Pyroscope for profiles
- Grafana Alloy as modern replacement for Grafana Agent
- Excellent correlation between different signal types

### Why Grafana Alloy?

**Modern Collector:**
- Successor to Grafana Agent
- Native OpenTelemetry support
- Lightweight and efficient
- Easy configuration

**Local Development:**
- Single collector for all signals
- Minimal resource usage
- Fast startup
- Easy to configure and debug

**Production Path:**
- Same collector in dev and prod (consistency)
- Supports advanced features (tail sampling, metrics generation from spans)
- Can run as sidecar or standalone
- Scales horizontally

## Alternatives Considered

### Alternative 1: Prometheus + ELK Stack

**Pros:**
- Prometheus very mature for metrics
- ELK (Elasticsearch, Logstash, Kibana) widely used

**Cons:**
- Two separate systems to manage
- No native distributed tracing
- Higher resource usage (Elasticsearch is heavy)
- More complex local development setup
- No unified correlation between signals

**Verdict:** Rejected - too complex and fragmented

### Alternative 2: Native Rust Tracing Only

**Pros:**
- Simpler integration (just tracing crate)
- Lower overhead
- Rust-idiomatic

**Cons:**
- No standardized export format
- Difficult to get metrics and traces together
- Limited visualization options
- Hard to integrate with external systems
- Custom backend development required

**Verdict:** Rejected - insufficient for production needs

### Alternative 3: Commercial APM (Datadog, New Relic)

**Pros:**
- Fully managed
- Excellent UX
- Minimal setup

**Cons:**
- Expensive, especially at scale
- Vendor lock-in
- Poor local development experience
- Can't run locally without external service
- Cost scales linearly with usage

**Verdict:** Rejected - too expensive and poor dev experience. However, OpenTelemetry allows us to switch to these later if needed.

### Alternative 4: Jaeger + Prometheus + Custom Logging

**Pros:**
- All open source
- Jaeger great for tracing
- Prometheus excellent for metrics

**Cons:**
- Three separate systems
- No profiling
- Manual correlation between signals
- More complex configuration
- OpenTelemetry provides better integration

**Verdict:** Rejected - OpenTelemetry + Grafana provides better integration

## Consequences

### Positive

- **Excellent developer experience**: Full observability locally with simple `cwrdd-make up`
- **Production parity**: Same instrumentation and tools in dev and production
- **Vendor neutrality**: Can change backends without code changes
- **Future proof**: OpenTelemetry is the industry standard going forward
- **Comprehensive**: Logs, metrics, traces, and profiles in one system
- **Cost effective**: Open source stack, scales with usage
- **Performance**: Low overhead instrumentation

### Negative

- **Learning curve**: Team needs to learn OpenTelemetry concepts and Grafana tools
- **Local resource usage**: Running full stack locally requires memory/CPU (mitigated by making it optional)
- **Configuration complexity**: Initial setup of Alloy and Grafana requires careful configuration
- **Rust OpenTelemetry maturity**: While stable, not as mature as some language implementations

### Mitigation Strategies

**Learning Curve:**
- Provide comprehensive documentation
- Create example instrumentation patterns
- Include observability in code review checklist
- Regular training sessions

**Resource Usage:**
- Make observability stack optional for lightweight development
- Provide `cwrdd-make up --minimal` flag that skips observability
- Use resource limits on containers
- Document minimum system requirements

**Configuration Complexity:**
- cwrdd-make handles all configuration automatically
- Provide tested, working configurations
- Document customization points
- Include troubleshooting guide

**Rust Maturity:**
- Use stable, well-tested crates
- Contribute back to opentelemetry-rust if we find issues
- Have fallback logging strategy if OpenTelemetry fails

## Implementation Plan

### Phase 1: Foundation (Week 1-2)

1. Add OpenTelemetry dependencies to Cargo.toml
2. Create telemetry initialization module
3. Set up basic tracing with opentelemetry-otlp
4. Add container definitions for Grafana stack
5. Update cwrdd-make to manage observability containers
6. Test end-to-end: app → Alloy → Grafana

### Phase 2: Instrumentation (Week 3-4)

1. Add HTTP request/response tracing middleware
2. Instrument database queries
3. Add Redis instrumentation
4. Create custom metrics for business logic
5. Add structured logging throughout application
6. Test and validate data in Grafana

### Phase 3: Dashboards (Week 5)

1. Create application overview dashboard
2. Create database performance dashboard
3. Create business metrics dashboard
4. Create infrastructure dashboard
5. Document dashboard usage

### Phase 4: Production (Week 6+)

1. Set up production Grafana stack (or Grafana Cloud)
2. Configure production OTLP endpoint
3. Set up alerts in Grafana
4. Document on-call runbooks
5. Train team on production observability

## Related Decisions

- **ADR-001**: Modular Monolith Architecture - Observability needs to work across all modules
- **ADR-002**: Server-Side Rendering - Need to instrument HTTP handlers effectively

## References

- [OpenTelemetry Specification](https://opentelemetry.io/docs/specs/otel/)
- [opentelemetry-rust](https://github.com/open-telemetry/opentelemetry-rust)
- [Grafana Alloy Documentation](https://grafana.com/docs/alloy/latest/)
- [Grafana Tempo](https://grafana.com/oss/tempo/)
- [Grafana Loki](https://grafana.com/oss/loki/)
- [Grafana Pyroscope](https://grafana.com/oss/pyroscope/)
- [tracing crate](https://docs.rs/tracing/)
