#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
fixture_pid=""
fixture_dir=""
cleanup() {
  if [[ -n "$fixture_pid" ]]; then
    kill "$fixture_pid" 2>/dev/null || true
    wait "$fixture_pid" 2>/dev/null || true
  fi
  if [[ -n "$fixture_dir" ]]; then rm -rf "$fixture_dir"; fi
}
trap cleanup EXIT
if [[ -z "${OHC_TEST_REDIS_URL:-}" ]]; then
  fixture_dir=$(mktemp -d)
  fixture_port=$(python3 - <<'PY'
import socket
with socket.socket() as listener:
    listener.bind(('127.0.0.1', 0))
    print(listener.getsockname()[1])
PY
)
  redis-server --bind 127.0.0.1 --port "$fixture_port" --save '' --appendonly no --daemonize no --dir "$fixture_dir" >"$fixture_dir/server.log" 2>&1 &
  fixture_pid=$!
  for _ in {1..50}; do
    if redis-cli -h 127.0.0.1 -p "$fixture_port" PING >/dev/null 2>&1; then break; fi
    sleep 0.1
  done
  redis-cli -h 127.0.0.1 -p "$fixture_port" PING >/dev/null
  running_pid=$(redis-cli -h 127.0.0.1 -p "$fixture_port" INFO server | tr -d '\r' | sed -n 's/^process_id://p')
  if [[ "$running_pid" != "$fixture_pid" ]]; then
    echo 'Redis fixture port is not owned by this test process' >&2
    exit 1
  fi
  export OHC_TEST_REDIS_URL="redis://127.0.0.1:$fixture_port/"
  export OHC_REDIS_SERVICE_ISOLATION=1
fi
if [[ "${OHC_REDIS_SERVICE_ISOLATION:-}" != 1 ]]; then
  echo 'Declare an owned disposable service with OHC_REDIS_SERVICE_ISOLATION=1' >&2
  exit 1
fi
export REDIS_URL="$OHC_TEST_REDIS_URL"
cargo test --locked --manifest-path scripts/redis-reconnect/Cargo.toml
