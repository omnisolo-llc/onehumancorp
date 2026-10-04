#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
# Metadata may populate a cold registry, but no build runs before lock parity.
cp Cargo.lock scripts/cash-receipts/Cargo.lock
cargo metadata --manifest-path scripts/cash-receipts/Cargo.toml --format-version 1 >/dev/null
python3 scripts/cash-receipts/verify_lock.py
cash_host=$(rustc -vV | sed -n 's/^host: //p')
test -n "$cash_host"
cargo fetch --locked --manifest-path scripts/cash-receipts/Cargo.toml --target "$cash_host"
