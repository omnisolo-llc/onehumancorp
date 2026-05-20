# Payment Processing Integration

## Problem Statement
While Stripe is an excellent default for the US and Europe, OHC needs to support businesses globally. Users outside these regions require local payment methods to effectively process transactions.

## Evaluated Tools
We evaluated the following regional payment providers:
1. **Razorpay**: The dominant payment gateway in India, essential for capturing the Indian SMB market.
2. **Mercado Pago**: The leading provider in Latin America, offering crucial local payment methods like Pix and OXXO.
3. **dLocal**: A strong cross-border payment platform for emerging markets, though sometimes more suited for larger enterprises rather than simple SMB integrations.

## Key Recommendation
- Payment processing must move beyond a single provider. We recommend **building a payment abstraction layer** within OHC. This layer will allow us to support Stripe by default but seamlessly swap in regional leaders like **Razorpay (India)** and **Mercado Pago (LATAM)** based on the user's location, ensuring global reach.
