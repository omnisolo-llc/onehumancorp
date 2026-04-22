# [Payment Processing] Integrate Mercado Pago for LATAM Markets

## Problem Statement
While Stripe is powerful, it is not available or widely adopted in many Latin American countries. Business owners in these regions need localized payment methods (like Pix in Brazil or cash payments via OXXO in Mexico) to successfully convert sales. Without local payment options, they cannot accept online payments effectively.

## Research Report
**Tool Analyzed:** Mercado Pago (Leading LATAM Payment Gateway)

*   **Capabilities:** Processes credit cards, debit cards, Pix (instant payments), and local cash networks (Boleto, OXXO) across Latin America.
*   **Ease of Use (for Non-Technical Users):** Similar to Stripe for the merchant. They authenticate via an OAuth-style flow or API keys to connect their OHC store to their Mercado Pago wallet.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Webhooks and REST APIs available for SaaS integration.
    *   *Standalone:* Cannot be self-hosted, relies on cloud APIs.
*   **Pricing:** Transaction fees vary by country and payment method (e.g., usually around 3-5% + fixed fee). No setup costs.
*   **Reputation:** The dominant and most trusted digital wallet and payment processor in Latin America.

## Design Doc
**Integration with OHC:**
*   **Trigger:** User selects "Mercado Pago" in the "Finance & Payments" settings.
*   **Action:** OHC initiates the Mercado Pago Connect OAuth flow. Once authorized, checkout sessions dynamically display localized payment options based on the buyer's region.
*   **User Interface:** The storefront checkout automatically adds Pix or local card options if the store currency is BRL, MXN, ARS, etc. The business owner sees unified revenue reports in the OHC app, regardless of the gateway used.
*   **AI Agent Synergy:** "The Accountant" reconciles Mercado Pago settlements alongside any other revenue streams and reports total balances accurately.

## Implementation Prompt
Add Mercado Pago as an alternative payment gateway to Stripe.
1.  Implement the Mercado Pago checkout flow for the public storefront.
2.  Add a connection settings page for merchants to link their Mercado Pago accounts.
3.  Ensure the order management system handles asynchronous payment states (e.g., a customer selecting a cash payment method like OXXO that pends for 24 hours).
4.  Abstract the revenue reporting so the owner sees a unified total across all enabled gateways.

## Priority
P1 (High) - Necessary for global expansion, especially in LATAM.

## Estimated Scope
Medium
