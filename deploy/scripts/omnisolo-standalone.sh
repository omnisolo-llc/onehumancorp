#!/usr/bin/env bash
# Native local backend plus desktop, with explicit child ownership.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
for tool in cargo npm curl openssl; do
  command -v "$tool" >/dev/null || { echo "$tool is required" >&2; exit 1; }
done
export OMNISOLO_MULTITENANT=false OMNISOLO_SOURCE_MODE=standalone
export OMNISOLO_STANDALONE_MODE=true
export OMNISOLO_TELEMETRY_ENABLED="${OMNISOLO_TELEMETRY_ENABLED:-false}"
export OMNISOLO_RUNTIME_DIR="${OMNISOLO_RUNTIME_DIR:-$ROOT/.omnisolo/runtime}"
export OMNISOLO_MEMORY_DIR="${OMNISOLO_MEMORY_DIR:-$OMNISOLO_RUNTIME_DIR/memory}"
export OMNISOLO_STATUS_DIR="${OMNISOLO_STATUS_DIR:-$OMNISOLO_RUNTIME_DIR/status}"
export PORT="${PORT:-18789}"
[[ "$PORT" =~ ^[0-9]+$ ]] && (( PORT > 0 && PORT < 65536 )) || { echo "Invalid local PORT" >&2; exit 1; }
export BACKEND_URL="http://127.0.0.1:$PORT"
umask 077
mkdir -p "$OMNISOLO_RUNTIME_DIR" "$OMNISOLO_MEMORY_DIR" "$OMNISOLO_STATUS_DIR"
if [[ -z "${OMNISOLO_SQLITE_KEY:-}" ]]; then
  key_file="$OMNISOLO_RUNTIME_DIR/.sqlite_key"
  if [[ ! -f "$key_file" ]]; then openssl rand -hex 32 > "$key_file"; fi
  chmod 600 "$key_file"
  export OMNISOLO_SQLITE_KEY="$(cat "$key_file")"
fi
# Never delete durable memory, downloads, or audit records based on their age.
cargo build --locked --release -p omnisolo --bin server
server_pid=""
cleanup() {
  local status=$?
  trap - EXIT INT TERM
  if [[ -n "$server_pid" ]]; then
    kill -TERM "$server_pid" 2>/dev/null || true
    wait "$server_pid" 2>/dev/null || true
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM
./target/release/server &
server_pid=$!
ready=false
for ((attempt=0; attempt<90; attempt++)); do
  if ! kill -0 "$server_pid" 2>/dev/null; then
    wait "$server_pid" || exit $?
    echo "Backend exited before becoming ready" >&2; exit 1
  fi
  if curl --fail --silent --max-time 2 "$BACKEND_URL/readyz" >/dev/null; then ready=true; break; fi
  sleep 1
done
[[ "$ready" == true ]] || { echo "Backend readiness timed out" >&2; exit 1; }
# Tauri dev owns and stops its Node child. Packaging remains desktop:build.
npm run desktop:dev
