#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_CASH_TEST_DATABASE_URL:?Owned PostgreSQL required}"
: "${OHC_CASH_TEST_REDIS_URL:?Owned Redis required}"
python3 scripts/cash-receipts/database_guard.py
if ! command -v redis-cli >/dev/null || ! cash_redis_reply=$(redis-cli --raw -u "$OHC_CASH_TEST_REDIS_URL" PING 2>/dev/null) || [[ "$cash_redis_reply" != PONG ]]; then
  echo 'Owned Redis fixture is unavailable; cash acceptance did not run.' >&2
  exit 1
fi
unset OHC_CASH_BASELINE STRIPE_API_KEY STRIPE_API_BASE
unset OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL REDIS_URL REDIS_URL_FILE
unset OMNISOLO_REDIS_URL_FILE JWT_SECRET_FILE OMNISOLO_JWT_SECRET_FILE
cash_runtime_home=$(mktemp -d)
trap 'rm -rf -- "$cash_runtime_home"' EXIT
export USERPROFILE="$cash_runtime_home"
export JWT_SECRET=public-local-cash-regression-key-only
export OMNISOLO_REDIS_URL="$OHC_CASH_TEST_REDIS_URL"
export OMNISOLO_DATABASE_URL="$OHC_CASH_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/cash-receipts/Cargo.lock
cargo metadata --offline --manifest-path scripts/cash-receipts/Cargo.toml --format-version 1 >/dev/null
python3 scripts/cash-receipts/verify_lock.py
python3 scripts/cash-receipts/source_contracts.py
python3 scripts/cash-receipts/prepare.py
before=$(sha256sum scripts/cash-receipts/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/cash-receipts/Cargo.toml -- --test-threads=1
python3 scripts/cash-receipts/prepare.py
test "$before" = "$(sha256sum scripts/cash-receipts/source-manifest.json)"
