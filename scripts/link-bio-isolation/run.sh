#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_BIO_TEST_DATABASE_URL:?Supply an isolated disposable PostgreSQL database}"
python3 scripts/link-bio-isolation/database_guard.py
python3 scripts/link-bio-isolation/test_database_guard.py
export OMNISOLO_DATABASE_URL="$OHC_BIO_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export JWT_SECRET=local-bio-regression-public-fixture-only
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/link-bio-isolation/Cargo.lock
cargo metadata --offline --manifest-path scripts/link-bio-isolation/Cargo.toml --format-version 1 >/dev/null
python3 scripts/link-bio-isolation/verify_lock.py
python3 scripts/link-bio-isolation/source_contracts.py
python3 scripts/link-bio-isolation/prepare.py
before=$(sha256sum scripts/link-bio-isolation/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/link-bio-isolation/Cargo.toml -- --test-threads=1
python3 scripts/link-bio-isolation/prepare.py
test "$before" = "$(sha256sum scripts/link-bio-isolation/source-manifest.json)"
