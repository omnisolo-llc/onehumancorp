issue_title: "Implement Mercado Pago Integration for LATAM Support"
issue_description: |
  # Architecture Research for Mercado Pago Integration

  ## Problem Statement
  Business owners in LATAM struggle with Stripe's limited availability and high cross-border fees. They need a trusted, local payment processor that supports alternative payment methods (like Pix in Brazil or cash payments) to process transactions locally without friction.

  ## Research Report
  Mercado Pago is the dominant payment gateway in Latin America.
  * **Ease of Use:** Connects directly to existing accounts. Instantly enables local checkout options.
  * **Pricing:** Variable by country, generally competitive locally.
  * **OHC Integration:** Supporting local payment methods is critical for global adoption, especially in emerging markets where users prefer local processors.

  ## Design Doc
  * **Trigger:** User generates an invoice or checkout link in OHC in a supported region.
  * **Action:** OHC creates a Mercado Pago payment preference and returns the payment URL.
  * **UX:** When creating an invoice or checkout, the user sees "Mercado Pago" as an available payment method. Paid invoices automatically update their status in OHC.
  * **Mobile UX:** Checkout securely redirects or embeds the Mercado Pago checkout tailored for 375px screens.

  ```mermaid
  graph TD;
      A[Customer Checkout] --> B{Region LATAM?};
      B -- Yes --> C[Mercado Pago Preference Created];
      B -- No --> D[Stripe/Default Checkout];
      C --> E[Customer Completes Payment via Pix/Card/Cash];
      E --> F[Mercado Pago Webhook Received];
      F --> G[Update Invoice Status to Paid];
      G --> H[Trigger Operations/Shipping Flow];
  ```

  ## Implementation Prompt
  Implementer Agent:
  1. Add Mercado Pago configuration to tenant settings.
  2. Implement full `create_checkout_preference` in `MercadoPagoClient`.
  3. Ensure `PaymentRouter` can route to Mercado Pago if configured.
  4. Implement `mercadopago_webhook_handler` to parse incoming IPN (Instant Payment Notification) messages and update order/invoice status.
  5. Ensure E2E tests pass for the Mercado Pago flow (using a mock or sandbox).

  ## Priority
  `P0`

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
