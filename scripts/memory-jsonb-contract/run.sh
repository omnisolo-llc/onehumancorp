#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_MEMORY_TEST_DATABASE_URL:?explicit disposable PostgreSQL prerequisite required}"
OHC_WIDGET_TEST_DATABASE_URL="$OHC_MEMORY_TEST_DATABASE_URL" python3 scripts/widget-chat-contract/database_guard.py
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2} CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/memory-jsonb-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/memory-jsonb-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/memory-jsonb-contract/verify_lock.py
python3 scripts/memory-jsonb-contract/prepare.py
before=$(sha256sum scripts/memory-jsonb-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/memory-jsonb-contract/Cargo.toml -- --test-threads=1 || status=$?
python3 scripts/memory-jsonb-contract/prepare.py
test "$before" = "$(sha256sum scripts/memory-jsonb-contract/source-manifest.json)"
exit "$status"
