# Scout 🔍: Integrate Razorpay for the Indian Market

## Problem Statement
Ananya (Boutique Owner in Bangalore) needs to accept UPI, RuPay, and local credit cards. Stripe's coverage in India is improving but often lacks the deep integration with UPI that local customers expect. She needs a trusted local payment gateway that her customers are comfortable with to reduce abandoned carts.

## Research Report
- **Tool**: Razorpay
- **Target Persona**: Ananya (Boutique Owner), Indian SMBs.
- **Evaluation**: Razorpay is the leading payment gateway in India. It supports UPI (the most popular method), all major cards, Netbanking, and various Wallets.
- **Ease of Use**: High. Comprehensive onboarding and a very stable dashboard.
- **Pricing**: ~2% per transaction for domestic cards/UPI. No setup fee or monthly maintenance fee for standard accounts.
- **Reputation**: Market leader in India, highly trusted by both merchants and consumers.
- **Cloud vs. Standalone**: Compatible with both. Cloud (managed OHC connection). Standalone (user-provided API Key/Secret).

## Design Doc
- **User Experience**: User selects "India" as their region during setup and is prompted to connect Razorpay.
- **Checkout**: The checkout page dynamically shows UPI (with app-specific deep links like GPay/PhonePe) and local payment options.
- **Reporting**: Razorpay settlement and revenue data are normalized into the OHC "Accountant" dashboard.

## Implementation Prompt
Add Razorpay as a secondary native payment provider. Implement the checkout flow and webhook handling to support UPI, RuPay, and Indian domestic cards. Ensure payment status updates are normalized into standard OHC order fulfillment events.
- **Acceptance Criteria**: Indian merchants can connect Razorpay. Customers can pay via UPI. Orders are marked paid automatically upon success.
- **Priority**: P1
- **Estimated Scope**: Medium
