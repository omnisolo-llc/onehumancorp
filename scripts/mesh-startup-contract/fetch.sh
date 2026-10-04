#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
# Resolve the harness package graph without compiling it. The metadata pass
# may download missing locked sources on a cold runner; reject any registry
# version drift before a build can execute, then verify the locked fetch.
cp Cargo.lock scripts/mesh-startup-contract/Cargo.lock
cargo metadata --manifest-path scripts/mesh-startup-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/mesh-startup-contract/verify_lock.py
host=$(rustc -vV | sed -n 's/^host: //p')
test -n "$host"
cargo fetch --locked --manifest-path scripts/mesh-startup-contract/Cargo.toml --target "$host"
