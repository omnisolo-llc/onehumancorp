# Title: Automate Shipping Labels and Tracking via Shippo

## Problem Statement
Business owners selling physical goods waste time manually copying customer addresses into carrier websites (USPS, FedEx, DHL) to buy shipping labels. They need a 1-click way to buy a label and automatically send tracking info to the customer.

## Research Report
- **Tool Evaluated:** Shippo
- **Benefits:** Aggregates dozens of carriers globally into a single API. Often provides discounted shipping rates.
- **Ease of Use:** Extremely high. Abstract away the carrier complexity.
- **Pricing:** Pay-as-you-go per label generated.
- **Cloud/Standalone:** API-based, works in both modes.

## Design Doc
1. **Trigger:** A customer places an order requiring physical shipping.
2. **Action:** The business owner views the order in OHC and clicks "Generate Shipping Label".
3. **UI Outcome:** OHC fetches the best rates from Shippo, the owner selects a carrier/speed, and a printable PDF label is instantly generated. Tracking information is automatically emailed to the customer.

## Implementation Prompt
Integrate Shippo to enable 1-click shipping label generation directly from the Orders dashboard. When an order is viewed, display real-time shipping rates based on the package weight and destination. Allow the user to purchase and print the label, and automatically update the order status to "Shipped" with the tracking link.

## Priority
P2

## Estimated Scope
Large
