#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_PUBLICATION_TEST_DATABASE_URL:?Supply an owned disposable PostgreSQL database}"
export OMNISOLO_DATABASE_URL="$OHC_PUBLICATION_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/site-publication/Cargo.lock
cargo metadata --offline --manifest-path scripts/site-publication/Cargo.toml --format-version 1 >/dev/null
python3 scripts/site-publication/verify_lock.py
python3 scripts/site-publication/prepare.py
before=$(sha256sum scripts/site-publication/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/site-publication/Cargo.toml -- --test-threads=1
python3 scripts/site-publication/prepare.py
test "$before" = "$(sha256sum scripts/site-publication/source-manifest.json)"
