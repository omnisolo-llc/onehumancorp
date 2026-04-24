# Mercado Pago Integration

**Title**: Implement LATAM Payment Processing via Mercado Pago
**Problem Statement**: OHC currently relies on Stripe, which is not fully supported or preferred in many Latin American countries. To truly serve "everyone," OHC needs a payment processor that handles local payment methods (e.g., PIX in Brazil, OXXO in Mexico).
**Research Report**:
- **Tool**: Mercado Pago API.
- **Ease of Use (End User)**: Standard OAuth flow to connect their Mercado Pago account. Familiar checkout experience for their local customers.
- **Pricing**: Percentage per transaction + fixed fee (varies significantly by country and payment method).
- **Cloud vs. Standalone**: Works in both. Webhooks required for asynchronous payment confirmation (e.g., cash payments at convenience stores).
**Design Doc**:
- **Trigger**: User selects "Mercado Pago" as their payment provider in settings and completes the OAuth flow.
- **Action**: Checkout sessions route through Mercado Pago instead of Stripe. Webhooks handle payment status updates (pending -> approved).
- **UI**: "Connect Mercado Pago" button in settings. Mercado Pago checkout options presented to buyers based on their region.
**Implementation Prompt**: Integrate the Mercado Pago checkout API as an alternative to Stripe. Allow business owners to connect their Mercado Pago accounts. The checkout flow must support Mercado Pago redirection or embedded checkout, handling asynchronous payment confirmations via webhooks.
**Priority**: P1
**Estimated Scope**: Large
