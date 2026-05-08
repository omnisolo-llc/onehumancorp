# Title: Expanding Checkout Options with Mercado Pago Integration

## Problem Statement
While Stripe is excellent for the US and Europe, small businesses in Latin America rely heavily on local payment methods (Pix in Brazil, OXXO in Mexico, local credit cards with installments). Without these options, checkout conversion drops dramatically. They need a payment provider their local customers trust.

## Research Report
- **Tool Evaluated**: Mercado Pago API
- **Benefit to Users**: Unlocks massive markets in LATAM by supporting local currency, local banking networks, and installment payments.
- **Ease of Use**: Owner connects their existing Mercado Pago account. OHC automatically generates the correct localized checkout screens for their customers based on region.
- **Pricing**: Standard localized processing fees (typically 3-5% depending on the country and payout speed).
- **Integration Risks**: Webhook payloads and API structures vary slightly by country within the Mercado Pago ecosystem. Sandbox testing can sometimes be unreliable compared to production.
- **Environment**: Works seamlessly in both Cloud and Standalone modes.

## Design Doc
- **Trigger**: User selects "Latin America" as their primary market in settings, prompting a recommendation to connect Mercado Pago.
- **Action**: User authenticates or enters API keys. OHC adds Mercado Pago as a payment method on invoices and booking links.
- **User Interface**: When generating an invoice, the user sees a single "Payment Link". The customer clicks it and sees localized payment options (e.g., Pix QR code or credit card installments).

## Implementation Prompt
Integrate Mercado Pago as an alternative payment gateway alongside the existing system. Allow users to connect their Mercado Pago credentials. When an OHC invoice or checkout link is generated for a user with this integration, redirect the customer to a Mercado Pago checkout flow that supports local payment methods. Handle successful payment webhooks to mark invoices as paid in OHC.

## Priority
P1

## Estimated Scope
Large