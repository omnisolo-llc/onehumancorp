# Checkpoint restore safety contract

The required run below compiles the complete production
checkpointer module and the exact Agent Protocol restore method against real
private temporary Git repositories. Ralph progress serialization types are
copied verbatim from their production module. Receiver-only protocol plumbing
is minimal and synthetic; this suite does not certify the full Agent runtime,
its tenant authorization, or mounted endpoint availability.

All tests, including PostgreSQL regressions, require an explicit disposable loopback database:

```
OHC_CHECKPOINT_TEST_DATABASE_URL=postgresql://postgres@127.0.0.1:55434/ohc_checkpoint_test \
  ./scripts/checkpoint-restore-contract/run.sh --test-threads=1
```

Tests use a single connection and private temporary tables, with the search
path restricted to `pg_temp`; they do not create, clear, or drop public tables.
The URL must identify the specifically named disposable test database.
The runner also requires Python 3 with PyYAML for the CI fixture wiring check;
the native CI job installs `python3-yaml` explicitly.
A missing database is a failing prerequisite, never a silent passing test.

The source manifest is checked before and after each run. Full native
builtin-agent compilation and repository `make lint` / `make test` remain
separate acceptance gates.
