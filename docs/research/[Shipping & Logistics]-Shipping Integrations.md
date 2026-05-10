# Automated Shipping Rates and Label Generation

## Problem Statement
Manually calculating shipping costs and buying labels at the post office wastes time and money. Business owners need real-time shipping rates at checkout and easy label printing.

## Research Report
Evaluated shipping integrations for real-time rates and label generation.

- **Ease of Use**: Transformative for e-commerce owners, streamlining fulfillment.
- **Pricing**: SaaS platforms charge monthly fees plus label markups.
- **Risks**: API rate limits from carriers, international customs form complexities.
- **Modes**: Cloud and Standalone applicable via integrations with aggregators like Shippo or EasyPost.

## Design Doc
When an order is placed, OHC calls a shipping aggregator API to get live rates. The business owner can click 'Generate Label' on the order page. OHC retrieves a PDF label from the API and saves the tracking number to the order.

## Implementation Prompt
Add real-time shipping rate calculation to the checkout process. Add a 'Print Label' button to the order details page that fetches and displays a printable PDF shipping label.

## Priority
P1

## Estimated Scope
Medium
