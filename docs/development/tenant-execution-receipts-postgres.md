# PostgreSQL receipt parity and mounted dispatch

Status: receipt persistence and mounted funded text dispatch are implemented and
focused-test verified. Full integrated workspace and hosted CI acceptance remain
separate requirements. See [the current dispatch contract](durable-tenant-text-dispatch.md)
for operator configuration, required request IDs and conservative accounting.

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
checks must refuse private output if authority has been lost. The canonical auth revocation INSERT/UPDATE trigger takes the same tenant/JTI
transaction advisory identity used by the final receipt shared-lock fence. A fresh
authority read and COMMIT occur inside that fence; no fence is held across provider I/O.

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

## Usage schema checkpoint (2026-10-03)

At the usage schema checkpoint, the source-bound receipt harness passed 53 cases with zero ignored or
filtered, including the 40-case revocation/storage foundation, retained SQLite
lifecycle cases, exact prepared-request bounds, explicit funding policy and an
actual forced-RLS PostgreSQL worker/provider/ledger transaction. The separate
mounted workflow gate passed 98 and builder compatibility gate passed 50; all
three passed strict all-target Clippy. SQLite schema enforcement passes nine cases.
These owned database/loopback results do not certify a live paid vendor or the
full integrated workspace. No production account or credit was changed.

The checked-in focused Cargo lock must be an exact registry/version/source/checksum
subset of the root lock. CI fetches this focused graph for the Rust host with
`--locked`; tests remain `--locked --offline`. The mandatory gate has a 56-case floor
and uses the existing PostgreSQL lane, without adding a runner.

The subsequent cached-input tariff fix passed all 99 workflow cases and strict
all-target Clippy after a genuine underreservation RED. Its shared pricing case
raises the required receipt and builder inventories to 54 and 51; their full
PostgreSQL reruns remain part of integrated validation.

Two root-discovered worker lifecycle cases retain the existing focused assertions
and directly exercise worker draining and cancellation against real SQLite
receipts. They raise the required workflow/receipt/builder inventories to
101/56/53. The controlled test inference returns only failure and records when its
future is cancelled; it supplies no fabricated provider output or billing usage.
