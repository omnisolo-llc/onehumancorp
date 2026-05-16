# [Payment Processing] Mercado Pago Evaluation for LATAM

## Title
Integrate Mercado Pago for LATAM Merchants

## Problem Statement
Small businesses in Latin America often cannot use Stripe or standard US/EU payment gateways. They rely on local payment methods (e.g., Pix in Brazil, OXXO in Mexico). Without local payment options, they lose online sales and resort to manual bank transfers.

## Research Report
- **Strategy**: API integration with Mercado Pago.
- **Persona**: LATAM-based merchants and regional e-commerce stores.
- **Advantages**: Dominant in LATAM, highly trusted, familiar to consumers, supports local cards and instant transfers (Pix).
- **Risks**: Higher percentage fees compared to US Stripe.
- **Pricing**: Varies by country (typically 3-5% + fixed fee), essential for the market.
- **Compatibility**:
  - **Cloud**: OAuth-like flow or Centralized routing.
  - **Standalone**: User supplies API keys via a wizard.

## Design Doc
- **Trigger**: Customer clicks "Pay Now" on an invoice/checkout page.
- **Action**: OHC redirects to Mercado Pago checkout or embeds their elements.
- **User Interface**: Business owner clicks "Connect Mercado Pago" to authorize OHC. "Mercado Pago" then appears as an active payment method on invoices.

## Implementation Prompt
Add Mercado Pago as a payment provider option in billing settings. Provide a "Connect Mercado Pago" button. Once connected, generated invoices must include a secure Mercado Pago payment link for customers.

## Priority
P2

## Estimated Scope
Medium
