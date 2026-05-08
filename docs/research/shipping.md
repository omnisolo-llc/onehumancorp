# [Shipping] Automated Rate Calculation and Label Generation

## Problem Statement
Small e-commerce businesses spend hours manually copying customer addresses into carrier websites, guessing shipping rates, and manually emailing tracking numbers back to customers. This process is prone to errors and scales very poorly.

## Research Report
**Tools Evaluated:** Shippo, EasyPost

*   **Ease of Use:** For the business owner, the experience is transformative. They go from manual data entry to a one-click label generation. The initial setup requires creating an account and linking carriers, but platforms like Shippo offer built-in default carrier accounts with discounted rates, lowering the barrier.
*   **Pricing:** Very friendly for small businesses. Often pay-as-you-go (e.g., ~$0.05 per label) plus the actual postage cost. Some offer free tiers for low volume.
*   **Reputation:** Shippo and EasyPost are highly regarded, stable, and widely used by major e-commerce platforms.

## Design Doc
**Trigger:** A physical product order is placed, or the user manually creates a shipment in OHC.
**Action:** OHC queries the shipping API for live rates and allows the user to purchase a label.
**User Sees:** An "Orders/Fulfillment" view. When clicking an order, they see the destination address (auto-validated). They can select box dimensions and instantly see rates from USPS, FedEx, UPS, etc. Clicking "Buy Label" generates a printable PDF and automatically triggers a tracking email to the customer.

## Implementation Prompt
Integrate a shipping API (e.g., Shippo) to provide fulfillment capabilities. The feature must include address validation, real-time rate fetching, and label purchase/generation. Include a simple way for the user to select the label format (standard PDF vs thermal printer). When a label is generated, the system should automatically save the tracking number to the order record.

## Priority
P2

## Estimated Scope
Medium

## Mode Compatibility
*   **Cloud:** Fully supported.
*   **Standalone:** Fully supported. Generating labels is an outbound API request. Tracking updates might require the standalone app to actively poll the API if it cannot receive webhooks, but the core label generation works perfectly.
