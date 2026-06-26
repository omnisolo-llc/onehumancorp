issue_title: "Feature: Agentic Smart Quotes & Frictionless Auto-Reconciliation Invoicing"
issue_description: |
  # Research Report: Agentic Smart Quotes & Frictionless Auto-Reconciliation Invoicing

  ## 1. Problem Statement
  Service-based small business owners (e.g., Carlos the Field Service Owner, Nora the Agency Principal) often experience friction and lost revenue because quoting, invoicing, and payment reconciliation are disjointed processes. Today, preparing a quote requires switching context to a specialized app, manually sending it via email, chasing approvals, manually converting it to an invoice, and finally trying to reconcile payments against the bank or Stripe feed. This process is time-consuming, error-prone, and not suitable for a mobile-first, non-technical operator.

  ## 2. Research Report
  - **Market Context**: Platforms like Quickbooks, FreshBooks, or specialized industry apps (e.g., Jobber, HoneyBook) offer strong invoicing but often feel like complex accounting suites. E-commerce platforms like Shopify focus heavily on carts and products, treating quotes as an afterthought (often requiring 3rd party apps).
  - **The OHC Opportunity**: Integrate quoting and invoicing natively into the core "Work Intake" and "Offers & Revenue" flows. Empower the AI agents to draft quotes from DMs, monitor approvals, issue invoices, and automatically reconcile payments without the owner needing to open a single spreadsheet.
  - **Competitor Gaps**:
    - *Shopify*: Primarily built for carts. B2B / Quoting is clunky or requires expensive apps.
    - *Quickbooks/Freshbooks*: Great for accounting, but lacks the frontend "Customer Service Agent" (The Ambassador) to instantly draft the quote from an Instagram DM or SMS.
    - *Jobber/Honeybook*: Good vertical SaaS, but siloed and sometimes complex for micro-SMEs to adopt. OHC's unified assistant approach handles this more elegantly.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry/DM] --> B(Work Triage / The Ambassador Agent)
      B --> C[Draft Quote / Proposal]
      C -->|Owner Approves| D[Customer Views Quote via Mobile/Web Link]
      D -->|Customer Accepts| E[The Sales Assistant converts to Invoice]
      E --> F[Payment Request Sent via Stripe Link]
      F --> G[Stripe Webhook]
      G --> H{Finance Agent / Auto-Reconciliation}
      H --> I[Ledger Updated & Owner Notified via Daily Summary]
  ```

  ### Data Model (PostgreSQL)
  - `Quote`: Contains customer details, line items, expiration date, and state (draft, sent, accepted, rejected).
  - `Invoice`: Linked to a `Quote` (optional), contains line items, due date, amount, and state (draft, sent, partially_paid, paid).
  - `LedgerEntry`: Immutable records of transactions.
  - `ReconciliationJob`: Async jobs tracking incoming webhooks to link payments to specific invoices.

  ### AI Integration
  - **Sales & Revenue Assistant**: Extracts line items from a conversational DM (e.g., "I need a standard lawn cleanup and gutter clearing") and maps them to catalog services to draft the quote.
  - **Operations Assistant**: Tracks accepted quotes and creates corresponding service tasks/bookings.
  - **Finance & Decision Assistant**: Monitors Stripe webhooks, automatically marks invoices as paid, and handles partial payments/deposits. Generates "outstanding invoice" summaries for the owner.

  ### Mobile UX Flow (375px)
  1. **Owner Feed**: "Carlos, you have a new quote request from Sarah."
  2. **Quote Draft View**: A clear, card-based view showing the AI-drafted quote. The owner can tap to edit line items or hit "Approve & Send".
  3. **Customer View**: A clean, single-page web view to review the quote, with a large primary "Accept & Pay Deposit" button.
  4. **Post-Payment**: The owner feed updates: "Sarah accepted the quote and paid the $50 deposit. The job is scheduled for Friday."

  ### Key Design Decisions
  - **Mobile-First Quoting**: Complex tables are avoided. Line items are rendered as cards on mobile.
  - **Agentic Handoffs**: The quote to invoice to payment pipeline is fully managed by the AI agents, requiring owner intervention only for initial approval or anomaly handling.

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Smart Quotes & Invoicing
  **Target Persona**: Carlos (Field Service), Nora (Agency)
  **Objective**: Build the end-to-end flow allowing the AI to draft a quote from a customer message, allowing the owner to approve it with one tap, converting it to an invoice upon customer acceptance, and handling the auto-reconciliation of the payment.
  **Acceptance Criteria**:
  - The Sales Assistant can generate a draft `Quote` based on a text prompt simulating a customer inquiry.
  - The UI (at 375px) displays the quote in a clear, card-based layout where the owner can approve it.
  - Upon customer acceptance (simulated or real), the system automatically generates an `Invoice` and associated payment link.
  - A mock or real Stripe webhook triggers the Finance Agent to successfully reconcile the payment against the invoice and update the ledger.
  - The owner's feed displays a notification of the accepted quote and reconciled payment.

  **Note to Implementer**: Design the specific database schemas and API endpoints. Ensure multi-tenant isolation and robust error handling for the payment webhooks.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []