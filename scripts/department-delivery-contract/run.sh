#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
: "${OHC_DEPARTMENT_TEST_DATABASE_URL:?Explicit owned department PostgreSQL URL required}"
python3 - <<'PY'
import importlib.util,os
spec=importlib.util.spec_from_file_location('guard','scripts/site-publication/database_guard.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);m.validate_owned_database(os.environ['OHC_DEPARTMENT_TEST_DATABASE_URL'])
PY
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
python3 scripts/department-delivery-contract/prepare.py
cp Cargo.lock scripts/department-delivery-contract/Cargo.lock
cargo metadata --offline --manifest-path scripts/department-delivery-contract/Cargo.toml --format-version 1 >/dev/null
python3 scripts/department-delivery-contract/verify_lock.py
before=$(sha256sum scripts/department-delivery-contract/source-manifest.json)
status=0
cargo test --locked --offline --manifest-path scripts/department-delivery-contract/Cargo.toml -- --include-ignored --test-threads=1 2>&1 | tee scripts/department-delivery-contract/run.log || status=$?
python3 scripts/department-delivery-contract/prepare.py
test "$before" = "$(sha256sum scripts/department-delivery-contract/source-manifest.json)"
python3 - <<'PY'
from pathlib import Path
log=Path('scripts/department-delivery-contract/run.log').read_text()
for name in ['manual_one_mirror_feed_dismissal_survives_late_provider_acceptance','pg_department_one_mirror_feed_dismissal_preserves_projection_and_acceptance','pg_manual_concurrent_legacy_retry_cross_tab_and_rls','pg_manual_stale_prepare_and_dismissal_cannot_send','pg_manual_rejection_retires_old_department_approval_but_new_review_recovers','pg_manual_department_concurrent_attempts_share_one_effect_fence','pg_department_inflight_manual_dismiss_preserves_visibility_and_receipt','manual_owned_http_validated_acceptance_and_malformed_rejection_unknown','manual_intent_revision_advances_only_for_new_admitted_identity','pg_modern_cancellation_and_snapshot_drift_cannot_be_bypassed','pg_receipt_and_tenant_rls','pg_approval_revocation_wins_before_claim','pg_concurrent_and_restarted_attempts_are_fenced','pg_credential_revocation_wins_before_claim','actual_meta_http_adapter_requires_a_correlated_whatsapp_id','actual_meta_http_adapter_separates_rejection_from_ambiguous_server_outcomes']:
    if not any(name+' ... ok' in line for line in log.splitlines()): raise SystemExit('Required test did not pass: '+name)
PY
exit "$status"
