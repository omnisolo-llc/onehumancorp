# Title: Alternative Payment Gateways for Global Reach

## Problem Statement
While Stripe is popular, many small businesses operate in regions where it isn't supported or preferred. They need localized payment options (e.g., Mercado Pago, Alipay, local bank transfers) to reduce friction for their customers and increase conversion rates.

## Research Report
*   **Competitors:** Mercado Pago (LATAM), Razorpay/Paytm (India), Alipay/WeChat Pay (China), Square.
*   **Ease of Use:** Must be a simple toggle or integration setup. The business owner shouldn't have to manage complex API credentials if possible.
*   **Pricing:** Varies wildly by region and provider (typically 1.5% - 3.5% + fixed fee per transaction).
*   **Reputation:** Local trust is paramount. Customers abandon carts if their preferred local payment method is unavailable.

## Design Doc
*   **Trigger:** User goes to Settings > Payments and selects their region/preferred providers.
*   **Actions:** OHC configures the checkout flow to dynamically display the appropriate payment methods based on the selected integrations and the buyer's location.
*   **User View:** The business owner sees a list of active payment gateways and aggregated payout information.

## Implementation Prompt
Expand the checkout system to support multiple alternative payment gateways. Implement integrations for at least two major regional providers (e.g., Mercado Pago, Razorpay). Ensure the checkout interface dynamically updates to show available options.

## Priority
P1

## Estimated Scope
Large
