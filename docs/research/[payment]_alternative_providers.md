# Title: Accept Local Payment Methods (Mercado Pago, Paytm)
## Problem Statement
Stripe is great for the US, but small businesses in Latin America or India need to accept local payment methods that their customers actually use, otherwise they lose sales at checkout.

## Research Report
Mercado Pago (LATAM) and Paytm (India) are dominant in their respective regions.
- **Ease of Use**: Both offer standard OAuth or API key integrations. OHC can guide the user to copy-paste the keys if OAuth isnt available.
- **Pricing**: Standard transaction fees apply, usually competitive for the region.
- **Reputation**: High trust and usage in their specific geographic markets.

## Design Doc
- **Trigger**: User navigates to Settings > Payments and selects their region.
- **Action**: OHC presents the relevant payment provider (e.g., Mercado Pago for Brazil). User enters API credentials or connects via OAuth. OHC checkout pages now offer this payment method.
- **User View**: A simple toggle to "Enable Mercado Pago" and a unified dashboard showing sales regardless of the payment gateway used.

## Implementation Prompt
Implement integrations for Mercado Pago and Paytm alongside the existing Stripe integration. Modify the checkout flow to dynamically display the appropriate payment options based on the business owners configured region and enabled providers. Ensure that successful payments from any provider update the OHC order status correctly and appear in the unified sales dashboard.

## Priority
P1

## Estimated Scope
Large

## Cloud vs Standalone Modes
- **Cloud Mode**: Fully supported. Webhooks from the payment provider update OHC order states globally.
- **Standalone Mode**: Webhooks from external providers will not reach a local machine. Polling the payment status API is required for standalone mode.
- **Risks**: Transaction failures and webhook delivery issues causing order state mismatches.
