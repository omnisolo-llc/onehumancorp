#!/usr/bin/env bash
# Native local API startup. Run npm run dev:web in another terminal for the UI.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
command -v cargo >/dev/null || { echo "Install the pinned Rust toolchain and add Cargo to PATH" >&2; exit 1; }
export OMNISOLO_MULTITENANT=false OMNISOLO_SOURCE_MODE=standalone
export TOKIO_WORKER_THREADS="${TOKIO_WORKER_THREADS:-2}"
export RAYON_NUM_THREADS="${RAYON_NUM_THREADS:-2}"
cargo build --locked -p omnisolo --bin server
# exec preserves the server's exit status and signal handling; no orphan wrapper.
exec ./target/debug/server "$@"
