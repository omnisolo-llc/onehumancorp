#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_STAFF_TEST_DATABASE_URL:?Supply an owned disposable PostgreSQL database}"
python3 - <<'PY'
import importlib.util,os
spec=importlib.util.spec_from_file_location('guard','scripts/site-publication/database_guard.py')
guard=importlib.util.module_from_spec(spec);spec.loader.exec_module(guard)
guard.validate_owned_database(os.environ['OHC_STAFF_TEST_DATABASE_URL'])
PY
export JWT_SECRET=public-local-staff-regression-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory: OMNISOLO_MULTITENANT=true
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
python3 scripts/staff-read-contract/verify_lock.py
python3 scripts/staff-read-contract/prepare.py
before=$(sha256sum scripts/staff-read-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/staff-read-contract/Cargo.toml -- --test-threads=1 || status=$?
python3 scripts/staff-read-contract/prepare.py
test "$before" = "$(sha256sum scripts/staff-read-contract/source-manifest.json)"
exit "$status"
