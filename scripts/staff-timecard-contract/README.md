# Staff timecard regression gate

This narrow gate compiles the complete production `staff_timecards.rs` and
`sync_transaction.rs` modules with the actual native timecard tests and real
`server_auth` canonical authority implementation. It never substitutes clock
persistence, receipt validation, or commit behavior.

The adapter provides only the `DB` fields and the timecard portion of the staff
router. The unchanged GET handler, method route, production parent mount,
`TimecardAccess` configuration, bearer middleware, SQLite bootstrap statements,
and SQLite additive upgrade are source-bound exact fragments. Both PostgreSQL
migrations are executed by the actual fixture. `source-manifest.json` records
whole-file hashes and each fragment's byte/line boundaries, literal, and hash.
The manifest must stay identical across execution.

The checked-in lock was derived from the existing shipping contract's cached
feature graph, with the native tests' already-cached `futures` and `jsonwebtoken` dependencies. Some direct
compatibility dependencies preserve that graph; removing them can cause an
unnecessary native rebuild. `verify_lock.py` rejects package identities absent
from the root lock. Fetching and execution both use `--locked`; the runner is
also offline and never silently regenerates the lock.

## Run

Use an owned, disposable loopback PostgreSQL database named `ohc_*_test` and a
fixture role permitted to create/drop test schemas and roles. No customer or
shared development database is acceptable. The runner rejects URL options,
remote hosts, unavailable PostgreSQL, and missing prerequisites before Cargo.
It isolates application configuration in a temporary home it alone owns.

```sh
bash scripts/staff-timecard-contract/fetch.sh
OHC_CLOCK_TEST_DATABASE_URL=postgres://fixture:password@127.0.0.1:5432/ohc_clock_timecard_test \
  CARGO_TARGET_DIR=target make test-staff-timecards
```

All PostgreSQL and SQLite scenarios execute without name filters or ignored
tests, plus shared commit-classification and actual SQLite-upgrade regressions.
Both the complete native workspace lane and the required `postgres-security`
lane create a dedicated timecard database. The focused lane fetches
the locked graph, and retains normal focused-gate result/log/source artifacts.
Fixture failures are failures, never skips. The focused gate does not replace
repository `make lint` or `make test`, provider verification, or deployment.
