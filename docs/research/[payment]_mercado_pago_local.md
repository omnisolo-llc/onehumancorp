# Title
Native Integration of Local Payment Methods (Mercado Pago)

# Problem Statement
Small business owners in Latin America cannot easily use Stripe and need a trusted local payment processor to accept credit cards and local methods like Pix or Pago Fácil natively within the OHC platform, avoiding complex third-party payment routing.

# Research Report
- **Tool:** Mercado Pago API.
- **Target Persona:** Global users outside the US/EU (e.g., LATAM).
- **Advantages:** Native integration within the OHC platform ensures a seamless onboarding experience without requiring the merchant to navigate complex third-party tools. Localized checkout UI components (Checkout Pro/API) that consumers trust.
- **Risks:** Settlement times can be longer. API is slightly less standardized than Stripe.
- **Pricing:** Standard transaction fees apply; merchants expect these.
- **Compatibility:** Cloud (Centralized webhook handling). Standalone (Requires a webhook relay or polling mechanism, similar to the Stripe integration).

# Design Doc
- **Integration Trigger:** User goes to Finance & Payments settings, sets their region to a LATAM country, and selects "Mercado Pago" as their payment provider.
- **User Flow:** User authenticates via Mercado Pago OAuth to link their seller account.
- **Action Flow:** At checkout, LATAM customers are presented with the Mercado Pago checkout flow (supporting local methods like Pix or OXXO). Upon successful payment, Mercado Pago sends a webhook to OHC, which triggers the Operations department to mark the order as paid and notify the business owner.

# Implementation Prompt
Integrate Mercado Pago as an alternative native payment provider. The checkout flow must dynamically switch to the appropriate provider based on the merchant's settings. Webhooks must normalize into standard OHC order fulfillment events. Ensure support for local payment methods like Pix (Brazil) and OXXO (Mexico) within the checkout flow.
- **Acceptance Criteria:** Merchant in a supported region can connect Mercado Pago natively. Customers can checkout using local methods. Orders are marked paid upon successful webhook receipt.
- **Priority:** P1
- **Estimated Scope:** Large
