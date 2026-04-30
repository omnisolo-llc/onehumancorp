# [Payment Processing] Mercado Pago Integration

**Title**: Integrate Mercado Pago for LATAM Local Payments

**Problem Statement**:
Small businesses operating in Latin America need to accept local payment methods (e.g., PIX in Brazil, OXXO in Mexico) that Stripe may not fully support or where Mercado Pago has stronger market penetration and trust. They need an alternative payment processor that is just as easy to set up as Stripe.

**Research Report**:
Mercado Pago is the dominant payment gateway in Latin America.
- **Ease of Use for Non-Technical Users**: Very familiar to LATAM users. Setup involves straightforward account linking.
- **Pricing**: Competitive per-transaction fees tailored to local markets.
- **Reputation**: Highly trusted across Latin America, offering seamless integration with local banking systems and alternative payment methods.

**Design Doc**:
- **Trigger**: The business owner selects their region during onboarding or in settings. If in LATAM, Mercado Pago is offered alongside or instead of Stripe.
- **Action**: The user connects their Mercado Pago account. OHC configures the checkout flow to use Mercado Pago's checkout session APIs.
- **User View**: When a customer checks out, they are presented with local payment options (credit cards, PIX, cash payments). The business owner sees the transaction in their "Finance & Payments" dashboard just like any other payment.

**Implementation Prompt**:
Integrate Mercado Pago as an alternative payment processor for the OHC platform. Provide an easy connection flow for the business owner. Update the checkout experience to route payments through Mercado Pago when selected, supporting local payment methods. Ensure transaction statuses (success, pending, failed) are properly synced back to OHC's internal finance tracking and trigger the same automated actions (e.g., order confirmation emails) as Stripe transactions. Must support both Cloud and Standalone environments.

**Priority**: P2
**Estimated Scope**: Medium
