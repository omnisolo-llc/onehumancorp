issue_title: "Implement the finding from this completed research report immediately"
issue_description: |
  **Title**: Blocked implementation for F13 (Provider native-client subscriptions)

  **Problem Statement**:
  The user asked to immediately implement the finding for F13: "API key, consumer plan and native-client subscription are distinct". The current request is to implement "Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference".

  **Research Report**:
  The application code already contains an active block for this exact constraint. Specifically, `src/server/harness/middleware/usage_meter.rs` correctly parses `OMNISOLO_USAGE_PAYER` and rejects attempts to relay subscriptions through the proxy:
  ```rust
  "native_subscription" => return Err("API proxy accepts managed_api or byok_api only; native_subscription sessions cannot be relayed".into()),
  "local" => return Err("API proxy accepts managed_api or byok_api only; local mode is not an API proxy target".into()),
  _=>return Err("API proxy accepts managed_api or byok_api only; subscription sessions cannot be relayed".into()),
  ```
  This fail-closed logic correctly restricts usage to authorized `managed_api` and `byok_api` paths, ensuring that consumer subscriptions (like a ChatGPT or Claude.ai account) are not pooled or relayed via standard API endpoints. Anthropic's terms explicitly forbid a third-party Claude.ai login/token relay, and OpenAI Codex distinguishes between Enterprise access tokens and ChatGPT subscriptions. Modifying the system to pass through subscription tokens or support unsupported combinations would violate these constraints and provider terms.

  Since the codebase already satisfies the F13 finding by strictly enforcing fail-closed restrictions against subscription relay, there is no further safe code implementation that can be done for this target. The task is functionally blocked from making further additions as the constraint is already completely addressed by existing logic.

  **Mission Queue Protocol Brief**:
  - Stable Target: F13
  - Persona/Journey: Subscription/Billing Configuration
  - Expected Business Result: Safely block unauthorized provider consumer subscription relays.

  **Implementation Prompt**: None. The work is a blocked no-work finding because the required provider authorization bounds and fail-closed logic are already implemented in `usage_meter.rs`.
issue_priority: "P0"
issue_category: "research"
issue_type: "report"
issue_label: ""
assignees: []
