# Global Payment Gateway Integration (Beyond Stripe)

**Problem Statement:**
While Stripe is great, it is not supported everywhere. Business owners in LATAM, India, or Southeast Asia need to accept payments using local, trusted providers (like Mercado Pago, Paytm, or Alipay). Without local payment options, cart abandonment rates skyrocket because customers cannot pay with their preferred local methods.

**Research Report:**
- **Evaluated Tools:** Mercado Pago (LATAM), Razorpay (India), Alipay (China), PayPal (Global).
- **Ease of Use:** Business owners need a simple "Connect Mercado Pago" button similar to Stripe Connect.
- **Pricing:** Varies by region, typically 2-3% per transaction plus a fixed fee.
- **Reputation:** Razorpay and Mercado Pago are highly trusted in their respective regions with robust developer APIs.
- **Cloud vs Standalone:** Fully supported in both modes via API integration and webhooks (with polling fallbacks for Standalone if necessary).

**Design Doc:**
- **Trigger:** Business owner navigates to "Payment Settings" and sees regional payment options based on their business location.
- **Action:** Clicking "Connect" routes them through the provider's OAuth or API key setup. Once connected, checkout pages dynamically offer these payment methods.
- **User Interface:** A simple toggle board of payment providers. The customer-facing checkout seamlessly adapts to show local options (e.g., PIX in Brazil via Mercado Pago).

**Implementation Prompt:**
Build support for an alternative payment provider (e.g., Mercado Pago or Razorpay) to serve international markets. The business owner must be able to connect their account via the app settings. Once connected, any generated invoices or storefront checkout pages must offer this new payment method to customers alongside or instead of Stripe.

**Priority:** P1
**Estimated Scope:** Large
