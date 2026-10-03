#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0
python3 scripts/proactive-worker-contract/verify_lock.py
python3 scripts/proactive-worker-contract/prepare.py
before=$(sha256sum scripts/proactive-worker-contract/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/proactive-worker-contract/Cargo.toml -- --test-threads=1
python3 scripts/proactive-worker-contract/prepare.py
test "$before" = "$(sha256sum scripts/proactive-worker-contract/source-manifest.json)"
