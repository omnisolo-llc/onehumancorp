issue_title: "[Research] OHC Multi-Currency & Instant Localized Invoicing Architecture"
issue_description: |
  # Research Report: Multi-Currency & Instant Localized Invoicing Architecture

  ## Problem Statement
  As small businesses and independent operators (e.g., Nora the Agency Principal, Leo the Music Tutor) expand their services globally, they face significant friction in handling multi-currency transactions and generating compliant, localized invoices. Traditional e-commerce platforms often require expensive third-party apps to handle complex tax rules (e.g., EU VAT, US State Sales Tax) and currency conversions. For non-technical owners, manually calculating tax or managing exchange rates is a massive barrier to growth.

  ## Research Report (Track 1)
  - **Competitor Analysis:** Shopify handles multi-currency via Shopify Markets, which is robust but heavily tied to their proprietary payment gateway and can be complex to configure for service-based businesses. Stripe Billing provides excellent invoicing and tax calculation (Stripe Tax), but it is a developer-first tool. SMBs need the power of Stripe Tax wrapped in an invisible, agent-managed workflow.
  - **OHC Gap:** Currently, OHC does not have a unified architecture for dynamic, multi-currency invoicing with automated localized tax compliance. Without this, users like Nora cannot easily bill international clients without manual intervention and risk of non-compliance.

  ## Design Doc (Track 2 & Track 3)
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INVOICE : generates
      CUSTOMER ||--o{ INVOICE : receives
      INVOICE ||--o{ INVOICE_LINE_ITEM : contains
      INVOICE {
          uuid id
          uuid tenant_id
          uuid customer_id
          string base_currency
          string target_currency
          decimal exchange_rate
          decimal total_amount
          string tax_region
          decimal tax_amount
          string status
      }
  ```

  ### Mobile UX Flow
  - **375px Viewport First:**
    - The Finance Agent surfaces an Action Card in the Agent Feed: "Drafted an invoice for [Client Name] in EUR (converted from your standard USD rates). Tax applied: EU VAT."
    - The user sees a clear summary of the invoice, the exchange rate applied, and the tax collected.
    - Touch Target: A large (44x44px minimum) "Approve & Send" button.
    - If the user needs to edit, they tap "Edit," which opens a simplified, native mobile form using native keyboards for numeric inputs.

  ### AI Agent Integration Points
  - **Finance Agent ("The Accountant"):** Automatically detects the customer's region based on their profile, queries real-time exchange rates, calculates the appropriate tax, and drafts the invoice.
  - **Operations Agent:** If the invoice is for a physical product, the Operations Agent holds the inventory until the international payment clears.

  ### Key Design Decisions and Why
  - **Centralized Ledger with Multi-Currency Support:** The database must store the base currency (e.g., USD) and the transaction currency (e.g., EUR) alongside the exchange rate at the time of the transaction. This ensures accurate financial reporting.
  - **Agent-Drafted Invoices:** Instead of the user creating an invoice from scratch, the system automatically drafts it upon project completion or order placement, reducing the workflow to a single tap.

  ## Implementation Prompt
  **Feature Name:** Agent-Driven Multi-Currency Localized Invoicing
  **Target Persona:** Nora the Agency Principal

  **Outcome:** When Nora completes a design project for a client in Germany, the Finance Agent automatically drafts a compliant EU VAT invoice in Euros, converts her standard USD rate, and presents it to Nora for 1-tap approval on her mobile device.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1.  User (Nora) opens the OHC mobile app.
  2.  The Agent Feed displays a card: "Project 'Berlin Rebrand' marked complete. Drafted invoice for €2,500 (incl. €475 VAT). Send now?"
  3.  User taps the card to view the invoice breakdown (must fit perfectly on 375px without horizontal scrolling).
  4.  User taps "Approve & Send".
  5.  The system uses Stripe (or a configured payment provider) to generate the payment link and emails the localized invoice to the client.

  **Next Actions for Implementer:**
  - Update the `Invoice` database schema to support `target_currency`, `exchange_rate`, and `tax_region` with proper multi-tenant isolation.
  - Implement the Finance Agent logic to draft invoices based on project completion events and customer location data.
  - Build the 375px mobile UI for the Invoice Action Card in the Agent Feed.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
