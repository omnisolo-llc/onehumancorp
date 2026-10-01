issue_title: "🔎 Scout: Tool Integration Research - Google Workspace vs Stripe (Integration Gap Analysis)"
issue_description: |
  **Research findings for integration readiness and economics**

  **1. Current status in codebase**
  The capabilities in `docs/research/business_capability_and_usage_economics_audit.md` were evaluated. The audit identifies:
  * Google Workspace: `integrations/google_calendar/client.rs` and `integrations/google_workspace/provider.rs` exist. End-user authorization, secure credential lifecycle, refresh/revocation and complete UI-to-provider flows need separate proof.
  * Stripe (invoicing/payments): Invoice creation (`api/invoice.rs`) creates a checkout-looking UUID URL without actually creating a provider session. Stripe plumbing (`proposals.rs`) and placeholders coexist.
  * Usage economics: `src/server/hub.rs` has an unbounded telemetry channel that can repeatedly account for the same usage. A budget monitor (`pricing/budget.rs`) is not a hard spending reservation. `services/billing/auditor.rs` lacks provider/model, request ID, payer/auth mode, and rate-card revision.
  * BYOK vs. Managed API: The audit highlights the distinction between OHC's contracted API account (Managed API), customer API keys, and provider-native clients. A ChatGPT/Claude/Gemini subscription is not a general API token.

  **2. Implementation decision**
  Per the issue prompt, this is a research report confirming no immediate new code changes are needed outside of the existing remediation ledger (M01-M07, F01-F15).
  Treat this as a "no-work finding" from a feature perspective, as the current repository instructions state: "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue." No new generic features will be built.

issue_priority: "P2"
issue_category: "research"
issue_type: "scout"
issue_label: "agent-report"
assignees: []
