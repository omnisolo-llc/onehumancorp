issue_title: "Agentic Smart Invoicing and Automated Receivables Architecture"
issue_description: |
  # Mission Queue Protocol: Agentic Smart Invoicing & Receivables

  ## 1. Problem Statement
  Service-based and project-based small business owners (like Nora the Agency Principal and Carlos the Handyman) spend a disproportionate amount of time drafting proposals, creating invoices, and chasing late payments. Existing tools (like QuickBooks or Stripe Invoicing) are either too accounting-heavy or require manual configuration of payment reminders and follow-ups. OHC needs an invisible, autonomous invoicing system where the AI drafts the proposal, sends the invoice, and automatically handles all receivables follow-ups without the owner needing to configure complex workflows.

  ## 2. Research Report
  - **Competitor Analysis:**
    - *HoneyBook/Dubsado:* Strong in the creative professional space, offering workflows. However, they rely on the user building manual automation templates.
    - *Stripe Invoicing:* Powerful APIs and solid UI, but requires the user to log into a separate dashboard and manually configure Smart Retries and reminder schedules.
    - *Shopify:* Heavily biased towards immediate e-commerce checkout. Invoicing (B2B/B2C services) is a weak point, usually requiring third-party apps.
  - **OHC Opportunity:** By integrating an "Agentic Receivables" system, the OHC Finance Agent can autonomously draft invoices based on chat context (e.g., WhatsApp messages with clients), schedule intelligent reminders based on the client's payment history, and collect payments via Stripe—all managed through 1-tap approvals in the owner's Agent Feed.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  Strict multi-tenant isolation using `tenant_id` with RLS.
  - `Invoice`: Core entity (status: draft, sent, partial, paid, overdue).
  - `LineItem`: Linked to `Invoice`, detailing services/products.
  - `PaymentSchedule`: Handles deposits and milestone payments.
  - `ReceivablesAction`: Logs AI actions (e.g., "Drafted reminder", "Sent final notice").

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INVOICE : creates
      INVOICE ||--|{ LINE_ITEM : contains
      INVOICE ||--o{ PAYMENT_SCHEDULE : defines
      INVOICE ||--o{ RECEIVABLES_ACTION : logs
      INVOICE {
          uuid id
          uuid tenant_id
          string status
          decimal total_amount
          timestamp due_date
      }
      PAYMENT_SCHEDULE {
          uuid id
          uuid invoice_id
          decimal amount
          string type
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Carlos
      participant App as OHC Mobile App (375px)
      participant Triage as Work Triage Agent
      participant Finance as Finance Agent
      participant Stripe
      actor Client

      Client->>Triage: "I need my sink fixed, quoted $200"
      Triage->>Finance: Context: Generate invoice for $200
      Finance-->>App: Card: "Drafted Invoice for Sink Repair. Send?"
      Carlos->>App: Taps "Approve"
      Finance->>Client: Sends Email/SMS with Stripe Link
      Note over Finance, Client: 3 Days Later (Invoice Overdue)
      Finance-->>App: Card: "Client hasn't paid. Send friendly reminder?"
      Carlos->>App: Taps "Approve"
      Finance->>Client: Sends SMS Reminder
      Client->>Stripe: Pays Invoice
      Stripe->>Finance: Webhook (Payment_Intent.succeeded)
      Finance-->>App: Notification: "Invoice Paid!"
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Feed:** The owner receives an Action Card when an invoice is drafted or overdue.
  - **1-Tap Approval:** A massive "Approve & Send" button (min 44x44px touch target) for sending invoices or reminders.
  - **Invoice View:** A translucent glass card displaying the total amount, client name, and a summarized breakdown. No complex line-item editing unless the user taps "Edit Details" (hidden under an advanced flow).
  - **Offline/Flaky Network:** If Carlos approves an invoice while in a basement with no signal, the action is queued locally using local CRDTs and sent when connection is restored.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant:** Monitors the ledger, detects overdue states, and drafts plain-language reminders. Modulates tone based on client history (e.g., friendly for first offense, firm for 30+ days overdue).
  - **Work Triage:** Parses incoming emails or DMs to extract quoting information and automatically passes it to the Finance Agent to draft the initial invoice.

  ### Zero Trust & Security
  - All invoicing operations are gated by SPIFFE SVIDs for inter-service communication.
  - Stripe webhook signatures are rigorously verified. Row Level Security guarantees tenant isolation.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** The owner (Carlos/Nora) receives ready-to-send invoices and payment reminders in their mobile feed, generated from client conversations or task completions.

  **Critical User Journey (CUJ):**
  1. Carlos marks a repair task as "Complete" in the OHC app.
  2. The Finance Agent detects the completion and drafts a $200 invoice based on the initial quote.
  3. Carlos receives an Action Card: "Task Complete. Send $200 invoice to John Doe?"
  4. Carlos taps "Approve". The invoice is sent via SMS.
  5. If John Doe doesn't pay by the due date, Carlos receives a new card: "Invoice #102 is overdue. Send reminder?" Carlos taps "Approve".

  **Acceptance Criteria:**
  - Implement the `Invoice` and `PaymentSchedule` data models with PostgreSQL RLS.
  - Build the Finance Agent capability to draft invoices and reminders based on task/booking status.
  - Implement the Mobile UI Action Cards (375px width, translucent glass styling, 44px touch targets).
  - Provide full E2E Playwright coverage for the approval flow.

  ## Priority & Scope
  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
