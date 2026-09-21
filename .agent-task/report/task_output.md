issue_title: "Atomic Two-Phase Commit for Budget Reservations"
issue_description: |
  # Title
  Atomic Two-Phase Commit for Budget Reservations

  # Problem Statement
  Currently, the AI operational budget management system (`BudgetManager` in `src/server/pricing/budget.rs`) records spend atomically via a single-phase counter update (`record_spend_cents`). However, in highly concurrent operational environments where external actions (such as LLM generation or tool execution) occur concurrently across multiple orchestrated workers, a single-phase debit happens *after* or *as* the expense is confirmed. This causes a risk of budget overruns if many large concurrent operations are initiated right near the budget limit. A robust financial and budget management system requires an atomic two-phase commit strategy: "reserve" funds *before* taking the action, and then "settle" or "release" the funds based on the exact outcome, ensuring hard budget stops are strictly enforced during concurrent executions.

  # Research Report
  - **Findings**: The `BudgetManager` structurally limits spend to a total amount but only maintains a single `current` amount (via an `AtomicI64`). There is no mechanism to track `allocated` (reserved) vs `settled` funds. The absence of this separation fails the strict atomic reserve/settle/release mandate in billing and budget management (which specifically asks for two-phase commit tracking separate `allocated` and `current` amounts to safely enforce hard budget stops during concurrent execution, as stated in the architecture memory constraints).
  - **Competitive Analysis**: Standard billing systems (e.g., Stripe authorizations) use a pre-authorization and capture workflow. Cloud quotas (like AWS or GCP API quotas) enforce hard reservations before dispatching long-running asynchronous tasks.
  - **Data/References**: The file `src/server/pricing/budget.rs` contains `BudgetManager` which relies solely on `record_spend_cents`.

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  sequenceDiagram
    participant Worker
    participant BudgetManager
    participant AIProvider

    Worker->>BudgetManager: 1. reserve_funds(estimated_cost)
    alt Budget Available
        BudgetManager-->>Worker: Ok(ReservationToken)
        Worker->>AIProvider: 2. execute_action()
        AIProvider-->>Worker: Result(actual_cost)
        Worker->>BudgetManager: 3a. settle_funds(ReservationToken, actual_cost)
    else Budget Exceeded
        BudgetManager-->>Worker: Err(InsufficientFunds)
    end
  ```

  ## Key Architecture Enhancements
  - **Data Model**: Update `BudgetManager` to track `reserved` and `settled` (or `current`) amounts concurrently using atomics. The total limit enforces `reserved + settled <= limit`.
  - **Two-Phase Commit Mechanics**:
    - `reserve(amount) -> Result<ReservationId, Err>`: Adds to `reserved`.
    - `settle(ReservationId, actual_cost)`: Removes from `reserved`, adds `actual_cost` to `settled`.
    - `release(ReservationId)`: Subtracts from `reserved` without changing `settled`.
  - **Multi-Tenant / Security Integrity**: The `BudgetManager` must ensure tenant context boundaries (e.g., logging budget telemetry) are preserved during the reservation and settlement phases securely.

  ## Mobile UX Flow
  For the non-technical owner/operator (e.g. Nora, Maya):
  - On a 375px viewport (mobile app dashboard), budget statuses are represented with a simple visual progress bar (styled with macOS-style Translucent Glass and UniFi modular dashboard cards).
  - The bar shows three states: Used (Settled), Pending (Reserved), and Remaining.
  - The UI will explicitly display "Pending AI actions..." with an estimated cost, updating automatically once settled, giving the owner real-time confidence that background automation won't exceed their hard limit.

  ## AI Agent Integration Points
  - OperationsAgent/Coordinator nodes dispatching LLM calls or API workloads must pre-calculate or estimate the cost envelope and call `reserve`.
  - Upon failure or completion, the agents must definitively `release` or `settle` the reservation in a `finally` or `Drop` block to prevent leaks.

  # Implementation Prompt
  Implementer: Refactor `BudgetManager` in `src/server/pricing/budget.rs` to support atomic two-phase budget reservation and settlement.
  1. Add tracking for reserved amounts alongside current/settled amounts.
  2. Implement `reserve_cents(amount_cents) -> Result<ReservationToken, Error>` that atomicaly ensures `settled + reserved + amount_cents <= total_limit_cents`.
  3. Implement `settle_cents(token, actual_amount_cents)` that transitions the reserved amount into settled amount.
  4. Implement `release(token)` to clear an unused reservation.
  5. Provide unit tests to simulate highly concurrent reserve and settle operations, proving that hard budget stops are strictly enforced. Maintain API telemetry emission requirements.
  6. Update E2E budget tests if necessary.

  # Priority
  P0

  # Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
