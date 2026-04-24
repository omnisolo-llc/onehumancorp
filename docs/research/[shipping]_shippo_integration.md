# Integrate Shippo for Automated Label Generation

## Problem Statement
Shipping physical goods is a manual nightmare. Maya needs to calculate shipping rates, print labels, and send tracking numbers without leaving OHC or dealing with carrier portals.

## Research Report
- **Tool Evaluated**: Shippo
- **Ease of Use**: Connects to dozens of carriers globally (USPS, FedEx, DHL) with one API.
- **Pricing**: Pay-as-you-go, no monthly fees for basic tier.
- **Standalone/Cloud**: Cloud API works for both modes.
- **Persona Fit**: Perfect for Maya (Baker) and Priya (Boutique).

## Design Doc
- **Integration Point**: Operations Agent, Customer Success Agent.
- **Trigger**: Order paid and marked ready to ship.
- **Action**: Fetch shipping rates, purchase label via Shippo, notify Customer Success Agent to send tracking.
- **User View**: Owner clicks "Print Label" on the order page. Tracking is auto-emailed.

## Implementation Prompt
Build a Shippo integration to fetch live shipping rates during checkout and generate shipping labels from the order dashboard. Provide a UI component to display package tracking statuses.

## Priority
P0

## Estimated Scope
Large
