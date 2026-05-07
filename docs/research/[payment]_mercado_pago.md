# Payment Processing: Mercado Pago

**Title**: Expand Payments with Mercado Pago for LATAM Users

**Problem Statement**: Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil natively within the OHC platform, avoiding complex third-party payment routing.

**Research Report**:
- Direct API integration with Mercado Pago for seamless LATAM coverage.
- **Pricing**: Standard transaction fees apply.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

**Design Doc**:
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout natively.
- Webhooks update the order status in OHC when payment succeeds.

**Implementation Prompt**: Add Mercado Pago as a payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow.
- **Priority**: P1
- **Estimated Scope**: Large
- **Acceptance Criteria**:
  - Checkout flow includes Mercado Pago for LATAM users.
  - Payment success updates order status correctly via webhooks.

**Strategy**: Integrate Mercado Pago's API directly for robust LATAM payment support.
