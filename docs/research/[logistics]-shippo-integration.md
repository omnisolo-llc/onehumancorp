# Tool Integration Research: Shippo

## Title
Integrate Shippo for Multi-Channel Shipping and Label Generation

## Problem Statement
Small business owners (like Carlos, who runs a local crafts shop, or Maya, who operates a boutique) often struggle with logistics when scaling their operations. They have to manually compare shipping rates across multiple carriers (USPS, UPS, FedEx, DHL), generate labels by typing addresses into carrier websites, and manually copy-paste tracking numbers back into their CRM or customer emails. This process is highly error-prone, consumes hours of their day, and prevents them from offering dynamic, real-time shipping rates at checkout. They need a single, unified place to purchase discounted labels, track packages, and automate shipping notifications without needing to be logistics experts.

## Research Report
Shippo is a leading multi-carrier shipping API and web application tailored heavily for e-commerce platforms and small businesses.
*   **Ease of Use:** Shippo is highly praised for its simplicity. It abstracts the complexities of individual carrier APIs into a single, cohesive interface. For non-technical users, once connected, it provides instant access to discounted rates (up to 90% off retail for USPS).
*   **Pricing:** Shippo offers a "Starter" tier that is free to use (no monthly subscription fee), where users only pay a very small per-label fee (or it's free if using default carriers). This makes it incredibly viable for small business owners with low or variable order volumes.
*   **Capabilities & Limits:** The Shippo API is extremely robust, supporting over 85 global carriers. The OAuth flow is standard and well-documented. Webhooks are reliable and can be used to push tracking updates (in-transit, delivered, etc.) back to OHC.
*   **Cloud vs. Standalone Viability:**
    *   **Cloud (Multi-tenant):** Shippo uses standard OAuth2, allowing OHC to connect multiple tenant accounts securely.
    *   **Standalone (Local):** Small business owners can still use Shippo in standalone mode by generating a personal API token in their Shippo dashboard and supplying it to their local OHC instance.

## Design Doc
1.  **Trigger:** When a new order is marked as "Ready to Ship" in OHC, or when a business owner explicitly clicks a "Generate Shipping Label" button on an order page.
2.  **Integration Flow:**
    *   **Authentication:** The user connects their Shippo account via OAuth (Cloud) or provides an API key (Standalone).
    *   **Action:** OHC sends the customer's shipping address, the origin address, and package dimensions/weight to Shippo.
    *   **User Sees:** A modal inside OHC displaying rates from various carriers (e.g., USPS Priority, UPS Ground).
    *   **Selection & Purchase:** The user selects a rate and clicks "Buy Label".
    *   **Result:** OHC retrieves the generated PDF label from Shippo for the user to print. The tracking number is automatically saved to the order in OHC.
3.  **Webhook Integration:** Shippo sends webhook events to OHC when the package status changes. OHC updates the order status internally, which can trigger an automated email or SMS to the customer.

## Implementation Prompt
Implement a shipping integration using Shippo. The user should be able to connect their Shippo account from the OHC integrations dashboard. Once connected, add a "Create Shipping Label" button to the order detail view. Clicking this should open a flow where the user can input package weight/dimensions (or use defaults), view a list of real-time shipping rates from their carriers, and purchase a label. Once purchased, the system should display a link to print the label (PDF) and automatically attach the tracking number to the order, changing the order status to "Shipped". Ensure the integration supports both an OAuth flow for cloud users and an API Key input for standalone users. Do not worry about building the actual email notifications yet, just ensure the tracking number is stored.

## Priority
P1

## Estimated Scope
Medium
