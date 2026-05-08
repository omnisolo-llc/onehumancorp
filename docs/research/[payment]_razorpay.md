## [Payment] Razorpay Integration
**Title**: Integrate Razorpay for Indian Market Payments
**Problem Statement**: Small business owners in India need to accept payments via UPI, RuPay, and local wallets, which global providers like Stripe don't always handle optimally or cost-effectively for the domestic market.
**Research Report**:
- **Tool**: Razorpay
- **Target Persona**: Indian SMBs, E-commerce sellers in India
- **Advantages**: Dominant in India, supports UPI, great developer experience.
- **Risks**: Strictly limited to Indian businesses (requires Indian registration).
- **Pricing**: Standard ~2% per transaction.
- **Compatibility**: Cloud (OAuth/API). Standalone (API Keys).
**Design Doc**:
- During onboarding, if the user selects India, Razorpay is offered as the primary payment gateway.
- Customers check out using Razorpay's standard UI, supporting UPI and local cards.
- Webhooks confirm payment success to OHC.
**Implementation Prompt**: Integrate Razorpay as a checkout provider. Implement order creation via Razorpay API, handle the frontend payment flow, and process webhooks for successful payments and refunds.
**Priority**: P2
**Estimated Scope**: Large
