# Title: Mercado Pago Integration for LATAM Markets
## Problem Statement
Stripe is not dominant or supported everywhere, especially in Latin America where alternative payment methods (like Pix in Brazil or OXXO in Mexico) are required to do business. Small business owners in LATAM lose sales if they cannot offer local, trusted checkout options.

## Research Report
* **Tool:** Mercado Pago API
* **What it does:** Processes payments, supports local credit cards, cash payments, and bank transfers across LATAM.
* **Ease of Use for Owners:** Standard payment gateway setup. Users in LATAM are very familiar with it.
* **Pricing:** Percentage per transaction, varies by country and settlement speed. No fixed monthly fee.
* **Cloud vs. Standalone:** Works perfectly in both via OAuth or standard API credentials.

## Design Doc
* **Trigger:** In checkout settings, owner selects "Mercado Pago" as their payment provider.
* **Action:** OHC redirects customers to Mercado Pago's secure hosted checkout or uses their JS library for inline checkout. Webhooks update order status to "Paid".
* **User Experience:** The business owner can accept Pix, local cards, and cash vouchers. The customer sees a familiar, localized checkout screen in Spanish or Portuguese.

## Implementation Prompt
Add Mercado Pago as an alternative payment provider to Stripe. The business owner must be able to connect their Mercado Pago account. The acceptance criteria: a customer in a supported country (e.g., Brazil) must be able to complete a checkout using a local payment method (like Pix), and the OHC order status must automatically update to "Paid" upon successful webhook receipt.

## Priority
P1

## Estimated Scope
Large
