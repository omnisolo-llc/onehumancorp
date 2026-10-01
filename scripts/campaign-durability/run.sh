#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_CAMPAIGN_TEST_DATABASE_URL:?Supply a disposable PostgreSQL test database with CREATE ROLE/SCHEMA authority}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/campaign-durability/Cargo.lock
cargo metadata --manifest-path scripts/campaign-durability/Cargo.toml --format-version 1 >/dev/null
python scripts/campaign-durability/verify_lock.py
python scripts/campaign-durability/prepare.py
before=$(sha256sum scripts/campaign-durability/source-manifest.json)
cargo test --locked --manifest-path scripts/campaign-durability/Cargo.toml -- --include-ignored --test-threads=2
python scripts/campaign-durability/prepare.py
after=$(sha256sum scripts/campaign-durability/source-manifest.json)
test "$before" = "$after" || { echo "Campaign sources changed during validation" >&2; exit 1; }
python scripts/campaign-durability/source_contract_test.py
