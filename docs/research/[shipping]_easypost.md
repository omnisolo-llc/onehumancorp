# Integrate EasyPost for Simplified Shipping Logistics

## Problem Statement
Small business owners who sell physical goods struggle with the logistics of shipping. Manually calculating shipping rates, going to the post office, and typing out tracking numbers for customers is incredibly time-consuming and error-prone. They need a system that calculates shipping automatically and prints labels directly from their dashboard.

## Research Report
*   **Tool:** EasyPost (or similar tools like Shippo/ShipEngine)
*   **Problem Solved:** Connects to multiple carriers (USPS, FedEx, UPS, DHL) to compare rates, generate shipping labels, and track packages.
*   **Ease of Use:** High for the owner. They just enter box dimensions and weight, and the system handles the rest.
*   **Pricing:** EasyPost offers a developer tier (free up to 120,000 shipments/year); users only pay postage costs.
*   **Reputation:** Highly reliable API used by many major e-commerce platforms.
*   **Environment:** Works seamlessly in both Cloud and Standalone modes.
*   **Advantages:** Instantly provides the cheapest shipping rate across multiple carriers; automates tracking updates for customers.
*   **Risks:** Requires accurate weight/dimension inputs from the owner, which they might get wrong initially, leading to postage adjustments.

## Design Doc
1.  **Trigger:** An order is marked as "Ready to Ship" in the OHC dashboard.
2.  **Action:** The user enters the package weight and dimensions (or selects a pre-saved box size). OHC fetches rates from connected carriers. The user clicks "Buy Label".
3.  **User Interface:** A printable PDF shipping label is generated. The order status automatically updates to "Shipped", and OHC emails the tracking number to the customer.
4.  **Tracking:** The dashboard displays a small progress bar for the shipment (e.g., "In Transit", "Out for Delivery", "Delivered") based on webhook updates from the carrier.

## Implementation Prompt
Build a streamlined shipping label generation tool for physical product orders. When an order needs to be fulfilled, allow the business owner to input package details and instantly view shipping rates from major carriers. Enable them to purchase and generate a printable PDF shipping label with one click. Upon generating the label, the system must automatically update the order status, attach the tracking number, and send an automated notification to the customer with a link to track their package.

## Priority
P2

## Estimated Scope
Medium
