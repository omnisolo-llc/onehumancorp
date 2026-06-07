issue_title: "Implement Instant Localized Invoicing Architecture"
issue_description: |
  # Mission Queue Protocol Brief

  ## Problem Statement
  Service-based and creative business owners (like Carlos the Handyman and Leo the Music Tutor) need to send professional, localized invoices to clients. Currently, many rely on external tools like QuickBooks or manual Word templates because e-commerce platforms like Shopify or Wix are too rigidly focused on shopping carts and physical products. This creates friction, delays payment, and disconnects invoicing from their core business management system.

  ## Research Report
  - **Market Context**: Platforms like Shopify handle orders well but struggle with service-based invoicing. Dedicated invoicing tools (FreshBooks, QuickBooks) are too complex for our target personas and add unnecessary costs.
  - **The OHC Opportunity**: We can build a native invoicing system that allows business owners to instantly generate, send, and track localized invoices directly from their mobile devices, deeply integrated with the central ledger and AI agents.
  - **Competitor Gaps**:
    - *Shopify*: Cumbersome for custom service invoicing.
    - *Square*: Good invoicing, but lacks the proactive AI agent management.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      Tenant ||--o{ Invoice : issues
      Customer ||--o{ Invoice : receives
      Invoice ||--o{ InvoiceLineItem : contains
      Invoice {
          uuid id
          uuid tenant_id
          uuid customer_id
          string status
          timestamp due_date
          string currency
          decimal total_amount
          string localized_pdf_url
      }
      InvoiceLineItem {
          uuid id
          uuid invoice_id
          string description
          decimal quantity
          decimal unit_price
      }
  ```

  ### Data Model & Invariants
  - **PostgreSQL Ledger**: Invoices are treated as first-class financial documents, linked to Tenants and Customers.
  - **Multi-Tenant Isolation**: Row-level security on `tenant_id` for all invoice tables.
  - **Localization**: Support for different currencies and date formats based on the customer's locale.

  ### AI Integration Points
  - **Sales Agent**: Can draft invoices based on chat history or accepted quotes.
  - **Finance Agent**: Tracks unpaid invoices and auto-drafts polite follow-up reminders.

  ### Mobile UX Flow (375px)
  1. **Dashboard**: "Create Invoice" button prominent on the finance tab.
  2. **Creation Flow**: Simple form to select customer, add line items (or select from predefined services), and set due date.
  3. **Preview**: Instantly generates a clean, professional PDF preview.
  4. **Send**: One-tap to send via email or SMS with a Stripe payment link.

  ## Implementation Prompt
  **Feature Name**: OHC Instant Localized Invoicing
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos finishes a plumbing job and instantly creates and sends an invoice from his phone before leaving the client's driveway.

  **Next Actions**:
  1. Define the SQL schema for `invoices` and `invoice_line_items` with strict RLS.
  2. Implement the API endpoints for CRUD operations on invoices.
  3. Build the mobile-first UI for creating and previewing invoices.
  4. Integrate the Finance Agent to monitor overdue invoices and suggest follow-ups.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
