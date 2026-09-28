issue_title: "✍️ Scribe: [blocked no-work finding: F13]"
issue_description: |
  **Title**: F13: API key, consumer plan and native-client subscription are distinct

  **Problem Statement**:
  The issue F13 states that API key, consumer plan, and native-client subscription are distinct, and asks to verify provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback, or rebilling direct inference. The audit/remediation ledger marks this as "Open".

  **Research Report**:
  Upon investigation, the existing codebase already safely handles the described issue. In `src/server/harness/middleware/provider_facade.rs`, the API proxy explicitly rejects unsupported subscription-relay modes, returning a `403 FORBIDDEN` error with the message "Provider-permitted native-client subscription hosting is not supported for proxying. Session-token relay, pooling, silent paid fallback, and rebilling direct inference are disabled." The premise of the report (that it is open and needs fixing) is technically hallucinated as the current codebase accurately addresses the requirements.
  Additionally, during testing, we hit an environment limitation: `cargo test` and related backend test commands repeatedly timed out (exceeding 400 seconds) after attempting to narrow the scope from full workspace tests.
