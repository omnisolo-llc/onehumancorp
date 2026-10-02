#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_SETUP_TEST_DATABASE_URL:?Supply an isolated disposable PostgreSQL database}"
export OMNISOLO_DATABASE_URL="$OHC_SETUP_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export JWT_SECRET=public-local-bootstrap-regression-fixture-only
export OMNISOLO_SETUP_TOKEN=public-local-setup-token-at-least-thirty-two-bytes
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/bootstrap-portable-roles/Cargo.lock
cargo metadata --offline --manifest-path scripts/bootstrap-portable-roles/Cargo.toml --format-version 1 >/dev/null
python3 scripts/bootstrap-portable-roles/verify_lock.py
python3 scripts/bootstrap-portable-roles/prepare.py
before=$(sha256sum scripts/bootstrap-portable-roles/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/bootstrap-portable-roles/Cargo.toml -- --test-threads=1
python3 scripts/bootstrap-portable-roles/prepare.py
test "$before" = "$(sha256sum scripts/bootstrap-portable-roles/source-manifest.json)"
