# Payment Processing Tools

**Title**: Evaluate Alternative Payment Processors (Mercado Pago, Razorpay)

**Problem Statement**:
While Stripe is OHC's primary processor, it is not available or the preferred method in all global markets (e.g., Latin America, India). Users in these regions need localized payment options to accept online and in-person payments.

**Research Report**:
Evaluated Mercado Pago (LATAM) and Razorpay (India).
- **Mercado Pago**: Dominant in Latin America (Brazil, Mexico, Argentina). Supports local payment methods like PIX in Brazil and OXXO in Mexico.
  - *Ease of Use*: Provides Checkout Pro which is easy to integrate.
  - *Pricing*: Varies by country, generally competitive for the region.
- **Razorpay**: Dominant in India. Supports UPI, Netbanking, and local wallets.
  - *Ease of Use*: Excellent API and developer docs.
  - *Pricing*: Standard 2% for most transactions.
- **Recommendation**: Build a Payment Provider interface in OHC's backend. Implement Mercado Pago and Razorpay as secondary options behind Stripe, allowing users to select their processor based on their business location.

**Design Doc**:
- **Trigger**: User selects their business country during onboarding. If in a supported region, they are offered local payment processors.
- **Action**: User connects their Mercado Pago or Razorpay account via standard OAuth or API key entry. The OHC checkout flow dynamically renders the appropriate payment widget based on the configured provider.
- **User Experience**: The checkout experience remains seamless for the customer. The business owner sees consolidated revenue metrics in the OHC dashboard, regardless of the underlying processor.

**Implementation Prompt**:
Abstract the payment processing logic in the Go backend into an interface (e.g., `CreatePaymentIntent`, `HandleWebhook`). Implement Mercado Pago and Razorpay adapters. Update the Flutter checkout UI to dynamically support UPI/PIX flows based on the active provider. Ensure idempotency and secure webhook handling for all new providers.

**Priority**: P2
**Estimated Scope**: Large
