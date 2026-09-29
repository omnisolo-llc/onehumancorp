issue_title: "F05: Current telemetry/cost reports are not an invoice-grade meter"
issue_description: |
  **Title**: Missing accurate tracking, attribution, and reconciliation for a trustworthy usage-based billing meter

  **Problem Statement**:
  Right now, OHC lacks a complete, reliable invoice-grade meter. Business owners using an AI tool need absolute clarity and confidence in their billing. They need to see usage mapped clearly to the tenant, specific AI provider, and billing mode. They need to trust that they won't be charged for duplicate usage, or billed for failed operations. Currently, the system uses simple event counters and `BudgetManager`, which only provides an overall limit and alerts based on projected costs, but does not offer the granular, idempotent tracking required to support true usage-based metered billing or "Bring Your Own Key" (BYOK) setups where OHC must not double-charge for inference.

  **Research Report**:
  - The `BudgetManager` (in `src/server/pricing/budget.rs`) enforces limits but lacks granular accounting needed for invoicing (e.g., distinguishing between different models, tools, or success/failure states).
  - The telemetry feedback loop issue (F01) was fixed, but the overarching gap (F05) remains: OHC needs durable, idempotent usage tracking.
  - The `UsageLedger` (in `src/server/harness/middleware/usage_ledger.rs`) tracks reservations and settlements but needs to be expanded or tightly integrated into a robust, tenant-specific billing engine.
  - BYOK (Bring Your Own Key) is partially accounted for but needs explicit tracking to ensure no double charging (e.g., OHC-funded resources vs. BYOK-funded LLM inference).
  - Stripe's usage-based billing documentation and general SaaS best practices dictate that metered billing requires immutable, precisely attributed usage records that can be reconciled against external providers.
  - The current cost reports use global snapshots and lack provider/model, request ID, payer mode, and rate-card version tracking.
  - Verified sources:
    - [Federal Reserve, 2026 employer report](https://www.fedsmallbusiness.org/reports/survey/2026/2026-report-on-employer-firms) (owners cite accuracy and integration as top AI challenges).
    - [Federal Reserve, nonemployer report](https://www.fedsmallbusiness.org/reports/survey/2025/2025-report-on-nonemployer-firms).
    - [OECD, Generative AI and the SME Workforce](https://www.oecd.org/en/publications/generative-ai-and-the-sme-workforce_2d08b99d-en.html).
    - [Jobber feature/pricing comparison](https://www.getjobber.com/pricing/).
    - [Stripe fulfillment documentation](https://docs.stripe.com/checkout/fulfillment).
    - [Claude for Small Business](https://www.anthropic.com/news/claude-for-small-business).

  **Design Doc**:
  - **Entity Types**:
    - `UsageRecord`: Represents a single, idempotent usage event. Must include: `tenant_id`, `provider`, `model`, `payer_mode` (OHC vs BYOK), `request_id`, `units_consumed` (e.g., tokens, seconds), `cost_cents`, `timestamp`, and `status` (reserved, settled, failed).
    - `Invoice`: Aggregates `UsageRecord`s for a billing period.
  - **Key Relationships**:
    - A Tenant has many `UsageRecord`s.
    - An Invoice belongs to a Tenant and aggregates many `UsageRecord`s.
  - **Integration Points**:
    - `UsageLedger` and `BudgetManager` must feed into or be replaced by a system that writes `UsageRecord`s.
    - The Stripe (or other payment provider) integration must read aggregated `UsageRecord`s to calculate the final bill.
  - **Architecture Diagram**:
    ```mermaid
    flowchart TD
      A[Agent Runtime/Cost Auditor] -->|Usage Event| B{Idempotency Check}
      B -->|New Event| C[Create UsageRecord]
      B -->|Duplicate| D[Discard]
      C --> E[(PostgreSQL)]
      E --> F[Billing Aggregator]
      F --> G[Stripe API / Invoices]
    ```
  - **UI/UX Flow (Mobile First)**:
    - **Persona Narrative (Nora, Solo Web Designer)**: Nora checks her OHC app to see if she's staying within her $99/mo plan allowance for AI usage. She navigates to the "Current Usage" screen (375px optimized). The screen shows her total OHC-funded spend, cleanly separated from her direct Anthropic (BYOK) usage. She taps "View Details" and sees an itemized list of tasks (e.g., "Drafted Proposal", "Answered Email"), building absolute trust that she is only billed for successful, accurate work and not double-charged for her own API key.
    - Owners view a "Current Usage" screen displaying total spend, broken down by provider/model and OHC vs BYOK.
    - Owners can view a list of individual usage events for transparency.

  **Comparative Feature Matrix**:
  | Feature | OHC (Proposed) | HoneyBook | Stripe Billing (Native) |
  |---|---|---|---|
  | **Tenant Isolation** | Strict, at the database level | N/A (single user focused) | Strong |
  | **Idempotent Usage Tracking** | Yes, via Request ID | Limited | Yes, native |
  | **BYOK Distinction** | Yes, explicit flag | No | Custom implementation needed |
  | **Model-Level Granularity** | Yes | No | Custom implementation needed |

  **Implementation Prompt**:
  - Implement a durable, idempotent `UsageRecord` data store (e.g., PostgreSQL table) that captures all necessary fields for invoice-grade metered billing.
  - Update the usage tracking middleware/services (e.g., `UsageLedger`, `BudgetManager`, or related components) to write `UsageRecord`s instead of just incrementing global counters.
  - Ensure that BYOK usage is explicitly marked and excluded from OHC-funded cost calculations to prevent double billing.
  - Create an internal API endpoint to aggregate usage for a specific tenant and billing period, suitable for integration with a payment provider (like Stripe).
  - Add comprehensive tests to verify idempotent record creation, accurate attribution (tenant, provider, payer mode), and correct aggregation for invoicing.

  **Priority**: P1
  **Estimated Scope**: Large
  **Strategy Admission**:
  - **Target ID**: F05
  - **Stage**: Run
  - **Observed Gap**: Current cost reports are not invoice-grade; lack granular attribution and idempotency.
  - **Evidence Level**: Documented in audit (`docs/research/business_capability_and_usage_economics_audit.md`).
  - **Baseline and Measurable Result with Denominator**: Baseline is 0% of usage events have robust, billable attribution. Result is 100% of generated `UsageRecord`s map to a tenant, provider, and payer mode.
  - **Dependencies/Reuse**: Reuse existing `UsageLedger` and telemetry infrastructure where possible.
  - **Non-Goals**: Do not implement the actual Stripe billing integration in this task (focus only on the internal usage tracking/metering).
  - **Authority Class**: Read/Write (internal accounting).
  - **Cost/Measurement Plan**: Cost is negligible for internal DB writes. Success is measured by the successful reconciliation of internal `UsageRecord`s against provider invoices in testing.
  - **Acceptance Checks**:
    - Verify that usage events are recorded idempotently.
    - Verify that usage is correctly attributed to the tenant, provider, and payer mode.
    - Verify that BYOK usage does not increase OHC-funded costs.
    - Verify that usage can be accurately aggregated for a billing period.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
