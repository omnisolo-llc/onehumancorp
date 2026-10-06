#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_SMS_TEST_DATABASE_URL:?Explicit owned SMS PostgreSQL URL required}"
python3 - <<'PY'
import importlib.util,os
spec=importlib.util.spec_from_file_location('guard','scripts/site-publication/database_guard.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);m.validate_owned_database(os.environ['OHC_SMS_TEST_DATABASE_URL'])
PY
export JWT_SECRET=public-local-sms-regression-signing-key OMNISOLO_STANDALONE_MODE=false OMNISOLO_MULTITENANT=true OMNISOLO_DATABASE_URL=sqlite::memory:
unset JWT_SECRET_FILE OMNISOLO_DATABASE_URL_FILE DATABASE_URL_FILE DATABASE_URL REDIS_URL REDIS_URL_FILE
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
cp Cargo.lock scripts/sms-verification-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/sms-verification-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/sms-verification-contract/verify_lock.py
python3 scripts/sms-verification-contract/prepare.py
before=$(sha256sum scripts/sms-verification-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/sms-verification-contract/Cargo.toml -- --include-ignored --test-threads=1 2>&1 | tee scripts/sms-verification-contract/run.log || status=$?
python3 scripts/sms-verification-contract/prepare.py
test "$before" = "$(sha256sum scripts/sms-verification-contract/source-manifest.json)"
python3 - <<'PY'
from pathlib import Path
log=Path('scripts/sms-verification-contract/run.log').read_text()
required=['pg_migration_installs_forced_tenant_rls','pg_claim_locks_fence_optout_demotion_and_deactivation','pg_revocation_winning_before_claim_produces_no_send','pg_rate_and_cooldown_are_durable','pg_empty_event_and_changed_generator_replay_are_terminal','pg_wrong_expired_replayed_and_cross_tenant_proofs_are_rejected']
for name in required:
    if not any(name+' ... ok' in line for line in log.splitlines()): raise SystemExit('Required PostgreSQL test did not execute successfully: '+name)
PY
exit "$status"
