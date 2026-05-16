# [Shipping & Logistics] ShipStation Integration Evaluation

## Title
Automated Label Generation via ShipStation Integration

## Problem Statement
Small business owners selling physical goods spend too much time copying addresses to find shipping rates and print labels. They need a single button natively within OHC to calculate rates, buy postage, and print labels.

## Research Report
- **Strategy**: API integration with ShipStation.
- **Persona**: E-commerce stores, crafters, physical goods sellers.
- **Advantages**: Aggregates carriers (USPS, UPS, FedEx) with discounted rates. Highly reliable API.
- **Risks**: ShipStation has its own monthly subscription cost, increasing merchant overhead. Complex API for international shipments.
- **Pricing**: Monthly subscription fee plus postage costs.
- **Compatibility**:
  - **Cloud**: OAuth or API Key.
  - **Standalone**: API Key.

## Design Doc
- **Trigger**: Merchant clicks "Buy Label".
- **Action**: OHC calls ShipStation to purchase the label and saves the PDF.
- **User Interface**: Merchant connects ShipStation. Live shipping rates show at checkout. Operations Agent highlights unfulfilled orders. Tracking numbers are auto-retrieved and emailed.

## Implementation Prompt
Integrate ShipStation to automate shipping label generation. The checkout flow must query live rates. The merchant order management view must allow purchasing/printing shipping labels natively. Tracking numbers must be automatically saved and sent to the customer.

## Priority
P1

## Estimated Scope
Large
