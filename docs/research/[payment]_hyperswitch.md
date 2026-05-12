# Payment Processing: Global Payments via Hyperswitch

## Title
Enable Global Payment Links and Invoices

## Problem Statement
Small business owners need to send invoices or payment links to customers to get paid online, but different countries require different payment gateways (Stripe in US, Razorpay in India, Mercado Pago in LATAM).

## Research Report
- **Tool Evaluated:** Hyperswitch
- **Ease of Use:** For the business owner, it's seamless (just "connect bank/provider").
- **Pricing:** Open source (free to host).
- **Reputation:** Fast-growing payment router that abstracts multiple gateways.
- **Cloud/Standalone Compatibility:** Excellent. API-driven, deployable locally or managed in the cloud.

## Design Doc
- **Integration Point:** "Invoices" section and a "Create Payment Link" quick action.
- **User Experience:** The business owner creates an invoice with line items, generating a secure link. When the customer clicks the link, they see local payment options based on their region.
- **System Behavior:** OHC routes the payment intent through Hyperswitch, which automatically selects the best configured underlying gateway (Stripe, PayPal, regional provider).

## Implementation Prompt
Create an "Invoice & Payment Link" generator. The UI should allow adding line items, taxes, and customer details to generate a professional invoice and a shareable URL. The payment page seen by the customer must be mobile-optimized, secure, and clearly display the business owner's branding.

## Priority
P0

## Estimated Scope
Large
