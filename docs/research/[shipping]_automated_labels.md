# Shipping & Logistics: Automated Labels

## Problem Statement
Manually calculating shipping costs and buying labels at the post office is incredibly time-consuming for product-based businesses.

## Research Report
**Selected Tool:** Shippo
Shippo abstracts the complexity of connecting to dozens of carriers globally into a single API.
- **Ease of use for non-technical users:** OHC will abstract the carrier selection; owners just see "Buy Label" and print it.
- **Pricing:** Pay-as-you-go model is very friendly for small volume sellers.
- **Reputation:** Highly reliable, extensive carrier network.

## Design Doc
**Integration with OHC:**
- **Trigger:** An order is marked as "Ready to Ship".
- **Action:** OHC calls Shippo API to generate a label based on predefined package sizes and customer address.
- **User Interface:** A "Purchase Shipping Label" button on the order details page. Shows the estimated cost before purchase.
- **Environment:** Cloud and Standalone.

## Implementation Prompt
**User-Facing Outcome:** The business owner can click one button to buy and print a shipping label for an order without leaving OHC.
**Acceptance Criteria:**
- Setup flow to enter default package dimensions and origin address.
- Ability to generate and download a PDF shipping label.
- Automatic tracking number generation and attachment to the order.

## Priority
P2

## Estimated Scope
Large
