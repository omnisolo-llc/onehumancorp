# [payment-processing] Global Alternative Payment Methods

**Title:** Integrate Alternative Local Payment Gateways

**Problem Statement:**
While Stripe is powerful, it is not universally dominant in all regions. Small business owners in LATAM, India, and Asia lose sales because they cannot offer preferred local payment methods (e.g., Mercado Pago, PIX, UPI, Alipay). They need localized checkout options to convert local customers.

**Research Report:**
* **Tools Evaluated:** Mercado Pago (LATAM), Razorpay (India), Alipay (China), dLocal.
* **Ease of Use:** Integrating regional market leaders directly benefits merchants by providing localized checkout flows that customers trust.
* **Key Advantages:**
  - Significantly higher conversion rates in specific regions.
  - Supports local currency settlements without exorbitant FX fees.
  - Familiar UI/UX for the end-consumer.
* **Risks:**
  - Handling multiple webhook formats and differing payout schedules.
  - Increased complexity in the checkout routing logic.
* **Pricing Estimate:** Varies heavily by region and provider (typically 1.5% - 3.5% + fixed fee per transaction).
* **Environment Support:** fully supported in Cloud mode. Standalone mode can connect to the API via external network requests.

**Design Doc:**
* **Trigger:** The business owner configures their "Store Region" in the OHC dashboard.
* **Actions:** OHC dynamically exposes the relevant regional payment provider (e.g., Mercado Pago for Brazil/Mexico) in the "Payments" settings. The user connects their account via OAuth or API key.
* **User Experience:** When an end-consumer goes to checkout, they see their trusted local payment method (like PIX or UPI) alongside standard credit card options.

**Implementation Prompt:**
Implement a modular payment checkout architecture that supports regional providers beyond Stripe. Start by integrating Mercado Pago to support LATAM markets. Ensure the checkout UI dynamically updates based on the merchant's configured region, and ensure payment success/failure webhooks accurately update the OHC order status.

**Priority:** P1
**Estimated Scope:** Large