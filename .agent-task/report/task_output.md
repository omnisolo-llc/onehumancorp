issue_title: "✍️ Scribe: [blocked no-work finding: F13]"
issue_description: |
  Target: F13: BYOK versus subscriptions

  **Research Summary:**
  The instruction from the user required me to pick an open item from the remediation ledger, with a preference for documentation generation ("Role: Principal Technical Writer & Scribe (L7)"). F13 ("BYOK versus subscriptions", "API key, consumer plan and native-client subscription are distinct") is marked as `Open` in `docs/research/native_migration_and_remediation.md`.

  However, investigation shows that F13 is already fully resolved in the codebase. As required by the audit constraints:
  - `src/server/harness/middleware/usage_meter.rs` explicitly rejects modes like `native_subscription` and `local` for API proxying.
  - `src/server/harness/middleware/provider_facade.rs` includes a check enforcing that BYOK keys only target the verified provider origin.
  - `src/server/harness/tests/provider_facade.rs` confirms these exact scenarios are fully tested (`facade_rejects_byok_api_if_tenant_key_absent_or_revoked` and `facade_rejects_byok_api_if_origin_is_unsupported`).
  - No session-token relay, pooling, or silent paid fallback is permitted.

  There is no missing capability or documentation regarding F13. F13 should simply be moved from `Open` to `Closed` in the remediation ledger. The instruction constraints note that "A no-work/blocked result with evidence is valid", and since there are no genuine documentation or code gaps for F13, I am submitting this as a blocked no-work finding.
