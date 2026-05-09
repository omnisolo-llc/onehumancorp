# Scout: Tool Integration Research Q2

## 4. Payment Processing
**Title**: Expand Payments with Mercado Pago for LATAM Users
**Problem Statement**: Non-US users in Latin America cannot rely solely on Stripe due to high fees, lack of local currency support, and specific local payment methods (like Pix in Brazil or OXXO in Mexico).
**Research Report**:
- Mercado Pago is the dominant payment provider in LATAM.
- Supports local payment methods which are critical for conversion (often >50% of transactions).
- Settlement times are faster locally compared to cross-border Stripe.
- Works consistently across all OHC deployment environments.
**Design Doc**:
- In the "Finance & Payments" settings, users select their region. If in LATAM, Mercado Pago is highlighted as the recommended provider.
- Setup involves a straightforward connection flow.
- Supports one-off payments and split payments for the eventual marketplace feature.
**Implementation Prompt**: Add Mercado Pago as a payment provider alternative to Stripe, allowing users in supported LATAM countries to accept local payment methods via the OHC checkout flow.
**Priority**: P1
**Estimated Scope**: Large
