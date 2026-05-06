# Cost Optimization

This document outlines the cost optimization strategies and implementations within the OneHumanCorp infrastructure, aimed at ensuring economic sustainability.

## Payment Routing and Transaction Fee Optimization
OneHumanCorp employs an intelligent payment router to minimize transaction fees for subscription charges and other high-value payments. Stripe's fee structure varies significantly based on the payment method used:
- **Credit Card:** 2.9% + $0.30 per transaction.
- **ACH:** 0.8% per transaction, capped at $5.00.

The `PaymentRouter::optimize_payment_method` evaluates the potential fee for both methods given a specific transaction amount. It routes transactions of **$50.00 or higher** to use ACH, because the ACH fee becomes strictly lower than the credit card fee at this threshold.

### Cost Savings
For a $1000.00 transaction:
- The standard Credit Card fee would be $29.30.
- The ACH fee is capped at $5.00.
This optimization yields a direct savings of **$24.30** for this single transaction.

The platform continuously evaluates the incoming charge amount and prefers ACH routing dynamically when it produces a cost saving, without requiring any manual intervention.

## LLM Token Efficiency
The AI layer uses an intelligent memory-based embedding and inference cache (implemented in `src/server/minimax.rs` and `pricing/cache.rs`). This dramatically cuts down redundant LLM inferences and repeated token context transmission.

## Storage Compression
User and agent-generated product images are transparently resized and converted into the highly efficient WebP format inside `src/server/storage/local_provider.rs`. This enforces tight quotas and drastically reduces both long-term storage footprints and outbound CDN transfer costs.
