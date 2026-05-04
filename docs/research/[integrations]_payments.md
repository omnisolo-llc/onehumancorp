# Payment Processing

## Title
[Payments] Global Alternative Payment Methods (Mercado Pago, Paytm, Alipay)

## Problem Statement
While Stripe is excellent, it is not supported or preferred in all regions. Business owners in LATAM, India, or China need local payment methods to successfully convert sales, as credit card penetration is lower and local wallets are dominant.

## Research Report
- **Evaluated Tools**: Mercado Pago (LATAM), Paytm (India), Alipay/WeChat Pay (China), Razorpay.
- **Ease of Use**: Varies. Razorpay provides a Stripe-like experience for India. Mercado Pago is dominant in LATAM.
- **Pricing**: Typically 2-3% per transaction, competitive with Stripe but in local currencies.
- **Settlement Speed**: Often faster than Stripe for local bank transfers (T+1 or instant).
- **Currency Support**: Highly localized.
- **Cloud vs Standalone**: Both modes support API calls, but webhook handling in Standalone requires secure local tunneling.

## Design Doc
- **Triggers**: Customer proceeds to checkout.
- **Actions**: The system detects the region or allows the user to select their preferred local payment provider, redirecting to the provider's secure checkout page, and handling the success webhook.
- **User View**: In the Finance settings, owners can toggle specific regional payment methods. Customers see familiar local payment options at checkout.

## Implementation Prompt
Integrate alternative payment providers to support global users, starting with Mercado Pago for LATAM and Razorpay for India. Allow the business owner to enable these options in their payment settings. Ensure the checkout flow seamlessly transitions to these providers when selected, and correctly records the payment success in the OHC Finance dashboard.
- **Acceptance Criteria**: Business owner can enable Mercado Pago and/or Razorpay in their Finance settings. During checkout, customers are presented with the enabled payment methods alongside any existing ones. Selecting an alternative payment method seamlessly transitions the customer to the respective provider's secure checkout flow. Upon successful payment, the order status is automatically updated to "Paid" in the OHC dashboard.

## Priority
P2

## Estimated Scope
Large
