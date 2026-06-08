issue_title: "Implement Instant Localized Invoicing Engine & Mobile-First UX"
issue_description: |
  ## Issue Title
  Implement Instant Localized Invoicing Engine & Mobile-First UX

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Leo (music tutor) need to quickly issue invoices to their customers that look professional, are accurate, and follow local tax laws. Current platforms either don't provide native invoicing, forcing users to use separate accounting software (like QuickBooks) which they find confusing, or they provide generic invoicing that doesn't cater to local requirements (like multi-currency or specific tax calculations). They need a system that can instantly generate localized invoices that feel premium and integrate directly with their unified ledger.

  ## Research Report
  *   **Shopify:** Has basic invoicing, but complex B2B or localized invoicing often requires expensive third-party apps.
  *   **Wix/Squarespace:** Invoicing is present but rudimentary; localizing for different tax jurisdictions is largely a manual setup process.
  *   **Stripe Invoicing:** Powerful but developer-focused. The UI is complex for a non-technical user.
  *   **OHC Differentiation:** The Finance & Payments agent handles this invisibly. It observes an order or a quote approval, determines the customer's location, applies the correct local tax rules (VAT, GST, State Sales Tax), and instantly generates a localized invoice with a payment link, all recorded seamlessly into the underlying multi-tenant ledger.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      INVOICE_REQUEST ||--o{ INVOICE_ROUTER : "Triggers"
      INVOICE_ROUTER }|--|| FINANCE_AGENT : "Delegates to"

      FINANCE_AGENT {
          string spiffe_identity "Zero Trust routing"
          string tenant_id "Multi-tenant isolation"
      }

      FINANCE_AGENT ||--o{ TAX_SERVICE : "Consults for localization"
      FINANCE_AGENT ||--o{ PAYMENT_GATEWAY : "Generates payment link"

      FINANCE_AGENT }|--|| LEDGER_ENTRY : "Records to"

      LEDGER_ENTRY {
          string invoice_id
          string currency
          decimal amount
          string tax_jurisdiction
      }

      LEDGER_ENTRY ||--o{ MOBILE_UI : "Syncs to"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First).
  *   **Invoice Generator Card:** A simple form to input the customer name and items. The agent auto-fills the rest based on context.
  *   **Preview Mode:** A beautiful, glassmorphic preview of the invoice exactly as the customer will see it.
  *   **Action Button:** A prominent "Send & Record" button.

  ### Mobile UX Flow
  1. **Trigger:** Leo finishes a lesson and taps "Send Invoice" for his student.
  2. **Review:** The agent has already drafted the invoice with the correct amount, currency, and local tax based on the student's profile. Leo reviews it in a frosted glass card.
  3. **Action:** Leo taps "Approve".
  4. **Fulfillment:** The customer receives a localized email with a secure payment link. The transaction is recorded in Leo's OHC ledger.

  ### AI Agent Integration Points
  *   **Finance Department:** Handles the generation, tax calculation, and ledger recording.
  *   **Customer Success Department:** Handles sending the invoice via the preferred channel (email, WhatsApp) and following up if it remains unpaid.

  ### Key Design Decisions
  *   **Invisible Localization:** The user should never have to manually calculate VAT or state tax; the system uses the customer's location to do this automatically.
  *   **Unified Ledger:** Every invoice must tie directly back to the core multi-tenant ledger to ensure accurate weekly financial reports without manual reconciliation.
  *   **Zero-Trust Isolation:** Financial data is highly sensitive; strict tenant isolation is required for all ledger reads and writes.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the architecture and UI for the "Instant Localized Invoicing Ledger".

  **Customer User Journey (CUJ):**
  1. Leo navigates to the Finance tab and selects "New Invoice".
  2. He selects a customer and a service (e.g., "Guitar Lesson").
  3. The system automatically calculates the correct local tax and generates a preview.
  4. He taps "Send", and the invoice is recorded in the ledger and sent to the customer.

  **Acceptance Criteria:**
  *   **Mobile Parity:** Perfect layout on 375px viewport with Translucent Glass aesthetics.
  *   **Localization:** The system must accurately apply different tax rates based on mocked customer locations.
  *   **Ledger Integration:** The generated invoice must create a corresponding, isolated entry in the database.
  *   **Isolation Guarantee:** Strict multi-tenant boundaries for financial records.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
