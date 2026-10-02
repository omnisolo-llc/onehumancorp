#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_QUOTE_TEST_DATABASE_URL:?Supply an isolated disposable PostgreSQL database with role/schema creation permission}"
export OMNISOLO_DATABASE_URL="$OHC_QUOTE_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/quote-acceptance/Cargo.lock
cargo metadata --manifest-path scripts/quote-acceptance/Cargo.toml --format-version 1 >/dev/null
python scripts/quote-acceptance/verify_lock.py
python scripts/quote-acceptance/source_contracts.py
python scripts/quote-acceptance/prepare.py
before=$(sha256sum scripts/quote-acceptance/source-manifest.json)
env -u STRIPE_API_KEY -u STRIPE_API_BASE -u PUBLIC_APP_URL cargo test --locked --manifest-path scripts/quote-acceptance/Cargo.toml -- --test-threads=1
python scripts/quote-acceptance/prepare.py
test "$before" = "$(sha256sum scripts/quote-acceptance/source-manifest.json)"
