**Title**: Integrate Mercado Pago for OHC

## Problem Statement
Stripe isn't widely used or supported for my customers in Latin America. I need to accept local payment methods like Pix in Brazil.

## Research Report
**Tool Evaluated:** Mercado Pago

**Findings:** Mercado Pago is the leading payment gateway in LATAM, supporting local payment methods (Pix, Boleto, local credit cards). They offer Checkout Pro (hosted) and Transparent Checkout (API). It's critical for LATAM market penetration.

**Pricing:** Varies by country, typically 3.99% - 4.99% + fixed fee per transaction.

**Cloud vs Standalone Mode:** Cloud handles webhooks easily. Standalone requires webhook proxying or polling for payment status.

## Design Doc
During checkout, if the business is located in LATAM, OHC offers Mercado Pago as a payment option. OHC generates a preference ID and redirects the user to the Mercado Pago secure checkout, then listens for payment status webhooks.

## Implementation Prompt
Add Mercado Pago as an alternative payment provider to Stripe. Allow business owners in supported countries to connect their Mercado Pago account and accept payments seamlessly via Checkout Pro. Ensure local payment methods are supported.

## Priority
P1

## Estimated Scope
Medium
