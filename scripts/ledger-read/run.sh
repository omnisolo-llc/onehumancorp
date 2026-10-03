#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_LEDGER_TEST_DATABASE_URL:?Supply an isolated disposable PostgreSQL database}"
export OMNISOLO_DATABASE_URL="$OHC_LEDGER_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export JWT_SECRET=local-ledger-read-regression-public-fixture-only
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/ledger-read/Cargo.lock
cargo metadata --offline --manifest-path scripts/ledger-read/Cargo.toml --format-version 1 >/dev/null
python3 scripts/ledger-read/verify_lock.py
python3 scripts/ledger-read/source_contracts.py
python3 scripts/ledger-read/prepare.py
before=$(sha256sum scripts/ledger-read/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/ledger-read/Cargo.toml -- --test-threads=1
python3 scripts/ledger-read/prepare.py
test "$before" = "$(sha256sum scripts/ledger-read/source-manifest.json)"
