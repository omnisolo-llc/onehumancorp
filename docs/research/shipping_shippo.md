# Shippo Integration for Automated Shipping Labels

## Problem Statement
Sellers of physical products waste countless hours manually copying customer addresses into carrier websites (USPS, FedEx) to buy shipping labels, then manually copying tracking numbers back to the customer.

## Research Report
- **Tool**: Shippo API
- **Evaluation**: Shippo aggregates dozens of shipping carriers globally into a single API. It handles address validation, real-time rate calculation, and label generation.
- **Ease of Use for Persona**: The owner just taps "Buy Label" on an order. They don't need to negotiate rates with carriers.
- **Pricing**: 5 cents per label, plus postage. Very affordable for small businesses.
- **Reputation**: Reliable, wide carrier support, excellent API documentation.

## Design Doc
- **Integration Point**: "Operations" department.
- **Trigger**: User views a physical product order and taps "Fulfill".
- **Actions**:
  - OHC calls Shippo to get live rates for the package dimensions.
  - User selects a rate. OHC purchases the label via Shippo.
  - Shippo returns a PDF label URL and tracking number.
  - "Customer Success" agent auto-emails the tracking link to the customer.
- **User View**: A "Buy Shipping Label" flow directly inside the order details page. A button to print the generated label from their phone or computer.

## Implementation Prompt
Add a shipping workflow to the Order Details page. Integrate the Shippo API to fetch real-time shipping rates based on order weight/dimensions. Allow the user to purchase a label, download the PDF for printing, and automatically send the generated tracking number to the customer.

## Priority
P1

## Estimated Scope
Large
