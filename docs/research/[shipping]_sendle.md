# [Shipping] Sendle Integration

## Title
Sustainable and Simple Shipping with Sendle

## Problem Statement
Priya (Boutique Owner) finds traditional shipping carriers confusing with their complex zones, weight charts, and hidden fees. She wants a simple, "flat-rate" shipping option that is easy to understand, carbon-neutral, and provides door-to-door service without her needing to wait at a post office.

## Research Report
- **Strategy**: Integrate Sendle API for quote generation and label printing.
- **Target Persona**: Priya (Boutique Owner), Eco-conscious merchants.
- **Advantages**: Simple, door-to-door flat-rate pricing. Carbon neutral (B-Corp). Integrated tracking and simplified parcel sizes.
- **Risks**: Primarily focused on Australia, USA, and Canada.
- **Pricing**: Flat rates based on parcel size (e.g., "Shoebox", "Briefcase").
- **Ease of Use**: Very high. No complex weight math; if it fits, it ships.
- **Compatibility**: Cloud & Standalone (API Key based).

## Design Doc
- **Integration with OHC**:
    - OHC fetches Sendle quotes based on the merchant's predefined parcel sizes.
    - Merchant selects Sendle for fulfillment, and OHC generates the label and schedules a pickup.
    - The "Ambassador" AI agent tracks the shipment and proactively notifies the customer of progress.
- **User View**: A "Ship with Sendle" button that shows a single clear price and generates a label in one click.

## Implementation Prompt
Implement Sendle as a native shipping and fulfillment provider. Provide real-time flat-rate shipping quotes during the checkout and fulfillment process. Enable one-click label generation and automated pickup scheduling via the Sendle API. Ensure tracking numbers are automatically synced to the order and shared with the customer.

## Priority
P1

## Estimated Scope
Medium
