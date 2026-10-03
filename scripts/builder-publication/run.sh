#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_BUILDER_TEST_DATABASE_URL:?Supply an owned disposable PostgreSQL database}"
export OMNISOLO_DATABASE_URL="$OHC_BUILDER_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/builder-publication/Cargo.lock
cargo metadata --manifest-path scripts/builder-publication/Cargo.toml --format-version 1 >/dev/null
python scripts/builder-publication/verify_lock.py
python scripts/builder-publication/prepare.py
before=$(sha256sum scripts/builder-publication/source-manifest.json)
cargo test --locked --manifest-path scripts/builder-publication/Cargo.toml -- --test-threads=1
python scripts/builder-publication/prepare.py
test "$before" = "$(sha256sum scripts/builder-publication/source-manifest.json)"
