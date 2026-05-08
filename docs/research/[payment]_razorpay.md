# Scout: Tool Integration Research [Q2]

## [Payment Processing] Issue Brief: Razorpay Integration

**Title**: Native Payment Integration for Indian Market via Razorpay

**Problem Statement**:
Small business owners in India need to accept payments via UPI, RuPay, and local debit cards to cater to their customers' preferences. While Stripe is available, many Indian merchants and customers prefer local providers like Razorpay which offer deeper integration with the India Stack (UPI, Aadhaar) and often have higher success rates for domestic transactions.

**Research Report**:
- **Tool**: Razorpay Payment Gateway.
- **Evaluation**: The leading payment processor in India. It provides a seamless checkout experience for UPI (the dominant payment method in India).
- **Ease of Use**: High. Razorpay's "Standard Checkout" is very intuitive for both merchants and customers.
- **Pricing**: Transparent (~2% for domestic transactions).
- **Reputation**: Highly trusted and widely used across all sectors in India.
- **Cloud vs. Standalone**: Works in both. Requires merchant to have their own Razorpay account.

**Design Doc**:
- During onboarding, if the merchant is in India, OHC offers Razorpay as a primary payment option.
- Merchant connects their Razorpay account via API keys or OAuth.
- At checkout, OHC dynamically switches to the Razorpay Standard Checkout for Indian customers.
- OHC listens for Razorpay webhooks to update order status to "Paid" and trigger fulfillment.
- "The Accountant" AI aggregates Razorpay data into the financial overview.

**Implementation Prompt**:
Integrate Razorpay as a native payment provider. The system should dynamically select Razorpay for merchants in India. Implement the checkout flow and webhook handling to normalize Razorpay events into the OHC order system.
- **Acceptance Criteria**: Merchant in India can connect Razorpay. Customers can pay via UPI/local cards. Orders are updated via webhooks.
- **Priority**: P1
- **Estimated Scope**: Large
