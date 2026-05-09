# LATAM Payment Processing Integration

**Problem Statement:**
While Stripe is the default for US/EU, business owners in Latin America lose sales if they cannot accept localized payment methods like Pix (Brazil), Boletos, or regional credit cards.

**Research Report:**
* **Tool Evaluated:** Mercado Pago API
* **Ease of Use:** Excellent regional adoption. The checkout flow is familiar to LATAM consumers.
* **Pricing:** Varies heavily by country and payment method (typically 3-5% + flat fee).
* **Reputation:** The undisputed leader in LATAM payment processing.
* **Hybrid Context:** Fully supported via REST APIs in both Cloud and Standalone modes.

**Design Doc:**
* **Trigger:** An owner generates an invoice or a customer reaches the checkout page on an OHC storefront.
* **Action:** OHC presents Mercado Pago as a payment option, handling the localized checkout flow.
* **User Experience:** The business owner connects their Mercado Pago account in settings. When they send a payment link, their local customers see familiar options (like QR codes for Pix) and can pay instantly.

**Implementation Prompt:**
Integrate a secondary payment gateway option specifically for LATAM markets. The user should be able to authenticate with their Mercado Pago account. Update the invoice generation and storefront checkout flows to support generating localized payment links and capturing webhook events for successful payments.

**Priority:** P1
**Estimated Scope:** Medium
