issue_title: "Implement High-Scale Multi-Currency Invoicing Architecture"
issue_description: |
  # Research Report: High-Scale Multi-Currency Invoicing Architecture

  ## Title
  Implement High-Scale Multi-Currency Invoicing Architecture

  ## Problem Statement
  Small business owners and operators (like Nora the agency principal or Priya the boutique owner) often operate across borders or need to send professional, localized invoices to international clients. Currently, OneHumanCorp (OHC) lacks a native, high-scale, multi-currency invoicing system that integrates seamlessly with our AI agents and the existing Stripe infrastructure. Relying on complex external tools creates friction. We need an invoicing architecture that scales, handles complex currency conversions automatically, and allows the "Finance Agent" (The Accountant) to autonomously draft, send, and follow up on invoices without technical setup from the user.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Stripe Invoicing:** Robust, but API-heavy and often requires developers to integrate deeply. Standalone Stripe Dashboard is disconnected from the business's core operational flow (tasks, CRM).
  - **Shopify / Wix:** E-commerce focused; their B2B invoicing features are often add-ons or clunky compared to dedicated tools like Xero or QuickBooks.
  - **Xero / QuickBooks:** Excellent for accounting, but not "assistant-first." They require manual data entry or complex sync setups.
  - **OHC Opportunity:** Build a native invoice data model that integrates tightly with our `Tenant` and `Customer` graphs. Leverage Stripe for the actual payment processing (PaymentIntents/Checkout Sessions), but let the OHC Finance Agent ("The Accountant") manage the lifecycle: drafting from project tasks, sending reminders, and reconciling payments, all via the mobile-first Agent Feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Project Tasks / Sales] -->|Trigger| B(Finance Agent - The Accountant)
      B -->|Drafts Invoice| C[PostgreSQL: Invoices Table]
      C -->|Line Items, Currency, Tax| D[PostgreSQL: Invoice_Items Table]
      B -->|Action Required| E[Mobile App Feed 375px]
      E -->|User Approves| F[Stripe Integration Layer]
      F -->|Creates Payment Link| G[Stripe API]
      G -->|Returns Link| F
      F -->|Updates DB & Notifies| C
      F -->|Sends Email/SMS| H[Customer]
      H -->|Pays via Stripe| I[Stripe Webhook]
      I -->|Marks Paid| C
      I -->|Notifies Owner| E
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Agent Feed:** A new card appears: "Draft Invoice ready for Nora Design Project. Total: $1,200 USD. Approve & Send?"
  - **Invoice Detail View:** Tapping the card opens a clean, glassmorphic view of the invoice. Shows line items, subtotal, tax, and a currency selector (e.g., USD to EUR conversion preview).
  - **Action:** Prominent "Send to Client" button.
  - **Empty/Loading States:** Skeleton loaders for invoice generation. "No pending invoices" empty state using truthful data from the backend.

  ### AI Agent Integration Points
  - **The Accountant (Finance Agent):** Triggered by project completion or manual request. Drafts the invoice by querying the tenant's services, the customer's details, and applying correct localized currency formatting.
  - **The Manager (Operations Agent):** Can trigger The Accountant when a task marked "billable" is completed.

  ### Key Design Decisions
  - **Data Model:** Introduce `invoices` and `invoice_items` tables with strict `tenant_id` isolation (RLS). Must store currency codes (ISO 4217) and amounts in the lowest denomination (e.g., cents) to avoid floating-point errors.
  - **Stripe as Execution Engine:** OHC owns the invoice *state* and *presentation*, but Stripe handles the actual payment collection to simplify compliance and security.
  - **Mobile-First Approval:** The owner doesn't build the invoice from scratch; they review and approve what the AI drafted based on the work context.

  ## Implementation Prompt
  **User-Facing Outcome:** As an agency owner (Nora), when I finish a project, I open OHC on my phone and see an invoice already drafted for the client, converted to their local currency if needed. I tap "Approve," and the client receives a professional, localized payment link.

  **CUJ & Acceptance Criteria:**
  1. Define the PostgreSQL schema for `invoices` and `invoice_items`, enforcing row-level security by `tenant_id`.
  2. Implement the backend service (Rust/Axum) to create, read, update, and delete invoices.
  3. Integrate with the existing Stripe module to generate a Payment Link when an invoice is approved.
  4. Create the mobile-first UI (Flutter/PWA) to view drafted invoices in the feed and approve them.
  5. Provide Playwright E2E tests: A user logs in, sees a drafted invoice in the feed, reviews the line items and currency, taps "Approve", and verifies the invoice status changes to 'Sent' and a mocked payment link is generated.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
