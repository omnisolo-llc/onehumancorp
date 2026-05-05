## [Payment Processing] Issue Brief: Localized Payments for LATAM

**Title**: Scout 🔍: Integrate Mercado Pago for LATAM Market Expansion
**Problem Statement**:
While Stripe is fantastic for the US/EU, it is not the dominant or most accessible payment method in Latin America. Business owners in LATAM need to accept local payment methods like PIX (Brazil), Boleto, and local credit cards to effectively run their businesses.

**Research Report**:
- **Tool**: Mercado Pago API.
- **Evaluation**: Mercado Pago is the dominant payment gateway in LATAM. Integrating it allows OHC to serve a massive demographic of small businesses in South and Central America.
- **Ease of Use**: Similar to Stripe, the user connects their Mercado Pago account via an OAuth flow or by pasting their secure keys natively in OHC.
- **Advantages**: Supports local payment methods which are critical for conversion (often >50% of transactions). Settlement times are faster locally compared to cross-border Stripe.
- **Risks**: API is slightly less standardized than Stripe. Handling specific local payment method webhooks (e.g., delayed Boleto settlement) adds complexity.
- **Pricing**: Standard payment gateway fees per transaction (~3-4%), no monthly cost to OHC.
- **Compatibility**: Works natively in both Cloud (via OHC platform account) and Standalone (user supplies API keys).

**Design Doc**:
- In the "Finance & Payments" settings, users select their region. If in LATAM, Mercado Pago is highlighted as the recommended provider alongside Stripe.
- User connects their Mercado Pago account.
- On the storefront checkout page natively in OHC, dynamically display "Pay with Mercado Pago" if configured.
- Handle Mercado Pago webhooks for asynchronous payment confirmations (e.g., when a user pays a Boleto offline) to update order status natively in OHC.

**Implementation Prompt**:
Implement an alternative payment gateway using Mercado Pago. Add Mercado Pago as a payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow. Update the checkout flow to support Mercado Pago's hosted checkout or API-based payment intents natively.
- **Acceptance Criteria**: Merchant in a supported region can connect Mercado Pago natively. Customers can checkout using local LATAM methods. Orders are marked paid upon successful webhook receipt, including delayed payment methods.
**Priority**: P1
**Estimated Scope**: Large
