# Spec: ML-Resilience 60-Second Timeout Rule
## Overview
The ML-Resilience rules require that AI agent jobs have a 60-second timeout with automatic retry (max 3 attempts). This rule applies to both Cloud and Standalone mode (Mode Parity).

Currently, `src/server/db.rs` specifies `let max_attempts = 10;`. Also, testing code `src/server/orchestration/state/parity_test.rs` specifically tests for `max_attempts = 10` for `execute_with_retry`.

This specification outlines changing `max_attempts` to 3 to comply with the ML-Resilience rules, and verifying the change via the parity test.

## Changes
1. Modify `src/server/db.rs` -> `execute_with_retry` method to use `let max_attempts = 3;` instead of `10`.
2. Update tests in `src/server/orchestration/state/parity_test.rs`:
   - Change assertions checking for 10 attempts to check for 3.
   - Example: `assert_eq!(*attempts.lock().unwrap(), 10);` -> `assert_eq!(*attempts.lock().unwrap(), 3);` in `test_execute_with_retry_chaos_exhaustion`.

## Trade-offs
None - this is explicitly requested as an absolute requirement in the prompt.
