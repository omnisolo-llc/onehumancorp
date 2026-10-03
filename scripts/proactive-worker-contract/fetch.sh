#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
python3 scripts/proactive-worker-contract/verify_lock.py
host=$(rustc -vV | sed -n 's/^host: //p')
test -n "$host"
cargo fetch --locked --manifest-path scripts/proactive-worker-contract/Cargo.toml --target "$host"
