# Recorded orders and share intent

The protected `GET /api/v1/growth/milestone` reads the authenticated business's
current order rows under its transaction-local PostgreSQL tenant context. A
query tenant can only confirm that identity; it cannot choose another business.
The response is private and not cached. Missing authentication, a mismatched
tenant and a database failure are distinct from an actual zero count.

`recorded_orders` counts every currently stored order row, including pending,
canceled and fulfilled statuses. It is not a paid-sale, fulfillment or delivery
count. Deleting a stored row can lower the current count. The existing order
thresholds 1, 10, 50, 100 and 1000 are derived with greater-than-or-equal comparisons,
so a count of 11 still contains the 10-order threshold. The read does not write
business_milestones or grant anything. It ignores old worker marker rows, which
cannot certify the current aggregate.

Amounts, exchange rates and currencies are not aggregated. No revenue, payment,
reward, credit or trial claim follows from this metric. `observed_at` is recorded
by the database with the count; the UI can refresh the read explicitly.

The team/referrals and dashboard consumers validate the exact owner and response
shape, hold data while canonical identity verification is pending, and retire it
when the owner changes. An order share requires a confirmed invitation from that
same still-verified account. The existing invitation lock, persisted pending
marker and unknown-outcome behavior remain authoritative for creation. The UI
shows aggregate wording and the confirmed link for review, and constructs only
WhatsApp/X share-intent URLs. Opening those URLs does not send a message, confirm
delivery/joining or earn a reward. Clipboard success follows the actual platform
promise.

## Separate unfinished capabilities

The worker's historical milestone/event logic, visitor and revenue projections,
referral rewards, public milestone cards and public website publication are
separate contracts. This read does not make those claims trustworthy or implement
their lifecycle. Hosted browser acceptance and whole-main compilation remain
required beyond the focused PostgreSQL and component checks.
