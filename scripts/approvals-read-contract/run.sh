#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_APPROVAL_TEST_DATABASE_URL:?owned PostgreSQL required}"
# Claims resolve production configuration even in this read-only probe. Keep
# implicit standalone files and ambient database/Redis/signing settings isolated.
unset JWT_SECRET_FILE OMNISOLO_JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL
unset REDIS_URL REDIS_URL_FILE OMNISOLO_REDIS_URL OMNISOLO_REDIS_URL_FILE
fixture_home=$(mktemp -d)
trap 'rm -rf -- "$fixture_home"' EXIT
export USERPROFILE="$fixture_home"
export JWT_SECRET=public-local-approval-regression-signing-key-only
export OMNISOLO_STANDALONE_MODE=false OMNISOLO_DATABASE_URL=sqlite::memory:
cp Cargo.lock scripts/approvals-read-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/approvals-read-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/approvals-read-contract/verify_lock.py
python3 scripts/approvals-read-contract/prepare.py
before=$(sha256sum scripts/approvals-read-contract/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/approvals-read-contract/Cargo.toml -- --test-threads=1
python3 scripts/approvals-read-contract/prepare.py
test "$before" = "$(sha256sum scripts/approvals-read-contract/source-manifest.json)"
