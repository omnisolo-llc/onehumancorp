# Issue Brief: Charitable Donations and Round-Up at Checkout

## Problem Statement
Many modern SMBs want to support social causes by allowing customers to round up their total or donate a fixed amount, but this requires clunky workarounds.

## Research Report
Checkout charity integrations improve brand perception and increase conversion rates slightly for socially conscious brands.

## Design Doc
**Architecture:**
- `Donation` line item type.
- Integration with charity APIs (e.g., Percent).
**AI Integration:**
- None.

## Implementation Prompt
Implement a 'Round up for Charity' feature in the checkout flow. Acceptance criteria: A mock checkout successfully rounds a $14.50 order to $15.00, allocating $0.50 to a donation line item.

## Priority
P3

## Estimated Scope
Medium
