#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_FIELD_TEST_DATABASE_URL:?Explicit owned field test URL required}"
python3 - <<'PY'
import importlib.util,os
spec=importlib.util.spec_from_file_location('guard','scripts/site-publication/database_guard.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);m.validate_owned_database(os.environ['OHC_FIELD_TEST_DATABASE_URL'])
PY
export OHC_SYNC_TEST_DATABASE_URL="$OHC_FIELD_TEST_DATABASE_URL"
export JWT_SECRET=public-local-field-regression-signing-key OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory: OMNISOLO_MULTITENANT=true
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL REDIS_URL REDIS_URL_FILE
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/field-boundary-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/field-boundary-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/field-boundary-contract/verify_lock.py
python3 scripts/field-boundary-contract/prepare.py
before=$(sha256sum scripts/field-boundary-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/field-boundary-contract/Cargo.toml "$@" -- --include-ignored --test-threads=1 || status=$?
python3 scripts/field-boundary-contract/prepare.py
test "$before" = "$(sha256sum scripts/field-boundary-contract/source-manifest.json)"
exit "$status"
