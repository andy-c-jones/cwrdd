# Build stage
FROM docker.io/library/rust:1.83-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source and templates
COPY src ./src
COPY templates ./templates

# Build the actual application
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM docker.io/library/alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/cwrdd-app /app/cwrdd-app

# Certs will be mounted at runtime
RUN mkdir -p /app/certs

EXPOSE 8443

CMD ["/app/cwrdd-app"]
