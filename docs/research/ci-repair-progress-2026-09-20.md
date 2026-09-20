# CI repair execution — September 20, 2026

Baseline main: `15a913e27462c939e804c1863d5e4d659e74c93d`.
The owner authorized implementation and pushing an isolated repair branch, and
reported a 60-concurrent-runner quota. No main merge, release or deployment is
part of this work. WebCodex is unavailable in this chat; the host checkout has
not been changed. Work uses a checksum-verified GitHub source snapshot whose Git
tree was verified against the source commit, and authenticated GitHub writes.

## Changes in this iteration

- Independently required headless Rust lint and Node lint/typechecks no longer
  serialize test execution. Strict warnings and all canonical Make gates remain.
- Compile the headless Rust test selection once with pinned Nextest 0.9.145.
  Compare its complete executable/test/ignored inventories against Cargo and
  each actual libtest binary before accepting the archive. Preserve Cargo
  doctests and the separate Tauri checks. Four isolated runners consume the
  same source-bound archive with one test process each and zero retries.
- Run sixteen isolated one-worker browser shards, retaining complete `./src`
  discovery including the nested Next application specs. Use zero retries for
  qualification, first-failure retained traces/videos, and exact result-set
  reconciliation. Missing, duplicated, retried, interrupted or wrong-source
  results cannot pass. The existing 30-minute performance definition is unchanged.
- Correct the shell panel CSS override to the existing 16px card design token,
  marketplace heading/empty-state selectors and the assistant mobile route and
  invalid selector. Login tests now use anonymous contexts and the real login
  form, verify tenant/role from the login response, and reload the session.

Ruling: use sixteen isolated browser shards for the repair run, rather than
stopping at the earlier plan's four/eight comparison. The owner prioritized PR
latency and provided the larger quota. This increases runner minutes and does
not establish a measured speedup. Keep per-runner browser/test concurrency at
one until shared-state and memory behavior are verified. Report hosted timing
rather than assume this graph meets either the 30-minute gate or 15-minute goal.

## Local evidence, not full application certification

The pinned Node 22.22.1 executable and locked npm dependencies were exported by
an isolated read-only hosted preparation job; all archive and lockfile hashes
were verified before local use. No credentials or built application assets were
included. Font assets were omitted from this temporary tooling transfer.

- `node --test scripts/*.test.mjs`: 75 passed, 0 failed, 0 skipped.
- `make lint-node`: ESLint with zero warnings and web/CLI typechecks passed.
- `node scripts/native-contracts.mjs`: 17/17 contract groups passed, including
  complete required-result negative tests and the independent PostgreSQL guard.
- Browser discovery: 1,688 unique tests in 453 files; 436 tests belong to the
  nested Next E2E directory. This is discovery, not execution.
- New evidence/CSS tests were observed failing before their implementations and
  passing afterward. Rust archive inventory/provenance unit tests: 5 passed.

The local container lacks Cargo and Docker. No local full `make lint` or
`make test` pass is claimed. Hosted compilation, archive execution, browser
results, source binding and the 30-minute aggregate must all pass before calling
this repair complete. In particular, the Anthropic guardrail endpoint and other
previously recorded product/browser failures remain under repair in this
iteration; do not infer that the entire failure ledger is closed.
