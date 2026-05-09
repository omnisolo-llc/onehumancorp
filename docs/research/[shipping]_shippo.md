# [Shipping] Shippo Integration

## Title
Integrate Shippo for Automated Label Generation

## Problem Statement
Priya (Boutique Owner) spends hours copying and pasting addresses into carrier websites to print shipping labels. She needs to click one button natively in OHC to buy and print a label without relying on complex external logistics aggregators that break the Radical Simplicity rule.

## Research Report
- **Strategy**: Build a native fulfillment interface.
- **Target Persona**: Priya (Boutique Owner), Maya (Home Baker)
- **Advantages**: Very high once configured natively. User just clicks 'Buy Label & Print' without leaving OHC. Aggregates rates from multiple carriers. No monthly fee for pay-as-you-go.
- **Risks**: International shipping requires complex customs declarations which might be hard to automate fully for non-technical users.
- **Pricing**: Free tier (pay per label + postage).
- **Compatibility**: Cloud and Standalone.

## Design Doc
- When an order is placed, OHC fetches live rates for shipping during checkout.
- The Operations agent shows the cheapest shipping option.
- The user clicks a native 'Fulfill Order' button, and OHC purchases the label and saves the tracking number.
- OHC automatically emails the customer the tracking number.
- **AI Integration**: The Customer Success Agent monitors tracking numbers natively and proactively notifies the customer if a delivery is delayed.

## Implementation Prompt
Implement a native shipping and fulfillment module. The checkout flow must show real-time shipping rates. The merchant dashboard must allow users to purchase and print shipping labels directly, and automatically attach the tracking number to the order and notify the customer.

## Priority
P1

## Estimated Scope
Large
