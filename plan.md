1.  **Add Database Migration for Hybrid Sync Metadata**
    *   Create a new migration file `srcs/server/db/migrations/032_hybrid_sync_metadata.sql`.
    *   Add `sync_status` (VARCHAR/TEXT, default 'pending') and `last_sync_at` (TIMESTAMPTZ/TIMESTAMP) columns to `swarm_memory_embeddings`.
    *   Update `srcs/server/db/BUILD.bazel` to include this new migration file in `embedsrcs`.
2.  **Define Go Interface for RAG Sync Service**
    *   Create a new file `srcs/server/hub/rag_sync.go`.
    *   Define the `SyncStatus` constants (`pending`, `synced`, `error`).
    *   Define the `RAGSyncRecord` struct with fields corresponding to the database columns. Remember to use `[]byte` for `vector_embedding` based on memory.
    *   Define the `RAGSyncService` interface with methods: `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync`.
    *   Implement the actual database logic for these methods using the `db.Provider` interface (as instructed in memory: "use the abstract db.Provider interface for database dependency injection rather than a concrete struct like *db.DB."). The methods must fetch and upsert records. In `ProcessIncomingSync`, branch using `provider.IsSQLite()` if needed, but since it's just `INSERT ... ON CONFLICT` for SQLite and Postgres, we might be able to handle it seamlessly, or explicitly check for existence and then update/insert.
3.  **Add OpenTelemetry Metrics**
    *   Add `RagRecordsSyncedTotal` and `RagSyncErrorsTotal` metrics in `srcs/server/telemetry/telemetry.go`.
    *   Export these variables and initialize them properly in `InitWithMeter` to avoid nil pointer panics in tests.
4.  **Write Tests for RAG Sync Service**
    *   Create `srcs/server/hub/rag_sync_test.go`.
    *   Write unit tests that mock the database provider or use a SQLite test provider to verify `FetchPendingSyncs`, `MarkSynced`, and `ProcessIncomingSync` logic.
5.  **Update Build Files**
    *   Run `bazelisk run //:gazelle -- update srcs/server/hub`
    *   Run `bazelisk run //:gazelle -- update srcs/server/telemetry`
6.  **Run Tests & Verification**
    *   Run `bazelisk test //srcs/server/hub/...`
    *   Run `bazelisk test //srcs/server/telemetry/...`
    *   Run `bazelisk test //srcs/server/db/...` to verify migrations load properly.
7.  **Pre-commit Step**
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
8.  **Complete Mission & PR Submission**
    *   Mark the mission file `2026-04-07T08-02-24Z.md` as `status: DONE`.
    *   Submit the code with a descriptive PR title.
