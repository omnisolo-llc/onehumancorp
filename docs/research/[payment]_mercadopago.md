# Title: Integrate Mercado Pago for LATAM Payment Processing

## Problem Statement
While Stripe works well in North America and Europe, many small business owners in Latin America (LATAM) rely on local payment methods (like Pix in Brazil or OXXO in Mexico). Without a localized payment provider, these businesses face high cart abandonment and struggle to accept online payments.

## Research Report
Mercado Pago is the leading payment processor in LATAM, supporting local currencies, installments, and local cash-based payment methods.
- **Ease of Use:** Merchant onboarding is familiar to LATAM users as it integrates with the broader Mercado Libre ecosystem.
- **Pricing:** Processing fees vary by country (e.g., ~3-5% for credit cards), but there are no setup or monthly fees. Settlement speed can be adjusted (instant settlement costs a higher fee, while 30-day settlement is lower).
- **Reputation:** The undisputed leader in the region. Trusted by millions of consumers.
- **Competitors:** dLocal, Ebanx. Mercado Pago is vastly superior for *small* local businesses, whereas dLocal targets enterprise cross-border merchants.
- **Cloud vs Standalone:** Fully supports both environments via standard API integrations and webhooks for payment confirmations.

## Design Doc
OHC will allow users to accept payments on their invoices or booking pages using Mercado Pago.
- **Trigger:** A customer attempts to pay an OHC invoice or complete a booking checkout.
- **Action:** OHC redirects the customer to a Mercado Pago hosted checkout page (Checkout Pro) or embeds a localized payment element. Upon successful payment, the invoice is marked as paid in OHC.
- **User Interface:** Business owners enter their Mercado Pago credentials in a "Payments" settings module. Customers see "Pay with Mercado Pago" on their checkout screens, alongside options for local methods like Pix.

## Implementation Prompt
Add Mercado Pago as an alternative payment gateway alongside Stripe. Allow the business owner to input their Mercado Pago Access Token in the settings. When generating a payment link for an invoice, generate a Mercado Pago "preference" and provide a checkout link to the user. Listen for Mercado Pago IPN (Instant Payment Notification) webhooks to automatically mark the corresponding invoice as paid in the OHC dashboard.

## Priority
P2

## Estimated Scope
Large