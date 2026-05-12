# Issue Brief: Integrated Tipping and Gratuity Distribution

## Problem Statement
Service businesses (tutors, cleaners) want to accept tips, but adding tipping logic to custom checkouts is complex.

## Research Report
Integrated tipping at checkout increases average order value (AOV) by up to 15% for service businesses.

## Design Doc
**Architecture:**
- Add `tip_amount` to `Order` entity.
- UX for selecting percentage-based tips at checkout.
**AI Integration:**
- None.

## Implementation Prompt
Update the checkout flow to support tipping. Allow the merchant to configure default tip percentages (e.g., 15%, 20%, 25%). Acceptance criteria: A mock checkout flow successfully captures an order with a calculated tip amount.

## Priority
P2

## Estimated Scope
Small
