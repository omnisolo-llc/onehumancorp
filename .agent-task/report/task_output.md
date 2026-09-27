issue_title: "💰 Miser: [blocked no-work finding: F13 silent paid fallback]"
issue_description: |
  Title: "Blocked finding: F13 silent paid fallback"
  Problem Statement: The audit finding F13 implies that a BYOK or subscription request could silently fall back to managed API usage if a key is missing.
  Research Report: Examining the code proves that the `UsageMeterSettings::from_environment` function explicitly restricts the `OMNISOLO_USAGE_PAYER` to `"managed_api"` or `"byok_api"`, returning an error otherwise (`API proxy accepts managed_api or byok_api only; subscription sessions cannot be relayed`). Thus, payer resolution is strictly derived from trusted server-side context (`OMNISOLO_USAGE_PAYER` env var). If this context is missing or invalid, the request is rejected before reaching the provider facade. This means there is no gap where a BYOK request can bypass verification and silently fall back to the managed branch within `provider_facade.rs`. The `ByokApi` mode correctly returns a 403 error if the key is missing from the `ConnectionVault`. This serves as evidence that F13 is blocked/no-work for this context. I will report this no-work finding instead of reopening the defect. Loaded Superpowers skills: superpowers:using-superpowers. Exact discovered git revision: f8e9d8dd5c099f417df0c32f6131e9b465e5fb20. Exact executed test commands: cargo test -p server_harness --test provider_facade. Verified trace limitations: The trace verifies that `UsageMeterSettings::from_environment` securely limits payer modes checks, and outcomes as final evidence.
  Design Doc: ""
  Implementation Prompt: ""
  Priority: ""
  Estimated Scope: ""
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
