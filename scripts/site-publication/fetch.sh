#!/usr/bin/env bash
# Fetch only the locked focused graph; all compilation/tests stay offline.
set -euo pipefail
cd "$(dirname "$0")/../.."
python3 scripts/site-publication/verify_lock.py
host=$(rustc -vV | sed -n 's/^host: //p')
test -n "$host"
cargo fetch --locked --manifest-path scripts/site-publication/Cargo.toml --target "$host"
python3 scripts/site-publication/verify_lock.py
