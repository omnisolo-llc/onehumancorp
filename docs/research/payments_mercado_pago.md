# Mercado Pago Integration for LATAM Payments

## Problem Statement
While Stripe is fantastic for the US and Europe, it lacks penetration and localized payment methods in Latin America. Business owners in LATAM need to accept local, deeply trusted payment methods (like PIX in Brazil, or OXXO/Boleto) to effectively run their businesses.

## Research Report
- **Tool**: Mercado Pago API
- **Evaluation**: Mercado Pago is the dominant fintech and payment gateway in LATAM. Its APIs allow for processing credit cards and generating local payment vouchers.
- **Ease of Use for Persona**: The business owner authorizes OHC to connect to their Mercado Pago account. OHC handles the complex checkout routing automatically.
- **Pricing**: Standard payment gateway fees (varies by country, usually ~3-5% + fixed fee). No monthly fixed cost.
- **Reputation**: The undisputed leader in LATAM e-commerce payments.

## Design Doc
- **Integration Point**: "Finance & Payments" department.
- **Trigger**: Business owner selects their country in settings; if in LATAM, Mercado Pago is offered as a payment provider.
- **Actions**:
  - Checkout generates a Mercado Pago preference and redirects the customer (or uses the Web Tokenize Checkout).
  - Webhooks notify OHC when a payment is settled (especially important for asynchronous payments like cash vouchers).
- **User View**: Customers see local payment options at checkout. The business owner sees incoming payments in their OHC dashboard alongside Stripe payments.

## Implementation Prompt
Add Mercado Pago as an alternative payment provider in the Finance settings. Update the public checkout flow to initiate a Mercado Pago checkout session when configured. Implement the Mercado Pago webhook handler to update order statuses when asynchronous payments clear.

## Priority
P1

## Estimated Scope
Large
