issue_title: "Architecture: Invoice-Grade Metering for Managed API and Customer-Funded Modes (OHC-09)"
issue_description: |
  **Title**: Architecture: Invoice-Grade Metering for Managed API and Customer-Funded Modes (OHC-09)

  **Problem Statement**:
  Currently, the telemetry and cost reports are not an invoice-grade meter (Audit Finding F05). We cannot confidently differentiate between OHC-funded model/tool usage and customer-paid BYOK (Bring Your Own Key) inference. As a result, we cannot accurately bill customers for resource consumption without risking duplicate charges or inaccurate cost attribution, blocking the transition to resource-based charging.

  **Research Report**:
  - The current codebase uses global snapshots for organization responses and lacks integer micro-unit durable records binding tenant, task, attempt, provider, model, payer, and rate revision.
  - According to `business_capability_and_usage_economics_audit.md`, an invoice-grade meter requires capturing durable, deduplicated events with specific IDs, payer/auth mode, exact input/output quantities, and external reconciliation references.
  - Competitive analysis indicates that usage billing platforms (like Stripe Metered Billing or Metronome) rely on idempotent, tenant-isolated event streams. Our current telemetry is not reliable enough for billing.
  - We need to establish sustainable resource rates and a credible BYOK offering where OHC only charges for actual hosted execution while excluding customer-direct provider usage.
  - Sourced from `docs/research/native_migration_and_remediation.md` F05 and `docs/research/business_capability_and_usage_economics_audit.md` sections 7 and 8.
  - Verified Sources:
    1. Internal Audit (F05): `docs/research/native_migration_and_remediation.md` (Source: Local repo)
    2. Internal Research: `docs/research/business_capability_and_usage_economics_audit.md` (Source: Local repo)
    3. Anthropic Claude Code Terms: https://code.claude.com/docs/en/legal-and-compliance
    4. OpenAI Business Overview: https://help.openai.com/en/articles/8792828-chatgpt-business-overview
    5. OpenAI Codex Authentication: https://learn.chatgpt.com/docs/auth
    6. Google Gemini CLI Quotas: https://geminicli.com/docs/resources/quota-and-pricing/
    7. Google Gemini API Billing: https://ai.google.dev/gemini-api/docs/billing

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
      participant App as Application Layer
      participant Meter as Usage Metering Service
      participant Ledger as Cost Ledger (PostgreSQL)
      participant Provider as AI Provider

      App->>Meter: Reserve capacity (Tenant ID, Auth Mode)
      Meter-->>App: Reservation confirmed
      App->>Provider: Execute workload
      Provider-->>App: Usage metrics (Tokens, Caches)
      App->>Meter: Record usage (Idempotent Request ID)
      Meter->>Ledger: Append immutable integer micro-unit usage
      Ledger-->>Meter: Confirmed
    ```
  - **Mobile UX Flow**:
    - 375px viewport target.
    - Owner accesses "Billing & Usage" via bottom tab.
    - A clean, un-cluttered card layout shows "Current OHC Bill" vs "Direct Provider Usage".
    - Usage is displayed in plain-language totals (e.g., "300 proposal generation tasks") rather than raw token counts, hiding the internal token complexity.
    - "Advanced Settings" switch reveals BYOK configuration and raw token logs.
  - **AI Agent Integration Points**:
    - Provider proxy/facade (`harness/middleware/provider_facade.rs`) must inject usage events into the metering pipeline.
    - Agents executing background tasks must pass their organizational binding and payer mode (managed vs BYOK).
  - **Key Design Decisions**:
    - Use immutable append-only ledger for usage events with integer subunits for precision to prevent silent history edits.
    - Strong separation of payer/auth mode at the event level to guarantee no duplicate BYOK debit.

  **Implementation Prompt**:
  "Implement an invoice-grade usage metering service that captures durable, deduplicated events. Ensure events include tenant ID, task ID, provider request ID, payer mode (OHC vs BYOK), and integer subunits for cost. The system must reserve a conservative capacity ceiling before starting new paid work and settle actual usage upon completion. Update the mobile-first UI to display estimated task cost, maximum authorized spend, and an itemized OHC bill separated from customer-direct provider usage."

  **Priority**: P1
  **Estimated Scope**: Large

  **Strategy Admission**:
  - Target ID: OHC-09 (measurement, paid conversion)
  - Selected Customer/Stage: Nora (agency principal), Stage 46-90 (measurement/paid conversion)
  - Evidence Level: Documented/Source-verified gap (F05)
  - Baseline/Result Metric: Accuracy of usage tracking (target: 0 duplicate charges, 0 unrecorded billable compute).
  - Dependencies/Reuse: Existing provider proxy and native usage events, Stripe billing client.
  - Non-goals: Creating a new payment gateway, modifying customer external SaaS subscriptions.
  - Authority Class: System-level accounting (requires strict isolation and exact authority).
  - Cost Plan: Negligible compute for the metering pipeline itself; enables accurate recovery of OHC serving costs.
  - Acceptance Checks: (Happy) Invoice correctly sums integer micro-units for OHC managed API. (Failure-path) Fails closed on invalid/overflow amounts; BYOK usage does not increment OHC bill.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
