issue_title: "Blocked no-work finding: F13"
issue_description: |
  **Title**: Blocked no-work finding: F13

  **Problem Statement**: The prompt requests implementing the finding from the completed research report (F13) immediately. F13 is described as "API key, consumer plan and native-client subscription are distinct" and notes "Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference".

  **Research Report**:
  I have investigated the codebase and found that the logic for F13 is already implemented. The file `src/server/harness/middleware/provider_facade.rs` contains the following logic:

  ```rust
  if let Some(meter) = state.meter.as_ref() {
      if meter.scope.payer == super::usage_ledger::PayerMode::NativeSubscription {
          return error_response(
              StatusCode::FORBIDDEN,
              "Provider-permitted native-client subscription hosting is not supported for proxying. \
               Session-token relay, pooling, silent paid fallback, and rebilling direct inference are disabled.",
          );
      }
  }
  ```

  This explicit check correctly identifies the `NativeSubscription` payer mode and explicitly rejects the unsupported combination, implementing exactly the constraints described in F13. It prevents session-token relay, pooling, silent paid fallback, and rebilling direct inference.
  Because this behavior is already safely handled in the codebase, no further code changes are required for F13. Therefore, this is a blocked no-work finding.

  **Executed test commands:**
  - `cargo test --lib middleware::provider_facade` (Timed out)
  - `cd src/server/harness && cargo test --lib middleware::provider_facade` (Passed)
  - `cargo test provider_facade` (Passed)

  **Verified trace limitations:**
  - E2E tests and broader Cargo test suite timed out due to the 400-second session limit in the testing environment (e.g., `make test-backend` timed out after 402.01s).
