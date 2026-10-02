# Recorded-order milestone gate

This gate compiles the complete mounted milestone read handler and its response
types from production source, with actual bearer validation and the canonical
PostgreSQL order and milestone schemas. Only the unused fields of GrowthState
are omitted. It does not execute unrelated growth handlers or worker actions.

Use an explicit loopback PostgreSQL URL whose database is named `ohc_*_test`:

```sh
OHC_MILESTONE_TEST_DATABASE_URL=postgres://postgres@127.0.0.1:55439/ohc_milestone_test \
  python3 scripts/focused_ci_gate.py order-milestones
```

The wrapper requires all 13 tests, rejecting failure, ignored or filtered cases.
The fixture creates isolated schemas and a real restricted LOGIN role with
forced RLS and one read connection. It rejects unsafe database targets before
connection and checks UTF-8 before creating fixtures. Registry dependencies must
match the root lockfile. Source extraction and schema hashes are retained.

Coverage includes signed raw and Unicode tenants, foreign-query denial, missing
authentication/forged headers, actual zero, thresholds after their exact boundary,
mixed currencies/statuses without revenue claims, stale worker markers, real
closed-pool/permission errors and no row/milestone mutation. A read-only count is
not evidence that an order was paid, delivered, fulfilled or awarded a reward.

The whole application compiler and actual browser journeys remain separate
required integration checks. The old worker/public-card/referral-reward paths
are not certified by this read gate.
