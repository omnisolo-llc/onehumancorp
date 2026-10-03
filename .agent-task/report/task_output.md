issue_title: "🔎 Scout: Tool Integration Research - Google Workspace vs Stripe (Integration Gap Analysis)"
issue_description: |
  **Research Target**: Evaluation of charging for compute/AI API usage, BYOK vs provider-native client access, and an understanding of the current integration boundaries. In `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`, there's a requirement to investigate "managed API vs customer API-key vs native client", plus integrating tools like Stripe or Google Workspace to prove a reusable operating loop. F13 notes "API key, consumer plan and native-client subscription are distinct" as currently blocked.

  **Research Report**:
  - **Managed API**: OHC acts as the reseller of compute, funding model usage on behalf of the customer, marking up or absorbing costs. Requires rigorous metering, isolated tenant metrics, and a reservation/spending cap system (as noted in F03, F05).
  - **Customer API Key / BYOK**: Customer directly pays the AI provider, and OHC charges a hosting/SaaS fee. Easy from a capital perspective for OHC but shifts complexity (getting keys, managing limits) to the small business owner.
  - **Provider-Native Client (e.g. Anthropic Claude Code, Gemini CLI)**: Allows end-user subscriptions, but the platform's terms of service usually prohibit pooling or proxying these tokens as general API tokens.

  *Integrations (Stripe / Google)*:
  Existing Stripe integration exists mainly via webhooks and API calls for proposals (`src/server/integrations/stripe`, numerous mock contracts like `closer_agent_cuj.mock-contract.ts` and `quote_deposit_pipeline.mock-contract.ts`).
  A true end-to-end flow from "inquiry" to "agreed work" to "collection" requires these connectors to be rock solid. F08 (fabricating checkout URLs) was marked closed, meaning it now uses a real provider session, but F05 (invoice-grade metering) is blocked.

  **Design Doc**:
  To unblock the integration and economics capabilities:
  1. Reconcile usage events to tenant IDs (unblocking F05) using the existing `hub.rs` feedback path.
  2. Implement strict boundaries between Managed API (tracked by OHC metering) and BYOK (tracked for visibility but not billed by OHC).
  3. Validate Google Workspace (for calendar/inbox) and Stripe (for deposits/invoices) end-to-end to close the loop on the "Prove a reusable operating loop" requirement.

  **Evidence (Superpowers Check):**
  Loaded skills: none
  Upstream revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d

  **Implementation Outcome**:
  This is a research report confirming no immediate new code changes are needed outside of the existing remediation ledger (M01-M07, F01-F15).
  Treat this as a "no-work finding" from a feature perspective, as the current repository instructions state: "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue." No new generic features will be built.
issue_priority: "P2"
issue_category: "research"
issue_type: "task"
issue_label: ""
assignees: []
