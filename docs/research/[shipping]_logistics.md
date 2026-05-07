# Shipping & Logistics Integration

## Title
Automated Shipping Label & Rate Generation (Shippo)

## Problem Statement
Small business owners selling physical goods waste hours manually calculating shipping rates at the post office and writing out shipping labels. They need a way to automatically calculate the cheapest shipping rate, generate a printable label, and provide a tracking number to the customer directly from their dashboard.

## Research Report
*   **Target Tools:** Shippo.
*   **Pros:** Aggregates multiple carriers (USPS, UPS, FedEx, DHL). Provides discounted rates out-of-the-box. Strong API for label generation and tracking.
*   **Cons:** Handling physical dimensions (weight, box size) requires strict data entry from the business owner, which can be a point of friction.
*   **Ease of Use for Non-Technical Users:** High, assuming product weights are configured. The owner just clicks "Generate Label" on an order.
*   **Pricing:** Pay-as-you-go (approx $0.05 per label) or monthly subscriptions for volume. Carrier postage fees are separate.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Excellent fit.
    *   *Standalone:* Excellent fit. Label generation is synchronous; tracking updates can be polled if webhooks aren't viable locally.

## Design Doc
1.  **Carrier Setup:** In Settings, user enables default carriers (e.g., USPS) and sets a default "box size".
2.  **Order Fulfillment:** On an "Order" page, the user sees a "Fulfill Order" button.
3.  **Label Generation:** Clicking it fetches the best rate, deducts the postage from their connected billing method, and generates a printable PDF label.
4.  **Tracking:** A tracking link is automatically emailed to the customer.

## Implementation Prompt
Build a "Shipping Fulfillment" feature. For physical orders, add a "Generate Shipping Label" button. Integrate with Shippo to calculate the cheapest USPS/UPS rate based on the destination address. Allow the user to download the generated label as a PDF for printing. Automatically extract the tracking number from the Shippo response and send an email notification to the customer with their tracking link.

## Priority
P2 (medium)

## Estimated Scope
Medium
