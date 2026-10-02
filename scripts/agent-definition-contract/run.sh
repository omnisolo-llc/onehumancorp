#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_AGENT_DEFINITION_TEST_DATABASE_URL:?explicit disposable PostgreSQL URL required}"
python3 scripts/agent-definition-contract/database_guard.py
python3 scripts/agent-definition-contract/test_database_guard.py
python3 scripts/agent-definition-authority/test_sqlite.py
node --test scripts/agent-definition-wiring.test.mjs

# This configuration belongs only to the fixture process; no application setting
# or external credential is read or changed by these tests.
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE REDIS_URL REDIS_URL_FILE
export JWT_SECRET=public-local-definition-regression-signing-key-only
export OMNISOLO_SETUP_TOKEN=public-local-definition-setup-token-at-least-thirty-two-bytes
unset OMNISOLO_SETUP_TOKEN_FILE
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory:
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/agent-definition-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/agent-definition-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/agent-definition-contract/verify_lock.py
python3 scripts/agent-definition-contract/prepare.py
before=$(sha256sum scripts/agent-definition-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/agent-definition-contract/Cargo.toml -- --test-threads=1 || status=$?
python3 scripts/agent-definition-contract/prepare.py
test "$before" = "$(sha256sum scripts/agent-definition-contract/source-manifest.json)"
exit "$status"
