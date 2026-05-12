# Issue Brief: AI-Powered Post-Purchase Upsell Offers

## Problem Statement
SMBs leave money on the table by not offering complementary products immediately after a purchase is made.

## Research Report
Post-purchase upsells (one-click offers shown on the 'Thank You' page) convert at a much higher rate than pre-purchase upsells because the customer's wallet is already open.

## Design Doc
**Architecture:**
- Upsell trigger post-checkout.
- `Order` modification logic.
**AI Integration:**
- AI analyzes the purchased item and autonomously selects the highest-converting complementary product to display as the upsell.

## Implementation Prompt
Build a post-purchase upsell feature. When an order is completed, display a one-click add-on offer before the final confirmation. Acceptance criteria: Accepting a mock upsell offer successfully appends the item to the existing order without requiring the user to re-enter payment details.

## Priority
P2

## Estimated Scope
Medium
