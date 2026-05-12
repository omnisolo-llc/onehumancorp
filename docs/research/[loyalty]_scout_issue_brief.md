# Issue Brief: Zero-Config AI Customer Loyalty & Rewards Program

## Problem Statement
Retaining customers is cheaper than acquiring new ones, but setting up a loyalty program involves complex points rules, tiers, and integrations that confuse small business owners.

## Research Report
Simple punch-card or points-based loyalty programs increase repeat purchase rates by 20%. OHC should provide an invisible loyalty system where the AI automatically tracks customer lifetime value (LTV) and autonomously issues rewards or discounts to high-value customers without manual configuration.

## Design Doc
**Architecture:**
- `Customer` entity augmented with LTV and Points metrics.
- `RewardEvent` entity for tracking issued discounts.
**AI Integration:**
- AI identifies 'at-risk' high-value customers and autonomously offers a 'win-back' discount code.

## Implementation Prompt
Develop a background service that continuously calculates customer LTV and automatically issues standardized reward tiers (e.g., 10% off after 5 purchases). Acceptance criteria: A customer reaching a mock purchase threshold automatically receives an email/SMS containing a uniquely generated discount code.

## Priority
P3

## Estimated Scope
Large
