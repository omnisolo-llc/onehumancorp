# Payment Processing - Mercado Pago

## Problem Statement
Stripe is great, but it's not popular or fully supported in all Latin American countries. OHC users in LATAM need a local, trusted payment processor that supports local payment methods (like Pix in Brazil or cash payments via OXXO in Mexico).

## Research Report
Mercado Pago is the leading payment processor in Latin America.
- **Ease of Use**: Familiar to LATAM users. Easy account creation.
- **Pricing**: Transaction-based fee, typically around 3-5% depending on the country and payment method. No monthly fee.
- **Reputation**: Highly trusted in LATAM.
- **Cloud/Standalone**: Cloud-based API.

## Design Doc
- **Trigger**: User sets their business country to a supported LATAM country.
- **Action**: OHC offers Mercado Pago as the default payment gateway instead of Stripe.
- **User View**: User connects their Mercado Pago account. Customers on the storefront see local payment options (Pix, Boleto, credit cards) at checkout.

## Implementation Prompt
Add Mercado Pago as a payment provider alternative to Stripe. Implement the checkout flow using Mercado Pago's API, supporting local payment methods based on the user's region.
- Acceptance Criteria: User in a LATAM region can select Mercado Pago. Checkout successfully processes payments via Mercado Pago and updates the OHC order status.

## Priority
P1

## Estimated Scope
Medium

---
