# Shipping integrity source-bound probe

Run `bash scripts/shipping-integrity-contract/run.sh` with an explicit loopback
`OHC_SHIPPING_TEST_DATABASE_URL` whose database is an ASCII `ohc_*_test` name.
The database must be disposable and permit the test owner to create/drop its own
random schemas and synthetic LOGIN/NOBYPASSRLS roles. The guard rejects unsafe
prerequisites before connecting. Missing storage is a failure, never a skip.
No real provider credentials, requests or paid labels are used. Provider calls
in the tests reach a loopback HTTP server with synthetic response evidence.

The harness compiles complete production fulfillment/shipping modules and the
real Shippo integration crate. The DB adapter contains only the pool/store fields
those modules use; it supplies no replacement persistence or authentication.
Private shipping/fulfillment mounting, canonical access setup and the outer
bearer-auth boundary are extracted from `lib.rs`,
with the real bearer Store/middleware and bypass-path helper. Source assertions
check that provider routes are outside all global owner/tenant/tier layers.
The full application and unrelated workers are not booted.

PostgreSQL fixtures execute the actual initial commerce table definitions,
actual delivery-task migration102, forward binding migration1029 and purchase migration1031. Runtime
queries use a separate non-owner LOGIN/NOBYPASSRLS role. The owner pool is used
only for fixture seeding, fault injection and independent readback. SQLite
fixtures execute actual bootstrap table definitions and the exact startup call
into the forward schema helper. This is not full migration-chain or application
startup certification.

Registry versions/checksums are checked against root Cargo.lock. Every run
records all touched runtime, auth/integration dependency, route, fixture, CI and
script inputs, and rejects a changed source fingerprint after execution. The
CI inventory floor is92 executed tests (70 mounted/storage tests, two payload-shaper tests, plus20 Shippo
client/provider tests); ignored or filtered cases cannot pass.
`make lint` and `make test` remain aggregate implementation acceptance gates.

Owner-authority regression fixtures use real canonical auth tables, current roles,
signed bearer tokens, and the shared canonical transaction helpers. They cover
matching versus distinct actual PostgreSQL relations, current-owner admission,
revocation while a write is blocked, post-provider revocation/role removal, and
canonical versus unsupported SQLite pools. Mounted SQLite purchase/readback is
verified against loopback provider responses. Purchase regressions now cover durable single-dispatch admission, PostgreSQL
concurrency and revocation while admission waits, SQLite file close/reopen,
changed-key/rate/actor rejection, exact receipt replay and GET-only reconciliation.
The same source-bound runner executes the Shippo client/provider unit tests,
including bounded pagination, exact identity, redirects and retry suppression.
Admission linearizes dispatch permission; revocation cannot undo an already
in-flight external request. No live provider purchase is performed.

Order reads compile the complete production order module and its exact parent
router merge under strict current-user bearer authentication. Seven additional
PostgreSQL/SQLite regressions cover primed-list read-after-write, exact detail
beyond the newest fifty, tenant/customer-join isolation, current-access revocation,
committed storage failure and private/no-store responses. The shipping browser
journeys prime the list before their isolated write, require immediate readback,
and load the exact detail before their unchanged loopback purchase assertions.
Dashboard unified-feed summaries retain their existing aggregate cache; this
checkpoint claims authoritative reads only for the dedicated list/detail APIs.
