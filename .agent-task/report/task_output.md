issue_title: "F05: Implement Invoice-Grade Telemetry and Reconciled Provider Invoicing"
issue_description: |
  Title: F05: Implement Invoice-Grade Telemetry and Reconciled Provider Invoicing

  Problem Statement:
  Currently, the system's telemetry and cost reporting do not function as an invoice-grade meter. Usage data lacks durable idempotency, proper payer/auth attribution, and integer subunits for precise financial calculation. Additionally, it risks duplicate billing for Bring-Your-Own-Key (BYOK) inference. For a non-technical business owner like Nora (agency principal), inaccurate usage attribution means they cannot trust OHC to calculate their operational costs correctly, leading to billing disputes and lost confidence in the platform's reliability. The system must establish a robust, tenant-isolated meter that accurately tracks provider usage and explicitly separates customer-funded BYOK requests from OHC-funded compute.

  Research Report:
  - Findings: The remediation ledger (`docs/research/native_migration_and_remediation.md`) identifies F05 ("Current telemetry/cost reports are not an invoice-grade meter") as a Blocked issue. The `RESEARCH.md` contract requires tracking "durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit."
  - Competitive Analysis: Mature billing platforms (e.g., Stripe Billing, Metronome) enforce strict idempotency keys, integer-based currency representations (e.g., cents), and separate event streams for different payer entities. They never mix internal SaaS costs with customer-direct API usage in the final invoice.
  - Strategy Admission:
    - Stable OHC target ID: OHC-09 (measurement, paid conversion)
    - Selected customer/stage: Nora (agency principal), Post-Activation/Retention stage
    - Evidence level: Documented (F05)
    - Baseline/result metric: 100% reconciliation of OHC-funded inference events with external provider invoices (denominator: total OHC model calls)
    - Dependencies/reuse: Existing cost UI, native usage events, and proxy components
    - Non-goals: Building a generic ERP system or processing raw provider API logs outside of the OHC proxy boundary
    - Authority class: Platform-level accounting (no owner approval needed for background metering)
    - Cost plan: Moderate increase in database storage for detailed usage event records
    - Acceptance checks: Happy path (OHC-funded request increments internal meter; BYOK request does not), Failure path (duplicate event UUID is rejected safely without double-counting)

  Design Doc:
  - Key design decisions and why: Introduce a new `usage_events` append-only ledger using precise integer subunits (micro-cents) to avoid floating-point errors. Every event must include a unique idempotency key, tenant ID, payer type (OHC vs. BYOK), and provider request ID. This ensures we can reconcile internal meters with external provider bills and guarantee we never double-charge the owner.
  - Architecture diagram:
    ```mermaid
    sequenceDiagram
      participant Worker as OHC Agent Worker
      participant Proxy as OHC Provider Proxy
      participant Ledger as Usage Ledger (PostgreSQL)
      participant Provider as External AI Provider

      Worker->>Proxy: Request Completion (Includes Tenant ID, Payer Type)
      Proxy->>Provider: Execute Request
      Provider-->>Proxy: Response + Usage Tokens/Cost
      Proxy->>Ledger: Append Event (Idempotency Key, Tokens, Micro-cents)
      alt BYOK Payer
          Ledger->>Ledger: Record as Customer-Funded (No OHC Invoice Impact)
      else OHC Payer
          Ledger->>Ledger: Record as OHC-Funded (Increments OHC Invoice)
      end
      Proxy-->>Worker: Return Result
    ```
  - UI wireframes:
    - Cost Dashboard Card: Clean Ubiquiti UniFi style card with macOS Translucent Glass materials.
    - Top left: "Current Billing Cycle". Top right: "Total: $12.45".
    - Body: Two distinct progress bars/metrics. One clearly labeled "Included OHC Usage" and another labeled "Your Direct Provider Usage (BYOK)".
    - Footer: "View detailed invoice" link.
  - Mobile UX flow:
    1. Owner navigates to "Settings" -> "Billing & Usage".
    2. Screen displays the Cost Dashboard Card, prioritizing the OHC-funded total to avoid confusion.
    3. Tapping the card expands a drawer detailing usage by workflow (e.g., "Drafting Proposals: $2.10"), hiding raw token counts behind an "Advanced Settings" toggle.
  - AI agent integration points: Agents must explicitly pass their execution context (tenant, workflow ID) to the proxy for every external call. Agents are blind to the actual cost calculation logic, which is handled securely by the proxy and ledger.

  Implementation Prompt:
  Implement an append-only `usage_events` table in PostgreSQL to serve as an invoice-grade meter. Ensure all currency values are stored as integer subunits (micro-cents) and enforce uniqueness constraints on a composite of tenant ID and provider request ID for idempotency. Update the provider proxy to explicitly classify and log every request as either OHC-funded or BYOK-funded. Acceptance criteria: 1) The database rejects duplicate usage events. 2) The cost dashboard UI distinctively separates BYOK usage from OHC usage. 3) Token counts and raw JSON are hidden from the primary mobile UI, following the "grandmother test."

  Priority: P1
  Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
