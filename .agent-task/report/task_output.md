issue_title: "Implement Mercado Pago Localized Payment Gateway for LATAM Invoices"
issue_description: |
  # Research Report: Localized Invoicing & Payment via Mercado Pago for LATAM

  ## Problem Statement
  Small business owners using OneHumanCorp (OHC) in Latin America struggle to collect payments via invoices because international gateways like Stripe lack deep support for local payment methods (e.g., Pix in Brazil, OXXO in Mexico, local installment plans). If a customer receives an invoice but cannot pay using their preferred local method, the business owner loses the sale or faces severe friction (falling back to manual bank transfers or cash). This breaks the OHC promise of an "assistant-led flow" that turns demand into collected revenue seamlessly.

  ## Research Report
  - **Market Context:** Our research (docs/reports/scout_tool_integration_research_report_q4.md, docs/reports/ohc_tool_integration_research_report.md) highlights that capturing the LATAM market requires local payment methods. Mercado Pago is the dominant gateway in this region.
  - **Competitor Gaps:** While Shopify offers local payment integrations, they often require complex third-party apps or high transaction fees. Simpler tools like Wix or GoDaddy have limited regional checkout customization.
  - **The OHC Opportunity:** By integrating Mercado Pago directly into OHC invoices, we allow owners to generate payment links that automatically adapt to the buyer's region, offering familiar methods like Pix or OXXO. When paid, the invoice status in OHC updates autonomously, closing the loop without the owner needing to reconcile accounts manually.

  ## Design Doc
  ### Architectural Flow
  ```mermaid
  sequenceDiagram
      participant Owner as Business Owner (Mobile App)
      participant OHC as OHC Backend
      participant MP as Mercado Pago API
      participant Customer as Buyer (Web Checkout)

      Owner->>OHC: Create Invoice (Selects Mercado Pago)
      OHC->>MP: Create Preference / Payment Link
      MP-->>OHC: Return Checkout URL
      OHC->>Customer: Send Invoice Email/SMS with URL
      Customer->>MP: Complete Payment (Pix, OXXO, Card)
      MP->>OHC: Webhook (payment.updated)
      OHC->>OHC: Mark Invoice as Paid
      OHC->>Owner: Push Notification (Invoice Paid!)
  ```

  ### Mobile UX Flow (375px)
  1. **Owner View (Invoice Creation):** Under "Invoices", the owner drafts a new invoice. In the "Payment Methods" section, they toggle "Mercado Pago" on.
  2. **Customer View (Checkout):** The customer taps the invoice link, landing on a mobile-optimized OHC-hosted page. Tapping "Pay Now" redirects them to the Mercado Pago checkout tailored to their local currency and options.
  3. **Owner View (Confirmation):** Once paid, the OHC dashboard immediately updates the invoice card from "Pending" (amber) to "Paid" (green) and sends a success notification.

  ### AI Agent Integration
  - **Sales/Finance Agent:** Can autonomously generate the invoice draft based on an accepted proposal or a chat interaction.
  - **Operations Agent:** Listens for the Mercado Pago webhook and triggers follow-up actions (e.g., sending a receipt, scheduling delivery).

  ## Implementation Prompt
  **Feature Name**: Mercado Pago Localized Invoice Payments
  **Target Persona**: Carlos the Field Service Owner (based in LATAM)
  **Outcome**: Carlos can send a repair invoice to a customer, allowing them to pay via their preferred local method (e.g., Pix). Carlos receives an instant notification when the invoice is paid, and the OHC system automatically marks it as complete.

  **Next Actions (for Implementer Agent):**
  1.  **Data Model Updates:** Extend the `Invoice` and `Tenant` entities in PostgreSQL to support Mercado Pago credentials and payment preference IDs.
  2.  **API Integration:** Implement a `MercadoPagoService` (using the official SDK or REST API) to generate payment links (Preferences) when an invoice is created.
  3.  **Webhook Handler:** Create an idempotent webhook endpoint (`/webhooks/mercadopago`) to listen for payment status updates and map them to the OHC invoice status.
  4.  **UI Updates:** Update the Flutter/Web invoice creation flow to allow selecting Mercado Pago, and update the customer-facing invoice view to render the payment link.
  5.  **E2E Testing:** Write Playwright E2E tests simulating the invoice creation and webhook fulfillment flow.

  **Acceptance Criteria:**
  - Owner can configure Mercado Pago credentials in settings.
  - Owner can generate an invoice linked to a Mercado Pago checkout.
  - Simulating a successful payment via Mercado Pago test webhooks correctly marks the OHC invoice as paid.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
