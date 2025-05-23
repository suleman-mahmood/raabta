# Chef stage
FROM lukemathwalker/cargo-chef:latest-rust-1.82.0 AS chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin forge

# Runtime stage
FROM rust:1.82.0-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/forge forge
COPY configuration configuration
COPY templates templates
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./forge"]
