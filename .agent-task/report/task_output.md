issue_title: "F13: BYOK versus subscriptions"
issue_description: |
  # Audit Report: F13 BYOK versus subscriptions

  ## Context
  Investigated finding F13 regarding API key, consumer plan, and native-client subscription distinction.

  ## Findings
  - The `UsageMeterSettings::from_environment` in `src/server/harness/middleware/usage_meter.rs` explicitly rejects `native_subscription` and `local` payer modes with the error: "API proxy accepts managed_api or byok_api only".
  - The `ProviderFacadeState` in `src/server/harness/middleware/provider_facade.rs` enforces that `byok_api` mode uses the verified tenant OpenAI/Anthropic keys bound to the provider origin, preventing fallback to another payer after revocation.
  - No session-token relay, pooling, or silent paid fallback is permitted by the proxy logic.
  - The required security isolation and fail-closed logic for unsupported combinations are already fully implemented.

  ## Conclusion
  As the required security controls and subscription relay rejections are already implemented correctly in the codebase, no further code modifications are required for this finding.
issue_priority: "High"
issue_category: "Security"
issue_type: "Audit"
issue_label: "ohc:lane:security"
assignees: []
