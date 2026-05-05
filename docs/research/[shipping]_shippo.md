# Title: Shipping & Logistics Integration via Shippo

## Problem Statement
Maya, the baker, and Priya, the boutique owner, need a way to ship physical products. Calculating shipping rates manually is error-prone, and buying labels at the post office is time-consuming. They need a system that automatically calculates shipping costs at checkout and lets them print shipping labels directly from their phone or computer.

## Research Report
Shippo provides a unified API for multiple shipping carriers (USPS, UPS, FedEx, DHL, etc.).
- **Ease of Use for Non-Technical Users**: Users simply input package dimensions and weight. Shippo handles the complex carrier routing and rate calculation behind the scenes. Printing a label is a one-click action.
- **Pricing**: Pay-as-you-go per label (usually $0.05 per label) plus postage costs. Very accessible for SMBs.
## Risks
- **Risks**: Carrier API downtimes and unexpected adjustments to package dimensions leading to extra charges.

## Reliability & Reputation**: Highly reliable API, excellent documentation, and strong partnerships with major carriers globally.
- **Environment Support**: Pure API, perfectly suited for Cloud and Standalone modes.

## Design Doc
The "Operations" (The Manager) agent handles logistics.
1. **Trigger**: An order for a physical product is placed.
2. **Action**: During checkout, Shippo's API provides live shipping rates. Post-purchase, the Operations agent drafts a shipping label.
3. **User View**: Priya opens the order in the OHC app, clicks "Print Label", and sticks it on the box. The tracking number is automatically emailed to the customer by the Customer Success agent.

## Implementation Prompt
Integrate the Shippo API to handle real-time shipping rate calculation at checkout and label generation in the admin dashboard. Add product weight and dimension fields to the inventory management UI. Create an "Orders" view where users can fulfill physical orders, purchase postage, and generate printable shipping labels in one click.

## Priority
P1

## Estimated Scope
Medium
