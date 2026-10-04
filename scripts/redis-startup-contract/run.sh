#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
unset REDIS_URL REDIS_URL_FILE OMNISOLO_REDIS_URL OMNISOLO_REDIS_URL_FILE
unset JWT_SECRET_FILE OMNISOLO_JWT_SECRET_FILE
private_state=$(mktemp -d)
trap 'rm -rf "$private_state"' EXIT
export USERPROFILE="$private_state" JWT_SECRET=public-local-startup-regression-key
unset OMNISOLO_DATABASE_URL OMNISOLO_DATABASE_URL_FILE DATABASE_URL DATABASE_URL_FILE
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/redis-startup-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/redis-startup-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/redis-startup-contract/verify_lock.py
python3 scripts/redis-startup-contract/prepare.py
before=$(sha256sum scripts/redis-startup-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/redis-startup-contract/Cargo.toml "$@" -- --test-threads=1 || status=$?
python3 scripts/redis-startup-contract/prepare.py
test "$before" = "$(sha256sum scripts/redis-startup-contract/source-manifest.json)"
exit "$status"
