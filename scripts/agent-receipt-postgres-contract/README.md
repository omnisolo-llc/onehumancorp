# PostgreSQL receipt and funded dispatch contracts

This harness requires an explicitly owned loopback `ohc_*_test` PostgreSQL database.
It compiles actual receipt persistence, authentication, portable schema, worker,
configured text adapter and UsageLedger code. Storage-only cases fail if they
reach inference. The funded worker case calls an owned loopback HTTP fixture and
checks persisted reservation before the response and observed settlement after it.
No live vendor account or paid call is used.

All 53 tests are mandatory. The additive migration 1022 and canonical revocation
trigger are exercised with a restricted PostgreSQL role and forced row-level
security. Cases cover reopen, exact replay/concurrent admission, one claim,
terminal proof/replay, tenant/actor isolation, current roles and token revocation
across lock waits and final commit, failed-commit rollback, restart uncertainty,
immutable transitions and bounded contention. All earlier SQLite and execution
cases are retained. Missing prerequisites fail setup rather than skipping tests.

The workflow harness separately executes authenticated HTTP through the same
worker/provider/ledger path. These focused source-bound gates do not replace the
full integrated workspace lint/test and deployment checks.

Usage-management cases compile the actual handlers and Hub ledger selector,
then PUT a spending limit and GET records through the real sealed-session proxy
under the restricted role. Deployment migrations install accounting schema;
request handling performs no DDL. Compatibility identity query fields must match
Claims, and unrelated fields remain rejected. Repeated migration preserves the
observed provider charge and immutable usage records.
