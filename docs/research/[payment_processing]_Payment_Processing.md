# Payment Processing Integration

## Title
Integrate Mercado Pago for Payment Processing

## Problem Statement
Stripe isn't always the best or most widely used option in Latin America. Business owners in LATAM need a familiar, trusted payment gateway that supports local payment methods and currencies.

## Research Report
**Tool Evaluated:** Mercado Pago
**Pricing:** Varies by country (~3-4% per transaction)
**Cloud/Standalone Support:** Cloud: Yes. Standalone: Yes (Webhook relays needed for local testing).

**Findings:**
Mercado Pago is dominant in LATAM, supporting local cards, bank transfers, and cash payments (like OXXO in Mexico). It has a clear API. Settlement speed and fees vary by country but are competitive locally.

## Design Doc
Add Mercado Pago as a payment provider option in the 'Settings > Payments' area. When enabled, checkout flows will redirect to Mercado Pago or use their transparent checkout. Payment statuses will be updated via secure webhooks.

## Implementation Prompt
Integrate Mercado Pago as an alternative checkout option. Ensure the checkout flow smoothly handles local payment methods and accurately records payment success/failure in the OHC order system.

## Priority
P1

## Estimated Scope
Large
