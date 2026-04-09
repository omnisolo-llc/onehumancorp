1. **Database Migration**
   - Use `write_file` to create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` containing `ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';` and `ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;`.
   - Use `replace_with_git_merge_diff` to update `srcs/server/db/BUILD.bazel` to include `"migrations/032_hybrid_sync_metadata.sql"` in the `embedsrcs` block.
   - Use `read_file` to verify `srcs/server/db/BUILD.bazel` is updated correctly.
   - Run `bazelisk test //srcs/server/db/...` via `run_in_bash_session` to verify the DB package still builds successfully.
2. **Go Interface Definition**
   - Use `write_file` to create `srcs/server/hub/rag_sync.go` implementing `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` as explicitly specified in the mission file `2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
   - Use `write_file` to create `srcs/server/hub/BUILD.bazel` to define a `go_library` for the `hub` package.
   - Use `read_file` to verify the newly created `srcs/server/hub/rag_sync.go` and `srcs/server/hub/BUILD.bazel`.
3. **Telemetry Metrics**
   - Use `replace_with_git_merge_diff` to modify `srcs/server/telemetry/telemetry.go` by declaring `RAGRecordsSyncedTotal` and `RAGSyncErrorsTotal` of type `metric.Int64Counter` in the `var` block and initializing them using `m.Int64Counter` inside `InitWithMeter(m mockableMeter)` function, providing string labels `"rag_records_synced_total"` and `"rag_sync_errors_total"` respectively.
   - Use `read_file` to verify `srcs/server/telemetry/telemetry.go` is modified correctly.
   - Use `run_in_bash_session` to run `bazelisk test //srcs/server/telemetry/...` to verify the package builds and tests pass.
4. **Testing**
   - Use `write_file` to create `srcs/server/hub/rag_sync_test.go` and implement a mock of `RAGSyncService` using `package hub_test` or `package hub`.
   - Use `replace_with_git_merge_diff` to update `srcs/server/hub/BUILD.bazel` to add a `go_test` target.
   - Use `read_file` to verify `srcs/server/hub/rag_sync_test.go` and `srcs/server/hub/BUILD.bazel` are modified correctly.
   - Use `run_in_bash_session` to run `bazelisk test //srcs/server/hub/...`
5. **Update Mission and Observability Files**
   - Use `run_in_bash_session` to execute `sed -i 's/status: PENDING/status: DONE/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to update the mission status.
   - Use `write_file` to create `.agent-task/status/$(date +%s).yml` containing a healthy heartbeat metric (`type: observability_heartbeat`, `agent: Guide`, `role: IMPLEMENTER`, `status: HEALTHY` and a `metrics` block).
   - Use `run_in_bash_session` to run `cat` or `ls` to verify the files were updated.
6. **Final Verification**
   - Run `bazelisk test //...` via `run_in_bash_session` to ensure the entire workspace passes.
7. **Pre-Commit**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
8. **Submission**
   - Use `submit` to push the branch titled "🗺️ Guide: Hybrid MCP RAG Protocol".
