issue_title: "OHC-06: Autonomous Payment Reconciliation & Exception Handling for OHC Invoices"
issue_description: |
  # Title: Autonomous Payment Reconciliation & Exception Handling for OHC Invoices

  # Problem Statement
  Currently, owners handle invoice generation manually, and payment follow-ups are time-consuming and error-prone. The existing system draft reminders but lacks a durable mechanism to distinguish between draft, sent, and delivered states. Furthermore, there is no system to prevent duplicate actions (like sending the same reminder twice) or to automatically stop follow-ups once a payment is made or canceled. This gap forces owners to manually supervise the payment cycle, reducing net time saved.

  # Research Report
  - **Market Landscape**: Leading SMB platforms (e.g., HoneyBook, Square, Stripe) offer automated invoice reminders and reconciliation.
  - **Verified Friction Points**: Owners frequently complain about "chasing payments" and the anxiety of accidentally double-billing or sending reminders to customers who have already paid.
  - **Current OHC Capability**: Code audit (F09) reveals that the receivables module logs a drafted reminder but does not fully implement the draft/delivery lifecycle, nor does it reliably halt on payment/cancellation.

  # Design Doc
  ## High-Level Architecture
  - **Entities**: `Invoice`, `PaymentEvent`, `ReminderState`.
  - **Relationships**: `Invoice` has many `PaymentEvent`s and one active `ReminderState`.

  ```mermaid
  graph TD
      A[Invoice Generated] --> B(Draft Reminder)
      B --> C{Standing Authority?}
      C -- Yes --> D[Send Reminder]
      C -- No --> E[Wait for Approval]
      E -- Owner Approves --> D
      D --> F(Wait for Payment)
      F -- Deadline Missed --> B
      F -- Payment Received --> G[Halt Reminders]
      G --> H[Reconcile Invoice]
  ```

  ## Comparative Feature Matrix
  | Feature | HoneyBook | Stripe | Current OHC | Proposed OHC |
  | :--- | :--- | :--- | :--- | :--- |
  | Automated Reminders | Yes | Yes | Draft Only | Yes (Agentic) |
  | Stop on Payment | Yes | Yes | No | Yes |
  | Avoid Duplicate Sends | Yes | Yes | No | Yes (Idempotent) |

  ## Persona-Specific User Journey Narrative
  Nora generates an invoice for a client using her preferred tool connected to OHC. The client misses the payment deadline. Instead of Nora having to remember to check and manually send a reminder, the OHC agent detects the missed deadline by monitoring the `PaymentEvent` stream. The agent drafts a reminder. If this action requires approval based on Nora's settings, a notification appears on her phone: "Reminder drafted - Waiting for approval." Nora taps to approve, and the reminder is sent. When the client finally pays, the agent immediately detects the payment, halts any future drafted reminders, and reconciles the invoice, all without Nora needing to intervene further.

  ## Rationale
  OHC should implement Autonomous Payment Reconciliation & Exception Handling because code audit F09 confirms the existing receivables code only logs drafted reminders without a complete delivery or recovery lifecycle, and "chasing payments" is a universally verified, high-friction pain point for non-technical service owners like Nora. By automating this securely with explicit authority checks, OHC will measurably reduce the net owner time spent on accounts receivable.

  ## Mobile UX Flow (375px)
  - The owner's feed displays an overview card: "Invoice #123 generated."
  - A notification later updates: "Reminder drafted - Waiting for approval" (if outside standing authority).
  - Once paid, a visual green checkmark updates the feed: "Invoice #123 paid - Reconciliation complete. Reminders halted."

  ## AI Agent Integration
  - The agent monitors `PaymentEvent` streams.
  - Upon detecting a missed deadline, the agent proposes a drafted reminder.
  - If a payment or cancellation event is detected, the agent immediately voids pending reminders.

  # Implementation Prompt
  - Implement the invoice lifecycle state machine, ensuring durable transitions between `draft`, `sent`, and `delivered`.
  - Ensure the reminder system idempotently checks the current invoice status before dispatching any communication.
  - Provide a clear, actionable UI in the owner's feed that surfaces the invoice status and required actions (e.g., approving a reminder if outside standing authority).
  - Critical User Journey: Owner sends an invoice -> Client misses deadline -> Agent drafts reminder -> Owner approves -> Client pays -> Agent halts further reminders and reconciles the invoice.

  # Priority
  P1

  # Estimated Scope
  Medium

  # Strategy Admission
  - Target ID: OHC-06
  - Stage: Run
  - Gap: Incomplete invoice lifecycle management and reconciliation.
  - Evidence: Code audit (F09) and standard owner complaints about chasing payments.
  - Business Result: Reduced owner time spent on payment collection and fewer erroneous customer communications.
  - Metric: Reduction in manual reminder interventions per invoice.
  - Dependencies: Existing Stripe/payment integration.
  - Authority: Drafting reminders is within standing authority; sending them may require approval unless explicitly configured otherwise.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
