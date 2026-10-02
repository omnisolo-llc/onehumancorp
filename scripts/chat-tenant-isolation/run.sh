#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_CHAT_TEST_DATABASE_URL:?Supply an owned disposable loopback ohc_*_test PostgreSQL database}"
python3 scripts/chat-tenant-isolation/database_guard.py
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/chat-tenant-isolation/Cargo.lock
cargo metadata --offline --manifest-path scripts/chat-tenant-isolation/Cargo.toml --format-version 1 >/dev/null
python3 scripts/chat-tenant-isolation/verify_lock.py
python3 scripts/chat-tenant-isolation/prepare.py
before=$(sha256sum scripts/chat-tenant-isolation/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/chat-tenant-isolation/Cargo.toml -- --test-threads=1
python3 scripts/chat-tenant-isolation/prepare.py
test "$before" = "$(sha256sum scripts/chat-tenant-isolation/source-manifest.json)"
