# Approval list read consistency

Run `bash scripts/approvals-read-contract/run.sh` with
`OHC_APPROVAL_TEST_DATABASE_URL` pointing to the owned local PostgreSQL database
`ohc_approval_test`. CI runs this through `scripts/focused_ci_gate.py`, which
requires at least 12 actual tests and rejects failed, ignored or filtered cases.
The runner gives production configuration an owned temporary home, a public
test signing key and an in-memory fallback database. It clears ambient
database, Redis and signing-file overrides and removes only its temporary
directory on exit. The explicitly supplied approval database remains the sole
PostgreSQL test target.

The probe compiles the production approval router, response types and fixture
boundary. It copies both complete production list-query methods verbatim from
`DepartmentOrchestrator`, and exercises their PostgreSQL reads using actual
committed, concurrent and failed transactions. It records all source hashes and
verifies that focused dependencies match the root lockfile. Seven contracts
cover stale primed pending/history variants, concurrent reads, failed writes,
tenant isolation, pagination/mobile payloads, and private/no-store headers. Five
existing imported cache/fixture-boundary tests run as well.

This is intentionally a read contract. The small orchestrator container exposes
only those production read methods. Decision execution and ledger calls panic;
neither has a success substitute. Authentication is supplied as Claims at the
router boundary. This probe does not prove authentication middleware, decision
execution, provider dispatch, or browser behavior.

Ten additional root-library tests in `approvals_readback_test.rs` call the real
orchestrator decision handler (five SQLite and five explicitly configured
PostgreSQL cases). Run them with `cargo test --locked -p omnisolo --lib
api::agents::approvals::readback_tests -- --test-threads=1` and
the same isolated PostgreSQL URL. The real browser persistence assertion also
checks the POST receipt, committed database state and reloaded HTTP list before
its unchanged control-inventory assertion.

All ten root-library cases are ordinary required tests. Native CI creates the
disposable approval database and passes its URL to `make test-backend`. Locally,
`bash scripts/approvals-read-contract/with-owned-postgres.sh` provisions a fresh
cluster, runs those ten real decision-handler cases and stops its owned cluster.
Pass a command after that wrapper to reuse its owned service for another gate.
The existing fourteen ignored-test gates and their selectors are unchanged.
