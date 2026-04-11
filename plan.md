1. **Mark Mission In Progress**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` status to `IN_PROGRESS` and agent to `Implementer`.
2. **Create Database Migration**
   - Create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` extending `autodream_memories` with `sync_status` and `last_sync_at`.
3. **Update Database BUILD.bazel**
   - Add the new migration to `embedsrcs` in `srcs/server/db/BUILD.bazel`.
4. **Implement Go Interface & Metrics**
   - Create `srcs/server/hub/rag_sync.go` with the requested `RAGSyncService` interface and telemetry metrics.
   - Create `srcs/server/hub/rag_sync_test.go` with mock tests verifying the interface structure.
   - Create `srcs/server/hub/BUILD.bazel` for the new `hub` package defining library and test dependencies.
5. **Run Tests**
   - Run `bazelisk test //srcs/server/hub:hub_test //srcs/server/db:db_test` to verify code changes.
6. **Complete pre commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Mark Mission Done & Submit**
   - Update the mission file to `status: DONE`.
   - Use Git commands to stage and commit changes with a descriptive PR title.
