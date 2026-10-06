#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
python3 scripts/department-delivery-contract/prepare.py
cp Cargo.lock scripts/department-delivery-contract/Cargo.lock
cargo metadata --manifest-path scripts/department-delivery-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/department-delivery-contract/verify_lock.py
cargo fetch --locked --manifest-path scripts/department-delivery-contract/Cargo.toml
