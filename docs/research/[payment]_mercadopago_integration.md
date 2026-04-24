# Integrate Mercado Pago for LATAM Payment Processing

## Problem Statement
Stripe is great, but not supported or widely used in many LATAM countries where local payment methods (like Pix in Brazil or OXXO in Mexico) are essential for conversion.

## Research Report
- **Tool Evaluated**: Mercado Pago
- **Ease of Use**: Dominant in LATAM, supports local payment methods automatically.
- **Pricing**: Standard local processing fees, no monthly cost.
- **Standalone/Cloud**: Robust API and webhooks, works in both.
- **Persona Fit**: Essential for international businesses needing local context and trust.

## Design Doc
- **Integration Point**: Finance & Payments Agent, Checkout Flow.
- **Trigger**: Customer initiates checkout.
- **Action**: Generate Mercado Pago preference and redirect to their secure checkout or use transparent checkout.
- **User View**: Buyers in LATAM see local payment options. Owner sees funds in their OHC dashboard alongside Stripe.

## Implementation Prompt
Integrate Mercado Pago SDK/API as an alternative payment provider. Update the checkout UI to support dynamic provider selection based on the tenant's region setting. Ensure webhooks reliably update order status.

## Priority
P2

## Estimated Scope
Medium
