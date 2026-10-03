# Actual spawned operations worker contract

This required gate compiles the unchanged production worker module, including
`start()` and its real `tokio::spawn` Send boundary, together with the actual
storage and fairness modules. Eleven tests are required: the original five
SQLite storage regressions, two polling regressions, the three original PostgreSQL
storage/row-lock cases, and a real spawned-worker SQLite/cache case. A helper-only test cannot certify this gate.

The preparation script extracts the exact production DB/DbStore declarations,
feed DTOs and cache getter, links the real common authentication dependency,
and includes the real HybridCache implementation (excluding its unrelated unit
test module). It does not replace the database, cache or authorization behavior.
The explicit test fixture configures the actual cache's supported memory-only
mode and an in-memory SQLite database. The PostgreSQL branch is compiled here;
the original PostgreSQL storage and fairness races run here on a disposable
loopback database; restricted-role full-server runtime remains a separate gate.

The test executes the actual `start()` loop and observes both a persisted factual
inventory alert and actual tagged-cache invalidation. The test runtime cancels
its spawned tasks at teardown. No provider or remote service is contacted.

`run.sh` uses the committed root-matching focused lock, offline execution and
source fingerprints. CI separately fetches only the locked host dependency
closure. Local runs require the shared native slot and the 900 MiB disk guard.
Whole-server and packaged-backend compilation remain mandatory acceptance.
