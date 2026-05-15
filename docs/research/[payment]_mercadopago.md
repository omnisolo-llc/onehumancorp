# [Payment Processing] Mercado Pago for LATAM

**Title**: Mercado Pago Integration for Regional Payments
**Problem Statement**: Stripe does not have optimal coverage or preferred local payment methods in LATAM, causing cart abandonment for business owners in these regions.
**Research Report**:
- **Target Persona**: Business owners operating in Latin America who need to offer local payment options to their customers.
- **Evaluation**: Mercado Pago is universally trusted in LATAM. Settlement speed is fast, and it supports local currencies, PIX (Brazil), and installment plans natively.
- **Ease of Use**: Medium. Account setup requires regional business documentation.
- **Pricing**: Pay-per-transaction, usually around 3-5% + fixed fee depending on the country.
- **Key Risks**: Higher dispute/fraud rates in certain regions, currency conversion complexities if payouts are in USD.
- **Compatibility**: Full API support for Cloud. Standalone requires secure handling of API keys locally.
**Design Doc**: Users in supported regions will see Mercado Pago as a payment provider option. They connect it securely, and checkout flows will route through Mercado Pago's hosted checkout.
**Implementation Prompt**: Add Mercado Pago as a selectable payment provider in checkout settings. Acceptance criteria: users can select Mercado Pago and transactions process successfully.
**Priority**: P1
**Estimated Scope**: Medium
