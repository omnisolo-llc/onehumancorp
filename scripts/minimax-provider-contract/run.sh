#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
python3 scripts/minimax-provider-contract/verify_lock.py
python3 scripts/minimax-provider-contract/prepare.py
before=$(sha256sum scripts/minimax-provider-contract/source-manifest.json)
cargo test --offline --locked --manifest-path scripts/minimax-provider-contract/Cargo.toml "$@"
python3 scripts/minimax-provider-contract/prepare.py
test "$before" = "$(sha256sum scripts/minimax-provider-contract/source-manifest.json)"
printf '%s\n' 'Provider source fingerprint remained unchanged during validation'
