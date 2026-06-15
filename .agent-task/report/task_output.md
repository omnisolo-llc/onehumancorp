issue_title: "Architecture & Design: Autonomous Quote-to-Cash & Smart Invoicing Agentic Workflow"
issue_description: |
  ## 1. Problem Statement
  Service-based and project-based small business owners (e.g., Nora the Agency Principal, Carlos the Handyman) experience massive friction between getting a lead, issuing a quote, converting it to an invoice, and actually getting paid. They use disjointed tools (like a CRM for leads, a Word doc for quotes, and Quickbooks for invoicing), leading to missed follow-ups, delayed payments, and hours of manual data entry.

  **The Gap:** OHC currently lacks an integrated, AI-driven Quote-to-Cash lifecycle. Owners need an assistant that doesn't just generate a PDF quote, but proactively follows up on it, converts approved quotes into scheduled work, and autonomously handles invoice generation, deposit collection, and late-payment reminders based on real-time business context.

  ## 2. Research Report
  - **Market Context:** Traditional tools (Quickbooks, Freshbooks) provide static templates for quotes and invoices but rely on the user to remember to send, follow up, and reconcile. Platforms like HoneyBook offer better workflow automation but are rigid and template-based rather than contextually aware and agent-driven.
  - **The OHC Opportunity:** OHC can unify the Sales & Revenue Assistant with the Finance Assistant. When a lead asks for a price via Instagram DM, the agent drafts the quote. Upon approval, it automatically generates the deposit invoice, updates the calendar, and creates project tasks. If unpaid, it intelligently follows up via the original channel (e.g., WhatsApp).
  - **Competitor Gaps:**
    - *HoneyBook/Dubsado:* Good workflows, but clunky mobile experience; poor integration with physical products or hybrid (service+product) models.
    - *Quickbooks/Xero:* Pure accounting; terrible at the pre-sales/quoting phase; no AI agent for customer communication.
    - *Shopify:* Extremely weak native quoting for B2B or custom services.

  ## 3. Design Doc

  ### Data Model (PostgreSQL - Multi-Tenant)
  - `Quote`: Contains items, pricing, terms, expiration date, and linked `Customer`. State machine: Draft, Sent, Viewed, Approved, Rejected, Expired.
  - `Invoice`: Linked to a `Quote` (optional) and `Customer`. Contains line items, tax, total, due date. State machine: Draft, Sent, Partially Paid, Paid, Overdue, Cancelled.
  - `Payment`: Linked to an `Invoice` (Stripe Payment Intent ID, amount, method, status).
  - `LedgerEntry`: Immutable record of financial transactions for the Finance Assistant to summarize.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B(Work Triage / Omnichannel Inbox)
      B --> C[Sales Assistant Agent]
      C -->|Drafts| D[Quote Entity]
      D -->|Owner Approves UI| E[Sent to Customer via SMS/Email]
      E -->|Customer Accepts link| F(Quote State: Approved)
      F --> G[Finance Assistant Agent]
      G -->|Generates| H[Deposit Invoice Entity]
      H --> I[Stripe Payment Link]
      I -->|Webhook Success| J[Invoice State: Paid]
      J --> K[Operations Assistant Agent]
      K -->|Schedules| L[Tasks / Calendar Events]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Quote Drafting Screen:**
    - Translucent glass card header: "Draft Quote for Sarah".
    - Auto-populated line items suggested by the AI based on the DM conversation context.
    - "Add Item" button (44x44px touch target).
    - Bottom sticky action bar: "Approve & Send Quote" (Primary), "Edit Details" (Secondary).
  - **Finance Dashboard (Mobile Feed):**
    - A clear, simple list: "Awaiting Deposit (2)", "Overdue (1)", "Paid This Week ($1,200)".
    - Tapping an overdue invoice shows an AI suggestion: "Draft WhatsApp reminder to John?" with a 1-tap "Send" button.

  ### AI Agent Integration Points
  - **Sales Assistant:** Listens for "pricing" or "estimate" intents in conversations. Queries the `Product/Service Catalog` and drafts a `Quote` object in the database.
  - **Finance Assistant:** Monitors `Quote` state changes. When approved, it drafts an `Invoice` and requests a payment link via the Stripe integration. Runs a daily background job (via AI Job Queue) to find overdue invoices and drafts polite follow-up messages for the owner to approve.

  ### Key Design Decisions
  - **State Machines:** Strict state transitions for Quotes and Invoices to prevent race conditions (e.g., can't pay an invoice that is in Draft).
  - **Omnichannel Follow-ups:** The Finance Assistant must remember the channel the Quote was requested on and prefer that channel for follow-ups (e.g., if quoting over WhatsApp, don't follow up via Email unless necessary).
  - **Zero-Touch Fallback:** If the AI is unsure about pricing (e.g., a highly custom job), it creates a blank quote draft, assigns it a "Needs Review" tag, and alerts the owner in the daily feed.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** As an agency principal (Nora), I receive a request for a new web design project via email. My OHC app notifies me with a pre-drafted Quote for $5,000 based on my standard rates. I tap "Send". The client approves it online. OHC automatically generates a 50% deposit invoice, sends the Stripe link, and upon payment, marks my project status as "Active" - all without me typing a single document or opening an accounting app.

  **CUJ & Acceptance Criteria:**
  1. Implement the `Quote` and `Invoice` PostgreSQL tables with row-level security (tenant_id).
  2. Implement the gRPC/REST endpoints for CRUD operations on Quotes and Invoices.
  3. Create the Sales Agent tool `draft_quote(customer_id, items, terms)` and Finance Agent tool `generate_invoice_from_quote(quote_id, deposit_percentage)`.
  4. Develop the Flutter mobile UI (375px optimized) for the "Review Quote" and "Invoice List" screens using the OHC Premium Token library (glassmorphism, clear visual hierarchy).
  5. Provide Playwright E2E tests: A test user logs in, navigates to the inbox, sees an AI-drafted quote, approves it, and verifies the resulting Invoice appears in the Finance feed.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []