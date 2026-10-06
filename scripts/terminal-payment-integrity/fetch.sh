#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
cp Cargo.lock scripts/terminal-payment-integrity/Cargo.lock
cargo metadata --manifest-path scripts/terminal-payment-integrity/Cargo.toml --format-version 1 >/dev/null
python3 scripts/terminal-payment-integrity/verify_lock.py
cargo fetch --locked --manifest-path scripts/terminal-payment-integrity/Cargo.toml
