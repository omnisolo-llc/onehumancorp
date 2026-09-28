issue_title: "Blocked no-work finding: F13"
issue_description: |
  **Title:** F13: BYOK versus subscriptions

  **Problem Statement:** The audit requested to evaluate "F13: BYOK versus subscriptions", checking that API key, consumer plan and native-client subscription are distinct, and that the API proxy rejects unsupported subscription-relay modes, session-token relay, pooling, etc.

  **Research Report:**
  The codebase already fully implements the requested constraints for F13.
  - In `src/server/harness/middleware/provider_facade.rs`, if `meter.scope.payer == PayerMode::NativeSubscription`, the facade explicitly returns an HTTP FORBIDDEN error stating: "Provider-permitted native-client subscription hosting is not supported for proxying. Session-token relay, pooling, silent paid fallback, and rebilling direct inference are disabled."
  - In `src/server/harness/middleware/usage_meter.rs`, `UsageMeterSettings::from_environment` rejects initialization if `OMNISOLO_USAGE_PAYER` is set to `native_subscription`.
  - In `src/server/harness/tests/provider_facade.rs`, `facade_rejects_native_subscription_proxying()` explicitly tests this behavior.
  Therefore, no new code changes are required as the features are already present and verified.
