# PostgreSQL receipt and funded dispatch contracts

This harness requires an explicitly owned loopback `ohc_*_test` PostgreSQL database.
It compiles actual receipt persistence, authentication, portable schema, worker,
configured text adapter and UsageLedger code. Storage-only cases fail if they
reach inference. The funded worker case calls an owned loopback HTTP fixture and
checks persisted reservation before the response and observed settlement after it.
No live vendor account or paid call is used.

All 97 tests are mandatory. The additive migration 1022 and canonical revocation
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

Dynamic-workflow cases compile the actual mounted API and bearer layer, full
manager, and source-extracted PostgreSQL/SQLite queue implementations. They use
canonical portable auth stores and real signed sessions, inspect persisted plan
bytes and job rows, and count rejected queue calls. Owner authorization, tenant
isolation on cache/reload, canonical filenames, legacy-plan preservation and
generic failure responses are covered. The PostgreSQL positive queue fixture
uses a table-owner pool and proves stored tenant identity; it does not certify
forced queue RLS. The existing batch queue path does not establish a tenant
context, unlike individual enqueue.

This bounded repair checks current-request owner authority. Queue commits and
JSON persistence remain separate; it does not establish atomic admission,
concurrent exactly-once confirmation, revocation fencing or completed execution.
The test JWT encoder reuses the auth crate's existing pinned jsonwebtoken version
solely to exercise correctly signed expired or tenant-less fixture sessions.

PostgreSQL queue encoding cases exercise actual migration-104 JSONB and separately
labeled legacy TEXT columns through enqueue, batch, requeue and independently
seeded dequeue. They check structured nested Unicode/numeric payloads, tenant,
parent, role, attempts and scheduling with structural JSONB comparison. The TEXT
fixtures also retain their existing serialized bytes. Existing
String serialization is retained, using PostgreSQL `::json` write expressions and
`payload::text` readback; no runtime schema conversion occurs. Escaped-NUL JSON
remains writable/readable as stored TEXT, while the existing dequeue role parser
still rejects it; JSONB retains its escaped-NUL rejection. These cases do not
claim support for every historical payload or change parser fallback policy.
