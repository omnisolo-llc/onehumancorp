1. **Update Mission File:**
   - I have marked the mission file `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` as `IN_PROGRESS` and assigned it to me.
2. **Implement DB Migration:**
   - I have created `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` which adds `sync_status` and `last_sync_at` columns to `autodream_memories`.
   - I have updated `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.
3. **Implement Go Hub Interface:**
   - I have created `srcs/server/hub/rag_sync.go` defining the `RAGSyncRecord` and `RAGSyncService` interface.
   - Included OpenTelemetry metric setup as required.
   - Added unit tests in `srcs/server/hub/rag_sync_test.go` and configured `BUILD.bazel` for `//srcs/server/hub`.
4. **Testing:**
   - I have verified tests pass with `bazelisk test //srcs/server/hub/...` and `bazelisk test //srcs/server/db/...`.
5. **Mark Mission as DONE:**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to `status: DONE`.
6. **Pre-commit Steps:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. **Submit Changes:**
   - Submit the PR using the `submit` tool.
