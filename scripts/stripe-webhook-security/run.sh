#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
cp Cargo.lock scripts/stripe-webhook-security/Cargo.lock
cargo metadata --manifest-path scripts/stripe-webhook-security/Cargo.toml --format-version 1 >/dev/null
python scripts/stripe-webhook-security/verify_lock.py
python scripts/stripe-webhook-security/source_contracts.py
python scripts/stripe-webhook-security/prepare.py
before=$(sha256sum scripts/stripe-webhook-security/source-manifest.json)
env -u STRIPE_WEBHOOK_SECRET -u STRIPE_WEBHOOK_SECRET_FILE -u STRIPE_LEDGER_WEBHOOK_SECRET -u STRIPE_LEDGER_WEBHOOK_SECRET_FILE cargo test --locked --manifest-path scripts/stripe-webhook-security/Cargo.toml -- --test-threads=1
python scripts/stripe-webhook-security/prepare.py
test "$before" = "$(sha256sum scripts/stripe-webhook-security/source-manifest.json)"
