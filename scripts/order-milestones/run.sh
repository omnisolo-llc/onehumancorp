#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_MILESTONE_TEST_DATABASE_URL:?Supply an isolated disposable PostgreSQL database}"
python3 scripts/order-milestones/database_guard.py
python3 scripts/order-milestones/test_database_guard.py
export OMNISOLO_DATABASE_URL="$OHC_MILESTONE_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export JWT_SECRET=local-bio-regression-public-fixture-only
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/order-milestones/Cargo.lock
cargo metadata --offline --manifest-path scripts/order-milestones/Cargo.toml --format-version 1 >/dev/null
python3 scripts/order-milestones/verify_lock.py
python3 scripts/order-milestones/source_contracts.py
python3 scripts/order-milestones/prepare.py
before=$(sha256sum scripts/order-milestones/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/order-milestones/Cargo.toml -- --test-threads=1
python3 scripts/order-milestones/prepare.py
test "$before" = "$(sha256sum scripts/order-milestones/source-manifest.json)"
