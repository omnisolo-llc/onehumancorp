# Stripe Payment Routing Optimization & Cost Savings

## Overview
As part of our commitment to maintaining a sustainable business model and providing cost-effective solutions, we have implemented an intelligent payment routing system for OneHumanCorp's Stripe integration. This optimization automatically selects the most cost-efficient payment method based on the transaction amount.

## The Strategy: Dynamic Method Selection
Stripe charges different transaction fees based on the payment method used:
- **Credit Card**: 2.9% + $0.30 per transaction
- **ACH Direct Debit**: 0.8% per transaction, capped at $5.00

For small transactions, Credit Cards are generally preferred due to their ubiquity and immediate processing. However, as the transaction value increases, the percentage-based fee for Credit Cards grows unbounded, while the ACH fee is capped at $5.00.

Our `PaymentRouter` dynamically calculates the fees for both methods and selects the optimal path. ACH is selected when its fee is lower than the Credit Card fee, and the transaction meets our minimum threshold for ACH ($50.00).

## Cost Savings Analysis
By routing eligible high-value transactions through ACH, OneHumanCorp significantly reduces its payment processing overhead.

### Examples

**Small Transaction: $10.00**
- Credit Card Fee: $0.59
- ACH Fee: $0.08
- Routing decision: **Credit Card** (Preferred for small amounts to reduce friction, even though ACH is technically cheaper. The absolute difference is negligible). Note: Our code threshold for ACH is $50.00.

**Medium Transaction: $50.00**
- Credit Card Fee: $1.75
- ACH Fee: $0.40
- Routing decision: **ACH**
- **Savings: $1.35 per transaction**

**Large Transaction: $1,000.00**
- Credit Card Fee: $29.30
- ACH Fee: $5.00 (Capped)
- Routing decision: **ACH**
- **Savings: $24.30 per transaction**

**Enterprise Transaction: $10,000.00**
- Credit Card Fee: $290.30
- ACH Fee: $5.00 (Capped)
- Routing decision: **ACH**
- **Savings: $285.30 per transaction**

## Implementation Details
The routing logic is encapsulated in `src/server/integrations/stripe/routing.rs` and integrated into the `StripeClient` (`src/server/integrations/stripe/client.rs`). Whenever a checkout session is generated for an amount, the system inherently uses this logic to pick the best available payment form.

This feature ensures that OHC infrastructure costs remain predictable and scalable while maximizing margins on larger tier subscriptions and bulk enterprise transactions.