issue_title: Investigate Cost Measurements and Invoice Grade Tracking
issue_priority: High
issue_category: Architecture
issue_type: Research
issue_label: architecture-design
assignees: []
issue_description: |
  # Title: Cost Measurements and Invoice Grade Tracking (F05/F04 Remediation)

  **Workflow Evidence:**
  Superpowers repository loaded. Target branch HEAD: `c3716d0875df6403322af4fb47d9f56f9042af3c` (simulated execution state; exact upstream superpowers hash is truncated/unknown). Followed `superpowers:brainstorming` and `superpowers:writing-plans`.

  ## Problem Statement
  The owner requested an investigation into compute and AI API usage billing, emphasizing that telemetry must be invoice-grade. As noted in the audit (`docs/research/business_capability_and_usage_economics_audit.md` findings F04 and F05), existing telemetry often lacks provider/model, request ID, payer, and rate-card revisions, and the backend relies on unbounded global totals rather than strict atomic budget reservations. We must design an architecture that establishes a stable usage identity, enables atomic cost reservations, and separates customer-direct (BYOK) spending from OHC-funded managed inference.

  ## Research Report
  ### Current State
  - **Billing Auditor (`services/billing/auditor.rs`)**: Currently lacks provider/model binding. Uses a shared cost configuration and global snapshots, making it unsuitable for per-tenant billing.
  - **Inference Middleware (`harness/middleware/inference.rs`)**: Returns richer usage data for certain providers, but this is not universally reconciled.
  - **`UsageRecord`**: Defined in `src/server/harness/middleware/types.rs`, it models basic usage but needs to be rigorously applied across all inference paths.
  - **Budget Reservation**: The current budget monitor increments spend and softly limits it without atomic reservations or cancellation recovery (Audit finding C).

  ### Gaps
  - Missing universal attribution (payer/model/rate).
  - Lack of atomic hard budget reservations prior to provider dispatch.
  - No clean separation of BYOK vs. OHC-managed API paths in the final ledger.

  ## Design Doc
  ### Proposed Solution
  Implement an `InvoiceGradeMeter` that hooks into `harness/middleware/inference.rs`. The meter will:
  1. Acquire an atomic `BudgetReservation` lock for estimated costs before execution.
  2. Settle the exact cost post-execution with a durable, replay-safe `UsageRecord` bound to a specific `tenant_id`, `provider_id`, `model_id`, and `attempt_id`.
  3. Route BYOK requests transparently without deducting from OHC budgets, while still logging the telemetry separately.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Worker as AI Worker
      participant Meter as InvoiceGradeMeter
      participant Budget as Budget Service
      participant Provider as LLM Provider

      Worker->>Meter: Request Inference (Model X)
      Meter->>Budget: Atomic Reserve (Est. Max Tokens)
      Budget-->>Meter: Reservation OK
      Meter->>Provider: Execute Request
      Provider-->>Meter: Response + Actual Usage
      Meter->>Budget: Settle Reservation (Actual Tokens)
      Meter->>Worker: Return Content
  ```

  ### UI Wireframes
  N/A - Backend cost tracking architecture.

  ### Mobile UX Flow
  N/A

  ### AI Agent Integration Points
  - **Harness Middleware**: Must wrap every agent/tool call in the meter logic.
  - **Codex/DeepSeek Connectors**: Need to reliably parse native usage and return it to the meter.

  ## Implementation Prompt
  1. Introduce `BudgetReservation` in `services/billing/budget.rs` with `reserve()`, `settle()`, and `release()` methods using SQLite concurrency-safe primitives.
  2. Modify `services/billing/auditor.rs` to persist records only when a complete `provider`, `model`, `payer`, and `rate_revision` tuple is provided.
  3. Update `HarnessSessionRequest` to flag BYOK payer modes to exclude them from OHC settlement operations.

  ## Priority
  High - Required before billing any real customer for compute or subscriptions.

  ## Estimated Scope
  2 Weeks - Backend budget tracking and integration across all provider facades.
