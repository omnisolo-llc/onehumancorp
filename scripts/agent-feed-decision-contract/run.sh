#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_FEED_TEST_DATABASE_URL:?explicit disposable PostgreSQL prerequisite required}"
python3 scripts/agent-feed-decision-contract/database_guard.py
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL REDIS_URL REDIS_URL_FILE
export JWT_SECRET=public-local-feed-regression-key-only OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory:
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/agent-feed-decision-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/agent-feed-decision-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/agent-feed-decision-contract/verify_lock.py
python3 scripts/agent-feed-decision-contract/prepare.py
before=$(sha256sum scripts/agent-feed-decision-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/agent-feed-decision-contract/Cargo.toml "$@" -- --test-threads=1 || status=$?
python3 scripts/agent-feed-decision-contract/prepare.py
test "$before" = "$(sha256sum scripts/agent-feed-decision-contract/source-manifest.json)"
exit "$status"
