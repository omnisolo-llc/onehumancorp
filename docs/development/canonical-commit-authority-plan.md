# Canonical transaction authority repair

Status: PostgreSQL capability, brand-handler final writes and the bounded
same-store startup classifier passed 65 source-bound cases and strict Clippy.
The same final source also passed workflow 101/101 and receipt PostgreSQL 56/56,
with strict all-target Clippy for both. CI inventory guards passed 22/22.
Publication writes/backend guard and the SQLite capability remain open. Existing funded/readback/usage
checkpoints remain committed through `1de943b`; the worst permitted cached-input
tariff repair is separately committed as `ec97410`, and root worker lifecycle
coverage as `98885c3`. The final source-bound workflow101, PostgreSQL56 and
builder53 inventories and all three strict Clippy gates passed at that checkpoint.

## Scope and ownership

This checkpoint adds `server_auth::commit_authority` and wires request-owned
brand draft/import writes through it. Publication submission/revocation is the
next independent consumer repair and is not covered by this checkpoint. Entitlement work owns
registration/tickets and portable registration-column upgrades; generation UX
work owns browser behavior and import transport/parsing. Preserve those changes.

Accepted publication jobs remain durable approvals. Their existing canonical
owner-role checks and public read contracts remain independent of logout or an
expired request token. This repair fences the request transaction accepting or
revoking that approval; it does not retroactively cancel accepted jobs.

## Binding and transaction boundary

- A `CanonicalPgAuthority` binds the actual `Store::portable_repo()` PostgreSQL
  pool to the supplied data pool. Require the same shared pool handle, not equal
  URL strings, copied user rows, or a caller-provided tenant. SQLx's immutable
  `Pool::options()` reference belongs to its shared pool allocation; cloned
  handles share that reference and independently constructed pools do not. Cover
  both cases in tests and keep the check encapsulated.
- Private builder storage receives the canonical auth pool only after the
  configured business pool passes the real same-store classifier. Separate-store, memory-authority/PG-data, and SQLite-authority/PG-data
  constructor combinations fail before paid inference or writes. Existing
  same-store SQLite workflow/receipt behavior remains supported and unchanged.
- Derive an opaque owner/token snapshot by reusing the existing exact-bearer,
  current-owner checks. Centralize those checks rather than inventing a second
  interpretation of membership, token identity or expiry.
- Begin an owned write transaction from the bound pool. A wrapper exposes its
  transaction only to the existing internal SQL helpers and owns final commit.
  Reject unsupported isolation, bound statements/locks, and lock canonical actor
  and role rows before business rows.
- After blocking business writes, acquire the same shared tenant/JTI transaction
  advisory fence used by receipt commits. Re-read canonical token revocation,
  current membership and the database clock within that transaction, then commit
  while the fence is held. Canonical revocation INSERT/UPDATE triggers hold the
  exclusive counterpart. No lock spans provider I/O.
- Preserve builder's derived UUID compatibility context for its data tables;
  authority always uses the raw canonical tenant. Any necessary local context
  restoration occurs while the same transaction fence remains held.
- A separate auth connection held across a different data transaction is not an
  accepted substitute. Connection/database failure would invalidate that claimed
  atomicity. Unsupported binding fails explicitly.

## Storage and HTTP wiring

Brand persistence already uses an explicit transaction; wrap that transaction's
creation and final commit. Publication request SQL should operate inside the
bound transaction instead of privately committing behind the HTTP boundary.
Keep storage-contract tests and accepted-worker operations distinct from bearer-
authorized HTTP writes, without a production unfenced fallback.

The website-import work will use the same bound write wrapper for its source
receipt. It must not introduce an independent pool or authority convention.

## Required evidence

Keep every existing behavior case. Positive brand fixtures will use real
canonical PostgreSQL auth on the same pool instead of a memory credential store
plus copied PostgreSQL role rows. Retain explicit negative separated-store cases.

Prepared cases exercise unsupported memory/PG binding before any provider call
and a canonical revoked-token writer racing an actual deferred brand COMMIT.
Add same-pool versus independent-pool binding, publication final-commit races,
revocation during blocked writes, unsupported isolation/fence setup, and timeout
or commit-failure outcomes. Use actual SQLite/PostgreSQL and loopback HTTP only.
Run the existing locked fingerprint gates and strict Clippy when the shared
native slot is explicitly available; no additional dependency graph is needed.

## Shared consumer contract under review

The initial carrier is an owned SQLx `Transaction<'static, Postgres>`, created
from the structurally bound canonical pool. Storage helpers receive a borrowed
`PgConnection`; the capability retains transaction ownership and final COMMIT.
Shipping should prepare corresponding borrowed `PgConnection`/`SqliteConnection`
storage functions without an internal commit. The PostgreSQL API below and the bounded
private-builder startup integration passed the final proofs above. Its SQLite
counterpart and other route integration remain open.

The verified initial capability flow is `CanonicalPgAuthority::bind(store, pool)`, then
`authorize(claims, headers)` returning an opaque authorized owner, then `begin()`
when a request-owned write is ready. Binding and authorization happen before
paid/provider effects; beginning the transaction happens after any draft model
I/O. The wrapper exposes canonical tenant/actor accessors, a borrowed connection,
and an owned `commit()` that performs the final fence. The authorized owner must
retain its own canonical pool binding, so an identity verified in one store
cannot be paired with another store later.

Extract the existing exact-bearer and current-owner checks into this auth module
and call them from WorkflowExecution as well. Retain its complete existing test
inventory and do not convert publicly supplied Claims directly into a trusted
transaction capability. Typed failures preserve database error sources for
server handling while returning only safe messages to clients.

After the PG proof, extend the shared capability to SQLite, using its real
write-intent serialization before the first authority snapshot and canonical
identity/revocation reads in the same transaction. Test competing revocation,
rollback, busy timeouts and separate-pool rejection with actual SQLite files.
Shipping purchase mounting depends on this verified portable capability. Its
provider and idempotency storage internals may be prepared independently. A
provider receipt after a previously committed accepted operation retains honest
observed outcome even if the original session is later revoked.

## Exact startup/store boundary

`run_server` currently resolves the shared database URL family through portable
`connect_from_environment` and legacy `DB::new` separately. Their independent
pool allocations cannot satisfy `CanonicalPgAuthority::bind` by URL equality.
The builder mount now passes the actual legacy `DB::postgres_pool()` handle to
`canonical_builder_pool`; SQLite/MySQL dummy handles are excluded without access.

The classifier uses transaction-scoped random advisory locks, real search paths
and canonical/business relation OIDs. Each operation/idle transaction is bounded
and both probe transactions roll back. An identical pool handle takes a zero-I/O
shortcut, including with a one-connection pool already checked out. Only a proven
common store reuses the canonical pool. Missing relations, a different schema,
an independent cloned database or an unavailable connection deny private storage
before generation. No schema, business data or permissions are modified.

The builder None branch compiles/runs the actual storage-independent router:
text drafting and local text-schema helpers remain available; saved sites/brands
return an explicit unavailable response. This is not SQLite persistence support.
The configured public publication data pool and accepted-worker selection are
not redirected. Their dummy-backend guard and final request-token fence are
separate open work, as are legacy CRUD UUID/v5 alias isolation and builder request
idempotency/readback. Do not describe this checkpoint as whole-builder isolation.
