# [Payment] Mercado Pago Integration

## Title
Integrate Mercado Pago for LATAM Payments

## Problem Statement
Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil natively within the OHC platform, avoiding complex third-party payment routing.

## Research Report
- **Strategy**: Direct integration with Mercado Pago for seamless LATAM coverage.
- **Target Persona**: Global users outside the US/EU.
- **Advantages**: Native integration within the OHC platform ensures a seamless onboarding experience without requiring the merchant to navigate complex third-party tools. Dominant in LATAM. Supports local payment methods (Pix in Brazil, OXXO in Mexico).
- **Risks**: Settlement times can be longer.
- **Pricing**: Variable by country (e.g., ~4-5% per transaction).
- **Compatibility**: Cloud and Standalone.

## Design Doc
- User selects their country during onboarding. If LATAM, Mercado Pago is offered alongside Stripe.
- User connects their Mercado Pago account.
- Customers see a "Pay with Mercado Pago" button at checkout.
- OHC automatically updates the order status when payment succeeds.
- **AI Integration**: Finance & Payments Agent seamlessly aggregates revenue across providers into a unified native dashboard.

## Implementation Prompt
Integrate Mercado Pago as an alternative native payment provider. The checkout flow must dynamically switch to the appropriate provider based on the merchant's settings, and automatically track successful payments. Ensure a seamless experience for both the merchant and the customer.

## Priority
P2

## Estimated Scope
Large
