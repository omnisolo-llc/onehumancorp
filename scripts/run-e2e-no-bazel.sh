#!/bin/bash
set -euo pipefail

ROOT="$(pwd)"
E2E_DOCKER_COMPOSE="${ROOT}/deploy/docker-compose.e2e.yml"

function wait_for_port() {
    local port=$1
    local name=$2
    echo "Waiting for $name on port $port..."
    for i in {1..30}; do
        if nc -z localhost "$port"; then
            echo "$name is ready!"
            return 0
        fi
        sleep 1
    done
    echo "Error: $name failed to start"
    return 1
}

export PROTOC="$(pwd)/bazel-out/k8-fastbuild/bin/external/protobuf+/protoc"

echo "[e2e-no-bazel] Building web app via Bazel..."
# Build the WASM binary (for wasm32 target)
npx @bazel/bazelisk build //src/app:app_web --platforms=@rules_rust//rust/platform:wasm32

echo "[e2e-no-bazel] Building server..."
cargo build --bin server

echo "[e2e-no-bazel] Starting infrastructure..."
docker compose -f "$E2E_DOCKER_COMPOSE" up -d

# Ensure cleanup
function cleanup() {
    echo "[e2e-no-bazel] Cleaning up..."
    if [ -n "${SERVER_PID:-}" ]; then
        kill "$SERVER_PID" || true
    fi
    docker compose -f "$E2E_DOCKER_COMPOSE" down
}
trap cleanup EXIT

wait_for_port 5432 "Postgres"
wait_for_port 6379 "Redis"

echo "[e2e-no-bazel] Starting server..."
export DATABASE_URL="postgres://ohc:ohc@localhost:5432/ohc"
export REDIS_URL="redis://localhost:6379"
export STANDALONE_MODE="true"

./target/debug/server &
SERVER_PID=$!

wait_for_port 18789 "App Server"

echo "[e2e-no-bazel] Running Playwright tests..."
npx playwright test

echo "[e2e-no-bazel] Done!"
