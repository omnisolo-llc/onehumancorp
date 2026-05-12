# Issue Brief: Automated Customer Referral Tracking Engine

## Problem Statement
Word of mouth is the biggest driver for SMBs, but tracking who referred whom and issuing rewards manually is impossible at scale.

## Research Report
Automated referral programs can increase new customer acquisition by 15%. OHC should automatically generate unique referral links for every buyer and track conversions natively.

## Design Doc
**Architecture:**
- `ReferralCode` and `ReferralEvent` entities.
- Discount code generation linked to successful referrals.
**AI Integration:**
- AI identifies top referrers and suggests sending them VIP 'thank you' gifts or exclusive discounts.

## Implementation Prompt
Create a system that automatically generates a unique referral link for every customer post-purchase. When a new customer buys using that link, automatically issue a discount code to the referrer. Acceptance criteria: A mock purchase using a referral link correctly triggers a reward payload to the referring user.

## Priority
P2

## Estimated Scope
Medium
