issue_title: 🔎 Scout: Tool Integration Research - Google Workspace vs Stripe (Integration Gap Analysis)
issue_description: |
  Loaded Superpowers revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)

  ## Research Report

  **Managed API**: OHC acts as the reseller of compute, funding model usage on behalf of the customer, marking up or absorbing costs. Requires rigorous metering, isolated tenant metrics, and a reservation/spending cap system (as noted in F03, F05).
  **Customer API Key / BYOK**: Customer directly pays the AI provider, and OHC charges a hosting/SaaS fee. Easy from a capital perspective for OHC but shifts complexity (getting keys, managing limits) to the small business owner.
  **Provider-Native Client (e.g. Anthropic Claude Code, Gemini CLI)**: Allows end-user subscriptions, but the platform's terms of service usually prohibit pooling or proxying these tokens as general API tokens.

  *Integrations (Stripe / Google)*:
  Existing Stripe integration exists mainly via webhooks and API calls for proposals (`src/server/integrations/stripe`, numerous mock contracts like `closer_agent_cuj.mock-contract.ts` and `quote_deposit_pipeline.mock-contract.ts`).
  A true end-to-end flow from "inquiry" to "agreed work" to "collection" requires these connectors to be rock solid. F08 (fabricating checkout URLs) was marked closed, meaning it now uses a real provider session, but F05 (invoice-grade metering) is blocked.

  **Design Doc**:
  To unblock the integration and economics capabilities:
  1. Reconcile usage events to tenant IDs (unblocking F05) using the existing `hub.rs` feedback path.
  2. Implement strict boundaries between Managed API (tracked by OHC metering) and BYOK (tracked for visibility but not billed by OHC).
  3. Validate Google Workspace (for calendar/inbox) and Stripe (for deposits/invoices) end-to-end to close the loop on the "Prove a reusable operating loop" requirement.
