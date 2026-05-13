# Regional Payment Processing Module

## Problem Statement
Relying only on cash or direct bank transfers limits sales, especially internationally. Owners need diverse, easy-to-setup payment options tailored to their region (e.g., LATAM, India, China).

## Research Report

**Market Context:**
Could not fetch info: HTTP Error 429: Too Many Requests

**Evaluated Tools:**

#### In-Depth Evaluation: Stripe
**Market Position**: The gold standard for developer-friendly payment gateways. Massive global reach but notably absent or weak in certain specific regions.
**Pricing**: Typically 2.9% + 30¢.
**Integration Approach**: Integration must use Stripe Elements or Checkout to ensure PCI compliance (no credit card data touches OHC servers or local SIPDB). Webhooks are critical for async payment confirmation.
**Persona Impact**: Reliable, trusted checkout experience for customers, reducing cart abandonment.

#### In-Depth Evaluation: PayPal
**Market Position**: Universal consumer trust. Customers often prefer it so they don't have to enter card details.
**Pricing**: Similar to Stripe, often slightly higher for certain transactions.
**Integration Approach**: Integration can be clunky historically but their newer REST APIs are better. OHC must support this alongside Stripe as a checkout option.

#### In-Depth Evaluation: Mercado Pago
**Market Position**: Absolutely critical for the Latin American market where Stripe/PayPal penetration is lower and local payment methods (like Pix in Brazil or OXXO in Mexico) are required.
**Pricing**: Varies wildly by country and payment method.
**Integration Approach**: Regional gateways require specific handling of async cash payments (e.g., customer prints a voucher and pays at a convenience store). OHC must be able to hold an order in 'pending' state potentially for days until the Mercado Pago webhook fires.

## Design Doc
Expand the billing module to support a pluggable architecture for regional payment gateways (Mercado Pago, Paytm, Alipay). The UI provides a 'connect' button for each; the backend handles the specific OAuth/API keys and normalizes webhook events for successful payments.

## Implementation Prompt
Create a checkout component that dynamically displays payment options based on the user's region or preference. Implement the backend handling to process payments through multiple regional gateways and update invoice status.

## Priority
P0

## Estimated Scope
Large
