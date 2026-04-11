1. **Database Migration for Hybrid MCP RAG Protocol**
   - Create a new migration file `srcs/server/db/migrations/032_hybrid_rag_sync.sql`.
   - Add columns `sync_status` and `last_sync_at` to the `swarm_memory` and `swarm_memory_embeddings` tables. Use standard SQL for SQLite compatibility: `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMP NULL`.
   - Update `srcs/server/db/BUILD.bazel` to include `migrations/032_hybrid_rag_sync.sql` in `embedsrcs`.

2. **Go Interface Definition for RAGSyncService**
   - Create `srcs/server/hub/rag_sync.go` (in package `hub`).
   - Define types `SyncStatus` (`pending`, `synced`, `error`), `RAGSyncRecord`.
   - Note the constraint: `vector_embedding` should be mapped to `[]byte` based on memory instructions.
   - Define the `RAGSyncService` interface with `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync` per the mission prompt.
   - Create a concrete implementation `ragSyncService` that interacts with `db.Provider` and satisfies the interface. We must not submit empty shells.

3. **Concrete Implementation of RAGSyncService**
   - Implement `FetchPendingSyncs`: query `swarm_memory_embeddings` for `sync_status = 'pending'`, fetching `memory_id`, `context`, `vector_embedding`, `sync_status`, `last_sync_at`. Note: since SQLite vector bindings use `BLOB` vs Postgres `BYTEA`, reading to `[]byte` is safe.
   - Implement `MarkSynced`: update `sync_status = 'synced'` and `last_sync_at = CURRENT_TIMESTAMP` for given IDs.
   - Implement `ProcessIncomingSync`: branch by `IsSQLite()`. On SQLite, use query/insert or upsert compatible with SQLite (`INSERT ... ON CONFLICT(memory_id) DO UPDATE SET context=excluded.context, vector_embedding=excluded.vector_embedding, sync_status='synced', last_sync_at=CURRENT_TIMESTAMP`). Since `swarm_memory_embeddings` has `memory_id` PRIMARY KEY, Postgres supports `ON CONFLICT (memory_id) DO UPDATE SET ...`. We'll write an upsert query that works across both or handle branching if needed.

4. **Metrics & Observability**
   - Add global variables `RagRecordsSyncedCounter` and `RagSyncErrorsCounter` of type `metric.Int64Counter` in `srcs/server/telemetry/telemetry.go`.
   - Inject initialization into `InitWithMeter` after `var errs []error`.
   - Add helper functions `RecordRagRecordsSynced` and `RecordRagSyncError` to use these counters.
   - Wait, remember the instruction: "When adding OpenTelemetry metrics meant for cross-package use, ensure the variable names in `telemetry.go` are exported (capitalized). To avoid nil pointer panics in unit tests for packages importing these metrics, initialize dummy metrics using `noop.NewMeterProvider().Meter("test").Int64Counter(...)`." I should do this for the package level declarations or inside tests? The instruction says: "ensure the variable names in telemetry.go are exported (capitalized)." and "initialize dummy metrics...". Actually, the global variables are currently declared but typically initialized in `InitWithMeter`. Wait, no, the memory says: "initialize dummy metrics using noop.NewMeterProvider().Meter("test").Int64Counter(...)". I'll do this in `rag_sync_test.go` or directly as global defaults in `telemetry.go`. Looking at `telemetry.go`, many are global variables.

5. **Test Implementation**
   - Write unit tests in `srcs/server/hub/rag_sync_test.go` utilizing an in-memory SQLite provider (`db.NewSQLiteProvider(":memory:")`).
   - Create the necessary tables (`swarm_memory_embeddings`) and mock data.
   - Ensure the tests achieve good coverage for `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync`.

6. **Mark Mission Done**
   - Rename `2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to `2026-04-07T08-02-24Z.md` if needed? Wait, the memory says "When claiming a pending mission from .agent-task/missions/ that incorrectly includes a descriptive suffix in its filename ... you must rename it using mv to strictly match the ISO-8601 format requirement ({timestamp}.md) during your status update."
   - Update `2026-04-07T08-02-24Z.md` to `status: DONE` and assign `agent: Jules`.

7. **Pre-commit and PR Submission**
   - Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.
   - Submit the PR with branch name `hybrid-rag-sync` and proper title `🧹 Maintainer: Implement Hybrid MCP RAG Sync Protocol`. (Wait, "Pull Requests authored by the Maintainer agent must be explicitly titled '🧹 Maintainer: <actual feature description>'.")
