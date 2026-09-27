issue_title: "True Provider Outcome Receipts and Explicit Simulation Boundaries (F12)"
issue_description: |
  **Title**: True Provider Outcome Receipts and Explicit Simulation Boundaries (F12)

  **Problem Statement**:
  Currently, as noted in the F12 audit finding, simulation paths, unknown provider outcomes, and generic approval workflows can masquerade as actual business completion. From an owner/operator perspective, this means the platform might report an invoice as "paid" or a delivery as "complete" when it only simulated the process or when a provider's state is actually unverified or failed. This false sense of completion risks real financial loss and customer dissatisfaction because the business owner believes work is done and money is collected when it isn't. The platform needs strict, explicit boundaries that separate verified provider outcomes from pending approvals or simulations.

  **Research Report**:
  - **Context**: The `docs/research/native_migration_and_remediation.md` audit identifies finding F12: "Simulation, unknown provider outcome and approval paths can look like completion." The status is currently Blocked.
  - **Findings**: The core issue is a lack of strict state transitions and cryptographic/durable proof of external provider (e.g., Stripe, Shopify) actions. Generic "status updates" in the UI or backend can bypass actual provider reconciliation.
  - **Strategy Admission**:
    - Target ID: F12 (Simulation and False Completion)
    - Customer/Stage: All segments (Nora, Maya, Carlos) / Post-Onboarding Execution
    - Evidence Level: Documented Audit Finding
    - Baseline: Current state allows generic status mutations to bypass provider verification.
    - Dependencies/Reuse: Existing `Postgres` schema for jobs/invoices, `tool_integrations.rs` for provider adapters.
    - Non-goals: Building a generic ERP; replacing the existing billing system entirely; implementing new payment gateways.
    - Authority Class: System-level state invariants.
    - Cost Plan: Minimal extra DB writes; synchronous API checks during state transitions.
    - Acceptance Checks: A simulated action must explicitly show as "Simulated" and cannot transition a real invoice to "Paid". Only a verified provider receipt can transition state.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
      participant App as Client UI (Tauri/Next)
      participant API as Backend API
      participant DB as Postgres (State)
      participant Provider as External Provider

      App->>API: Request Status Update (e.g., Mark Paid)
      API->>DB: Check current state
      alt Is Simulation/Test Mode?
        API->>DB: Record Simulated Event
        API-->>App: Return "Simulated Success" (Explicit UI)
      else Is Live Mode?
        API->>Provider: Verify Outcome / Fetch Receipt
        alt Provider Confirms Success
          API->>DB: Store Receipt ID & Update State to "Completed"
          API-->>App: Return "Success"
        else Provider Fails/Unknown
          API->>DB: Update State to "Pending/Needs Attention"
          API-->>App: Return "Error/Needs Manual Verification"
        end
      end
    ```
  - **Mobile UX Flow**: On a 375px viewport, items in a "Simulated" or "Pending" state will have distinct, high-contrast badges (e.g., striped yellow for test, solid red for needs attention). The "Mark as Complete" button is disabled unless accompanied by a verified provider receipt ID or explicit owner override with a warning modal.
  - **AI Agent Integration Points**: Agents cannot unilaterally mark financial or delivery milestones complete. They must submit a `ProviderReceipt` token to the orchestration hub, which the hub verifies before updating the `Postgres` ledger.
  - **Key Decisions**: Decouple the "requested state" from the "verified state" in the data model. Introduce a required `receipt_id` or `provider_verification_token` for all terminal state transitions.

  **Implementation Prompt**:
  Implement a strict state machine for invoices and deliverable jobs that requires a verified provider receipt to reach a "Completed" or "Paid" state.
  1. Add a `provider_receipt_id` and `verification_status` to the relevant Postgres tables.
  2. Modify the API endpoints to reject status updates to "Completed" if they lack a valid provider receipt, unless explicitly flagged as a simulation (which must result in a "Simulated_Completed" state, not true "Completed").
  3. Ensure the UI clearly distinguishes between these states, displaying warnings if an action is blocked due to unverified provider outcomes.
  4. Write acceptance tests verifying that generic status updates cannot bypass provider receipt validation.

  **Priority**: P0
  **Estimated Scope**: Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []