1. **Database Migration**
   - Create a new migration file `srcs/server/db/migrations/032_hybrid_sync_metadata.sql`
   - Add `sync_status` and `last_sync_at` columns to `autodream_memories` table
   - `sync_status VARCHAR(50) DEFAULT 'pending'`
   - `last_sync_at TIMESTAMPTZ NULL`
   - Update `srcs/server/db/BUILD.bazel` to include this new migration file in `embedsrcs`

2. **Verify Database Migration**
   - Run `ls srcs/server/db/migrations/ | tail -n 5` and `cat srcs/server/db/BUILD.bazel` to verify the creation and modification.

3. **Go Interface Definition**
   - Create `srcs/server/hub`
   - In `rag_sync.go`, define the `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` interfaces as requested.
   - Add OpenTelemetry metrics `rag_records_synced_total` and `rag_sync_errors_total` to the package.
   - Create `srcs/server/hub/BUILD.bazel` with appropriate dependencies.

4. **Verify Go Interface Definition**
   - Run `ls -la srcs/server/hub/` and `cat srcs/server/hub/rag_sync.go` to verify the files were written correctly.

5. **Unit Tests**
   - Write tests: `rag_sync_test.go` and `rag_sync_impl_test.go` for the mock and concrete implementation.

6. **Verify and Run Tests**
   - Run tests across all affected packages: `bazelisk test --config=local //srcs/server/hub/... //srcs/server/db/...`

7. **Pre-commit Steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit**
   - Mark the mission file as done.
   - Commit files.
   - Invoke the `submit` tool to finish.
