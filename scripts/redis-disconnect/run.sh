#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_REDIS_SERVICE_ISOLATION:?Declare an owned disposable Redis process with OHC_REDIS_SERVICE_ISOLATION=1}"
python3 - <<'PY'
import os
from scripts.ignored_rust_gate import validate_service_url
if os.environ['OHC_REDIS_SERVICE_ISOLATION'] != '1': raise ValueError('Owned Redis declaration required')
url = validate_service_url(os.environ['OHC_TEST_REDIS_URL'], 'redis')
if os.environ['REDIS_URL'] != url: raise ValueError('Redis endpoints must match')
validate_service_url(os.environ['OMNISOLO_DATABASE_URL'], 'postgres', 'ohc_ignored_postgres')
if os.environ.get('OMNISOLO_STANDALONE_MODE') != 'false': raise ValueError('Explicit hosted-mode test configuration required')
PY
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
private_state=$(mktemp -d)
trap 'rm -rf "$private_state"' EXIT
export USERPROFILE="$private_state"
cp Cargo.lock scripts/redis-disconnect/Cargo.lock
cargo metadata --manifest-path scripts/redis-disconnect/Cargo.toml --format-version 1 >/dev/null
python3 scripts/redis-disconnect/verify_lock.py
python3 scripts/redis-disconnect/prepare.py
before=$(sha256sum scripts/redis-disconnect/source-manifest.json)
cargo test --locked --manifest-path scripts/redis-disconnect/Cargo.toml -- --include-ignored --test-threads=1
python3 scripts/redis-disconnect/prepare.py
after=$(sha256sum scripts/redis-disconnect/source-manifest.json)
test "$before" = "$after" || { echo 'Redis probe sources changed during execution' >&2; exit 1; }
