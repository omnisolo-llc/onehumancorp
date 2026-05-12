# OHC Tool Integration: Shippo for Automated Logistics

## Title
Implement Shippo Integration for Real-Time Rates and Labels

## Problem Statement
Manual shipping calculation leads to lost revenue (undercharging) or lost sales (overcharging/cart abandonment). Business owners spend too much time manually copying addresses into carrier websites to print labels.

## Research Report
- **Tool Evaluated:** Shippo
- **Why Shippo?** Aggregates major carriers (USPS, UPS, FedEx, DHL) into a single API. Simplifies international shipping documentation.
- **Ease of Use:** Business owners input package dimensions and weight; the system handles the rest. Customers see accurate shipping costs at checkout.
- **Pricing:** Pay-as-you-go per label or monthly plans for higher volume. Very transparent.
- **Reputation:** Reliable, widely used in e-commerce, strong API uptime.

## Design Doc
- **Trigger:** (1) Customer reaches checkout; (2) Business owner marks an order as "Ready to Ship".
- **Action:** (1) OHC queries Shippo API with origin, destination, and item weights to return live rates. (2) OHC requests a shipping label from Shippo for the selected rate and saves the PDF.
- **User View:** Customers see exact shipping costs before paying. Business owners see a "Print Label" button directly on the Order Details page.

## Implementation Prompt
Build the Shippo integration. The storefront checkout must display real-time shipping rates based on the cart's contents and the business's location. In the merchant dashboard, orders requiring shipping must have a one-click "Generate Label" feature that retrieves and stores a printable shipping label via the Shippo API, automatically updating the order status to "Shipped" with a tracking number.

## Priority
P1

## Estimated Scope
Medium
