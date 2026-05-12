# [Payment Processing] Mercado Pago

**Problem Statement**: Small businesses in Latin America often cannot use Stripe or prefer local payment methods (like Pix in Brazil or local credit cards). They need a payment processor that works seamlessly in their region to accept online payments.

**Research Report**:
- **Target Persona**: LATAM-based small businesses.
- **Ease of Use**: Mercado Pago is widely used in LATAM and offers good merchant tools.
- **Pricing**: Varies by country, typically a percentage + fixed fee per transaction.
- **Reputation**: Dominant player in LATAM e-commerce.
- **Cloud/Standalone**: Works in both, though webhooks for payment confirmation are required.

**Design Doc**:
- **Trigger**: Customer initiates checkout on an OHC-powered storefront.
- **Action**: OHC redirects to Mercado Pago checkout or uses their API to process the payment.
- **User View**: Business owner can select Mercado Pago as an alternative to Stripe in the payment settings.

**Implementation Prompt**: Add Mercado Pago as a payment provider option. Allow merchants to connect their Mercado Pago credentials so customers can select it at checkout.

**Priority**: P2
**Estimated Scope**: Large
