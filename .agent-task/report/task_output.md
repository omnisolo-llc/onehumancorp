issue_title: "Architecture: Invisible CFO (Autonomous Financial Reconciliation Ledger)"
issue_description: |
  # [Architecture] Autonomous Financial Reconciliation Ledger

  ## Title
  Invisible CFO: Autonomous Financial Reconciliation Ledger

  ## Problem Statement
  Small business owners like Carlos (the handyman) and Maya (the baker) handle money across multiple channels: cash, Instagram DMs (Venmo/CashApp), invoices, and card readers. Currently, they spend hours every Sunday trying to match payments in their bank accounts to invoices or orders. If an invoice is partially paid, or if there's a chargeback, they lose track. OHC needs an "Invisible CFO"—an autonomous ledger that instantly reconciles multi-channel payments against orders and bookings, ensuring small business owners know exactly who owes them money and what their cash flow is without ever opening a spreadsheet or a complex accounting dashboard.

  ## Research Report
  *   **QuickBooks/Xero**: Powerful but built for accountants, not business owners. Require extensive manual categorizations and chart of accounts setup. Fails the "Grandmother Test."
  *   **Stripe**: Excellent at processing the transaction but relies on the platform to handle multi-channel (like cash or external Venmo) reconciliation.
  *   **Square**: Good for POS but struggles when services (invoices) and external payments are mixed, leaving owners confused about outstanding balances.
  *   **The OHC Gap**: OHC lacks a unified, multi-tenant financial ledger that automatically records every transaction (credit, debit, deposit, refund, external cash) against a business event (order, booking, quote) and uses an AI Finance Agent to chase down unpaid balances and categorize expenses autonomously.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ LEDGER_ACCOUNT : owns
      LEDGER_ACCOUNT {
          string id PK
          string type "Asset | Liability | Revenue | Expense"
          decimal balance
          string currency
      }
      LEDGER_ACCOUNT ||--o{ LEDGER_ENTRY : contains
      LEDGER_ENTRY {
          string id PK
          string transaction_id FK
          decimal amount
          string direction "Credit | Debit"
          timestamp recorded_at
      }
      TENANT ||--o{ TRANSACTION_INTENT : creates
      TRANSACTION_INTENT {
          string id PK
          string reference_id "Order/Booking ID"
          string status "Pending | Reconciled | Failed"
          decimal expected_amount
      }
      TRANSACTION_INTENT ||--o{ LEDGER_ENTRY : triggers
  ```

  ### UI Wireframes & Screen Flow (375px First)
  *   **The "Money" Tab**: A clean, single mobile screen showing:
      *   **Big Number**: "Available Cash" (Big, green text).
      *   **Action Card**: "3 Invoices Unpaid - Tap to auto-remind via SMS."
      *   **Recent Activity Feed**: "Maya paid $50 (Deposit for Cake) via Apple Pay."
  *   **No complex tables**: No debits/credits visible to the user. Just "Money In", "Money Out", and "Owed to You."

  ### Mobile UX Flow
  1.  User opens OHC App -> Taps "Money" tab.
  2.  Views "Available Cash" and a list of pending/overdue payments.
  3.  User taps "Remind All." The AI Finance Agent drafts personalized WhatsApp/SMS messages to the late clients.
  4.  User approves with one tap. Messages are sent via the background job queue.
  5.  When a client pays via the generated link, the Ledger autonomously reconciles the payment and a push notification is sent: "Carlos paid his $200 invoice. Bank deposit tomorrow."

  ### AI Agent Integration Points
  *   **AI Finance Agent**: Subscribes to the Ledger's event stream. If a `TRANSACTION_INTENT` remains un-reconciled past the due date, it triggers a workflow to draft a reminder.
  *   **AI Operations Agent**: Uses the Ledger's state to automatically release held inventory if a deposit fails to clear within 24 hours.

  ### Key Design Decisions
  *   **Double-Entry Ledger Pattern**: While hidden from the user, the backend will strictly use an immutable double-entry ledger to ensure financial integrity and prevent race conditions.
  *   **Event-Sourced Architecture**: Every financial event is appended to an event stream (e.g., Kafka/NATS), allowing AI agents to react asynchronously without blocking the core payment flow.
  *   **Zero-Trust Identity**: Ledger modifications require strict multi-tenant authorization contexts, authenticated via SPIFFE/SPIRE.

  ## Implementation Prompt
  **Context**: We need to implement the core Autonomous Financial Reconciliation Ledger. This system will serve as the immutable source of truth for all money moving in and out of a tenant's business.
  **User-Facing Outcome**: Maya should see a simple dashboard showing her total balance and outstanding invoices. When a payment is made via any channel (Stripe, Cash, external app marked manually), the system must instantly reflect the updated balance and mark the associated order/booking as 'Paid'.
  **Acceptance Criteria**:
  1.  Implement a highly-concurrent, immutable double-entry ledger service.
  2.  Ensure strict tenant isolation—a tenant must only ever be able to query or mutate their own ledger accounts.
  3.  Emit domain events for all ledger entries to the NATS event bus for AI agents to consume.
  4.  Design the APIs to support idempotent retries to handle unreliable mobile network conditions.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
