# Stripe Transaction Fee Optimization Analysis

**Date:** 2026-07-06
**Goal:** Document the cost savings achieved through Payment Routing optimization and Payout batching in OneHumanCorp.

## Overview
OneHumanCorp integrates with Stripe for processing payments and distributing funds to connected accounts. To ensure our operators retain the maximum amount of revenue, we implemented two strategic optimizations:
1. **Dynamic Payment Routing:** Automatically selecting between ACH and Credit Card based on transaction amount.
2. **Payout Batching:** Aggregating small micro-payouts into larger bulk payouts to minimize fixed transfer fees.

This document breaks down the concrete savings based on these features.

---

## 1. Dynamic Payment Routing (ACH vs Credit Card)

Stripe applies different fee structures based on the payment method:
- **Credit Card (Standard):** 2.9% + $0.30 per successful transaction.
- **ACH (Direct Debit):** 0.8% per transaction, capped at a maximum of $5.00.

Our system routes any transaction over our configured minimum threshold (defaulting to $50.00) to ACH when the ACH fee is lower than the Credit Card fee.

### Scenario: Large Service Payment
*A field service owner bills a customer for a $5,000 project.*

- **Unoptimized (Credit Card):**
  - Fee: ($5,000 * 0.029) + $0.30 = $145.00 + $0.30 = **$145.30**
- **Optimized (ACH):**
  - Fee: ($5,000 * 0.008) = $40.00 (Capped at **$5.00**)
- **Savings:**
  - $145.30 - $5.00 = **$140.30 saved on a single transaction.**

---

## 2. Payout Batching

Stripe charges connected accounts for Instant Payouts (often a flat fee + percentage) and standard payouts (often a flat $0.25 fee per transfer). By defaulting to batching smaller payouts into a single daily or weekly bulk transfer, we drastically reduce the fixed-fee burden on micro-transactions.

### Scenario: High Volume Micro-Transactions
*A baker completes 100 small custom orders in a week, each generating a $10.00 payout.*

- **Unoptimized (100 individual payouts):**
  - 100 payouts * $0.25 flat fee = **$25.00 in fixed fees.**
- **Optimized (Batched into 1 payout):**
  - Total volume: $1,000.00
  - 1 payout * $0.25 flat fee = **$0.25 in fixed fees.**
- **Savings:**
  - $25.00 - $0.25 = **$24.75 saved per 100 transactions.**

## Conclusion
By engineering these cost features natively into the OHC platform, we implicitly improve the unit economics for every owner on the platform without requiring them to understand complex billing structures.
