# Transaction Fee Optimization & Cost Analysis

## Overview
As part of our commitment to keeping OneHumanCorp accessible to small businesses at every budget level, we have implemented an intelligent **Payment Routing System** to minimize Stripe transaction fees for high-value transactions.

## The Problem
Small businesses using standard Credit Card processing via Stripe face steep fees on high-value items or services.
- **Stripe Credit Card Fee:** 2.9% + $0.30 per transaction.

For a custom baker selling a $50 cake, the fee is acceptable ($1.75). However, for a handyman booking a $1,000 service, the fee balloons to **$29.30**.

## The Optimization (Implemented)
We built the `PaymentRouter` in `src/server/integrations/stripe/routing.rs` to dynamically route transactions over $50 to **Stripe ACH**, which has significantly lower fees.
- **Stripe ACH Fee:** 0.8%, capped at **$5.00** per transaction.

### Intelligent Routing Logic
The system dynamically calculates both the Credit Card fee and the ACH fee for any transaction amount.
- If `amount >= $50.00` and `ach_fee < card_fee`, it automatically selects `PaymentMethod::Ach`.
- Otherwise, it defaults to `PaymentMethod::CreditCard`.

## Before & After Cost Analysis

| Transaction Amount | Payment Method Used | Before (Credit Card Only) | After (Dynamic Routing) | Net Cost Savings |
|--------------------|---------------------|---------------------------|-------------------------|------------------|
| **$10.00**         | Credit Card         | $0.59                     | $0.59                   | **$0.00**        |
| **$49.99**         | Credit Card         | $1.75                     | $1.75                   | **$0.00**        |
| **$50.00**         | ACH                 | $1.75                     | $0.40                   | **$1.35**        |
| **$250.00**        | ACH                 | $7.55                     | $2.00                   | **$5.55**        |
| **$1,000.00**      | ACH                 | $29.30                    | $5.00 (Capped)          | **$24.30**       |
| **$10,000.00**     | ACH                 | $290.30                   | $5.00 (Capped)          | **$285.30**      |

## Impact
By capping the maximum transaction fee at $5.00 for large purchases (such as custom service quotes, retainers, and high-ticket items), we drastically reduce the cost barrier for our users. This feature works completely invisibly, requiring zero setup from the small business owner, perfectly aligning with OHC's product vision.
