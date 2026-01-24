FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
  protobuf-compiler \
  libprotobuf-dev \
  && \
  rm -rf /var/lib/apt/lists/*

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release -p entrypoint

FROM debian:trixie-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/entrypoint /usr/local/bin
RUN apt-get update && apt-get install -y --no-install-recommends \
  curl \
  && \
  rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/usr/local/bin/entrypoint"]
