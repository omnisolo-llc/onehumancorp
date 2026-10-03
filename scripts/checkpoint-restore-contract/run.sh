#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_CHECKPOINT_TEST_DATABASE_URL:?explicit disposable PostgreSQL URL required}"
python3 scripts/checkpoint-restore-contract/ci.test.py
python3 scripts/checkpoint-restore-contract/prepare.py
before=$(sha256sum scripts/checkpoint-restore-contract/source-manifest.json)
status=0
cargo test --locked --manifest-path scripts/checkpoint-restore-contract/Cargo.toml -- "$@" || status=$?
python3 scripts/checkpoint-restore-contract/prepare.py
test "$before" = "$(sha256sum scripts/checkpoint-restore-contract/source-manifest.json)"
exit "$status"
