#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/voice-provisioning-gate/Cargo.lock
cargo metadata --offline --manifest-path scripts/voice-provisioning-gate/Cargo.toml --format-version 1 >/dev/null
python scripts/voice-provisioning-gate/verify_lock.py
python scripts/voice-provisioning-gate/prepare.py
before=$(sha256sum scripts/voice-provisioning-gate/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/voice-provisioning-gate/Cargo.toml -- --test-threads=1
python scripts/voice-provisioning-gate/prepare.py
test "$before" = "$(sha256sum scripts/voice-provisioning-gate/source-manifest.json)"
