#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_WIDGET_TEST_DATABASE_URL:?explicit disposable PostgreSQL prerequisite required}"
python3 scripts/widget-chat-contract/database_guard.py
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL REDIS_URL REDIS_URL_FILE
export JWT_SECRET=public-local-widget-regression-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory:
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/widget-chat-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/widget-chat-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/widget-chat-contract/verify_lock.py
python3 scripts/widget-chat-contract/prepare.py
before=$(sha256sum scripts/widget-chat-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/widget-chat-contract/Cargo.toml -- --test-threads=1 || status=$?
python3 scripts/widget-chat-contract/prepare.py
test "$before" = "$(sha256sum scripts/widget-chat-contract/source-manifest.json)"
exit "$status"
