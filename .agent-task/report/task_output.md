issue_title: "F05: Invoice-Grade Telemetry and Cost Reporting"
issue_priority: "High"
issue_category: "Finance"
issue_type: "Architecture"
issue_label: "ohc:lane:finance"
assignees: []
issue_description: |
  # Title: Architect Design for Invoice-Grade Telemetry (F05)

  ## Problem Statement
  Current telemetry and cost reports in OneHumanCorp are not an invoice-grade meter (F05 from the remediation ledger). The existing usage events do not provide durable, idempotent usage tracking with proper payer, auth mode, rate attribution, and reconciliation necessary for accurate and reliable billing. Additionally, BYOK (Bring Your Own Key) inference could mistakenly be debited as managed inference. The goal is to design an architecture that captures and settles usage with financial precision.

  ## Research Report
  - **Source Date:** 2026-09-18
  - **Study Population:** Owner needs outlined in `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`.
  - **Scope:** Evaluate resource-based charging and customer-funded inference to replace legacy global summaries.
  - **Uncertainties:** Exact workload distribution, stable external reconciliation references, and latency of provider reporting.
  - **Metric Definitions:** Usage must be recorded in integer subunits (e.g., micro-cents or raw token counts) bound to tenant/task/attempt/provider/model/payer IDs.

  ## Design Doc
  The architecture introduces a new `UsageMeter` service that sits between the `harness_worker` (or API gateway) and the `billing_auditor`.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Agent Runtime/Worker] -->|Raw Usage Event| B(UsageMeter Service)
      B -->|Idempotent Event Check| C[(PostgreSQL: usage_ledger)]
      C -.->|If Duplicate| B
      B -->|Cost Attribution (Rate Card)| D[(PostgreSQL: tenant_balance)]
      B -->|Settled Event| E[Billing Auditor]
      E -->|Invoice Generation| F[Stripe Integration]
      B -->|Reconciliation Job| G[External Provider Sync]
  ```

  ### UI Wireframes
  *Not strictly applicable to a backend telemetry change, but an administrative view will show:*
  - A table of raw usage events with columns for Timestamp, Provider, Model, Payer (OHC vs BYOK), Tokens/Units, and Estimated Cost.
  - A tenant balance summary showing "Maximum Authorized Spend", "Current Settled Usage", and "Pending Reconciliation".

  ### Mobile UX Flow
  - Screen 1: Dashboard overview with a clear "Current Usage Cost" metric.
  - Screen 2: Tap to view details, showing a breakdown of costs by AI model and tool execution, explicitly separating BYOK usage (which shows $0.00 OHC cost).

  ### AI Agent Integration Points
  - The `harness_worker` and any local service adapters (e.g., `omnisolo.memory`) must inject `attempt_id`, `tenant_id`, and `provider_id` into all telemetry payloads.
  - Agents must query the `UsageMeter` to check if a hard budget reservation exists before initiating expensive provider tasks.

  ## Implementation Prompt
  1. Create a new `UsageMeter` rust module under `src/server/services/billing/`.
  2. Implement idempotent ingestion of usage events utilizing `attempt_id` and provider request IDs to prevent double counting.
  3. Update `agent_session_data` and event pipelines to emit detailed usage records containing integer subunits for cost and clear `payer` attribution.
  4. Implement a background reconciliation worker to sync provider-reported usage with locally observed telemetry.

  ## Priority
  High

  ## Estimated Scope
  2-3 weeks for full backend implementation and rigorous financial testing.