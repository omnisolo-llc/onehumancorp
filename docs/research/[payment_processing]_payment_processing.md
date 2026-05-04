# [Payment Processing] Global Payment Alternatives via Mercado Pago & Razorpay

**Title**: Implement Localized Payment Gateways (Mercado Pago / Razorpay)

**Problem Statement**:
While OHC relies heavily on Stripe, Stripe is not supported in all global markets, or it lacks dominance in local payment methods. For example, a business owner in Brazil relies heavily on Pix (often via Mercado Pago), and a merchant in India relies on UPI (often via Razorpay). To fulfill OHC's mission of empowering "anyone", the platform must support the payment gateways that small business owners actually use in their local regions, without confusing them with technical setup.

**Research Report**:
I evaluated adding Mercado Pago (LATAM) and Razorpay (India) as Stripe alternatives within OHC.
- **Mercado Pago**: Absolute dominance in Latin America. Supports Pix (Brazil), local credit cards, and cash payments (e.g., Boleto). The API is mature and supports webhook notifications for asynchronous payments (like Boleto).
- **Razorpay**: The dominant player in India. Flawless UPI integration, which is critical as UPI handles the vast majority of small transactions in India.
- **Stripe Local Methods**: Stripe supports some local methods (like Pix and UPI), but merchant account creation in these specific countries via Stripe Connect can be highly restrictive or currently unsupported compared to the local giants.
- **Conclusion**: We need an abstracted "Payment Provider" interface in the OHC backend. When a user in Brazil sets up their store, the AI "Accountant" should seamlessly configure Mercado Pago. When in India, it configures Razorpay.

**Design Doc**:
- **Integration Point**: Resides within the "Finance & Payments" (The Accountant) department.
- **Triggers & Flow**:
  1. During onboarding, OHC detects the user's country.
  2. If the country is best served by Mercado Pago or Razorpay, the "Accountant" guides them through an OAuth/API key setup tailored to that provider.
  3. The OHC checkout UI dynamically swaps its backend payment intent generator to point to the correct provider.
  4. Webhooks from the localized provider update the order status in OHC.
- **User View**: A simple "Payments" settings page that automatically recommends the best gateway for their region. A seamless checkout experience for their customers supporting Pix/UPI.

**Implementation Prompt**:
Refactor the checkout and payment intent architecture to support multiple payment gateways behind a unified interface. Implement integrations for Mercado Pago (targeting LATAM/Pix) and Razorpay (targeting India/UPI). The onboarding flow must automatically detect the user's region and recommend the appropriate gateway. Ensure the checkout UI gracefully handles asynchronous payment methods (like waiting for a Pix scan or UPI approval) with real-time UI updates powered by provider webhooks.

**Priority**: P2
**Estimated Scope**: Large
