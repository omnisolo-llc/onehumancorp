#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_DRAFT_TEST_DATABASE_URL:?Supply an isolated disposable PostgreSQL database}"
export OMNISOLO_DATABASE_URL="$OHC_DRAFT_TEST_DATABASE_URL" OMNISOLO_STANDALONE_MODE=false
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/onboarding-draft-merge/Cargo.lock
cargo metadata --manifest-path scripts/onboarding-draft-merge/Cargo.toml --format-version 1 >/dev/null
python scripts/onboarding-draft-merge/verify_lock.py
python scripts/onboarding-draft-merge/prepare.py
before=$(sha256sum scripts/onboarding-draft-merge/source-manifest.json)
cargo test --locked --manifest-path scripts/onboarding-draft-merge/Cargo.toml -- --test-threads=1
python scripts/onboarding-draft-merge/prepare.py
test "$before" = "$(sha256sum scripts/onboarding-draft-merge/source-manifest.json)"
