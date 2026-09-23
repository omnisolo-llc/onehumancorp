issue_title: "Dual-Mode Cost Accounting and BYOK Reservation Pipeline"
issue_description: |
  **Title**: Dual-Mode Cost Accounting and BYOK Reservation Pipeline

  **Problem Statement**:
  Currently, the OmniSolo platform lacks a reconciled, dual-mode charging mechanism to accurately separate OHC-funded managed API usage from customer-paid BYOK (Bring Your Own Key) inference. Without robust usage identity, rate attribution, and atomic hard budget reservations, we cannot confidently introduce resource-based charging or support a BYOK model without risking double-billing customers or exposing the platform to unmetered liability. The current cost telemetry exhibits tenancy and reservation defects, blocking our confidence in scaling to a reliable usage-billed execution environment.

  **Research Report**:
  *   **Market Context**: Standard Small Business SaaS tools typically bundle API costs or rely entirely on BYOK (e.g., TypingMind, Cursor). However, a fully autonomous agent team acting under standing authority can quickly consume substantial compute (waits, retries, multi-step generation).
  *   **Provider Evidence**: OpenAI's API billing and Anthropic's Claude Code terms clearly separate consumer subscriptions from general-purpose API key usage. A ChatGPT/Claude subscription cannot be arbitrarily pooled or proxied.
  *   **Code Audit Findings (F04, F05)**: The codebase reveals that model paths disagree on usage, and current telemetry reports are not invoice-grade meters. `BudgetManager` (in `src/server/pricing/budget.rs`) implements limit checks but does not fully integrate dual-mode billing separation.
  *   **Conclusion**: We must implement an invoice-grade meter with idempotent events that completely isolates OHC compute costs (CPU, memory, storage) from customer-direct inference costs, ensuring no duplicate BYOK debit occurs.

  ### Feature Comparison Matrix
  | Feature | OmniSolo Managed API | OmniSolo BYOK | Typical Vertical SaaS |
  | :--- | :--- | :--- | :--- |
  | **Inference Cost Payer** | OHC | Customer | Customer or Bundled |
  | **Usage Visibility** | Estimated Task Cost | Read-only tokens | None / Hidden |
  | **Hard Spend Limits** | Yes (Atomic Reservations) | Yes (Local Cap) | Rarely |
  | **Provider Choice** | Curated | Flexible | Locked |

  ### Persona-Specific Journey (Nora)
  Nora, a solo web designer, operates under tight margins. She prefers OHC's managed API for simple client onboarding flows to avoid setting up her own OpenAI account. However, for deep research tasks, she connects her own Anthropic API key. When executing a task, Nora needs clear visibility: "Is this task using my prepaid OHC balance, or is it billing my personal Claude account?" Without dual-mode accounting, Nora is terrified she might be double-charged or exceed her tight monthly budget. This feature provides her with a transparent "Billing & Limits" screen, clearly separating OHC usage from her BYOK inference, giving her the confidence to delegate work.

  **Design Doc**:
  *   **Architecture & Entity Types**:
      *   `UsageEvent`: Must include `tenant_id`, `project_id`, `task_id`, `request_id`, `payer_mode` (ManagedAPI vs. BYOK), and `auth_mode`.
      *   `BudgetManager` (enhancement): Introduce a mode flag to bypass inference debiting for BYOK tenants, while still logging the event for owner visibility.
      *   `InvoiceMeter`: An aggregator that translates `UsageEvent`s into invoice lines using integer subunits (cents).
  *   **Data Flow**:
      *   Agents emit `UsageEvent`s -> Ingestion Queue -> Deduplication & Attribution -> `BudgetManager` reservation check -> `InvoiceMeter` accounting -> Durable Ledger.

  ### System Architecture
  ```mermaid
  graph TD
      A[Agent Runtime] -->|Emit UsageEvent| B(Ingestion Queue)
      B --> C{Payer Mode Check}
      C -->|Managed API| D[BudgetManager: Debit Reserve]
      C -->|BYOK| E[BudgetManager: Log Only]
      D --> F[InvoiceMeter: Accounting]
      E --> F
      F --> G[(Durable Ledger)]
      G --> H[Owner Billing Dashboard]
  ```

  *   **Integration Points**: Extend the existing `CostAuditor` to respect the `payer_mode`. Ensure `tool_integrations` and prompt adapters explicitly declare which payer is active before dispatching requests.
  *   **Mobile UX Flow (375px)**: The "Billing & Limits" screen will show two distinct sections: "OmniSolo Platform Usage" (estimated task cost, remaining prepaid balance) and "Your Connected API Keys" (read-only usage visibility, no OHC charges applied).

  **Implementation Prompt**:
  Develop the `DualModeMeter` service. It must ingest usage events with an explicit `payer_mode` enum (`ManagedAPI` or `BYOK`).
  1.  **Critical User Journey**: When a task executes under BYOK, the system tracks token volume for analytics but strictly ignores these tokens when calculating the OHC invoice total.
  2.  **Acceptance Criteria**:
      *   A new `PayerMode` enum is added and plumbed through the agent execution context.
      *   Unit tests prove that a task running with `PayerMode::BYOK` results in $0 OHC inference cost but accurately increments the `tokens` counter for visibility.
      *   Concurrency tests prove that `BudgetManager` accurately rejects managed-API tasks if the OHC budget is exhausted, but allows BYOK tasks if only OHC limits are reached (subject to BYOK quotas).
      *   The existing `CostAuditor` correctly partitions these events.

  **Priority**: P1

  **Estimated Scope**: Medium

  **Strategy Admission**:
  *   **OHC target ID**: OHC-10 (Cash, cost and business portability)
  *   **Launch/Run stage**: Launch (prerequisite for sustainable economics)
  *   **Observed/inferred gap**: Observed (F04, F05 in remediation ledger)
  *   **Evidence level**: Documented / Code-verified
  *   **Baseline and measurable result**: Baseline is current inaccurate single-pool tracking. Result is 0% duplicate BYOK inference charges and 100% test-verified separation of managed vs BYOK costs.
  *   **Dependencies/reuse**: Existing `BudgetManager`, `CostAuditor`, `telemetry_store`.
  *   **Non-goals**: Not implementing an arbitrary fixed 20% markup or subscription tiers; not implementing a visual drag-and-drop workflow builder.
  *   **Authority class**: Financial/Billing (Requires strict tenant boundaries and atomic budget locks).
  *   **Cost/measurement plan**: Will measure inference cost separation in unit tests without live provider credentials.
  *   **Acceptance checks**: Happy-path: Valid BYOK event results in no invoice debit. Failure-path: Managed API request exceeding reserved budget fails safely without executing the LLM call.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
