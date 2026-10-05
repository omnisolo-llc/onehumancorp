#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
python3 scripts/redis-reconnect/verify_lock.py
redis_host=$(rustc -vV | sed -n 's/^host: //p')
test -n "$redis_host"
cargo fetch --locked --manifest-path scripts/redis-reconnect/Cargo.toml --target "$redis_host"
