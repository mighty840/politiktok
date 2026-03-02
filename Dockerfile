# ---------- chef: install cargo-chef ----------
FROM rust:1-bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

# ---------- planner: prepare build recipe ----------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------- builder: compile the application ----------
FROM chef AS builder

# Install sccache for faster rebuilds
RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache

# Install bun (for tailwind CSS build)
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

# Install dioxus-cli
RUN cargo install dioxus-cli@0.7.3

# Cook dependencies from recipe (cached layer)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Copy full source tree
COPY . .

# Build the fullstack application
RUN dx bundle --release --fullstack

# ---------- runtime: minimal production image ----------
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --gid 1000 app && \
    useradd --uid 1000 --gid app --create-home app

WORKDIR /app

# Copy built artifacts from builder
COPY --from=builder /app/target/dx/politiktok/release/web /app/public
COPY --from=builder /app/target/release/politiktok /app/politiktok

RUN chown -R app:app /app

USER app

EXPOSE 8000

ENV RUST_LOG=info
ENV IP=0.0.0.0
ENV PORT=8000

CMD ["/app/politiktok"]
