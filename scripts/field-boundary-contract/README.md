# Field operation boundary contract

This focused gate imports the complete real field routers, mutation/planning code,
appointment projection, canonical authentication/transaction authority, and exact
production mount lines. Only the process-wide mesh/Hub boundaries are inert test
adapters. No auth claims, database effects, CAS checks, or SQL are mocked. The real
cache module's original tests remain discovered even though private route reads no
longer trust a process-global relationship cache.

Run `run.sh` with an explicit `OHC_FIELD_TEST_DATABASE_URL` pointing to a dedicated
loopback PostgreSQL database named `ohc_field_test`. The shell and Rust guard reject
other destinations before connecting. Fixture schemas, restricted SCRAM LOGIN
roles, and a separately named database for the negative binding probe belong to the
owned test cluster. The probe database alone is dropped with FORCE after closing
its pool to account for asynchronous SQLx socket teardown. No live data is touched.

`with-owned-postgres.sh` creates a disposable PostgreSQL cluster, runs the gate and
stops it in one retained execution namespace. PostgreSQL tools must already be on
PATH. It uses TCP loopback only, public synthetic test credentials, and UTF-8/UTC.
The ordinary gate is wired into required CI with a minimum executed-test floor;
filtered, skipped, failed, interrupted or zero-discovery runs cannot certify it.

Coverage includes signed owner/current role and revocation, exact pool binding and
alternate schema/database rejection, forged tenant headers, foreign/missing IDs,
real restricted-role writes, observed timestamp CAS and concurrency, terminal
states, exact receipt replay/conflict, transaction rollback, actual route schema,
route preview versus committed preparation, and malformed legacy relations.

The application does not dispatch routes, staff assignments or customer messages
from the preparation endpoints. A committed route means locally stored `prepared`
route/job records. An appointment marked completed is an authenticated owner's
recorded status, not independent physical-work verification. Whole-app compilation,
full make lint/test, browser execution and provider verification remain separate.

The gate also imports the actual mounted `/api/v1/sync/events` handler and complete
shared durable sync helpers. Its runner supplies the guarded owned database to all
original sync tests and executes ignored DB cases; the minimum inventory is 89.
This includes 14 offline-appointment cases, genuine restricted-role PostgreSQL
transactions, expiry/revocation while effects are pending, and a real SQLite
compatibility probe. Existing SQLite owner transaction capability remains usable;
the mounted appointment sync route was and remains PostgreSQL-only and returns a
blocked per-item result when no proven canonical PostgreSQL authority exists.

The full gate includes ten canonical child-route cases for `/sync/offline` and
`/sync/operation-intents`, including an actual HTTP completion receipt followed by
both exact draft-quote child requests and idempotent replay. Quote/intent receipts
prove persisted local work, not external delivery, an issued quote or invoice.
All three mounted PostgreSQL sync routes reject SQLite-only write authority while
the existing SQLite canonical transaction capability remains usable.
