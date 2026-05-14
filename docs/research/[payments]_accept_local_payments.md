# Accept Local Payments with Mercado Pago

**Problem Statement:** Maria (a shop owner in LATAM) can't use Stripe because her customers prefer local payment methods like PIX or OXXO. She needs a way to accept online payments that her customers actually use.

**Research Report:** Stripe is not dominant in LATAM. Mercado Pago is the clear leader, supporting local payment methods, installments, and easy settlement. It has a robust API, clear pricing, and high trust in the region. Works well in both cloud and standalone via API keys.

**Design Doc:** User enters their Mercado Pago Access Token in settings. When generating an invoice or checkout link in OHC, they can select "Mercado Pago". OHC creates a preference via API and redirects the customer to Mercado Pago's hosted checkout to complete the payment.

**Implementation Prompt:** Add a Mercado Pago credentials section in settings. Implement a checkout flow that creates a Mercado Pago payment preference and provides the customer with a link to pay using local methods.

**Priority:** P0

**Estimated Scope:** Large
