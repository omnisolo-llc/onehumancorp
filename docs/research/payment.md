# [Payment] Regional Payment Processors (Mercado Pago, Paytm, Alipay)

## Problem Statement
While Stripe is the global default, it is not always the preferred or even available option for small businesses operating in Latin America, India, or China. Business owners in these regions lose sales because they cannot offer the payment methods their local customers expect.

## Research Report
**Tools Evaluated:** Mercado Pago (LATAM), Paytm (India), Alipay (China)

*   **Ease of Use:** For the business owner, connecting an account usually involves API keys or OAuth. The end-customer experience is highly localized and trusted.
*   **Pricing:** Transaction-based pricing (percentage + fixed fee per transaction), competitive within their specific regions. No upfront monthly costs.
*   **Reputation:** These are the dominant and most trusted payment methods in their respective markets.

## Design Doc
**Trigger:** User sets up invoicing or a checkout link and selects their region.
**Action:** User connects their preferred regional payment gateway.
**User Sees:** An option in their billing/settings to activate regional providers alongside or instead of Stripe. When they send an invoice from OHC, the payment page displays the Mercado Pago / Paytm / Alipay button, allowing the customer to pay using local funds.

## Implementation Prompt
Expand the OHC payment and invoicing system to support regional payment providers, starting with an architecture that allows modular payment gateways. Implement at least one regional provider (e.g., Mercado Pago) as a proof of concept. The integration should allow the business owner to connect their account and generate payment links that route through the regional provider. Ensure the system can handle currency conversions or enforce matching currencies.

## Priority
P1

## Estimated Scope
Medium

## Mode Compatibility
*   **Cloud:** Fully supported. Cloud webhooks can securely receive payment success notifications.
*   **Standalone:** Requires a robust polling mechanism or redirect-based verification, as local standalone instances cannot reliably receive asynchronous internet webhooks from the payment gateways without complex tunneling.
