# [Payment Processing] Integrate Mercado Pago for LATAM Expansion

## Problem Statement
While Stripe covers the US and Europe well, businesses operating in Latin America need local payment methods (like Pix in Brazil, or OXXO in Mexico). Without these, they cannot process online payments effectively, limiting their business growth.

## Research Report
**Evaluated Tool:** Mercado Pago API
**Alternatives Considered:** dLocal, EBANX
**Pros:** Dominant player in LATAM. Supports a massive variety of local payment methods (cash vouchers, local credit cards, bank transfers). Strong consumer trust in the region.
**Cons:** API documentation can be fragmented; support is localized.
**Ease of Use for Non-technical Users:** The user connects their Mercado Pago account via a simple OAuth flow, instantly enabling local payment options at checkout for their customers.
**Pricing:** Transaction percentage + fixed fee (varies heavily by country and payment method).
**Deployment:** Cloud and Standalone compatible via standard OAuth and webhooks.

## Design Doc
**Integration with OHC:**
- **Trigger:** A customer initiates checkout in a supported LATAM country.
- **Action:** OHC routes the payment intent to Mercado Pago instead of Stripe, generating a checkout session or native UI component.
- **AI Agent Interaction:** "The Accountant" logs the pending payment, monitors the Mercado Pago webhook for success, and reconciles the localized currency to the owner's dashboard.
- **User View:** A "Payments" setting allowing the owner to connect Mercado Pago. Customers see familiar local payment options at checkout.

## Implementation Prompt
Integrate the Mercado Pago API as an alternative payment gateway. Implement the OAuth connection flow for tenants. Update the checkout UI to support Mercado Pago checkout sessions and handle webhooks for payment status updates (pending, approved, rejected).

## Priority
P2

## Estimated Scope
Large
