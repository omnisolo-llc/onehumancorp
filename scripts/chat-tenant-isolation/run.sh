#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_CHAT_TEST_DATABASE_URL:?Supply an owned disposable loopback ohc_*_test PostgreSQL database}"
python3 scripts/chat-tenant-isolation/database_guard.py
# The imported Redis pool tests resolve production configuration. Isolate their
# standalone files and prevent ambient operator secrets/endpoints from leaking in.
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL REDIS_URL REDIS_URL_FILE
fixture_home=$(mktemp -d)
trap 'rm -rf "$fixture_home"' EXIT
export USERPROFILE="$fixture_home"
export JWT_SECRET=public-local-chat-regression-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory:
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/chat-tenant-isolation/Cargo.lock
cargo metadata --offline --manifest-path scripts/chat-tenant-isolation/Cargo.toml --format-version 1 >/dev/null
python3 scripts/chat-tenant-isolation/verify_lock.py
python3 scripts/chat-tenant-isolation/prepare.py
before=$(sha256sum scripts/chat-tenant-isolation/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/chat-tenant-isolation/Cargo.toml -- --test-threads=1
python3 scripts/chat-tenant-isolation/prepare.py
test "$before" = "$(sha256sum scripts/chat-tenant-isolation/source-manifest.json)"
