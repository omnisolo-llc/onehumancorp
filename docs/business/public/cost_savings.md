# OHC Stripe Transaction Fee Optimization

As part of the initiative to ensure OneHumanCorp is economically sustainable, we have implemented an intelligent payment routing system designed to minimize Stripe transaction fees.

## Strategy

Stripe's typical transaction fees are:
- **Credit Cards:** 2.9% + $0.30 per transaction
- **ACH Direct Debit:** 0.8% per transaction, capped at $5.00

For high-value transactions, defaulting to Credit Cards results in unnecessarily high processing fees. To optimize this, the `PaymentRouter` evaluates the transaction amount and intelligently routes eligible transactions through ACH to maximize cost savings.

## Logic Implementation

The system dynamically calculates the optimal payment method:

1. Calculate the standard Credit Card fee: `(amount * 0.029) + $0.30`
2. Calculate the ACH fee: `amount * 0.008` (capped at `$5.00`)
3. Compare the calculated fees.
4. **Condition for ACH Routing:** If the transaction amount is $50.00 or higher AND the ACH fee is lower than the Credit Card fee, route the payment to ACH.

## Cost Savings Impact

By utilizing ACH for transactions over $50:
- A $100 transaction saves **$2.40** (CC fee: $3.20 vs ACH fee: $0.80).
- A $1,000 transaction saves **$24.30** (CC fee: $29.30 vs ACH fee: $5.00 cap).
- A $10,000 enterprise transaction saves **$285.30** (CC fee: $290.30 vs ACH fee: $5.00 cap).

This optimization acts as a powerful lever to decrease operational costs and boost margins on premium enterprise and high-volume merchant plans, directly supporting the "Zero Waste, High Leverage" mission of the platform.
