# PostgreSQL receipt parity plan

Status: implemented and focused-test verified; full application/provider and
hosted CI acceptance remain separate requirements. SQLite lifecycle commit
`a6ed6bc` remains the reviewed foundation; HTTP/provider dispatch and hard monetary
budgets are separate outstanding work.

## Contract

Preserve the existing request fingerprint, original authority, internal completion
proof, one-claim transition, identical terminal replay and unknown-outcome behavior.
Use raw tenant identifiers, the configured AppDatabase and the existing
`server_auth::seaorm_store::begin_tenant_transaction` context helper. No process-wide
system context or parallel tenant convention. PostgreSQL must use a fresh
READ COMMITTED statement snapshot; reject an incompatible isolation configuration
rather than accepting stale authority after a lock wait.

Add migration 1022 with forced tenant RLS, immutable admission identity, checked
phase/generation/lease transitions, bounded UTF-8 output and deletion protection.
It depends on canonical users only; portable auth tables are initialized by their
existing migration code before runtime use.

## Transaction ordering and bounds

1. Validate exact tenant/actor/token identity before opening a transaction.
2. Acquire a bounded connection and establish the existing tenant context.
3. Apply a 3-second statement timeout and a 1-second lock timeout.
4. Lock current actor and role rows before receipt rows. The actor lock serializes
   concurrent request admission for that actor; receipt row locks fence terminal
   and recovery races, including a different current owner reading the same tenant.
5. Read the database clock and current authority after blocking locks. Recheck
   current token/expiry after later blocking writes and immediately before commit.
6. Bound each storage operation as a whole. A lost/failed storage acknowledgement
   may be reconciled using the same request/proof; it never authorizes provider retry.

A token revocation row can be inserted concurrently without touching the user row.
The implementation must not pretend the user lock serializes token revocation.
Tests must cover revocation committed while an operation waits, and final commit
checks must refuse private output if authority has been lost. A stronger atomic
revocation ordering, if required, must reuse a shared auth-side lock protocol with
all revocation writers rather than a receipt-only advisory lock.

## Evidence before completion

The dedicated source-bound harness must use an explicitly owned loopback test
PostgreSQL database, fresh isolated schemas and restricted NOSUPERUSER/NOBYPASSRLS
roles. Verify actual RLS state and exact runtime role. Test persistent reopen,
concurrent idempotency, one claim, exact terminal replay, altered payload refusal,
role/token revocation after waits, deferred commit failure, expired-claim recovery,
corruption/transition rejection and bounded contention. Storage fixtures must panic
if inference is invoked. Retain and run the existing SQLite lifecycle tests.

All new behavior starts RED with PostgreSQL explicitly unavailable. Final evidence
must include strict Clippy, nonzero discovered case counts, source fingerprints and
root-lock dependency identity. Full application/HTTP/provider and hosted CI remain
additional acceptance requirements.

## Focused local result (2026-10-02)

The source-bound harness ran 38/38 cases, zero ignored or filtered: ten actual
restricted-role PostgreSQL cases and 28 existing SQLite/execution cases. Actual
PostgreSQL tests include deferred terminal commit rejection, revocation during
authority/receipt lock waits and expired persisted claim recovery without retry.
Strict all-target Clippy passed. These are storage-contract results, not mounted
HTTP/worker/provider acceptance or a claim of metered execution readiness.

The checked-in focused Cargo lock must be an exact registry/version/source/checksum
subset of the root lock. CI fetches this focused graph for the Rust host with
`--locked`; tests remain `--locked --offline`. The mandatory gate has a 40-case floor
and uses the existing PostgreSQL lane, without adding a runner.
