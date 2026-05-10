# Shipping & Logistics: Shippo

## Problem Statement
E-commerce business owners hate going to the post office to figure out shipping costs. They want to automatically charge the customer the exact shipping rate and print the label from their desk.

## Research Report
Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) into one API.
- *Ease of Use*: Excellent for the developer, abstracts away carrier specifics.
- *Pricing*: Pay-as-you-go (5¢ per label) or monthly subscription. Often provides discounted USPS rates.
- *Reputation*: Very reliable, great documentation.

## Design Doc
- *Trigger*: Customer enters their address at checkout (fetches live rates). Business owner clicks "Fulfill Order" in OHC dashboard.
- *Action*: OHC calls Shippo to get rates during checkout. During fulfillment, OHC calls Shippo to purchase a label and generates a PDF for the owner to print.
- *User Interface*: In the Order details view, a "Buy & Print Shipping Label" button. A modal shows carrier options and prices.

## Implementation Prompt
Integrate Shippo to handle real-time shipping rates at checkout and label generation in the admin dashboard. The business owner should be able to view an order, select a box size, see rate quotes from USPS/UPS, purchase the label, and download the PDF directly within OHC.

## Priority
P2

## Estimated Scope
Large

## Environment Support
Cloud, Standalone.
