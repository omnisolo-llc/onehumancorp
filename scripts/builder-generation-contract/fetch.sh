#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
python3 scripts/builder-generation-contract/verify_lock.py
cargo fetch --locked --manifest-path scripts/builder-generation-contract/Cargo.toml
python3 scripts/builder-generation-contract/verify_lock.py
