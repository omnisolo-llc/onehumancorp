issue_title: "🔎 Scout: Tool Integration Research - [F13 Blocked No-Work Finding]"
issue_description: |
  # Blocked No-Work Finding for F13

  The current task targets the F13 finding from `native_migration_and_remediation.md`, which states:
  "API key, consumer plan and native-client subscription are distinct: Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference".

  The research report states that:
  "Provider-permitted native-client subscription hosting is still a separate integration/terms/quotas decision, not generally implemented."

  As an Implementer, I am tasked with repairing this. However, since the fundamental architecture for provider-permitted native-client subscription hosting and the explicit rules and distinctions across different providers (OpenAI, Anthropic, Google) are not yet fully implemented or decided upon (as per `business_capability_and_usage_economics_audit.md` which states "This is product due diligence, not a legal opinion. No supported native-client subscription mode was demonstrated end-to-end in OHC. The existing Codex adapter and API/OAuth credential types are reuse assets, not that demonstration"), I cannot write concrete code to enforce these boundaries without inventing policies or creating incomplete architectural pieces that violate the strict no-fabrication rules.

  The audit mandates: "Confirm the proposed deployment against the full terms rather than treating consumer credentials as BYOK API keys." and "Verify current plan eligibility and embedding terms, and never silently switch an exhausted subscription session to a paid OHC API key."

  Since this requires a broader architectural decision and provider terms validation before code can be written, this is a blocked no-work finding.

issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
