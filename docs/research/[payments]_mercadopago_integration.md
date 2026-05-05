# Title
Payment Processing: Mercado Pago for LATAM Market

# Problem Statement
While Stripe covers North America and Europe well, business owners in Latin America need local payment methods (like PIX in Brazil, or OXXO in Mexico). Without local payment support, OHC cannot serve the massive SMB market in LATAM effectively.

# Research Report
**Tool Analyzed:** Mercado Pago
Mercado Pago is the leading payment processor in Latin America, offering a wide array of local payment methods.
- **Ease of Use (for non-technical users):** The integration for the merchant is similar to Stripe (OAuth flow). The checkout experience for customers is familiar and trusted in LATAM.
- **Pricing:** Variable by country, but standard for the region.
- **Reputation:** The undisputed leader in LATAM payments.
- **Integration Risk:** The API is sometimes less predictable than Stripe's, and webhook delays can occur. Rigorous idempotency and background job retries are required.
- **Cloud/Standalone:** Fits perfectly into a Cloud SaaS model.

# Design Doc
- **Trigger:** A user in a LATAM country signs up and navigates to the "Finance & Payments" settings.
- **Actions:**
  1. User selects Mercado Pago and completes the OAuth connection flow.
  2. During checkout on the storefront, OHC detects the region and renders the Mercado Pago Checkout Pro or transparent checkout component.
  3. Customer pays using local methods (e.g., PIX).
  4. Mercado Pago sends an IPN (Instant Payment Notification) webhook.
  5. OHC verifies the signature, updates the order status, and triggers fulfillment.
- **User Experience:** The merchant sees a simple "Connect Mercado Pago" button. The buyer sees familiar, local payment options natively embedded in the checkout flow.

# Implementation Prompt
Integrate Mercado Pago as an alternative payment gateway alongside Stripe, targeting LATAM users. Implement the OAuth onboarding flow for merchants and the checkout integration for buyers. Acceptance criteria include successful processing of a test payment via PIX (or equivalent local method), robust webhook handling for asynchronous payment confirmation, and clear UI switching between Stripe/Mercado Pago based on merchant configuration.

# Priority
P2

# Estimated Scope
Large
