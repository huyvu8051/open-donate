## Multi-stage build for Leptos (Axum) app
##
## Build:
##   docker build -t open-donate:local .
##
## Run:
##   docker run --rm -p 3000:3000 \
##     -e LEPTOS_SITE_ADDR=0.0.0.0:3000 \
##     open-donate:local

# --- Chef base stage ---
FROM rust:1-bookworm AS chef

# Install cargo-chef
RUN cargo install cargo-chef
WORKDIR /app

# --- Planner stage ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- Builder stage ---
FROM chef AS builder

# System deps commonly needed by sqlx/postgres + wasm tooling + asset pipeline.
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    pkg-config \
    libssl-dev \
    clang \
    mold \
    nodejs \
    npm \
  && rm -rf /var/lib/apt/lists/*

# Install nightly + wasm target (cargo-leptos defaults to nightly).
RUN rustup toolchain install nightly --profile minimal --allow-downgrade \
  && rustup default nightly \
  && rustup target add wasm32-unknown-unknown

# Install cargo-leptos + sass (optional but common for Leptos starters).
RUN cargo install cargo-leptos --locked \
  && npm install -g sass

# Copy recipe from planner
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this is the caching Docker layer!
# cargo-leptos builds two targets: WASM (frontend) and host (backend).
RUN cargo chef cook --release --recipe-path recipe.json --features "ssr"
RUN cargo chef cook --release --recipe-path recipe.json --target wasm32-unknown-unknown --features "hydrate"

# Now copy the actual source code
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY public ./public
COPY style ./style
COPY locales ./locales
COPY shared ./shared
COPY migrations ./migrations

# If the repo has other build-time assets/config, copy them too.
COPY ./*.yaml ./

# Build the app (dependencies are already cached)
RUN cargo leptos build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Leptos output layout from cargo-leptos:
# - site assets: target/site
# - server binary is typically in target/release/<LEPTOS_OUTPUT_NAME>
COPY --from=builder /app/target/release/open-donate /app/open-donate
COPY --from=builder /app/target/site /app/site

ENV LEPTOS_OUTPUT_NAME="open-donate" \
    LEPTOS_SITE_ROOT="site" \
    LEPTOS_SITE_PKG_DIR="pkg" \
    LEPTOS_SITE_ADDR="0.0.0.0:3000" \
    LEPTOS_RELOAD_PORT="3001"

EXPOSE 3000

CMD ["/app/open-donate"]
