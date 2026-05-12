# OHC Tool Integration: Razorpay for India Market

## Title
Implement Razorpay for Localized Payments in India

## Problem Statement
Stripe's limitations in certain markets, specifically India, prevent business owners in those regions from accepting local payment methods (UPI, domestic cards), severely limiting their conversion rates.

## Research Report
- **Tool Evaluated:** Razorpay
- **Why Razorpay?** The dominant payment gateway in India, supporting UPI, RuPay, and local net banking—essential for serving the Indian market.
- **Ease of Use:** Standard merchant onboarding (KYC required by Indian law). Checkout experience is optimized for local consumers.
- **Pricing:** ~2% per transaction for domestic cards/UPI.
- **Reputation:** Market leader in India, robust API.

## Design Doc
- **Trigger:** Customer selects "Pay" at checkout (when the merchant is based in India).
- **Action:** OHC initializes a Razorpay order and displays the Razorpay checkout modal. Upon successful payment, Razorpay sends a webhook, and OHC marks the order as paid.
- **User View:** Business owners in supported regions see Razorpay as a provider option in Payment Settings. Customers experience a familiar, localized checkout flow supporting UPI apps (GPay, PhonePe).

## Implementation Prompt
Develop the Razorpay payment gateway integration. Add Razorpay to the list of supported payment providers in the merchant settings. Update the checkout flow to dynamically load the Razorpay checkout widget if the merchant uses this provider. Implement secure webhook handling to verify payment signatures and update order statuses accordingly.

## Priority
P2

## Estimated Scope
Medium
