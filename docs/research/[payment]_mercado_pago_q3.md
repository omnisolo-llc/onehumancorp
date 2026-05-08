## [Payment Processing] Issue Brief: Mercado Pago Integration for LATAM

**Title**: Scout 🔍: Integrate Mercado Pago to Unlock LATAM Markets
**Problem Statement**:
Stripe is not universally supported or preferred in all regions. In Latin America, businesses need a payment processor that supports local payment methods (e.g., Pix in Brazil, OXXO in Mexico) to avoid losing sales.
**Research Report**:
- **Tool**: Mercado Pago
- **Evaluation**: The dominant payment gateway in LATAM. Supports a wide array of local payment methods, installments, and wallet payments.
- **Ease of Use**: Setup is straightforward for users in supported countries.
- **Pricing**: Competitive percentage + fixed fee, varies by country and payment method.
- **Cloud vs. Standalone**: Works well in both via standard API integrations.
**Design Doc**:
- User selects Mercado Pago as their payment provider in OHC settings.
- OAuth flow to connect their Mercado Pago account.
- Checkout flow dynamically displays Mercado Pago options based on the buyer's region.
- Webhooks handle asynchronous payment confirmations (e.g., for cash payments like OXXO).
**Implementation Prompt**:
Implement Mercado Pago as an alternative payment gateway. Create the OAuth connection flow. Update the checkout UI to support Mercado Pago's Checkout Pro or custom checkout. Implement robust webhook handling for asynchronous payment status updates.
**Priority**: P1
**Estimated Scope**: Large
