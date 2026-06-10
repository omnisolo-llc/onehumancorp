issue_title: "Agentic Invoicing & Payment Collection System"
issue_description: |
  # Mission Queue Protocol: Agentic Invoicing & Payment Collection System

  ## Problem Statement
  For professional services, agencies, and B2B operators (like Nora, the Agency Principal), managing cash flow is a constant struggle. Creating invoices, sending them to clients, tracking who hasn't paid, and sending polite but firm follow-ups requires significant administrative overhead. Traditional platforms (like QuickBooks or FreshBooks) treat invoicing as a manual data-entry task and are detached from the actual work context and customer conversations. The owner needs an assistant that knows when a project is completed, automatically drafts the invoice with line items, and follows up autonomously until the deposit or payment is collected.

  ## Research Report
  **Market Context & Competitor Analysis:**
  - **Shopify/Wix:** Geared heavily toward immediate checkout for products/services. They lack B2B-style, terms-based invoicing with milestone deposits.
  - **QuickBooks/Xero/FreshBooks:** Robust accounting software, but overly complex for solopreneurs. They lack native AI agents to read emails, summarize project work, and automatically draft line items based on recent conversations.
  - **Stripe Invoicing:** Powerful API but requires the user to build the frontend or use Stripe's dashboard, which is disconnected from OHC's unified assistant shell.
  - **The OHC Opportunity:** Instead of a generic "Create Invoice" form, OHC leverages the Finance & Decision Assistant to proactively suggest: "Project X is marked complete. Should I draft and send an invoice for $2,500?" OHC uses Stripe behind the scenes but presents a simple, conversational approval card to the owner.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Triage Feed:** The owner sees an action card in their feed: "Draft Invoice ready for Nora's Design Project."
  2. **Review Screen:** Tapping the card opens a translucent glass modal. The Assistant has already populated the client's name, project details, and line items based on previous task/proposal memory.
  3. **Edit & Send:** The owner can tap any line item to adjust the price or quantity. A single "Approve & Send" button generates the PDF, creates a Stripe Payment Link, and emails the client.
  4. **Tracking & Auto-Reminders:** The invoice appears in the "Finance" tab. If unpaid 2 days before the due date, the Assistant drafts a polite reminder email and asks the owner, "Send reminder to Nora?" (or sends it autonomously if the permission profile allows).

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Finance AI Assistant] -->|Drafts Line Items| B(Invoice Service)
      B --> C[(PostgreSQL: Invoices & Line Items)]
      B --> D[Stripe Integration]
      D -->|Creates PaymentIntent/Link| E(Stripe API)
      B --> F[Notification & Email Service]
      F -->|Sends PDF & Link| G(Client Inbox)
      C --> H[Agent Feed]
      H -->|Proposes Reminder Action| I(Owner Mobile App)
  ```

  ### AI Agent Integration Points
  - **Finance & Decision Assistant:** Monitors task completion events. Triggers invoice drafting. Generates line items based on proposal context stored in OHC Memory.
  - **Customer Relationship Assistant:** Handles the communication. Drafts personalized emails attaching the invoice. Follows up politely if payment is delayed.

  ### Multi-Tenant Data Model
  - `invoices`: `id`, `tenant_id`, `client_id`, `status` (draft, sent, paid, overdue), `due_date`, `currency`, `total_amount`, `stripe_invoice_id`.
  - `invoice_line_items`: `id`, `tenant_id`, `invoice_id`, `description`, `quantity`, `unit_price`, `amount`.
  *(RLS policies enabled on all tables via `tenant_id`)*

  ## Implementation Prompt
  **Goal:** Implement the backend and mobile-first frontend for the Agentic Invoicing System.
  **CUJ:**
  1. A service business owner (Nora) logs into the OHC Flutter app.
  2. She navigates to the Finance tab and clicks "New Invoice".
  3. The AI Assistant pre-fills client data.
  4. She reviews the generated invoice, approves it, and the system creates a Stripe-backed payment link and persists the invoice to the DB.

  **Acceptance Criteria:**
  - Create database migrations for `invoices` and `invoice_line_items` with strict RLS multi-tenancy.
  - Implement gRPC/REST API endpoints for invoice CRUD.
  - Build the frontend UI in Flutter following the macOS Translucent Glass and UniFi modular dashboard design systems (optimized for 375px mobile view).
  - Add integration with the existing OHC Assistant to support generating and summarizing line items from context.
  - Write 100% unit test coverage for the new services and add a complete Playwright E2E test verifying the invoice creation flow from the frontend to the DB.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
