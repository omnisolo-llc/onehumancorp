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

## Advanced Financial Modeling for SMBs

### 13. Predictive Cash Flow Modeling
Small businesses live and die by cash flow, not just P&L. The Accountant agent must provide a predictive cash flow model, visualizing expected incoming funds (based on outstanding invoices and historical sales data) against expected outgoing expenses (rent, payroll, supplier costs). It should generate an Action Card if it predicts a cash shortfall in the next 30 days.

### 14. Automated Capital Allocation
For businesses with healthy margins, sitting on excess cash is inefficient. The Financial Advisor agent can suggest automated capital allocation strategies. For example, if the business has maintained a 6-month runway reserve, the agent might suggest automatically moving 10% of all future profits into an interest-bearing business savings account or increasing the marketing budget.

### 15. Real-Time COGS Tracking
Cost of Goods Sold (COGS) is notoriously difficult for small businesses to track accurately as supplier prices fluctuate. The platform must allow users to input dynamic COGS data. The Accountant agent can then track real-time profitability on a per-item basis, instantly alerting the owner if a specific product line suddenly becomes unprofitable due to rising material costs.

### 16. Least-Cost Routing Algorithms
### 17. Interchange Plus Pricing Models
### 18. Refund Fee Recovery
### 19. Multi-Currency Settlement
### 20. Tax Liability Automation
### 21. Least-Cost Routing Algorithms
### 22. Interchange Plus Pricing Models
### 23. Refund Fee Recovery
### 24. Multi-Currency Settlement
### 25. Tax Liability Automation
