# Issue Brief: LATAM Payments via Mercado Pago

## Title
Localized Payment Support for LATAM Markets (Mercado Pago)

## Problem Statement
"Stripe is great, but my customers in Argentina want to pay with Pago Fácil or Rapipago." To be truly "The Small Business App for Everyone," OHC must support the payment methods people actually use in their local markets.

## Research Report
- **Tool**: Mercado Pago Checkout Pro / API.
- **Ease of Use**: High. The "Stripe of LATAM."
- **Persona Fit**:
    - **Carlos (Handyman in Mexico)**: Accepts local cards and cash-voucher payments.
    - **Priya (Boutique in Brazil)**: Supports PIX (instant payments), which is critical for conversion.
- **Cloud vs. Standalone**:
    - **Cloud**: Full integration with webhook callbacks.
    - **Standalone**: Supports redirect-based payments even on local dev environments.
- **Pricing**: Transparent transaction-based fees (varies by country, approx. 3-5%).
- **Competitive Analysis**: While OHC defaults to Stripe, Mercado Pago is non-negotiable for success in Latin America.

## Design Doc
- **Integration**: "The Accountant" (Finance Agent) handles Mercado Pago credentials and reconciliation.
- **User Experience**:
    - During Setup, OHC asks: "Where is your business located?"
    - If LATAM, OHC suggests Mercado Pago alongside Stripe.
    - Payments flow into the same "Financial Fog-free" dashboard.

## Implementation Prompt
Implement a Mercado Pago provider for the OHC Payment Service. Support Checkout Pro for web and mobile (Flutter). Ensure that "The Accountant" agent can reconcile Mercado Pago transactions alongside Stripe. Support local payment methods like PIX (Brazil) and cash vouchers (Argentina/Mexico).

## Priority
P1 (High for Global Growth)

## Estimated Scope
Medium
