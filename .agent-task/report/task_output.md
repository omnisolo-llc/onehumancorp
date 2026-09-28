issue_title: 🛡️ Sentry: [blocked no-work finding: F13]
issue_description: |
  # Blocked No-Work Finding for F13

  The codebase audit reported in `docs/research/native_migration_and_remediation.md` listed defect F13 as "API key, consumer plan and native-client subscription are distinct | Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference".

  After inspecting the codebase, specifically `src/server/harness/middleware/usage_meter.rs`, it is clear that this finding has already been remediated. The `usage_meter.rs` explicitly limits `PayerMode` by failing on `native_subscription` and `local` payer configurations (returning "API proxy accepts managed_api or byok_api only; native_subscription sessions cannot be relayed").

  Furthermore, `src/server/harness/middleware/provider_facade.rs` verifies that BYOK routes bind securely to the required provider origin by enforcing the `https` scheme and the `api.openai.com` domain.

  Because the required validation and isolation mechanisms for F13 are already implemented, there is no work to be done.
issue_priority: low
issue_category: security
issue_type: bug
issue_label: ohc:security
assignees: []
