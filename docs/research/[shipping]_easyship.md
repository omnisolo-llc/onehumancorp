# Easyship Shipping Integration

## Problem Statement
Small business owners selling physical goods struggle with calculating accurate shipping rates, printing labels, and providing tracking numbers to customers, especially for international orders. They need an automated logistics tool built into their storefront.

## Research Report
Easyship is a cloud-based shipping software that connects sellers with couriers globally.
- **Ease of Use**: Very user-friendly dashboard. It abstracts the complexity of dealing with multiple couriers into a single platform.
- **Pricing**: Free tier available (up to 50 shipments/month). Pay only for the shipping costs.
- **Reputation**: Highly rated for its global courier network and transparent pricing.
- **Environment**: Cloud API. Can be integrated into both Cloud and Standalone modes (needs internet).

## Design Doc
**Trigger**: Business owner receives a new order in the OHC dashboard.
**Action**: Business owner clicks "Fulfill Order". OHC pings Easyship API for rates, generates a label, and gets a tracking number.
**User Experience**: The business owner sees shipping options, clicks to buy a label, and prints it directly. The customer automatically receives an email with the tracking link.

## Implementation Prompt
Integrate Easyship to handle shipping fulfillment. When an order is placed, fetch and display shipping rates. Allow the business owner to purchase and print shipping labels directly from the order details page in the OHC dashboard. Automatically update the order status with the tracking number.

## Priority
P1

## Estimated Scope
Medium
