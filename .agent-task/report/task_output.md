issue_title: "[F12] Durable Exception and Unknown Outcome Reconciliation Architecture"
issue_description: |
  ## Problem Statement
  Currently, simulation paths, unknown provider outcomes, and approval pathways can inadvertently appear as "completed" actions within the platform. This presents a critical safety issue for small business owners (like Nora the agency principal or Maya the baker), where ambiguous outcomes (e.g., a payment that is pending reconciliation or an invoice delivery that hasn't received a callback) might trigger downstream fulfillment or mislead the owner into believing a job is fully executed. We need a robust architecture to guarantee truthful states, exact authority validation, handling of stale approvals, and robust reconciliation of unknown provider outcomes.

  ## Research Report
  The One Human Corp capability audit (F12) explicitly calls out that: "Simulation, unknown provider outcome and approval paths can look like completion." The remediation requires: "Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths."

  **Current Behavior vs. Market Expectation:**
  - **Current OHC:** A step may be marked complete if an API call is made, without durable verification of the outcome. Approvals might remain valid even if the underlying business object (e.g., an invoice amount) changes.
  - **Market Baseline (Stripe, Shopify, Quickbooks):** Systems of record strictly delineate between `intent_created`, `processing`, `requires_action`, `succeeded`, and `failed`. Webhooks or polling are required to transition states.
  - **The "Grandmother Test":** The owner needs absolute certainty. If an AI agent says "Invoice sent," it must be provably delivered. If a payment is "received," the funds must be verified. If a state is unknown, it must explicitly say "Waiting for confirmation" or "Needs attention."

  ## Design Doc
  ### Data Model & Invariants
  1.  **Durable State Machine:** Every external action (payment, message, booking) must transition through a strict state machine: `Draft` -> `Pending_Approval` -> `Executing` -> (`Success_Receipt` | `Failed_Exception` | `Unknown_Needs_Reconciliation`).
  2.  **Exact Authority & Revocation:** Approvals must be cryptographically or durably bound to a hash of the payload. If an invoice amount changes, any existing approval for the old amount is immediately revoked.
  3.  **Idempotency Keys:** Every provider interaction must use a durable idempotency key linked to the specific attempt.
  4.  **Reconciliation Ledger:** "Unknown" states enter a reconciliation queue that actively queries the provider (e.g., Stripe, SendGrid) until a terminal state is reached.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner UI (Mobile)
      participant AI Coordinator
      participant Reconciliation Ledger
      participant External Provider (e.g., Stripe)

      Owner UI (Mobile)->>AI Coordinator: Approve Action (Payload Hash)
      AI Coordinator->>Reconciliation Ledger: Create Intent (Status: Executing)
      Reconciliation Ledger->>External Provider: Execute API Call (Idempotency Key)

      alt Network Timeout / Unknown
          External Provider--xReconciliation Ledger: Error/Timeout
          Reconciliation Ledger->>Reconciliation Ledger: Update Status: Unknown_Needs_Reconciliation
          Reconciliation Ledger-->>Owner UI (Mobile): State: "Waiting for Confirmation"
      else Success
          External Provider-->>Reconciliation Ledger: Success Receipt
          Reconciliation Ledger->>Reconciliation Ledger: Update Status: Success_Receipt
          Reconciliation Ledger-->>Owner UI (Mobile): State: "Completed"
      end

      loop Background Reconciliation
          Reconciliation Ledger->>External Provider: Poll/Webhook for Terminal State
          External Provider-->>Reconciliation Ledger: Actual Terminal State
          Reconciliation Ledger->>Reconciliation Ledger: Update Status (Success/Fail)
      end
  ```

  ### Mobile UX Flow (375px)
  - **Activity Feed Card:** Displays clear status badges using Premium Design Standards (Translucent Glass, UniFi modular cards).
  - **Pending Actions:** A distinct section for items awaiting owner approval.
  - **Needs Attention:** A red/amber highlighted section for `Unknown_Needs_Reconciliation` states, providing exactly one clear action (e.g., "Check Stripe for payment status").

  ### AI Agent Integration
  - Agents must query the `Reconciliation Ledger` for the *terminal* state of prior actions before proceeding with dependent tasks (e.g., do not send a "Thank you" email until the payment receipt is confirmed).

  ## Implementation Prompt
  Implement the `Reconciliation Ledger` and `Durable State Machine` for external provider actions in the Rust backend.
  1. Define a `ProviderAction` entity with strict states (`Draft`, `Pending_Approval`, `Executing`, `Success_Receipt`, `Failed_Exception`, `Unknown_Needs_Reconciliation`).
  2. Implement payload hashing for approvals to ensure exact authority; invalidate approvals if the payload changes.
  3. Create a background reconciliation worker that polls providers for actions stuck in the `Unknown_Needs_Reconciliation` state.
  4. Ensure all UI endpoints return the precise state and never map an unknown/executing state to "completed".

  ### Strategy Admission
  - **Target ID:** OHC-08 (Durable exceptions and recovery)
  - **Segment:** All digital service workflows.
  - **Evidence:** Audit finding F12.
  - **Dependencies:** Existing provider integrations (Stripe, Email).
  - **Non-Goals:** Building a full two-phase commit distributed transaction coordinator.
  - **Acceptance:** Tests must prove that a timed-out provider request enters an "Unknown" state and requires active reconciliation before downstream steps proceed.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []