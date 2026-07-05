issue_title: "Agentic Instant Localized Invoicing & Auto-Reconciliation Architecture"
issue_description: |
  # Agentic Instant Localized Invoicing & Auto-Reconciliation Architecture

  ## Problem Statement
  For service-based operators (like Nora, an agency principal, and Carlos, a field service owner), managing invoices, chasing payments, and reconciling them against bank statements is a significant time sink. Traditional platforms (e.g., Quickbooks, Xero, Freshbooks) treat this as a manual double-entry accounting exercise. They often require the owner to generate an invoice, send it via a separate email, manually check if the payment cleared, and manually reconcile the bank feed. This disconnected flow creates friction and delays revenue collection for non-technical users who simply want to get paid.

  ## Research Report
  - **Competitor Landscape**:
    - **Shopify / Stripe Invoicing**: While Stripe offers programmatic invoicing, their dashboard is too complex for Carlos or Nora to use on the fly on mobile. Shopify is e-commerce first, treating services/custom invoices as an afterthought.
    - **Quickbooks / Xero**: Excellent at accounting, poor at agentic workflows. They tell you an invoice is late but do not autonomously negotiate payment terms, send SMS reminders in the local language, or intelligently bundle open invoices.
    - **Square**: Good basic invoicing for in-person and simple services, but lacks deep CRM context and multi-agent coordination.
  - **The OHC Opportunity**:
    - OHC needs an invisible, agent-driven localized invoicing pipeline. The system should generate an invoice from a simple chat prompt or approved quote, localize the currency/language for the client, distribute it via the optimal channel (SMS, Email, WhatsApp), and automatically reconcile the payment via webhooks.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant Owner (Mobile)
      participant OperationsAgent
      participant FinanceAgent
      participant Ledger (PostgreSQL)
      participant PaymentGateway (Stripe)
      participant Client

      Owner (Mobile)->>OperationsAgent: "Job done. Bill John $500 for roof repair."
      OperationsAgent->>FinanceAgent: Request invoice generation (Amount: 500, Client: John)
      FinanceAgent->>Ledger: Create Pending Invoice (Tenant Isolation)
      FinanceAgent->>PaymentGateway: Create Payment Link/Intent
      PaymentGateway-->>FinanceAgent: Link generated
      FinanceAgent->>OperationsAgent: Draft Invoice ready
      OperationsAgent-->>Owner (Mobile): Push Action Card "Approve $500 Invoice to John?"
      Owner (Mobile)->>OperationsAgent: Taps "Approve & Send"
      OperationsAgent->>Client: Send SMS/WhatsApp with Payment Link
      Client->>PaymentGateway: Pays Invoice
      PaymentGateway-->>FinanceAgent: Webhook (Payment Succeeded)
      FinanceAgent->>Ledger: Update Invoice Status (Paid)
      FinanceAgent-->>Owner (Mobile): Push Notification "$500 Received from John"
  ```

  ### Mobile UX Flow (375px First)
  1. **Trigger**: Carlos finishes a job. He opens OHC and types or speaks: "Bill John $500 for the roof repair."
  2. **Action Card**: The feed immediately shows a translucent Glassmorphism action card:
     - **Title**: Draft Invoice for John Doe
     - **Details**: Roof Repair - $500.00
     - **Channel**: SMS (Primary contact)
     - **Buttons**: [ Approve & Send ] [ Edit ] [ Discard ]
  3. **Payment Receipt**: Once paid, a green unified notification card appears in Carlos's feed summarizing the deposit without him needing to open a separate finance tab.

  ### AI Agent Integration Points
  - **Operations Agent**: Acts as the conversational interface for the owner, translating natural language into structured invoice creation intents.
  - **Finance Agent**: Handles the heavy lifting of Stripe API interactions, localized currency formatting, tax calculation, and webhook reconciliation.
  - **Customer Success Agent**: Can step in if the invoice goes unpaid, drafting a polite follow-up reminder based on the client's past interaction history.

  ### Key Design Decisions
  - **Zero-Touch Reconciliation**: The ledger must instantly reflect the paid status upon Stripe webhook receipt, eliminating manual bank matching.
  - **Idempotency**: All payment API calls must use strict idempotency keys to handle flaky mobile networks gracefully.
  - **Tenant Isolation**: All ledger operations must be strictly scoped by `tenant_id` at the database level.

  ## Implementation Prompt
  Implement the Finance Agent's core invoicing capability and the corresponding Mobile UI Action Card.
  - **Backend**: Extend the gRPC API and PostgreSQL schema to support `Invoice` and `InvoiceLineItem` entities with strict tenant isolation. Integrate Stripe Checkout/Payment Links generation. Implement the Stripe webhook handler to transition invoice state from `draft` -> `pending` -> `paid`.
  - **AI Coordination**: Update the Operations/Finance agent prompts to recognize invoicing intents from natural language and output structured invoice payloads.
  - **Frontend**: Build the `DraftInvoiceActionCard` component in Flutter following the OHC Premium Token library (translucent materials, 44x44px touch targets). The component should allow the user to approve the draft with a single tap.
  - **Acceptance Criteria**: A simulated natural language prompt from Carlos results in a draft invoice card on the mobile UI. Approving the card generates a Stripe link. Simulating a Stripe webhook payment success automatically marks the invoice as paid in the database and updates the UI.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
