# Shipping & Logistics - Shippo

## Problem Statement
Sellers of physical goods struggle to figure out shipping costs, print labels, and provide tracking numbers to customers. Going to the post office is time-consuming.

## Research Report
Shippo aggregates multiple carriers (USPS, UPS, FedEx, DHL) and provides discounted rates.
- **Ease of Use**: API is clean. The user dashboard is straightforward.
- **Pricing**: Pay-as-you-go (5¢ per label + postage) or Pro at $10/month.
- **Reputation**: Very reliable for small to medium e-commerce.
- **Cloud/Standalone**: Cloud API.

## Design Doc
- **Trigger**: An order for a physical product is placed.
- **Action**: OHC fetches shipping rates from Shippo. When the user fulfills the order, OHC generates a label via Shippo API.
- **User View**: Business owner sees a "Print Shipping Label" button on the order details page. Clicking it deducts the postage cost and downloads a PDF label. Tracking info is auto-emailed to the customer.

## Implementation Prompt
Integrate Shippo for physical product fulfillment. Allow users to purchase and print shipping labels directly from the OHC order management screen. Automatically update the order with tracking information.
- Acceptance Criteria: System calculates shipping rates at checkout. User can click "Print Label" to generate a Shippo label. Customer receives tracking email.

## Priority
P1

## Estimated Scope
Large

---
