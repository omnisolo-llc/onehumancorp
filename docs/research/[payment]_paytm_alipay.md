# Scout: Tool Integration Research

## [Payment] Issue Brief: Localized Payments for Asia (Paytm & Alipay)
**Title**: Integrate Paytm and Alipay for Asian Market Expansion
**Problem Statement**:
Small business owners in India and China need to accept payments via the most popular local methods — Paytm (UPI/Wallet) and Alipay. Stripe's standard checkout doesn't always provide the localized experience or settlement speed that these merchants require for their local entities.

**Research Report**:
- **Tools**: Paytm for Business API (India) and Alipay Global API (China/Global).
- **Evaluation**:
  - **Ease of Use**: Medium. Requires merchant onboarding with the respective providers.
  - **Pricing**: Standard per-transaction fees (often lower than international credit cards for local transactions).
  - **Reputation**: Dominant market leaders in their respective regions.
  - **Cloud vs. Standalone**: Both supported.
- **Key Advantages**: Unlocks OHC for millions of merchants in India and China. Supports UPI (India's instant payment system).
- **Risks**: Regional regulatory compliance and KYC requirements.

**Design Doc**:
- **User Flow**: Merchant selects their country during onboarding. If India, Paytm is featured. If China/SE Asia, Alipay is featured.
- **Integration**: Redirect-based checkout or QR code generation for Wallets.
- **User Experience**: Customer sees a familiar "Pay with Paytm" or "Pay with Alipay" button on the OHC storefront.

**Implementation Prompt**:
Integrate Paytm and Alipay as alternative payment gateways. The system should dynamically offer these providers based on the merchant's settings. Ensure the checkout flow handles the redirect/callback pattern for these wallets and correctly updates order status via webhooks.

**Priority**: P1
**Estimated Scope**: Large
