#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_SYNC_TEST_DATABASE_URL:?Supply an isolated PostgreSQL test database}"
export OMNISOLO_DATABASE_URL="$OHC_SYNC_TEST_DATABASE_URL"
export OMNISOLO_STANDALONE_MODE=false
export JWT_SECRET=local-sync-regression-secret
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/onboarding-durability/Cargo.lock
cargo metadata --manifest-path scripts/onboarding-durability/Cargo.toml --format-version 1 >/dev/null
python scripts/onboarding-durability/verify_lock.py
python scripts/onboarding-durability/prepare.py
before=$(sha256sum scripts/onboarding-durability/source-manifest.json)
cargo test --locked --manifest-path scripts/onboarding-durability/Cargo.toml -- --include-ignored --test-threads=2
python scripts/onboarding-durability/prepare.py
after=$(sha256sum scripts/onboarding-durability/source-manifest.json)
test "$before" = "$after" || { echo "Sources changed during validation" >&2; exit 1; }
