# Portable canonical write authority

Status: the shared capability passed ten real-database behavior cases, five
actual-auth cleanup/expiry unit cases, and strict all-target Clippy for both
harnesses. The original missing-API RED and the initial 9-pass/1-failure fixture
result remain recorded. The latter used the wrong SQLx commit-hook convention;
only the fixture changed before all ten cases passed. PostgreSQL checkpoint
`d92fbeb` remains independently verified. SQLite startup and consumer mounting
are still open; this checkpoint makes no claim that those paths are complete.

## Capability

Mirror `CanonicalPgAuthority`, `AuthorizedPgOwner` and `OwnerPgTransaction` as
`CanonicalSqliteAuthority`, `AuthorizedSqliteOwner` and
`OwnerSqliteTransaction`. The public flow stays `bind`, `authorize`, `begin`,
borrowed `connection`, and final `commit`. Bind only the credential repository's
actual shared SQLx SQLite pool. Equal URLs, independent connections to the same
file, copied identity rows and distinct in-memory databases do not create a
capability. This first capability needs no schema migration.

Use the shared exact signed-bearer verification and raw tenant/actor identity.
Acquire `BEGIN IMMEDIATE` before any canonical authority snapshot, so SQLite's
actual writer serialization covers all current user/role and token-revocation
writers. Require foreign keys enabled, ordinary isolation and a writable
connection. Read active canonical membership, owner/admin roles, JTI revocation
and database time in the same transaction. Recheck before COMMIT. Provider work
never occurs while the write transaction is held.

## Bounded connection lifecycle

SQLite SQL execution continues on a worker thread after an async caller stops
waiting. A timeout alone must not leave a queued write or an unbounded busy wait.
Own a pooled connection with a cleanup guard before altering any setting. Save
its actual busy timeout, lower that connection's timeout to at most one second,
and use SQLx's transaction manager for BEGIN/COMMIT/ROLLBACK. Acquisition,
authority checks and final COMMIT each have a bounded async deadline.

On ordinary success, confirm transaction closure and restore the original
timeout before returning the connection. On cancellation or drop, retain that
same connection until rollback and restoration finish; cleanup never submits
COMMIT. A connection whose cleanup is unconfirmed must never reenter the pool.
Handle cleanup-task cancellation, runtime shutdown and restoration errors
explicitly, with safe diagnostics and no promise that an in-memory database
survived failed cleanup. Unconditional close-on-drop is unsuitable: closing the
only connection to `sqlite::memory:` destroys its data.

Once COMMIT is submitted, lack of acknowledgement means unconfirmed storage.
Do not return a definitive no-effect result or retry the business effect.

## Startup preservation (next stage)

Trace configured selection before changing initialization. The current server
constructs portable and legacy pools independently. Auth uses the portable pool;
legacy business repositories use the legacy pool. There is no independent
AUTH_DATABASE selector in the current production entry point, but public
constructors can intentionally use separate stores and must remain rejected.

For a single resolved SQLite deployment, construct the configured legacy pool
once with its existing encryption/options/hooks, then wrap that exact pool in
`AppDatabase` before constructing repositories and `Store`. Preserve existing
accounts and business records. Do not substitute a legacy pool for an already
configured separate auth store. Keep PostgreSQL/MySQL selection, public reads
and accepted-job behavior outside this SQLite change. Test the actual startup
adapter with existing accounts, multiple/default selectors and configured
connection hooks; distinguish preserving SQLCipher options from proving a
working encrypted SQLite engine.

## Evidence

Use actual SQLite files and a one-connection in-memory database. Cover same-pool
success, independent same-file pool/copied-store rejection, canonical revocation
in both orderings, role/expiry changes, busy BEGIN and cancellation, dropped
writes, deferred constraint failure, cleanup failure/cancellation and option
restoration. Keep PostgreSQL and workflow inventories intact. Shared consumers
may prepare borrowed SQL helpers; they must wait for this verified capability
instead of adding local weaker checks.

The builder inventory now contains 75 cases (the previous 65 plus ten new
SQLite cases). This bounded proof executed the ten new cases; the combined
75-case PostgreSQL/SQLite gate remains due before claiming the combined run.
The five private cases run from the actual server_auth library, using the same
dependencies/features as its manifest. They also run in the full workspace.
