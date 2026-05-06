# Title: Integrate Mercado Pago for LATAM Payment Processing

## Problem Statement
Small business owners in Latin America face high friction using Stripe due to limited local payment methods and currency conversion issues. They need a localized payment gateway that supports local cards, Pix (Brazil), and cash payments via convenience stores.

## Research Report
Mercado Pago is the leading payment processor in Latin America, native to Mercado Libre.
- **Ease of use:** High for users accustomed to the Mercado Libre ecosystem. Offers simple checkout links.
- **Pricing:** Competitive local rates, variable by country. Eliminates international cross-border fees.
- **Reputation:** The undisputed leader in e-commerce payments across LATAM.
- **Cloud/Standalone:** Cloud API. In standalone mode, local API keys would be configured by the user.

## Design Doc
- **Trigger:** User creates an invoice or a customer attempts to check out from a booking page.
- **Action:** Generates a Mercado Pago checkout session and redirects the customer to complete payment. Listens for webhooks to update invoice status.
- **User Interface:** A new "Payment Providers" setting allowing users to connect their Mercado Pago account. Invoices display a "Pay with Mercado Pago" button alongside or instead of Stripe.

## Implementation Prompt
Add Mercado Pago as an alternative payment gateway. Allow business owners to authorize their Mercado Pago account via an OAuth or API key flow in settings. Update the invoicing and checkout flow so that if Mercado Pago is enabled, the customer is redirected to a Mercado Pago hosted checkout page to complete the transaction. Ensure local payment methods (like Pix) are supported.

## Priority
P2

## Estimated Scope
Medium
