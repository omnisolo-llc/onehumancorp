# [Shipping & Logistics] Integrate EasyPost for Automated Shipping

## Problem Statement
Sellers of physical products (like handmade jewelry or art prints) waste hours manually calculating shipping costs at the post office or copy-pasting addresses into separate carrier websites to print labels. They need an automated way to charge customers the correct shipping rate at checkout and print labels with one click from their phone.

## Research Report
**Tool Analyzed:** EasyPost (Multi-carrier Shipping API)

*   **Capabilities:** Real-time shipping rates, label generation, address verification, and tracking across 100+ carriers (USPS, FedEx, UPS, DHL, etc.).
*   **Ease of Use (for Non-Technical Users):** Invisible to the user. They simply input package dimensions/weight in OHC, and OHC handles the API calls to EasyPost to get rates and buy labels.
*   **Cloud vs. Standalone:**
    *   *Cloud:* Standard REST API integration.
    *   *Standalone:* Requires cloud API access; cannot be self-hosted.
*   **Pricing:** 120,000 free shipments per year for the Developer plan. 1¢ per label after. Very affordable.
*   **Reputation:** The industry standard for modern shipping APIs, known for high uptime and simplifying complex carrier integrations.

## Design Doc
**Integration with OHC:**
*   **Trigger:** Customer reaches checkout (rate calculation) OR Business owner clicks "Fulfill Order" (label purchase).
*   **Action:** At checkout, OHC calls EasyPost to fetch live rates based on product weight. When fulfilling, OHC buys the label via EasyPost and saves the tracking URL.
*   **User Interface:** The owner sees a "Print Shipping Label" button on the order details screen in the OHC app. Clicking it generates a PDF they can send to their printer.
*   **AI Agent Synergy:** "The Operations Manager" monitors tracking statuses via EasyPost webhooks and triggers "The Ambassador" to email the customer when the package is out for delivery.

## Implementation Prompt
Integrate EasyPost to automate shipping logistics for physical products.
1.  Add weight and dimension fields to the physical product setup flow.
2.  During customer checkout, fetch and display dynamic shipping rates based on the delivery address.
3.  Add a "Purchase & Print Label" button to the order management screen for the owner.
4.  Automatically save the tracking number and update the order status to "Shipped".

## Priority
P1 (High) - Critical for any user selling physical goods online.

## Estimated Scope
Large
