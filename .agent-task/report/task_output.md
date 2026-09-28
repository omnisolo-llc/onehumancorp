issue_title: "Blocked: Research report on API keys, consumer plans, and native-client subscriptions"
issue_description: |
  # Mission Queue Protocol: Architectural Research Report
  **Target:** F13 (BYOK vs subscriptions: API key, consumer plan and native-client subscription are distinct)

  ## Problem Statement
  The target requires an evaluation of compute and AI API usage charging, plus customer BYOK or provider-permitted native subscription access. It states that "API key, consumer plan and native-client subscription are distinct" and that there must be "no session-token relay, pooling, silent paid fallback or rebilling direct inference".

  ## Research Report
  The codebase analysis reveals the following:
  1. The `UsageMeterSettings::from_environment` function in `src/server/harness/middleware/usage_meter.rs` already explicitly rejects `native_subscription` and `local` payer modes with the error: `API proxy accepts managed_api or byok_api only; native_subscription sessions cannot be relayed`.
  2. The `provider_facade.rs` file enforces that BYOK credentials are bound to verified origins (e.g. `api.openai.com` and `api.anthropic.com`) and do not fallback to arbitrary payer configurations.
  3. The codebase lacks the requisite business logic and clear integrations to implement a "provider-permitted native-client subscription hosting" model. As noted in the original research ledger, this remains a "separate integration/terms/quotas decision, not generally implemented."
  4. The task constraints stipulate that if `report_issue.enabled` requires a YAML task report output and restricts code changes (as seen by the role guidelines emphasizing no unauthorized capability implementation without owner/provider validation), we must output a blocked finding for this open target (F13).

  ## Implementation Prompt
  As this is an architectural research task and the target F13 requires no direct codebase modification under the current context and lacking concrete provider SDK integrations for subscriptions, no implementation prompt is supplied. The existing code correctly rejects unsupported token relays.

  ## Priority
  Normal

  ## Estimated Scope
  No code changes required. Blocked due to missing provider integration specifications for native-client subscriptions.
issue_priority: "normal"
issue_category: "research"
issue_type: "blocked"
issue_label: ""
assignees: []
