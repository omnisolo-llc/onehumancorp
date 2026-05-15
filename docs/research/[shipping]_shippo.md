# Shippo - Multi-Carrier Shipping Integration

## Problem Statement
Calculating shipping rates, generating labels, and tracking packages across multiple carriers is a manual nightmare for physical product sellers.

## Research Report
Shippo is a multi-carrier shipping API.
- **Ease of Use for SMBs**: High. Abstracts multiple carriers behind a single interface.
- **Pricing**: Pay-as-you-go model (per label fee).
- **Reputation**: Established and reliable API.
- **Competitive Analysis**: Excellent multi-carrier support without requiring individual carrier accounts for basic functionality.

## Design Doc
**Trigger**: Customer places an order requiring shipping, or business owner clicks "Generate Label".
**Actions**:
- OHC requests shipping rates from Shippo based on package dimensions and weight.
- OHC purchases a label via Shippo API and retrieves tracking info.
**User Experience**: Business owner can easily view rates, purchase a label, and print it directly from the OHC order dashboard.

## Implementation Prompt
**User-facing Outcome**: A business owner can instantly calculate shipping rates, generate shipping labels, and track packages without leaving OHC.
**Acceptance Criteria**:
- Real-time shipping rate calculation at checkout.
- Business owner can purchase and print shipping labels from the dashboard.
- Tracking numbers are automatically synced to the order.

## Priority
P2 (Medium)

## Estimated Scope
Large
