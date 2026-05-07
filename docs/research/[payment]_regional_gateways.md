# [Payment] Universal Payments with Mercado Pago & Alipay

**Title**: Implement Alternative Payment Gateways for Global Reach

**Problem Statement**:
Small business owners outside of the US and Europe often cannot use Stripe or find its fees too high. They need local payment processing options that their customers trust and already use, such as Mercado Pago in LATAM or Alipay in Asia.

**Research Report**:
- **Evaluated Tools**: Stripe (Baseline), Mercado Pago, Alipay, Razorpay.
- **Findings**: While Stripe is the default, relying solely on it excludes large portions of the global market. Mercado Pago dominates Latin America, and Alipay is essential for the Chinese market. Razorpay is the standard for India. Adding regional leaders is critical for adoption.
- **Ease of Use**: These platforms provide standard checkout pages that handle the heavy lifting of security and local currency conversions, meaning the small business owner just needs to connect their account.
- **Pricing**: Varies wildly by region but generally highly competitive within their local markets.
- **Cloud vs Standalone**: Works in both. Keys can be managed globally per tenant in Cloud, or stored locally in Standalone.

**Design Doc**:
- **Trigger**: The user configures their "Store Location" or "Payment Methods" in the OHC settings.
- **Action**: OHC determines the best supported regional payment provider (e.g., Mercado Pago for Brazil) and prompts the user to authenticate or enter their API credentials.
- **User View**: When generating an invoice or checkout link, the customer sees familiar local payment options. The business owner sees a unified "Payments Received" dashboard regardless of the gateway used.

**Implementation Prompt**:
Build a pluggable payment architecture that supports Stripe alongside at least one regional alternative (e.g., Mercado Pago). The user should be able to select their preferred payment provider in the settings. When they create an invoice, it should generate a checkout link using the configured provider. All successful payments must be recorded and displayed in a unified "Payments" view in the OHC app.

**Priority**: P1
**Estimated Scope**: Large
