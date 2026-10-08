ts metric reports (`billing_api.rs:117-171`); it must not become the billing authority without trusted producer identity, replay protection and reconciliation. Missing usage means unknown/pending, not free and not permission to guess a debit.

### E. Commercial output can still be a placeholder

The mounted proposal intake writes a fixed $5,000 proposal and $2,500 deposit with website-redesign scope, independent of the submitted inquiry (`proposals.rs:825-845`). The invoice path persists an invented checkout URL. These are concrete gaps in existing features, not reasons to commission a new proposal or payment subsystem. Earlier research also identified reminder logic that logs drafting before implementing it; verify each completion claim against the actual artifact/provider state.

**Consequences:** current cost dashboards should not be treated as invoice-grade meters, and the presence of a business screen should not be advertised as an autonomously completed process. None of these defects was repaired by this document.

## 4. What online owner accounts reveal

This is purposive qualitative reading, not a random survey, authenticated interview study or measurement of which need is statistically most common. Forum publication times were not reliably established from the returned relative timestamps; record access date rather than invent exact dates. Examples, monetary amounts and business identities are self-reported. Comments are distinguished from original posts. Vendor customer stories are curated.

| Source | Reported situation | Our inference to validate against OHC |
|---|---|---|
| [Owner story O1](https://www.reddit.com/r/smallbusiness/comments/1w33g2r/customer_ordered_a_product_paid_deposit_but/) | A container-conversion business received a deposit, finished the unit and could not collect the balance; it occupied shop space while cash was needed. | Track contractual/payment state, correct contact, reminders, delivery holds and cash exceptions. Drafting anoth