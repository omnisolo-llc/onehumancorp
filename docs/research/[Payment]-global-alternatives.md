# Payment Processing: Global Alternatives

## Title
Integrate Local Payment Providers (Mercado Pago, Razorpay)

## Problem Statement
Stripe is not available or preferred everywhere. Business owners in LATAM, India, or other regions need to accept payments using the tools their local customers trust, otherwise they lose sales at checkout.

## Research Report
- **Tools Evaluated:** Mercado Pago (LATAM), Razorpay (India), Paystack (Africa), Alipay/WeChat Pay (China).
- **Ease of Use:** All have modern APIs, though not as streamlined as Stripe.
- **Pricing:** Varies locally, typically 1.5% - 3% + fixed fee.
- **Reputation:** Mercado Pago dominates LATAM; Razorpay is the standard in India.
- **Cloud vs Standalone:** Webhooks for payment success are required, which is standard in Cloud. Standalone requires polling or an OHC cloud webhook relay.

## Design Doc
- **Trigger:** User selects their country in "Payment Settings". OHC recommends the best local provider.
- **Action:** User connects their account. OHC generates payment links or embeds checkout components for that provider.
- **User View:** When creating an invoice or selling a service, the customer sees the local payment options (e.g., PIX in Brazil via Mercado Pago).

## Implementation Prompt
Expand the OHC billing/payment module to support alternative gateways based on the user's region. Create an abstraction layer so the user experience of generating an invoice or a payment link remains identical, but the underlying processor (e.g., Mercado Pago, Razorpay) changes based on their connected account. Ensure webhook handling (or polling for standalone) is reliable to mark invoices as paid.

## Priority
P1

## Estimated Scope
Large
