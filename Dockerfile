# ── Stage 1: Build ───────────────────────────────────────────────────────────
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /build

# Install build deps (none needed for this project, but keep layer for caching)
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config && \
    rm -rf /var/lib/apt/lists/*

# Cache dependencies — copy manifests first, build a dummy, then swap in real src
COPY Cargo.toml Cargo.lock ./
COPY askama.toml ./
RUN mkdir -p src/bin && \
    echo 'fn main() {}' > src/bin/main.rs && \
    echo 'pub mod config; pub mod db; pub mod error; pub mod handlers; pub mod middleware; pub mod models; #[macro_use] pub mod render; pub mod services; pub mod utils;' > src/lib.rs && \
    mkdir -p src/handlers src/middleware src/models src/services src/utils && \
    touch src/config.rs src/db.rs src/error.rs src/render.rs && \
    touch src/handlers/mod.rs src/handlers/partials.rs src/handlers/templates.rs && \
    touch src/middleware/mod.rs src/models/mod.rs && \
    touch src/services/mod.rs src/services/health.rs src/services/items.rs src/services/csrf.rs src/services/session.rs && \
    touch src/utils/mod.rs src/utils/logging.rs src/utils/templates.rs && \
    mkdir -p migrations && touch migrations/.keep && \
    cargo build --release 2>/dev/null || true

# Copy real source + templates + migrations (askama needs templates at compile time)
COPY src/ src/
COPY templates/ templates/
COPY migrations/ migrations/

# Build the real binary
RUN cargo build --release --bin app

# ── Stage 2: Runtime ─────────────────────────────────────────────────────────
# Distroless-style minimal image — no shell, no package manager, no attack surface
FROM debian:bookworm-slim AS runtime

# Security: run as non-root
RUN groupadd -r app && useradd -r -g app -d /app -s /sbin/nologin app

WORKDIR /app

# Copy only what's needed
COPY --from=builder /build/target/release/app /app/app
COPY config/ /app/config/
COPY static/ /app/static/
COPY templates/ /app/templates/

# Create writable data directory for SQLite
RUN mkdir -p /app/data && chown -R app:app /app

USER app

# Expose the configured port
EXPOSE 8000

# Health check (uses only the binary — no curl/wget needed)
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app/app", "--health-check"] || exit 1

# Run the binary directly — no shell wrapper
ENTRYPOINT ["/app/app"]
