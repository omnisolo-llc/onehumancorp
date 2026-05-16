# Title: Integrate EasyPost for Streamlined Shipping & Label Generation

## Problem Statement
Small e-commerce and craft businesses waste significant time manually copying customer addresses into carrier websites (like USPS or FedEx) to buy shipping labels. They need a way to automatically calculate shipping costs during checkout and print labels directly from their order management dashboard.

## Research Report
EasyPost provides a unified API for dozens of global shipping carriers.
- **Ease of Use:** For the business owner, the experience is seamless—they just click "Generate Label." Setup requires creating an EasyPost account and linking carrier accounts (or using EasyPost's default discounted USPS rates).
- **Pricing:** 120,000 free shipments per year, paying only the actual postage cost. This is an incredible value for small businesses compared to platforms that charge monthly fees for label access.
- **Reputation:** Extremely reliable API with high uptime and excellent documentation.
- **Competitors:** Shippo, ShipStation. ShipStation is a full standalone app (often too complex), while Shippo is similar to EasyPost but EasyPost's developer API and generous free tier make it ideal for a white-labeled integration.
- **Cloud vs Standalone:** Excellent in both. In Cloud, we can manage it centrally. In Standalone, users provide their own EasyPost API key.

## Design Doc
OHC will connect orders to EasyPost to allow instant label purchasing and automated customer tracking emails.
- **Trigger:** A business owner views an "Unfulfilled" order and clicks "Buy Shipping Label."
- **Action:** OHC sends package dimensions and addresses to EasyPost, displays the rate, and upon confirmation, purchases the label and stores the tracking number.
- **User Interface:** The order details view will have a "Shipping" section where the user can enter box dimensions, select a carrier rate, and download a printable PDF label. The tracking number will automatically attach to the order.

## Implementation Prompt
Create a shipping integration using EasyPost. In the settings, allow users to input an EasyPost API key. On the Order Management screen, add a workflow to purchase a shipping label. This workflow should fetch live rates based on the customer's address and the merchant's origin address, allow the merchant to select a rate, and then generate a printable PDF label. Automatically save the resulting tracking number to the order record.

## Priority
P2

## Estimated Scope
Large