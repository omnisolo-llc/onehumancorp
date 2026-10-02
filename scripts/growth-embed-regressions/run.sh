#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_GROWTH_TEST_DATABASE_URL:?Supply an isolated local PostgreSQL test database}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/growth-embed-regressions/Cargo.lock
cargo metadata --offline --manifest-path scripts/growth-embed-regressions/Cargo.toml --format-version 1 >/dev/null
python3 scripts/growth-embed-regressions/verify_lock.py
python3 scripts/growth-embed-regressions/prepare.py
before=$(sha256sum scripts/growth-embed-regressions/source-manifest.json)
set +e
cargo test --locked --offline --manifest-path scripts/growth-embed-regressions/Cargo.toml -- --test-threads=1
result=$?
set -e
python3 scripts/growth-embed-regressions/prepare.py
after=$(sha256sum scripts/growth-embed-regressions/source-manifest.json)
test "$before" = "$after" || { echo "Sources changed during validation" >&2; exit 1; }
exit "$result"
