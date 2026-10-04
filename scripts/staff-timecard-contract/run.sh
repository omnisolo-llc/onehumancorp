#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_CLOCK_TEST_DATABASE_URL:?explicit owned loopback PostgreSQL fixture required}"
python3 scripts/staff-timecard-contract/database_guard.py
if ! command -v psql >/dev/null || ! clock_pg_reply=$(PGCONNECT_TIMEOUT=5 psql --no-psqlrc --quiet --tuples-only --no-align --dbname="$OHC_CLOCK_TEST_DATABASE_URL" --command='SELECT 1' 2>/dev/null) || [[ "$clock_pg_reply" != 1 ]]; then
  echo 'Owned PostgreSQL fixture is unavailable; clock acceptance did not run.' >&2
  exit 1
fi
unset JWT_SECRET_FILE OMNISOLO_JWT_SECRET_FILE
unset OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL OHC_DATABASE_URL
unset REDIS_URL REDIS_URL_FILE OMNISOLO_REDIS_URL OMNISOLO_REDIS_URL_FILE
unset OHC_SYNC_TEST_DATABASE_URL OMNISOLO_POSTGRES_ADMIN_URL
clock_runtime_home=$(mktemp -d)
trap 'rm -rf -- "$clock_runtime_home"' EXIT
export USERPROFILE="$clock_runtime_home"
export JWT_SECRET=public-local-clock-regression-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory: OMNISOLO_MULTITENANT=true
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
python3 scripts/staff-timecard-contract/verify_lock.py
python3 scripts/staff-timecard-contract/prepare.py
before=$(sha256sum scripts/staff-timecard-contract/source-manifest.json)
status=0
# No filters, ignored tests, or backend fallback: both stores must execute.
cargo test --locked --offline --manifest-path scripts/staff-timecard-contract/Cargo.toml -- --test-threads=1 || status=$?
python3 scripts/staff-timecard-contract/prepare.py
if [[ "$before" != "$(sha256sum scripts/staff-timecard-contract/source-manifest.json)" ]]; then
  echo 'Clock source changed during verification; final-source evidence rejected.' >&2
  exit 1
fi
exit "$status"
