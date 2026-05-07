# ShipStation Logistics and Fulfillment Integration

## Problem Statement
Small business owners selling physical goods struggle with the manual process of calculating shipping costs, buying postage, and printing labels. Copy-pasting addresses into different carrier websites causes errors, and failing to provide tracking numbers to customers results in support emails. They need a simple way to click "Ship" and get a label instantly.

## Research Report
ShipStation is a dominant platform for e-commerce fulfillment and shipping automation.
- **Ease of Use**: Designed specifically to aggregate orders and streamline label printing for small to medium merchants.
- **Capabilities**: Integrates with dozens of carriers (USPS, UPS, FedEx, DHL), calculates rates in real-time, generates printable labels, and automates customer tracking emails.
- **Competitors**: EasyPost, Shippo. ShipStation is more of a complete, standalone user-facing app for the merchant, whereas EasyPost is more developer-focused.
- **Reputation**: High industry standard, though some users note the UI can be busy.
- **Pricing**: Plans start at $9.99/month for 50 shipments, which is affordable for new businesses.
- **Deployment**: Provides strong APIs and webhooks. Compatible with both Cloud and Standalone environments.

## Design Doc
OHC will integrate with ShipStation by syncing orders. When an order is placed on the OHC storefront, it is pushed to the merchant's ShipStation account. The merchant uses ShipStation to buy the label and print it. ShipStation then sends a webhook back to OHC containing the tracking number. OHC updates the order status to "Shipped" and displays the tracking number to both the merchant and the customer.

## Implementation Prompt
Add a "Fulfillment" setting where users can connect their ShipStation account via an API key or OAuth. When a new physical product order is placed, automatically forward the order details (items, shipping address) to ShipStation. Listen for shipping confirmation events from ShipStation. When received, update the order in OHC to "Shipped" and display the tracking number clearly in the order details view on the merchant dashboard.

## Priority
P1

## Estimated Scope
Medium
