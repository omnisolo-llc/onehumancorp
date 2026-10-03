#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_SERVICE_TEST_DATABASE_URL:?Supply an owned disposable PostgreSQL database}"
export OMNISOLO_DATABASE_URL="$OHC_SERVICE_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export JWT_SECRET=local-service-creation-regression-public-fixture-only
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/service-creation/Cargo.lock
cargo metadata --offline --manifest-path scripts/service-creation/Cargo.toml --format-version 1 >/dev/null
python3 scripts/service-creation/verify_lock.py
python3 scripts/service-creation/prepare.py
before=$(sha256sum scripts/service-creation/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/service-creation/Cargo.toml -- --test-threads=1
python3 scripts/service-creation/prepare.py
test "$before" = "$(sha256sum scripts/service-creation/source-manifest.json)"
