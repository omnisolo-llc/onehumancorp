1. **Update mission status**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to change `status` from `PENDING` to `IN_PROGRESS` and add `agent: Implementer`.
   - Verify using `cat` inside `run_in_bash_session` to check the mission file.

2. **Add Hybrid Sync Metadata Migration**
   - Create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` to add `sync_status` and `last_sync_at` columns to `autodream_memories`.
   - Update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.
   - Verify using `ls srcs/server/db/migrations/032_hybrid_sync_metadata.sql` inside `run_in_bash_session`.

3. **Define RAG Sync Go Interface**
   - Create `srcs/server/hub/rag_sync.go` with the requested `RAGSyncService` interface, structs, and OpenTelemetry metrics (`rag_records_synced_total`, `rag_sync_errors_total`).
   - Create `srcs/server/hub/rag_sync_test.go` with a basic mock implementation and test.
   - Create `srcs/server/hub/BUILD.bazel` to define `hub` and `hub_test` targets.
   - Verify using `bazelisk build //srcs/server/hub/...` and `bazelisk test //srcs/server/hub/...` inside `run_in_bash_session`.

4. **Mark mission as complete**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to change `status` from `IN_PROGRESS` to `DONE`.
   - Verify using `cat` inside `run_in_bash_session` to check the final mission status.

5. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
