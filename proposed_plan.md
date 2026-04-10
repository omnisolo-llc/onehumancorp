1. **Claim the Correct Mission**:
   - `sed -i 's/status: PENDING/status: IN_PROGRESS/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   - `sed -i 's/agent: Researcher/agent: Implementer/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`

2. **Database Migration**:
   - Create `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql` with:
     ```sql
     ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
     ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
     ```
   - Add `"migrations/032_add_hybrid_sync_metadata.sql",` to `embedsrcs` in `srcs/server/db/BUILD.bazel`.

3. **Go Implementation**:
   - Ensure `srcs/server/hub` directory exists (I just created it and its `BUILD.bazel`).
   - Create `srcs/server/hub/rag_sync.go` defining the `RAGSyncService` interface and structs exactly as described in the mission doc.
   - Create `srcs/server/hub/rag_sync_test.go` with unit tests for the interface.
   - Run `export PATH="$PATH:$HOME/go/bin" && bazelisk run //:gazelle` to update `BUILD.bazel` if needed. (Oh wait, Gazelle might remove it if I don't set it up right, but my `BUILD.bazel` looks good. I will just use `BUILD.bazel` I created).

4. **Add Metrics**:
   - Inside `srcs/server/hub/rag_sync.go`, add OpenTelemetry global vars `var ragRecordsSyncedTotal metric.Int64Counter` and `ragSyncErrorsTotal` using `otel.Meter("github.com/onehumancorp/mono/srcs/server/hub").Int64Counter(...)`.
   - Update `deps` in `srcs/server/hub/BUILD.bazel` to include `@io_opentelemetry_go_otel//:otel` and `@io_opentelemetry_go_otel_metric//:metric`. (Already did that in previous step).

5. **Verify Changes**:
   - `ls -l srcs/server/hub/` to verify files.
   - `grep "032_add_hybrid_sync_metadata.sql" srcs/server/db/BUILD.bazel` to verify.
   - `export PATH="$PATH:$HOME/go/bin" && bazelisk test //srcs/server/hub/... //srcs/server/db/...` to run tests and ensure no compilation errors.

6. **Complete Mission**:
   - `sed -i 's/status: IN_PROGRESS/status: DONE/g' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`

7. **Pre commit steps**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

8. **Submit**:
   - Submit the PR.
