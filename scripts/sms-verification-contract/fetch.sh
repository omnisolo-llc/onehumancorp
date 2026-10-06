#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
cp Cargo.lock scripts/sms-verification-contract/Cargo.lock
cargo metadata --manifest-path scripts/sms-verification-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/sms-verification-contract/verify_lock.py
cargo fetch --locked --manifest-path scripts/sms-verification-contract/Cargo.toml
