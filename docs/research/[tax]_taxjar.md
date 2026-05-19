# 🔍 Scout: Tool Integration Research - TaxJar (Stripe Tax)

## Title
Integrate TaxJar (Stripe Tax) for Automated Sales Tax Calculation and Filing

## Problem Statement
Small business owners, especially those in e-commerce, struggle significantly with navigating complex, ever-changing sales tax laws across different jurisdictions (nexus). Calculating the correct tax rate at checkout, tracking when they cross nexus thresholds, and manually filing returns across multiple states or countries is a massive administrative burden and a significant legal risk. They need a system that automatically calculates the right tax at the point of sale and simplifies the reporting and filing process, freeing them up to focus on growing their business rather than acting as a tax compliance officer.

## Research Report
*   **Tool:** TaxJar (Now part of Stripe as Stripe Tax)
*   **Market Position:** A leading solution for automated sales tax compliance, heavily favored by small to medium e-commerce businesses due to its ease of use and direct integrations with major platforms (Shopify, WooCommerce, Amazon).
*   **Capabilities & Limits:**
    *   **Auto-Rates:** Provides real-time sales tax calculations at checkout based on precise customer location.
    *   **Nexus Tracking:** Monitors sales and transactions to alert business owners when they are approaching or have exceeded economic nexus thresholds in different states.
    *   **Automated Filing (AutoFile):** Can automatically submit returns and remit payments to state jurisdictions.
    *   **API Quality:** Robust, well-documented RESTful API. Offers sandbox environments for testing. Webhooks are available for synchronization.
*   **SaaS Viability & Pricing:**
    *   **Pricing Model:** Typically tiered based on the number of transactions per month, making it accessible for early-stage small businesses while scaling with their growth. Stripe Tax offers pay-as-you-go pricing (e.g., a small percentage per transaction).
    *   **Modes:** Highly viable for Cloud (multi-tenant) environments where the OHC platform acts as the intermediary. For Standalone (local, private) modes, businesses would need to provide their own API keys, but the integration logic remains the same.
*   **Reputation & Ease of Use:** Excellent reputation for simplifying a complex process. The user interface for non-technical users (the dashboard showing where they owe tax) is highly praised for clarity.

## Design Doc
*   **Trigger:** The integration is primarily triggered during the checkout/invoicing flow. When an order is being finalized, OHC sends the transaction details (origin address, destination address, line items) to TaxJar.
*   **Action:** TaxJar returns the precise sales tax amount to be collected. OHC adds this to the order total. Post-transaction, OHC securely syncs completed order data to TaxJar for nexus tracking and future filing.
*   **User Experience (OHC Dashboard):**
    *   A "Tax & Compliance" section where the user connects their TaxJar/Stripe account.
    *   A visual widget showing their current nexus status (e.g., "You are nearing the threshold in California").
    *   A toggle to enable "Automated Tax Calculation at Checkout."
    *   The business owner sees simple summaries, while the complex routing happens invisibly.

## Implementation Prompt
Implement an integration with the TaxJar (Stripe Tax) API to provide automated sales tax calculations for OHC merchants.
*   **Acceptance Criteria 1 (Connection):** A merchant can securely connect their existing TaxJar account to their OHC profile (via API key or OAuth).
*   **Acceptance Criteria 2 (Real-time Calculation):** During the checkout or invoice generation process, the system must accurately fetch and apply the correct sales tax rate based on the buyer's shipping address and the merchant's nexus profile.
*   **Acceptance Criteria 3 (Sync):** Completed orders must automatically sync to the merchant's TaxJar account to ensure their reporting dashboard is up-to-date for filing.
*   **Acceptance Criteria 4 (Visibility):** The OHC dashboard should display a basic summary of the merchant's active tax jurisdictions (nexus) pulled from the integration.

## Priority
P1 (High) - Compliance is a critical pain point that blocks growth for SMBs. Solving this builds immense trust.

## Estimated Scope
Medium