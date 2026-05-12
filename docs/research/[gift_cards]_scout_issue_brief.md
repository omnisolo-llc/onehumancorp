# Issue Brief: Digital Gift Card Generation and Management

## Problem Statement
SMBs want to sell gift cards to increase cash flow, especially during holidays, but managing the accounting and redemption of digital gift cards is complex.

## Research Report
Gift cards are a high-margin product for SMBs. 20% of gift cards are never redeemed, representing pure profit. OHC should offer a seamless way to generate and email digital gift cards.

## Design Doc
**Architecture:**
- `GiftCard` entity with balance tracking.
- Email delivery service.
**AI Integration:**
- None.

## Implementation Prompt
Implement digital gift cards as a purchasable product type. Create logic to generate a unique redemption code and email it to the recipient. Acceptance criteria: Purchasing a mock gift card successfully generates a code that can be applied to a subsequent checkout.

## Priority
P2

## Estimated Scope
Medium
