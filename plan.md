1. **Explore `srcs/server/db/migrations` and find the correct number for the new migration.**
   - We see the highest migration number is `031`.
   - Use tool `write_file` to create `srcs/server/db/migrations/032_hybrid_rag_sync.sql`. It will use `ALTER TABLE ADD COLUMN` for `sync_status` and `last_sync_at` to the `autodream_memories` table. We know `autodream_memories` holds the RAG context from the `024_autodream_memories.sql` migration.
   - Verify: `cat srcs/server/db/migrations/032_hybrid_rag_sync.sql`
2. **Update `srcs/server/db/BUILD.bazel` to include `032_hybrid_rag_sync.sql` in `embedsrcs`.**
   - Use tool `replace_with_git_merge_diff` on `srcs/server/db/BUILD.bazel` to add `"migrations/032_hybrid_rag_sync.sql"` to the `embedsrcs` list.
   - Verify: `grep "032_hybrid_rag_sync.sql" srcs/server/db/BUILD.bazel`
3. **Implement `srcs/server/hub/rag_sync.go` and `BUILD.bazel`.**
   - Use tool `write_file` to create `srcs/server/hub/rag_sync.go`. It will define the types and interfaces requested in the mission: `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService`.
   - Use tool `write_file` to create `srcs/server/hub/BUILD.bazel` for the `hub` package.
   - Verify: `cat srcs/server/hub/rag_sync.go` and `cat srcs/server/hub/BUILD.bazel`
4. **Update `srcs/server/telemetry/telemetry.go`.**
   - Use tool `replace_with_git_merge_diff` to add OpenTelemetry counters: `RagRecordsSyncedTotal` and `RagSyncErrorsTotal` inside `srcs/server/telemetry/telemetry.go`. Also, add helper functions `RecordRagRecordSynced` and `RecordRagSyncError` to increment these metrics.
   - Verify: `grep -A 5 "RagRecordsSyncedTotal" srcs/server/telemetry/telemetry.go`
5. **Implement Unit Tests.**
   - Use tool `write_file` to create `srcs/server/hub/rag_sync_test.go` with simple mock-based tests to verify the interface data flow.
   - Run tests: `bazelisk test //srcs/server/hub/...` in bash session.
6. **Mark Mission as Completed.**
   - Use tool `replace_with_git_merge_diff` to update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to change `status: PENDING` to `status: DONE` and assign the correct agent.
   - Verify: `head -n 5 .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
7. **Pre Commit.**
   - Run `pre_commit_instructions` tool to complete pre commit steps ensuring all code checks and reflection are finalized.
