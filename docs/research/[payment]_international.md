## [Payment] Issue Brief: Alternative Payment Processors (Mercado Pago, etc.)

**Title**: Scout 🔍: Integrate Alternative Payment Gateways for Emerging Markets
**Problem Statement**: While Stripe is great for the US and Europe, small businesses in regions like LATAM or India often need local payment processors (e.g., Mercado Pago, Paytm) to accept regional payment methods and access funds faster.
**Research Report**:
- **Tools Evaluated**: Mercado Pago, Alipay, Paytm.
- **Evaluation**: To serve a global user base, OHC must support regional payment giants. Mercado Pago is essential for LATAM, supporting local cards, Pix, and cash payments.
- **Ease of Use**: User selects their region during setup and clicks "Connect Mercado Pago" instead of Stripe.
- **Pricing**: Payment processors charge standard transaction fees which the merchant is accustomed to.
- **Cloud vs. Standalone**: Both modes support standard API/OAuth integrations, though webhooks require public endpoints (Cloud handles this well; Standalone needs a proxy or direct API polling).
**Design Doc**:
- The onboarding flow dynamically suggests payment providers based on the user's selected country.
- User authorizes Mercado Pago via OAuth.
- The checkout flow renders the appropriate payment widget (e.g., Mercado Pago's checkout pro or custom checkout).
- Webhooks update the order status to "Paid" in OHC.
**Implementation Prompt**: Implement Mercado Pago as an alternative payment gateway to Stripe. Abstract the payment layer so the checkout UI can seamlessly swap between providers. Handle payment success/failure webhooks to update the OHC order state.
**Priority**: P1
**Estimated Scope**: Medium
