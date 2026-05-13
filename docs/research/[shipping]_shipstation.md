# Title: ShipStation Integration for Automated Fulfillment

## Problem Statement
E-commerce businesses waste hours manually typing customer addresses into carrier websites (USPS, FedEx) to buy shipping labels. They need a system that automatically imports orders, calculates shipping rates, and prints labels in bulk. For a non-technical user, logistics is a nightmare of spreadsheets and manual data entry.

## Research Report
**Market Analysis & Pain Points:**
- **Friction:** Copy-pasting addresses leads to typos, misdeliveries, and wasted time.
- **Competitors:** Shippo, EasyPost. ShipStation is the industry leader for SMBs due to its vast carrier network and user-friendly web interface.
- **ShipStation API:** Extremely comprehensive. Allows creating orders, fetching rates, and generating label PDFs.
- **Reputation & Ease of Use:** Very high among e-commerce operators.
- **Pricing:** Starts at $9.99/month.

**Key Advantages:**
- Solves the physical fulfillment problem, which is the biggest bottleneck for product-based businesses.
- Access to heavily discounted USPS/UPS rates.

**Integration Risks:**
- Handling international customs declarations via API can be complex.
- Managing partial fulfillments or split shipments.

**Environment Support:**
- **Cloud:** Full support.
- **Standalone:** Full support. Particularly useful for standalone as it can interface with local thermal printers.

## Design Doc
**Trigger:**
User connects ShipStation via API key in the "Shipping" settings.

**Action:**
When an order is placed on an OHC storefront, OHC automatically pushes the order details to ShipStation. Once the user prints the label in ShipStation, ShipStation sends a webhook back to OHC with the tracking number.

**User View:**
The business owner manages their orders in OHC. When an order is "Ready to Ship", it automatically appears in their ShipStation account. Once shipped, the OHC order status automatically changes to "Fulfilled" and the tracking number appears on the order page, ready to be sent to the customer.

## Implementation Prompt
Integrate ShipStation for seamless order fulfillment and tracking.
- Build an automatic sync that pushes new 'Paid' OHC orders to the ShipStation API.
- Implement a webhook receiver to listen for ShipStation 'Shipment Created' events.
- Update the OHC order status to 'Fulfilled' and attach the tracking number automatically when the webhook is received.
- Display a quick link to the ShipStation order directly from the OHC order detail view.
- (Focus on the user flow of orders moving from Paid to Fulfilled without prescribing internal state machine mechanics.)

## Priority
P1

## Estimated Scope
Medium
