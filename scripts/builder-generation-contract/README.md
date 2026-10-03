# Source-bound builder generation contract

Run `OHC_BUILDER_GENERATION_TEST_DATABASE_URL=postgres://.../ohc_builder_generation_test bash scripts/builder-generation-contract/run.sh`.

The loopback disposable database is mandatory. Tests use actual configured
OpenAI-compatible HTTP transport, actual signed authority and revocation checks,
production JSON validation and actual Postgres storage under a restricted
RLS-enforced role. Fixtures contain synthetic test records only; production
sources contain no successful fake generator. `prepare.py` imports production
files and copies the exact production inference adapter, checks dependencies
against the repository Cargo.lock and fingerprints all covered sources.

These are focused local contract tests, not live-provider or full-server
certification. Full Cargo/Next/desktop and hosted CI gates remain required.

The combined gate preserves the original generation/admission cases, imported durable receipt lifecycle cases, and funded admission cases. Persistence modules, receipt sources and all included SQL are source-fingerprinted.

The mandatory inventory is 84 cases. Canonical brand writes require the actual
same PostgreSQL pool as the credential store. Startup can reuse that pool for
private builder storage only after proving that the configured business pool
shares its real advisory-lock namespace, search path and canonical/business
relation IDs. The proof performs no schema/data writes, rejects missing or
ambiguous identity, and bypasses a second connection for an identical handle.
Real cloned-database fixtures demonstrate why equal names/catalog IDs do not
establish that namespace. Restricted roles remain restricted.

SQLite and MySQL dummy PostgreSQL handles are excluded. Storage-independent
text drafting remains available; SQLite builder persistence is unfinished.
Separate configured stores are retained; rejected private binding never moves
business data. Public publication reads and previously accepted workers are
separate from the request bearer lifetime. Their broader backend guard and
request-owned publication fence remain distinct follow-on work.

Owned database teardown distinguishes real clients from PostgreSQL autovacuum
maintenance. The lifecycle tests observe a real slow autovacuum, exercise normal
template clone/drop while it is active, and retain a real client to prove that
client leaks still fail with bounded, credential-free ownership diagnostics.
Only autovacuum workers are excluded; unknown backend types remain blocking.

A deterministic real after-release interleaving covers the pinned SQLx pool
shutdown race. The max1 clone fixture acquires and acknowledges closure of its
known backend before pool shutdown, then verifies the pool and server drain.
The dependency version and the real cloned-catalog/namespace assertion stay intact.

The ten SQLite capability cases use real file-backed and max-one-connection
in-memory databases. They verify exact-pool binding, canonical revocation,
bounded/cancelled write intent, deferred COMMIT failure and an actual blocked
COMMIT that can succeed after its acknowledgement deadline. Five additional
cleanup/expiry unit tests live in the actual server_auth crate. The portable
capability does not by itself fix production SQLite startup or builder storage.
