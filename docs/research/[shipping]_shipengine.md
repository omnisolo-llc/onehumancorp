# [Shipping & Logistics] ShipEngine Integration

## Title
ShipEngine Integration for Automated Shipping Rates and Labels

## Problem Statement
David the Craftsman spends hours manually entering addresses into different carrier websites to find the cheapest shipping rate for his handmade goods. He needs a single tool within OHC to compare rates and print labels instantly.

## Research Report
- **Strategy**: Integration with ShipEngine API.
- **Advantages**: One API connects to dozens of carriers globally (USPS, UPS, FedEx, DHL, etc.). Excellent documentation and reliability.
- **Risks**: Account approval processes. Handling complex edge cases like customs forms for international shipping.
- **Pricing**: Pay-as-you-go based on label volume; very transparent.
- **Ease of Use**: Highly abstracted API simplifies a complex domain.
- **Compatibility**: Fully compatible with Cloud and Standalone modes.

## Design Doc
- User connects their carrier accounts via the ShipEngine dashboard.
- OHC calls ShipEngine to display real-time shipping rates at checkout.
- When an order is fulfilled, OHC requests a shipping label and tracking number from ShipEngine.
- The label is presented to the user for printing.

## Implementation Prompt
Integrate ShipEngine API to fetch real-time shipping rates during checkout and generate shipping labels upon order fulfillment. Store tracking information with the OHC order record.

## Priority
P1

## Estimated Scope
Medium
