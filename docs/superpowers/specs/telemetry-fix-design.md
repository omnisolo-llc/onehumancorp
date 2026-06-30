# OHC Standalone Telemetry Opt-in & PII Audit Design

## Problem
Currently, OHC could leak PII in traces (`tracing::warn!`, `tracing::error!`), and the Standalone offline mode might try to exfiltrate telemetry if not strictly gated.

## Goals
1. Implement lint/test to enforce PII is not logged unless marked safe, which is achieved with the `pii_leakage_check.sh`.
2. Fix any remaining PII leak bugs (e.g. `billing_api.rs`).
3. Ensure no unconsented telemetry/data exfiltration occurs when `telemetry_enabled` is false, specifically auditing standalone mode and data syncs.
4. Verify tests pass.

## Implementation Plan
1. Fix PII leakage in `src/server/api/billing_api.rs` (done).
2. Write integration tests to prove that telemetry is not sent when `telemetry_enabled=false`. The `telemetry_test.rs` already checks this condition, but we'll ensure `pii_leakage_check.sh` is perfectly passing.
3. Add a check to `server/telemetry/mod.rs` to stop ANY network/disk usage if telemetry is disabled.
4. Run `bazelisk test //...` to ensure all tests pass.
