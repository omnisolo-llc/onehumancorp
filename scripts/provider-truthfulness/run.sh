#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/provider-truthfulness/Cargo.lock
cargo metadata --offline --manifest-path scripts/provider-truthfulness/Cargo.toml --format-version 1 >/dev/null
python scripts/provider-truthfulness/verify_lock.py
python scripts/provider-truthfulness/prepare.py
before=$(sha256sum scripts/provider-truthfulness/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/provider-truthfulness/Cargo.toml -- --test-threads=1
python scripts/provider-truthfulness/prepare.py
test "$before" = "$(sha256sum scripts/provider-truthfulness/source-manifest.json)"
