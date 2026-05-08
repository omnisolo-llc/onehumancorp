# Title: Real-time Shipping Rates and Labels via EasyPost
## Problem Statement
Physical product sellers spend hours manually calculating shipping costs for customers and standing in line at the post office to buy labels. They need accurate, automated shipping costs at checkout and instant label printing.

## Research Report
* **Tool:** EasyPost API
* **What it does:** Aggregates USPS, UPS, FedEx, DHL, and international carriers for rating and label generation.
* **Ease of Use for Owners:** High. EasyPost simplifies managing multiple carriers.
* **Pricing:** Free for up to 120,000 shipments per year.
* **Cloud vs. Standalone:** Works smoothly in both environments using API keys.

## Design Doc
* **Trigger:** Customer enters shipping address at checkout; owner clicks "Generate Label" in OHC admin.
* **Action:** OHC fetches live rates during checkout. For fulfilling orders, OHC requests a PDF shipping label and tracking number.
* **User Experience:** Customers see exact shipping costs rather than flat rates. Owners can print shipping labels directly from their OHC dashboard and automatically email tracking numbers to customers.

## Implementation Prompt
Integrate EasyPost to automate shipping workflows. The business owner should be able to enter their package dimensions and weight on a product. The acceptance criteria is that a customer sees live shipping rates at checkout based on their address, and the owner can click a single button to generate and download a PDF shipping label for the paid order.

## Priority
P1

## Estimated Scope
Large
