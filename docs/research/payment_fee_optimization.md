# 💰 Stripe Transaction Fee Optimization (ACH vs. Card)

To maintain OHC's economic sustainability and keep user costs low, we've implemented an intelligent payment router that chooses between Credit Card and ACH for Stripe transactions.

## Fee Comparison

| Payment Method | Stripe Fee (Standard) | Minimum Amount for OHC |
| :--- | :--- | :--- |
| **Credit Card** | 2.9% + $0.30 | None |
| **ACH Direct Debit** | 0.8% (Capped at $5.00) | $50.00 |

## Optimization Logic

The `PaymentRouter` in `src/server/integrations/stripe/routing.rs` automatically evaluates every transaction.

### Decision Rule
We route to **ACH** if:
1. The transaction amount is **>= $50.00**.
2. The total ACH fee (0.8% capped at $5) is strictly less than the Credit Card fee (2.9% + $0.30).

### Potential Savings Examples

| Amount | Credit Card Fee | ACH Fee | **OHC Savings** |
| :--- | :--- | :--- | :--- |
| $20 | $0.88 | N/A (Card) | $0.00 |
| $100 | $3.20 | $0.80 | **$2.40** |
| $500 | $14.80 | $4.00 | **$10.80** |
| $1,000 | $29.30 | $5.00 (Cap) | **$24.30** |

## Implementation
The logic is integrated into `StripeClient::create_checkout_session`, ensuring that high-value transactions automatically utilize the most cost-effective payment rail. This optimization directly contributes to OHC's ability to offer a generous free tier by reducing overhead on paid conversions.
