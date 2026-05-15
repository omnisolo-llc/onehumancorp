# [payment] Issue Brief: Global Alternative Payments

**Title**: Mercado Pago & Razorpay Integration for Emerging Markets
**Problem Statement**: As a seller in Latin America or India, my customers don't always have credit cards to use Stripe. I need them to be able to pay using local methods like Pix (Brazil) or UPI (India) so I don't lose sales.
**Research Report**:
- Evaluated Tools: Mercado Pago (LATAM), Razorpay (India), Alipay (China), PayPal (Global).
- Ease of Use: Mercado Pago is dominant in LATAM with easy APIs. Razorpay is the standard for India. Both offer drop-in UI components similar to Stripe Elements.
- Pricing: Standard localized processing fees (typically 2-3%).
- Reputation: Both are market leaders in their respective regions.
- Environment: Cloud and Standalone (standard API/Webhook architecture).
- Recommendation: Abstract the payment provider layer. Implement Mercado Pago for LATAM users and Razorpay for India users to truly support "anyone".
**Design Doc**:
- **Integration Flow**: Based on the business's country setting, OHC dynamically offers the correct payment provider connection in the Finance department. User clicks "Connect Mercado Pago" or "Connect Razorpay" and completes OAuth.
- **Actions**: Replaces Stripe Checkout with Mercado Pago Checkout / Razorpay Checkout for public storefront purchases. Webhooks update OHC order status to "Paid".
- **User Interface**: No change to the public storefront other than the payment modal matching the local provider. The Finance dashboard normalizes all transactions into a single "Revenue" view regardless of the underlying processor.
**Implementation Prompt**: Abstract the checkout and payment webhook processing to support multiple providers, starting with Mercado Pago and Razorpay alongside the existing Stripe integration. When a user in a supported region connects their account, the storefront must route checkout sessions to the correct provider. Acceptance criteria: A successful mock checkout using Mercado Pago/Razorpay, webhooks successfully marking the order as paid, and revenue appearing in the Finance dashboard.
**Priority**: P1
**Estimated Scope**: Large
