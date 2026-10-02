#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_APPOINTMENTS_TEST_DATABASE_URL:?Supply an isolated disposable PostgreSQL database}"
python3 scripts/operations-appointments/database_guard.py
export OMNISOLO_DATABASE_URL="$OHC_APPOINTMENTS_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export JWT_SECRET=local-search-regression-public-fixture-only
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/operations-appointments/Cargo.lock
cargo metadata --offline --manifest-path scripts/operations-appointments/Cargo.toml --format-version 1 >/dev/null
python3 scripts/operations-appointments/verify_lock.py
python3 scripts/operations-appointments/source_contracts.py
python3 scripts/operations-appointments/prepare.py
before=$(sha256sum scripts/operations-appointments/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/operations-appointments/Cargo.toml -- --test-threads=1
python3 scripts/operations-appointments/prepare.py
test "$before" = "$(sha256sum scripts/operations-appointments/source-manifest.json)"
