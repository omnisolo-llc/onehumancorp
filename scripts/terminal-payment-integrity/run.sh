#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_TERMINAL_TEST_DATABASE_URL:?An owned disposable PostgreSQL test database is required}"
command -v cargo >/dev/null || { echo 'Rust runtime unavailable; terminal protocol tests did not run.' >&2; exit 1; }
unset STRIPE_API_KEY STRIPE_SECRET_KEY STRIPE_API_BASE
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0
node --test scripts/terminal-payment-integrity/source.test.mjs
cp Cargo.lock scripts/terminal-payment-integrity/Cargo.lock
cargo metadata --offline --manifest-path scripts/terminal-payment-integrity/Cargo.toml --format-version 1 >/dev/null
python3 scripts/terminal-payment-integrity/verify_lock.py
python3 scripts/terminal-payment-integrity/prepare_token.py
python3 scripts/terminal-payment-integrity/manifest.py
before=$(sha256sum scripts/terminal-payment-integrity/source-manifest.json)
cargo test --locked --offline --manifest-path scripts/terminal-payment-integrity/Cargo.toml -- --test-threads=1
python3 scripts/terminal-payment-integrity/manifest.py
test "$before" = "$(sha256sum scripts/terminal-payment-integrity/source-manifest.json)"
