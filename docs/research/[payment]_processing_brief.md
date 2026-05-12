# Title: Expand Global Checkout Support via Mercado Pago

## Problem Statement
Many small business owners operating outside the US and Europe (specifically in Latin America) cannot use Stripe. They need a localized payment gateway that their customers trust, supporting local payment methods (like Pix in Brazil or OXXO in Mexico).

## Research Report
- **Tool Evaluated:** Mercado Pago
- **Benefits:** The dominant payment processor in Latin America, offering high conversion rates and support for regional payment methods.
- **Ease of Use:** Customers get a familiar, trusted checkout experience.
- **Pricing:** Standard payment processing fees (percentage + fixed fee per transaction), no monthly cost.
- **Cloud/Standalone:** Fully supported via API in both modes.

## Design Doc
1. **Trigger:** Business owner selects their operating country during onboarding or in settings.
2. **Action:** If in LATAM, Mercado Pago is offered as the primary checkout provider alongside or instead of Stripe.
3. **UI Outcome:** The business owner sees a simple "Connect Mercado Pago" button. Once connected, all generated invoices and storefront checkouts automatically route through the Mercado Pago gateway.

## Implementation Prompt
Add Mercado Pago as a primary payment gateway option for invoicing and storefront checkouts. Build an onboarding flow allowing users to connect their Mercado Pago credentials. Ensure the checkout UI dynamically updates to show local payment options (e.g., Pix) when this gateway is active.

## Priority
P1

## Estimated Scope
Medium
