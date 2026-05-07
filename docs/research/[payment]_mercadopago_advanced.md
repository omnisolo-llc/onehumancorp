# Expand LATAM Payment Acceptance with Mercado Pago

## Problem Statement
While Stripe is great for US/EU markets, many small business owners in Latin America (Brazil, Mexico, Argentina) lose sales because customers prefer or require local payment methods like PIX (Brazil), OXXO (Mexico), or local credit card installments. They need a payment processor built for the LATAM market to increase conversion rates.

## Research Report
**Tool Evaluated:** Mercado Pago API
- **Ease of Use:** Familiar to almost all LATAM users. The setup requires a Mercado Pago account, which many business owners already have.
- **Pricing:** Varies by country, typically around 3-4% + fixed fee per transaction, but supports local, cheaper methods like PIX (often ~1%).
- **Reputation:** The undisputed leader in LATAM e-commerce payments. It is critical for trust and conversion in these regions.
- **Deployment:** Cloud mode is standard. Standalone mode requires ensuring webhooks from Mercado Pago can reach the local instance securely (e.g., via a relay) to confirm asynchronous payments like OXXO.

## Design Doc
- **Trigger:** User selects "Mercado Pago" in their store's payment settings and authenticates.
- **Action:** At checkout, the OHC storefront generates a Mercado Pago preference and redirects the user (or loads the Web Tokenizer). Webhooks update the OHC order status from "Pending" to "Paid".
- **User View:** A simple toggle to "Enable Mercado Pago". Buyers see options to pay with local methods natively on the checkout page.

## Implementation Prompt
Integrate Mercado Pago as an alternative checkout provider to support LATAM merchants. Implement the Checkout Pro or Checkout API to allow users to pay with local methods (e.g., PIX, Boleto, OXXO) and installments. Ensure that asynchronous payment notifications (webhooks) correctly update the corresponding order status in the OHC platform.

## Priority
P1

## Estimated Scope
Medium