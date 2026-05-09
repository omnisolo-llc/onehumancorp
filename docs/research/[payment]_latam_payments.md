# Payment Processing: LATAM Payments

## Problem Statement
Many small businesses in LATAM cannot use Stripe due to availability or high fees, preventing them from accepting online payments seamlessly.

## Research Report
**Selected Tool:** Mercado Pago
Mercado Pago is the dominant force in Latin American payments, supporting essential local methods like Pix (Brazil) and OXXO (Mexico).
- **Ease of use for non-technical users:** The checkout experience for customers is familiar and trusted. For owners, account connection is straightforward.
- **Pricing:** Competitive local rates, varies by country.
- **Reputation:** The undisputed leader in the LATAM region.

## Design Doc
**Integration with OHC:**
- **Trigger:** Customer reaches the checkout step on an OHC storefront.
- **Action:** OHC generates a Mercado Pago preference and redirects the user or opens the checkout widget. Webhooks confirm payment success.
- **User Interface:** A simple toggle in OHC settings: "Enable Mercado Pago". Customers see standard Mercado Pago checkout flows.
- **Environment:** Cloud and Standalone (API-based, requires webhook processing).

## Implementation Prompt
**User-Facing Outcome:** Business owners in LATAM can connect Mercado Pago and start accepting local payment methods immediately.
**Acceptance Criteria:**
- Secure OAuth connection or API key setup.
- Support for key local payment methods (Pix, credit cards).
- Automatic order status updates upon successful payment via webhooks.

## Priority
P1

## Estimated Scope
Medium
