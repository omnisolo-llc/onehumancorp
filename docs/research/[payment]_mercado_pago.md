## [Payment Processing] Issue Brief: Localized Payments for LATAM

**Title**: Scout 🔍: Integrate Mercado Pago for LATAM Market Expansion
**Problem Statement**:
While Stripe is fantastic for the US/EU, it is not the dominant or most accessible payment method in Latin America. Business owners in LATAM need to accept local payment methods like PIX (Brazil), Boleto, and local credit cards to effectively run their businesses.
**Research Report**:
- **Tool**: Mercado Pago API.
- **Evaluation**: Mercado Pago is the standard in LATAM. Integrating it allows OHC to serve a massive demographic of small businesses in South and Central America.
- **Ease of Use**: Similar to Stripe, the user connects their Mercado Pago account via an OAuth flow or by pasting their secure keys.
- **Pricing**: Standard payment gateway fees per transaction (~3-4%), no monthly cost to OHC.
- **Cloud vs. Standalone**: Works natively in both Cloud and Standalone modes.
**Design Doc**:
- "Settings" -> "Payments".
- Add a "Connect Mercado Pago" button alongside Stripe.
- On the storefront checkout page, dynamically display Mercado Pago if configured.
- Handle Mercado Pago webhooks for asynchronous payment confirmations (e.g., when a user pays a Boleto offline).
**Implementation Prompt**:
Implement an alternative payment gateway using Mercado Pago. Allow users to connect their account. Update the checkout flow to support Mercado Pago's hosted checkout or API-based payment intents. Ensure the order status accurately reflects asynchronous payment settlements via webhooks.
**Priority**: P1
**Estimated Scope**: Medium
