# Mercado Pago Integration

## Problem Statement
Small businesses in Latin America struggle to accept payments using standard US-centric gateways like Stripe, which often lack local support and preferred payment methods (like Pix in Brazil). This limits their ability to capture local sales.

## Research Report
Mercado Pago is the dominant payment processor in Latin America. It offers a wide range of local payment options and handles currency conversions.
*   **Ease of use (end user):** High for customers as it offers familiar local payment methods.
*   **Pricing:** Transaction fees vary by country but are competitive for the region (around 3.99% - 4.99% + fixed fee).
*   **Reputation:** Extremely strong across LATAM. Essential for operating in countries like Brazil, Argentina, and Mexico.

## Design Doc
OHC will integrate Mercado Pago as a payment gateway option.
1.  **Trigger:** User goes to "Payment Settings" and selects "Connect Mercado Pago".
2.  **Action:** Standard OAuth flow or API key input depending on environment.
3.  **User Sees:** Upon successful connection, "Mercado Pago" becomes an available checkout method for their customers, displaying options like Pix or local bank transfers.

## Implementation Prompt
Integrate Mercado Pago as an alternative payment provider in the OHC checkout flow.
*   Add a "Mercado Pago" option in the payment settings page, allowing the business owner to connect their account.
*   Implement the checkout flow to redirect customers to Mercado Pago's secure payment page or use their transparent checkout component.
*   Handle webhook notifications from Mercado Pago to update order status in OHC automatically.
*   Acceptance Criteria: A user can connect their Mercado Pago account, and a simulated checkout successfully processes a payment and updates the order status.

## Priority
P1

## Estimated Scope
Large
