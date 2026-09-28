issue_title: "Validate BYOK Inference Model Usage and Cost Accounting Mechanics"
issue_description: |
  # Mission Queue Protocol Brief

  ## Problem Statement
  Owners need a way to bring their own API keys (BYOK) for inference to manage costs directly with providers, while OmniSolo needs to accurately track this usage without double-billing the customer or losing visibility into system-driven AI actions. There is currently uncertainty around the exact mechanics of recording BYOK usage separately from OHC-managed API usage, ensuring tenant isolation, and reconciling costs correctly in the dashboard.

  ## Research Report
  - **Context:** The usage audit (2026-09-18) requires separating OHC-funded model/tool usage from customer-paid BYOK inference.
  - **Findings:** The codebase references `OPENAI_API_KEY` across multiple services (e.g., `src/ui/next/src/app/integrations/ProviderConnections.tsx` and `src/server/harness/middleware/provider_facade.rs`). It correctly identifies that BYOK usage should not be billed by OHC. However, the exact data structure for itemized billing and the UI representation for BYOK vs. Managed API costs need concrete design.
  - **Sources:** Codebase search (`grep -rn "API" src/ | grep -i "key"`), `docs/research/business_capability_and_usage_economics_audit.md`, `docs/business/public/verified_business_capabilities.md`.

  ## Design Doc
  - **Architecture:**
    - **Entities:** `UsageEvent` needs a `payer_type` enum (`OHC_MANAGED`, `CUSTOMER_BYOK`).
    - **Integration Points:** The telemetry/billing auditor (`src/server/services/billing/auditor.rs`) must classify events based on the active connection mode used by the provider facade.
  - **UI Flow:**
    - The Cost Dashboard (mobile 375px first) displays two distinct sections: "OmniSolo Infrastructure Costs" and "Your Direct API Usage (Estimated)".
  - **AI Integration:** Agents operating under BYOK must still log token usage to OHC for operational visibility, but these logs must hit a zero-cost tariff in the OHC ledger.

  ```mermaid
  graph TD
      A[Agent Request] --> B{Active Connection}
      B -->|OHC Managed| C[Provider Facade + OHC Key]
      B -->|BYOK| D[Provider Facade + Customer Key]
      C --> E[Log UsageEvent payer:OHC_MANAGED]
      D --> F[Log UsageEvent payer:CUSTOMER_BYOK]
      E --> G[Ledger Debit OHC Wallet]
      F --> H[Dashboard Visible Only, Zero OHC Debit]
  ```

  ## Implementation Prompt
  Implement the `payer_type` classification in the backend billing auditor and update the Cost Dashboard UI to split costs into "OmniSolo Infrastructure" and "Your Direct Provider Usage". Ensure that BYOK events do not decrement the customer's OmniSolo spending limit. Add tests verifying that `CUSTOMER_BYOK` events result in zero OHC cost.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - **OHC Target ID:** OHC-10 (Cash, cost and business portability).
  - **Stage:** Launch.
  - **Observed Gap:** Lack of verifiable isolation between BYOK inference costs and OHC-managed API billing in the current telemetry pipeline.
  - **Evidence Level:** Code audit and architecture review.
  - **Baseline:** Currently, all usage might be treated uniformly, risking double billing or inaccurate dashboards.
  - **Result Metric:** 100% of BYOK tokens logged without triggering OHC wallet debits.
  - **Dependencies:** Provider facade connection resolution logic.
  - **Non-Goals:** Building a new ledger system from scratch; changing the actual API pricing card.
  - **Authority Class:** Reversible internal changes (read/draft telemetry).
  - **Acceptance Checks:**
    - *Happy Path:* A BYOK request logs usage, updates the dashboard "Estimated Direct" bucket, and does not change OHC balance.
    - *Failure Path:* A request missing a valid `payer_type` fails closed and does not execute the LLM call to prevent unmetered spend.

  ## Context Metadata
  - **Source Dates**: 2026-09-18-usage-audit
  - **Study Populations**: Solo web/design/marketing professional (Nora).
  - **Uncertainties**: Exact rendering of provider lag in the UI; handling of rate-limited BYOK keys.
  - **Scope**: Re-evaluating existing billing logic for dual-mode support without expanding into new domains.
  - **Loaded Skills**: `using-superpowers`, `brainstorming`.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
