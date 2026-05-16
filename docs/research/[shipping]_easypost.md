# Title: Multi-Carrier Shipping & Label Generation

## Problem Statement
Small business owners selling physical goods waste significant time manually copying customer addresses into different carrier websites (USPS, FedEx, UPS) to compare rates, buy labels, and then copy tracking numbers back to the customer. They need an automated way to calculate rates at checkout and print labels with one click.

## Research Report
**Tool Analyzed**: EasyPost
**Ease of Use**: Excellent for developers, and translates to a very seamless experience for the end-user. The business owner just needs to provide package dimensions and weight.
**Reputation**: Highly reliable API, used by major e-commerce platforms. Excellent uptime.
**Pricing**: Free up to 120,000 shipments per year (developer plan), which more than covers the average small business. They just pay the actual carrier postage rates.
**Environment**: Cloud API. Can be utilized from a Standalone environment via external API calls.
**AI Integration**: AI could predict package sizes based on historical order data to pre-fill dimensions, saving the owner even more time.

## Design Doc
**Integration Trigger**: User configures their "Ship From" address and default package sizes in OHC.
**Actions Taken**:
- At checkout, OHC calls EasyPost to fetch live shipping rates based on the cart contents and customer address.
- When fulfilling an order, the owner clicks "Buy Label". OHC calls EasyPost to purchase the postage.
- A PDF label is generated for printing, and a tracking number is automatically emailed to the customer.
**User View**: The owner sees a "Fulfill Order" button. Clicking it shows shipping options. They select one, click "Print Label", and the tracking info is automatically updated.

## Implementation Prompt
Integrate EasyPost for shipping automation. Add a UI for the owner to input their origin address and default box sizes. Modify the checkout flow to dynamically display shipping rates from EasyPost. In the order management view, add a flow to purchase and print a shipping label, automatically attaching the resulting tracking number to the order and triggering a dispatch notification to the customer.

## Priority
P1

## Estimated Scope
Large
