# Title: Payment Processing for LATAM via Mercado Pago

## Problem Statement
Small business owners in Latin America often struggle to accept digital payments because standard western gateways like Stripe do not support local payment methods (e.g., PIX in Brazil, OXXO in Mexico). To be a truly global platform, OHC needs a payment integration that allows users in these regions to easily accept local payment methods without technical hurdles.

## Research Report
Mercado Pago is the leading payment gateway in Latin America.
- **Ease of Use for Non-Technical Users**: The user enters their Mercado Pago credentials or goes through an OAuth flow. Afterwards, local payment options appear automatically on their checkout page.
- **Pricing**: Standard gateway fees applied per transaction. No monthly fixed costs.
## Risks
- **Risks**: Handling complex local refund policies and potential fraud in emerging markets.

## Reliability & Reputation**: Dominant player in LATAM, highly reliable for local payment methods that other gateways fail to cover.
- **Environment Support**: Works in both Cloud and Standalone modes via API and webhooks.

## Design Doc
The "Finance & Payments" (The Accountant) agent manages regional payment routing.
1. **Trigger**: A customer in Brazil attempts to buy a digital product from an OHC storefront.
2. **Action**: The checkout page dynamically offers PIX as a payment option via Mercado Pago.
3. **User View**: The business owner sees the successful payment in their OHC financial dashboard alongside standard credit card transactions, seamlessly converted or displayed in their local currency.

## Implementation Prompt
Integrate the Mercado Pago API as an alternative payment gateway. Build a settings module where users can select their preferred payment provider based on their region. Update the checkout UI to support Mercado Pago's payment flows (including asynchronous methods like PIX) and ensure the Finance agent can accurately track and report on these transactions.

## Priority
P2

## Estimated Scope
Large
