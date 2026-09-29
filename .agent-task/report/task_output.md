issue_title: "evaluate compute/API charging and BYOK"
issue_description: |
  # Research Report: Evaluate compute/API charging and BYOK, including provider-permitted subscription access

  ## Target Identity
  **Target ID:** OHC-14 (Evaluate compute/API charging and BYOK)
  **Segment:** Core Platform / All Personas
  **Stage:** Research

  ## Problem Statement
  One Human Corp needs a sustainable, usage-based model for model/inference costs that replaces the suspended $99 fixed-subscription hypothesis. Owners must be able to use managed API access (OHC-funded) or customer-funded access (Bring Your Own Key / provider-permitted subscription access) without being double-charged or facing security/terms violations.

  ## Research Report
  - **Context:** The earlier fixed $99 subscription hypothesis and the 300-step allowance are suspended. The platform needs to evaluate metered compute/API charging, customer BYOK, and provider-permitted native subscription clients.
  - **Provider Constraints (BYOK / Subscription):**
    - **OpenAI:** Distinguishes between ChatGPT Business, API usage, and Enterprise access tokens. General subscription sharing or token relay is prohibited. API keys are supported for programmatic access.
    - **Anthropic:** Permits hosting the unmodified Claude Code binary with direct user authentication and payment. Prohibits third-party Claude.ai login/token relay and end-user usage resale/intermediation.
    - **Google:** Gemini API billing is tied to Google Cloud projects. A generic Workspace subscription is not a Gemini API entitlement.
  - **Cost Accounting Model:** OHC serving cost must be calculated based on OHC-funded model/tool usage + allocated CPU/memory/GPU + infrastructure (DB, storage) + paid external tools + support/idle capacity. Customer-paid BYOK inference is excluded from OHC's provider expense and debit.
  - **Evidence Needed for Usage Billing:** Requires durable, deduplicated events with tenant/task/attempt IDs, payer and auth mode, exact quantities, rate-card version, and reconciliation reference.

  ## Current Baseline & Identified Gap
  - **Current Behavior:** The system tracks usage partially but lacks an invoice-grade meter (F05 is Blocked). The proxy rejects unsupported subscription-relay modes and verifies tenant OpenAI keys (F13).
  - **Gap:** The system needs a robust, idempotent meter for usage billing that clearly separates managed API costs from customer-funded BYOK costs, captures all resource usage (including retries/failures), and supports atomic reservations and reconciliation.

  ## Design Doc
  - **Architecture:**
    - Entities: `UsageEvent`, `TenantBudget`, `BillingMeter`, `ProviderReconciliation`
    - Relationships: A `UsageEvent` is tied to a `TenantBudget`. The `BillingMeter` aggregates `UsageEvent`s based on the `payer` and `auth_mode`.
  - **User Flow (Owner Perspective):**
    1. Owner configures billing preference (Managed API vs. BYOK).
    2. Owner views a dashboard showing **estimated task cost and maximum authorized spend**.
    3. Owner sees an itemized OHC bill (for platform compute, managed inference) and a separate section for customer-direct provider usage.
    4. The system automatically reserves budget before tasks and reconciles actual usage post-task.

  ## Implementation Prompt
  **Outcome:** Implement an invoice-grade billing meter that accurately tracks resource usage, differentiates between OHC-funded and customer-funded (BYOK) costs, and provides an itemized cost dashboard for owners.
  **Critical User Journey:**
    - System captures a model inference event → Event is tagged with payer and auth mode → Usage is atomically reserved and settled → Owner views reconciled cost on dashboard.
  **Acceptance Criteria:**
    - Usage records must use integer subunits for decimal precision and include durable, idempotent event IDs.
    - The system must capture failed attempts and retries as part of the cost (unless caused by an OHC defect that is explicitly credited).
    - Customer-funded BYOK usage must not be double-charged as OHC managed inference.
    - The meter must support atomic reservations before spending and reconcile incomplete streams.
    - The dashboard must clearly separate OHC-funded costs from customer-direct costs.
    - Ensure strict adherence to provider API terms (no session-token relay).

  ## Strategy Admission
  - **OHC Target ID:** OHC-14
  - **Stage:** Platform Engineering
  - **Observed/Inferred Gap:** Observed gap (F05 Blocked). Current telemetry is not invoice-grade.
  - **Evidence Level:** Implemented components (F03, F04) show partial readiness; architecture limits identified.
  - **Measurable Result:** 100% accurate attribution of model costs to either OHC or BYOK; Zero instances of double-charging.
  - **Dependencies/Reuse:** Builds on F01 (one-way pipeline), F02 (tenant isolation), F03 (budget increments), and F06 (secure connection vault).
  - **Non-Goals:** Does not include implementing new provider API integrations beyond those already supported (OpenAI/Stripe). Does not include building a full ERP system.
  - **Authority Class:** Platform-level infrastructure.
  - **Cost/Measurement Plan:** Monitor billing pipeline performance and storage costs for usage events.
  - **Happy-path Checks:** Usage event correctly attributed and billed to the correct payer; dashboard reflects accurate totals.
  - **Failure-path Checks:** Missing usage data or invalid auth mode fails closed; over-budget attempts are denied before execution.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
