#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_AGENT_RECEIPT_TEST_DATABASE_URL:?explicit owned PostgreSQL database required}"
OHC_AGENT_DEFINITION_TEST_DATABASE_URL="$OHC_AGENT_RECEIPT_TEST_DATABASE_URL" python3 scripts/agent-definition-contract/database_guard.py
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE REDIS_URL REDIS_URL_FILE
export JWT_SECRET=public-local-receipt-postgres-fixture-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_MULTITENANT=false OMNISOLO_DATABASE_URL=sqlite::memory:
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cargo metadata --locked --offline --manifest-path scripts/agent-receipt-postgres-contract/Cargo.toml --format-version 1 >/dev/null
export NODE_PATH="$PWD/scripts/agent-workflow-contract/node_modules:$PWD/src/ui/next/node_modules${NODE_PATH:+:$NODE_PATH}"
python3 scripts/agent-workflow-contract/verify_node_lock.py
python3 scripts/agent-receipt-postgres-contract/verify_lock.py
python3 scripts/agent-receipt-postgres-contract/prepare.py
before=$(sha256sum scripts/agent-receipt-postgres-contract/source-manifest.json)
generated_before=$(sha256sum scripts/agent-receipt-postgres-contract/generated.rs scripts/agent-receipt-postgres-contract/core_pg.sql)
echo "$generated_before"
status=0
cargo test --locked --offline --manifest-path scripts/agent-receipt-postgres-contract/Cargo.toml -- --test-threads=1 || status=$?
test "$generated_before" = "$(sha256sum scripts/agent-receipt-postgres-contract/generated.rs scripts/agent-receipt-postgres-contract/core_pg.sql)"
python3 scripts/agent-receipt-postgres-contract/prepare.py
test "$before" = "$(sha256sum scripts/agent-receipt-postgres-contract/source-manifest.json)"
test "$generated_before" = "$(sha256sum scripts/agent-receipt-postgres-contract/generated.rs scripts/agent-receipt-postgres-contract/core_pg.sql)"
exit "$status"
