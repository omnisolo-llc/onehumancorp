# Razorpay Integration for OHC

## Problem Statement
While OHC relies heavily on Stripe, Stripe is not available or preferred in all regions. For small business owners in India (a massive market), Razorpay is the dominant payment gateway. Without Razorpay support, OHC cannot effectively serve the Indian SMB market, preventing users from accepting local payment methods like UPI, RuPay, and local net banking.

## Research Report
- **Features & API Suitability**: Razorpay provides a comprehensive REST API. Features include Payment Links, Subscriptions, Invoices, and Smart Routing. Webhooks are available for payment status updates.
- **Pricing**: Transparent transaction-based pricing (typically 2% for domestic cards/UPI in India, 3% for international). No setup fees.
- **Ease of Use for Non-Technical Users**: Moderate to High. Requires KYC for the merchant, but the API integration itself is straightforward once API keys are obtained.
- **Cloud vs. Standalone**: Works well in both. Webhooks require a public endpoint in Cloud.
- **Advantages**: Critical for the Indian market; supports UPI (Unified Payments Interface), which is essential.
- **Risks**: Strict regulatory environment in India (RBI guidelines).

## Design Doc
- **Integration Point**: "The Accountant" (Finance & Payments).
- **Trigger**: User selects "Razorpay" as their payment provider in settings and enters their API Key/Secret.
- **Action**: OHC uses Razorpay Orders API to create orders for checkout. Webhooks update payment status in OHC.
- **User View**: Customers see Razorpay checkout options (UPI, cards, net banking) on the storefront. Business owner sees Razorpay payouts and balances in the Finance dashboard.

## Implementation Prompt
Implement Razorpay as an alternative payment gateway. Allow Indian merchants to configure their Razorpay API Key and Secret. Update the checkout flow to dynamically use Razorpay instead of Stripe if configured. Implement webhook listeners to handle successful payments and update order statuses in OHC. Ensure support for generating Razorpay Payment Links for manual invoices.

## Priority
P1

## Estimated Scope
Large
