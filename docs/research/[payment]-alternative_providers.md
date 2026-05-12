# Alternative Payment Providers for Global Markets

## Problem Statement
Stripe isn't supported or preferred in all regions, locking out businesses in LATAM and Asia.

## Research Report
Evaluated Mercado Pago (LATAM), Paytm (India), and Razorpay (India). Mercado Pago is essential for LATAM. Razorpay is dominant in India. Both provide robust APIs.

## Design Doc
Add payment gateway selection during OHC store setup. The owner inputs their API keys for the selected provider. The checkout flow dynamically routes to the correct provider.

## Implementation Prompt
Implement settings to allow adding Mercado Pago and Razorpay credentials. Update the checkout component to support these gateways as alternatives to Stripe.

## Priority
P1

## Estimated Scope
Large
