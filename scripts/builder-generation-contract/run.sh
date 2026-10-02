#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_BUILDER_GENERATION_TEST_DATABASE_URL:?Supply an owned disposable PostgreSQL database}"
python3 - <<'PY'
import importlib.util,os
spec=importlib.util.spec_from_file_location('guard','scripts/site-publication/database_guard.py')
guard=importlib.util.module_from_spec(spec);spec.loader.exec_module(guard)
guard.validate_owned_database(os.environ['OHC_BUILDER_GENERATION_TEST_DATABASE_URL'])
PY
export JWT_SECRET=public-local-builder-generation-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory:
unset REDIS_URL REDIS_URL_FILE JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
python3 scripts/builder-generation-contract/verify_lock.py
cargo metadata --locked --offline --manifest-path scripts/builder-generation-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/builder-generation-contract/verify_lock.py
python3 scripts/builder-generation-contract/prepare.py
before=$(sha256sum scripts/builder-generation-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/builder-generation-contract/Cargo.toml -- --test-threads=1 || status=$?
python3 scripts/builder-generation-contract/prepare.py
test "$before" = "$(sha256sum scripts/builder-generation-contract/source-manifest.json)"
exit "$status"
