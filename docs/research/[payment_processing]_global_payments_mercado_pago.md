# Title: Expanded Global Payments via Mercado Pago Integration

## Problem Statement
While Stripe covers North America and Europe well, business owners in Latin America need local payment methods (PIX in Brazil, OXXO in Mexico, local credit cards) to effectively sell online. Relying solely on Stripe alienates a massive segment of the global small business market.

## Research Report
**Findings & Evaluation:**
- **Mercado Pago:** The dominant payment processor in Latin America. It supports all critical local payment methods, including cash vouchers and bank transfers, which are essential in unbanked populations.
- **Alternatives evaluated:** dLocal, Ebanx. While good, Mercado Pago has better brand recognition among local consumers and simpler APIs for standard checkout flows.
- **Ease of Use:** Similar to Stripe, the user connects their Mercado Pago account via OAuth.
- **Cloud vs Standalone:** Works well in Cloud. Standalone mode requires careful handling of webhook endpoints.

## Design Doc
**Integration with OHC:**
We introduce a "Payment Gateway Provider" abstraction in the OHC backend. When processing a checkout session, the system checks the tenant's configured provider.
If Mercado Pago is connected, the OHC backend creates a Preference via the Mercado Pago API and redirects the user to the Mercado Pago checkout flow (or embeds it via their Pro checkout).
Webhooks from Mercado Pago (IPN - Instant Payment Notification) hit the OHC backend to confirm payment status. The Finance Agent ("The Accountant") is notified to mark the order as paid, triggering the Operations Agent to notify the business owner.

## Implementation Prompt
**User-Facing Outcome & Acceptance Criteria:**
- Business owners in LATAM regions can choose Mercado Pago as their payment provider instead of Stripe.
- Customers can pay using local methods like PIX, OXXO, and local bank transfers on the OHC storefront.
- Payment statuses (pending, paid, failed) sync correctly with the OHC order dashboard.
- The setup process involves a simple OAuth redirect to Mercado Pago.

## Priority
P2

## Estimated Scope
Medium
