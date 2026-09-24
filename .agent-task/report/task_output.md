---
issue_title: Evaluate compute/API charging and BYOK
issue_description: |
  Review of current code, owner stories, and provider products to establish existing capabilities and gaps.
  Evaluation of compute/API charging and BYOK (Bring Your Own Key).

  # Scope and evidence discipline
  Source baseline used for this evaluation: fix/bazel-modernization-and-cleanup.

  # Findings
  1. API key, consumer plan, and native-client subscription are distinct access modes and must be handled separately.
  2. The proxy correctly rejects unsupported subscription-relay modes; verified tenant OpenAI keys bind to the provider origin and do not fall back to another payer after revocation (F13).
  3. Provider-permitted native-client subscription hosting (such as Claude Code) is a separate integration/terms/quotas decision, and is not generally implemented. The unmodified Claude Code binary is permitted for hosting under specific conditions, but third-party Claude.ai login/token relay and end-user usage resale are strictly prohibited.
  4. Cost and plan UI exist, but the system is not yet demonstrated to be an authoritative, reconciled usage-billing system.
  5. There's a defect (F05) where usage telemetry is not invoice-grade. Unbounded repeated processing per input event in `src/server/hub.rs` causes multiple counting of tokens (the telemetry queue feeds into itself).

  # Recommendations
  - Extend existing paths with reliable accounting.
  - Implement and enforce distinct payer/auth modes (Managed API, Customer API key / cloud account, Provider-native client with subscription).
  - Resolve the unbounded token accounting issue in `src/server/hub.rs` before setting rates or publishing a usage-billing system.
  - Before choosing rates, measure actual model/tool usage, compute and reserved capacity, storage/network/queues, failed attempts, idle allocation, payment collection and support.
issue_priority: P1
issue_category: Research
issue_type: Report
issue_label: [research, billing, byok]
assignees: []
---
