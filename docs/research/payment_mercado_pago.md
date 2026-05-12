# Integrate Mercado Pago for LATAM Payment Processing

## Problem Statement
While Stripe is the global standard, it is not always the best (or even supported) option for small business owners operating in Latin America. These users need a trusted, localized payment processor that supports local payment methods (like Pix in Brazil or OXXO in Mexico) and settles funds in local currencies without exorbitant cross-border fees.

## Research Report
**Tool**: Mercado Pago
Mercado Pago is the leading digital payment platform in Latin America (backed by Mercado Libre).
- **Ease of use**: High consumer trust in LATAM; business setup requires standard KYC for the region.
- **Pricing**: Varies by country, but generally competitive with local alternatives and avoids international conversion fees for LATAM merchants.
- **Reputation**: Ubiquitous in LATAM, essential for capturing the unbanked or underbanked market.
- **Environment**: Supports robust webhooks and REST APIs, working seamlessly in both Cloud and Standalone environments.

## Design Doc
The integration will add Mercado Pago as an alternative payment gateway alongside Stripe in OHC.
- **Trigger**: User navigates to Settings -> Payments and selects "Connect Mercado Pago" instead of Stripe.
- **Actions**: OHC generates a Mercado Pago checkout preference for orders, handles the redirect to the Mercado Pago hosted checkout, and listens for webhooks (IPN) to mark orders as paid.
- **User View**: A simple toggle to set Mercado Pago as the active payment provider. A dashboard view showing recent payouts and current Mercado Pago balance (if the API permits).

## Implementation Prompt
Add a new Payment Provider option for "Mercado Pago" in the settings. When selected, the user must input their Mercado Pago Access Token and Public Key. Update the checkout flow: if Mercado Pago is the active provider, generate a Checkout Preference via their API and redirect the customer to the provided `init_point` URL. Implement a webhook handler to receive `payment` updates and update the corresponding OHC order status to "Paid". Ensure the UI clearly shows which payment provider is currently active.

## Priority
P1

## Estimated Scope
Large
