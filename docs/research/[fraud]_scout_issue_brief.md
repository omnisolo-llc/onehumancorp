# Issue Brief: Automated Fraud Detection and Order Flagging

## Problem Statement
SMBs are highly vulnerable to chargebacks from fraudulent orders, which can cripple their cash flow.

## Research Report
Chargeback fees and lost inventory are devastating. OHC must provide enterprise-grade fraud detection built-in.

## Design Doc
**Architecture:**
- Integration with Stripe Radar or custom risk scoring.
- `RiskScore` linked to `Order`.
**AI Integration:**
- AI models analyze transaction patterns (e.g., mismatching billing/shipping, multiple failed attempts) to flag orders for manual review.

## Implementation Prompt
Implement a fraud scoring system for all inbound orders. Flag orders that exceed a certain risk threshold and prevent automatic fulfillment. Acceptance criteria: A mock order with high-risk parameters is successfully flagged and placed in a 'Pending Review' state.

## Priority
P1

## Estimated Scope
Large
