# LATAM Payment Processing Integration via Mercado Pago

**Title**: LATAM Payment Processing Integration via Mercado Pago
**Problem Statement**: Users in Latin America need a reliable, localized way to accept payments, as global providers like Stripe are not universally accessible or preferred by their customers.

**Research Report**:
- Mercado Pago is the dominant payment processor in LATAM, supporting local payment methods (like Pix in Brazil or OXXO in Mexico).
- **Ease of Use**: Familiar to the target demographic, straightforward onboarding for merchants.
- **Pricing**: Competitive local rates, no monthly fees.
- **Reputation**: Highly trusted across Latin America.
- **Cloud vs Standalone**: Fully supported in both modes via API integrations.
- **Key Advantages**: Unlocks the LATAM market by supporting essential local payment methods.
- **Key Risks**: Varying settlement speeds and currency fluctuations across different countries.

**Design Doc**:
- Users in supported regions see Mercado Pago as a payment option in the "Settings > Payments" area.
- They authenticate their existing Mercado Pago account or create a new one.
- The OHC storefront checkout seamlessly redirects or embeds the Mercado Pago checkout flow, returning the user to a success page upon completion.

**Implementation Prompt**: Add Mercado Pago as a native payment option for LATAM users, enabling them to accept local payment methods effortlessly on their storefronts.

**Priority**: P0
**Estimated Scope**: Large
