# 🔍 Scout: Razorpay (Payment Processing for India)

## Title
Integrate Razorpay to Support the Indian SMB Market

## Problem Statement
While Stripe is the global default, it is not the dominant player for local payment methods in India. Indian small business owners need to accept UPI, RuPay cards, and local net banking effortlessly. Without a native, trusted payment gateway like Razorpay, OHC will struggle to gain traction in one of the world's largest SMB markets due to checkout friction.

## Research Report
**Razorpay** is a leading payment gateway in India, tailored specifically for the nuances of the Indian financial ecosystem. It provides a comprehensive API and standard checkout UI components.

**Pros for Non-Technical Users:**
- Supports all popular Indian payment methods natively (UPI, cards, wallets, net banking).
- Familiar checkout experience for Indian consumers increases conversion rates.
- Handles complex Indian compliance and settlement requirements.

**Integration Risks:**
- Indian payment regulations (like mandatory 2FA/OTP for cards) can complicate the checkout flow compared to simple Stripe charges.
- Requires the merchant to complete a strict KYC (Know Your Customer) process with Razorpay before going live, which can delay onboarding.
- Webhook reliability and latency can vary; robust idempotency and retry logic are crucial.

**Pricing:**
- Standard pricing for Indian gateways (typically around 2% per transaction). No setup fees.

**Environment Support:**
- Cloud-based. Standalone mode works identically but requires the user to input their Razorpay API Keys.

## Design Doc
- **Integration:** The merchant enters their Razorpay Key ID and Key Secret in the OHC payments settings page.
- **Data Flow:** When a customer checks out, OHC creates an order via the Razorpay API and passes the Order ID to the frontend. The frontend launches the Razorpay Checkout widget. Upon success, Razorpay sends a webhook to OHC to verify the payment signature and mark the order as paid.
- **Action:** The "Finance & Payments" agent monitors these transactions, manages refunds through the API, and reconciles payments.

## Implementation Prompt
Implement Razorpay as a primary payment provider alternative to Stripe. Create the backend logic to generate Razorpay Orders and a secure webhook handler that strictly verifies the `x-razorpay-signature` header using the merchant's secret. Update the checkout frontend to dynamically load the Razorpay checkout script and handle the payment lifecycle if Razorpay is the active gateway. Ensure the OHC data model can store Razorpay-specific transaction IDs and order IDs alongside Stripe data.

## Priority
P1

## Estimated Scope
Large
