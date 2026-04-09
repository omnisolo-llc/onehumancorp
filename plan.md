1. **Mark Mission File as IN_PROGRESS:**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to have `status: IN_PROGRESS` and `agent: Implementer`.
2. **Create DB Migration:**
   - Create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` extending `swarm_memory_embeddings` (the primary context table based on memory `005_sip.sql`) with `sync_status` and `last_sync_at` columns.
3. **Update Database BUILD.bazel:**
   - Add `migrations/032_hybrid_sync_metadata.sql` to the `embedsrcs` of `db` go_library in `srcs/server/db/BUILD.bazel`.
4. **Implement Go Interfaces and Metrics:**
   - Add opentelemetry metrics to `srcs/server/hub/rag_sync.go`.
   - Add `BUILD.bazel` in `srcs/server/hub` for the package.
5. **Implement Mock/Service Logic for Tests:**
   - Write unit tests in `srcs/server/hub/rag_sync_test.go`.
6. **Pre-commit Steps & Verification:**
   - Run `bazelisk test //srcs/server/hub/... //srcs/server/db/...`.
   - Ensure proper testing, verification, review, and reflection are done.
7. **Mark Mission File as DONE:**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to `status: DONE`.
8. **Submit changes:**
   - Call `submit` tool to push changes.
