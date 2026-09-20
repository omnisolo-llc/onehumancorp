# syntax=docker/dockerfile:1.7
# Build on the target architecture (native runner preferred, QEMU supported).
# Dependency compilation is a real exported Docker layer, not just a cache mount
# that disappears when an ephemeral GitHub runner is destroyed.
FROM rust:1.95.0-bookworm AS chef
# The planner/cook stage does not yet have .cargo/config.toml. Bound it too,
# rather than letting every CPU launch memory-heavy compiler/linker processes.
ENV CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 MALLOC_ARENA_MAX=2
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential ca-certificates libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --version 0.1.78 --locked
WORKDIR /src

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --release --locked --recipe-path recipe.json \
    --package omnisolo --package omnisolo_builtin_agent --package omnisolo_harness_worker --bins
COPY . .
RUN cargo build --locked --release \
    -p omnisolo -p omnisolo_builtin_agent -p omnisolo_harness_worker --bins

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 libstdc++6 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 omnisolo \
    && mkdir -p /data && chown omnisolo:omnisolo /data
COPY --from=builder /src/target/release/server /usr/local/bin/server
COPY --from=builder /src/target/release/omnisolo-builtin-agent /usr/local/bin/omnisolo-builtin-agent
COPY --from=builder /src/target/release/omnisolo-harness-worker /usr/local/bin/omnisolo-harness-worker
COPY --from=builder /src/src/server/migrations /src/server/migrations
COPY --from=builder /src/src/server/db/migrations /src/server/db/migrations
ENV OMNISOLO_BUILTIN_AGENT_BINARY=/usr/local/bin/omnisolo-builtin-agent
WORKDIR /data
USER omnisolo

FROM runtime AS agent
ENTRYPOINT ["/usr/local/bin/omnisolo-builtin-agent"]

FROM runtime AS worker
ENTRYPOINT ["/usr/local/bin/omnisolo-harness-worker"]

FROM runtime AS server
EXPOSE 18789 50051
ENTRYPOINT ["/usr/local/bin/server"]
