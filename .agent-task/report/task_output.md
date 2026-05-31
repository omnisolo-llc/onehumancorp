issue_title: "Implement Mercado Pago as a Payment Gateway Alternative to Stripe for LATAM"
issue_description: |
  **Problem Statement**
  Currently, OneHumanCorp strictly defaults to Stripe as its payment processor. However, small business owners in Latin American markets (such as Brazil and Mexico) find Stripe inaccessible due to lack of compatibility with local alternative payment methods (e.g. Pix, OXXO) and higher currency conversion fees. Without localization in payments, conversion drops significantly for the target personas operating in LATAM.

  **Research Report**
  Competitors generally focus their primary efforts on western gateways. The research indicates that integrating MercadoPago directly addresses this crucial gap for LATAM SMBs, bringing them local payment capability directly inside the platform's simplified environment.

  **Design Doc**
  - **Payment Router Integration:** Expand `PaymentRouter` in `src/server/integrations/stripe/routing.rs` to officially support and route `MercadoPago` based on the transaction currency (e.g., `BRL` or `MXN`).
  - **Client Capability Implementation:** Update `MercadoPagoClient` (`src/server/integrations/mercadopago/client.rs`) to accurately create and handle checkout preferences, acting as the peer alternative to the `StripeClient`.
  - **Mock Data Handling Validation:** Implement comprehensive unit testing asserting `MercadoPago` routing occurs for targeted currencies, minimizing impact on the primary `StripeClient`'s logic.

  **Implementation Prompt**
  Enhance OHC's payment routing capabilities to fully support Mercado Pago for LATAM. Modify the `PaymentRouter` (`src/server/integrations/stripe/routing.rs`) to detect specific currencies like `BRL` and `MXN` and route to Mercado Pago. Implement the respective logic in `src/server/integrations/mercadopago/client.rs` allowing seamless localized checkout preferences creation to parallel Stripe.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
