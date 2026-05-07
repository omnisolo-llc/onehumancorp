# Title: Localized Payment Processing for the Indian Market via Paytm

## Problem Statement
While Stripe is excellent globally, many local markets prefer specific payment methods. In India, small business owners rely heavily on UPI and local wallets. If an OHC storefront only offers credit card processing via Stripe, they will lose a massive percentage of local sales due to payment friction. They need a payment option their local customers actually use.

## Research Report
Paytm is a dominant payment gateway and digital wallet in India.
- **Ease of Use**: Well-understood by Indian merchants and consumers. The merchant onboarding process is localized and handles local compliance (KYC).
- **Pricing**: Competitive local rates, often lower for UPI transactions compared to international credit card processing fees.
- **Reputation**: Extremely high trust among Indian consumers. Ubiquitous in both online and offline retail.
- **Comparison**: For the Indian market, integrating Paytm (or Razorpay) alongside or instead of Stripe is mandatory for conversion rates, as UPI is the primary method of digital transaction.
- **Cloud vs Standalone**: Payment webhook callbacks are required to mark invoices as "Paid", meaning it functions natively in Cloud mode but needs a secure local webhook endpoint or polling fallback in Standalone mode.

## Design Doc
- **Triggers & Actions**: When a customer checks out on an OHC-generated storefront or receives an invoice, they can select "Pay with Paytm/UPI". The payment is processed securely, and OHC updates the order status to "Paid" upon receiving the success webhook.
- **User Experience**: In the OHC "App Settings" under "Payments", users in the India region see an option to "Connect Paytm". Once connected, their storefronts automatically show the Paytm payment button. The OHC dashboard will show these transactions seamlessly alongside any others.

## Implementation Prompt
Add localized payment gateway support for the Indian market.
- **User-Facing Outcome**: Indian business owners can accept payments via Paytm and UPI on their OHC storefronts and invoices, increasing their sales conversions.
- **Acceptance Criteria**:
  - A "Connect Paytm" option is available in the payment settings.
  - Customers can complete a checkout process using Paytm.
  - Successful payments automatically update the associated order or invoice status to "Paid" in OHC.

## Priority
P1

## Estimated Scope
Medium
