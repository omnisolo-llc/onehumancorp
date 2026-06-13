issue_title: "Implement Intelligent Automated Quoting & Invoicing System"
issue_description: |
  # Research Report: Intelligent Automated Quoting & Invoicing System

  ## Problem Statement
  Service-based SMBs (like Nora the Agency Principal, Carlos the Field Service Owner) spend an inordinate amount of time translating scattered context—emails, DMs, project notes, calendar events, and informal chats—into formal quotes and invoices. Currently, they must manually assemble project details, calculate line items, create quotes in external software (or generic templates), send them for approval, convert them to invoices upon acceptance, and track payments. This manual process is error-prone, delays revenue collection, and requires continuous context-switching away from mobile operations. The existing platform lacks an agent-driven capability that can instantly draft, send, and manage quotes/invoices from ambient conversational context on a 375px screen.

  ## Research Report
  - **Market Context**: Legacy platforms (QuickBooks, FreshBooks, Xero) provide robust invoicing but are fundamentally accounting ledgers requiring manual data entry. They lack contextual awareness of the prior customer conversations that generated the work. CRMs (HubSpot) attempt to bridge this but remain overly complex for micro-SMBs. New AI-native tools (like Durable or specialized AI executive assistants) offer simple invoicing but are not deeply integrated into a multi-channel operational assistant.
  - **The OHC Opportunity**: OHC's unique advantage is the "Unified Agent Feed." Because the Customer Assistant handles intake and the Operations Assistant manages tasks, the Sales/Finance Assistant can autonomously intercept signals (e.g., a customer agreeing to a service via DM) to instantly draft a quote. This removes the manual data-entry step entirely.
  - **Competitor Gaps**:
    - *Shopify*: Excellent for physical product checkout, but highly complex for service-based quoting without expensive third-party apps.
    - *Stripe Invoicing*: Powerful backend, but the mobile dashboard is geared toward developers, not operators running a business from their phone.
    - *Wix/Squarespace*: Provide basic quote forms, but they are passive—waiting for the user to type in line items rather than inferring them from project context.

  ## Design Doc
  ### Architecture & Data Model (PostgreSQL)
  The core architecture involves extending the existing ledger and invoice definitions into a comprehensive Quoting-to-Invoice pipeline managed by the Finance Agent.
  - **`Quote` Entity**: `id`, `tenant_id`, `client_id`, `status` (Draft, Sent, Accepted, Rejected), `total_amount`, `currency`, `valid_until`.
  - **`QuoteLineItem` Entity**: Linked to `Quote`.
  - **`Invoice` Entity** (Extension): Link to an accepted `Quote_id` to trace the origin. Ensure Stripe Integration (`stripe_payment_link`, `stripe_invoice_id`) remains the source of truth for payment status via Webhooks.
  - **Multi-Tenant Isolation**: Row-Level Security (RLS) on all new tables (`quotes`, `quote_line_items`) locked to `app.current_tenant`.

  ### AI Agent Integration Points
  - **Finance Agent (The Accountant)**: Exposes a new tool capability: `DraftQuoteFromContext`. When the Work Triage system identifies an intent to price a service, it invokes this tool with the project context. The agent generates the structured quote and pushes an "Action Card" to the Agent Feed.
  - **Customer Assistant (The Ambassador)**: Drafts the accompanying email/message when sending the quote or invoice.

  ### Mobile UX Flow (375px First)
  1. **Trigger**: Nora receives an email from a client requesting a new design sprint.
  2. **Agent Feed Card**: An Action Card appears: "Draft Quote for Design Sprint? (Estimated: $2500)".
  3. **Review & Edit (Mobile)**: Tapping "Review" opens a full-screen, clean, translucent glass UI. The agent has pre-filled line items (e.g., "Strategy Session - $500", "Figma Mockups - $2000"). Touch targets are large (>44px). Nora can easily tap to adjust amounts or delete lines.
  4. **Approval**: Nora taps a large "Send Quote" button.
  5. **Conversion**: Once the client accepts (via a generated web link), a new Action Card appears: "Convert Quote to Invoice and Request 50% Deposit?".
  6. **Payment**: The client pays via Stripe Checkout; the OHC system automatically reconciles the ledger and updates the invoice status to "Paid".

  ## Implementation Prompt
  **Feature Name**: Autonomous Quote-to-Invoice Pipeline
  **Target Persona**: Nora (Agency Principal) / Carlos (Field Service)
  **User-Facing Outcome**: The user can approve and send formal, itemized quotes generated automatically from chat/email context. Upon client acceptance, quotes are seamlessly converted into Stripe-backed invoices, with all actions managed via simple mobile Action Cards in the Agent Feed.

  **Acceptance Criteria**:
  1.  Define the database schema for Quotes and Quote Line Items with strict RLS multi-tenancy.
  2.  Extend the `InvoiceService` gRPC definitions (or create a new `QuoteService`) to handle quote CRUD and quote-to-invoice conversion.
  3.  Implement the `DraftQuoteFromContext` capability within the relevant AI Agent (Finance/Sales), allowing it to parse unstructured text into structured line items.
  4.  Create the mobile-first UI flow for reviewing, editing, and approving the drafted Quote, utilizing the OHC Design Tokens (translucent materials, large touch targets).
  5.  Ensure Stripe payment link generation is correctly tied to the finalized invoice.
  6.  Full E2E Playwright tests simulating the entire quoting flow from generation to acceptance.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
