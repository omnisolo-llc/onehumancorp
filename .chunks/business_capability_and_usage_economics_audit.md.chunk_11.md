               + payment collection, support and incident handling
                 + attributable idle/shared capacity
```

Customer-paid BYOK inference is tracked for owner visibility but excluded from OHC's provider expense and from an OHC inference debit. Keep OHC revenue, customer business revenue and money the customer pays directly to a provider separate. Also separate variable contribution from fixed engineering/admin/acquisition cost and company profit.

Define compute precisely: active CPU-seconds, provisioned memory-time, GPU-time or reserved sandbox-time are different resources. A worker waiting on an API may use little CPU but still reserve memory and a browser. Avoid double-charging overlapping bundles; disclose whether waiting/reserved time is billable. Compare shared workers and idle suspension using actual workloads, without weakening tenant isolation.

A 20% markup is not a 20% margin: an illustrative $10 cost sold at $12 yields $2, or **16.7% of revenue before other costs**. Small compute bills may not fund support. A minimum prepaid balance, explicit reserved-capacity charge or separately priced support could address this, but no particular fee is selected here. Test owner preference and unit economics rather than quietly reinstating the old subscription.

### Minimum evidence required for usage billing

Capture durable, deduplicated events with tenant/project/task/attempt and provider request IDs; payer and auth mode; provider/model; actual input/output/cache or tool quantities; resource unit and measured interval; rate-card version; estimated/reserved/settled/refunded state; and external reconciliation reference. API balances, subscription quota and customer cash are not interchangeable units.

Use integer subunits or decimal arithmetic with sufficient precision, immutable adjustments rather than silent history edits, trusted producers and tenant-specific reads. Reserve a conservative ceiling before starting new paid work, bound concurrent work