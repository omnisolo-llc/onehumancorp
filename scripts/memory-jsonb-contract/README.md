# Native consolidated-memory PostgreSQL contract

This gate compiles the complete production `VectorRepository`, unchanged, from
`src/agents/builtin/memory_store.rs`. It extracts the actual active and legacy
`consolidated_memory` table DDL and uses PostgreSQL with the real pgvector
extension. No fake vector type, copied persistence implementation, external LLM,
provider account, or production database is used.

Prerequisites: the repository's pinned Rust toolchain/dependency cache and an
explicit disposable loopback PostgreSQL database named `ohc_*_test` with pgvector
available. The account must be able to create private fixture schemas and the
extension. Every PostgreSQL case uses its own schema; SQLite cases use memory.

```sh
OHC_MEMORY_TEST_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/ohc_memory_test \
  python3 scripts/focused_ci_gate.py memory-jsonb-contract
```

The runner verifies dependency alignment with root Cargo.lock and rejects changed
source during execution. CI runs all 27 cases without ignores or filters:

- Native JSONB writes: nested/unicode values, SQL NULL, JSON null, exact large
  numeric values, malformed-input rejection without mutation or data disclosure
- Four metadata read paths: ID, recent list, semantic search, conflict pairs
- Corrupt-row errors rather than missing/partial results in ID/list/conflict reads,
  fallible PostgreSQL semantic-search timestamp decoding, and NULL embedding errors
- Same-ID cross-tenant conflicts reject atomically in PostgreSQL and SQLite
- Owner-override authority, legacy PostgreSQL TEXT table preservation when the
  active CREATE IF NOT EXISTS migration is applied, and SQLite free-text metadata
- Byte-preserving legacy PostgreSQL TEXT writes, including free text and JSON
  whitespace, and the repository owner-override read/edit/write sequence
- Atomic conflict resolution: write/validation/delete failure rolls back all
  changes; an actual foreign-tenant loser cannot be deleted using forged input

The legacy check applies the actual table migration statement, not the entire
historical or active migration chain. Full migration/application suites remain
separate acceptance gates. PostgreSQL metadata validation is selected from the
actual table column type on the same transaction connection: JSONB requires JSON;
legacy PostgreSQL and SQLite TEXT preserve arbitrary text. No deployed schema or
existing data is rewritten. The owner-override witness tests repository operations;
it does not claim the historical memory API is mounted in the application.

The original hosted failure occurred twice in workflow run 37153128293, job
111292482176, on aa377e53: PostgreSQL rejected parameter $12 as TEXT for JSONB.
