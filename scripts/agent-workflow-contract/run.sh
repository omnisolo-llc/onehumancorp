#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export JWT_SECRET=public-local-agent-workflow-regression-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory:
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/agent-workflow-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/agent-workflow-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/agent-workflow-contract/verify_lock.py
python3 scripts/agent-workflow-contract/prepare.py
before=$(sha256sum scripts/agent-workflow-contract/source-manifest.json)
test_status=0
cargo test --locked --offline --manifest-path scripts/agent-workflow-contract/Cargo.toml -- --test-threads=1 || test_status=$?
python3 scripts/agent-workflow-contract/prepare.py
test "$before" = "$(sha256sum scripts/agent-workflow-contract/source-manifest.json)"
exit "$test_status"
