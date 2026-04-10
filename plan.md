1. **Update .agent-task/missions**
   - Pick the pending mission: `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
   - Update its status to `IN_PROGRESS` and `agent: Jules`.

2. **Database Migration**
   - Create `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql` based on the prompt instructions. Add `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMP NULL` to `swarm_memory` table (which is the primary context table per our earlier `005_sip.sql` check, since `rag_memories` isn't the primary one). Note: Memory "The primary context table for storing and synchronizing agent RAG memories in the OHC backend is `swarm_memory` (defined in `005_sip.sql`), not hypothetical tables like `rag_memories` or `autodream_memories`."
   - Update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.

3. **Metrics & Observability Updates**
   - Update `srcs/server/telemetry/telemetry.go` to add OpenTelemetry counters for `ragRecordsSyncedTotal` and `ragSyncErrorsTotal`. Ensure they are initialized in `InitWithMeter` as per Memory instruction: "When updating OpenTelemetry metrics in `srcs/server/telemetry/telemetry.go`, declare new global counters as unexported variables (e.g., `ragRecordsSyncedTotal`) and initialize them inside the `InitWithMeter` function using the provided `m mockableMeter` (e.g., `m.Int64Counter`), rather than calling methods on a global meter directly."
   - Add recording helper functions: `RecordRAGSyncRecord(ctx context.Context)` and `RecordRAGSyncError(ctx context.Context)`.

4. **Go Interface Definition**
   - Create `srcs/server/hub/rag_sync.go` defining the interfaces from the prompt.
   - Create `srcs/server/hub/rag_sync_test.go` and implement basic tests with mocks.
   - Create `srcs/server/hub/BUILD.bazel` to define the target `go_library` and `go_test` for the `hub` package.

5. **Complete pre-commit steps**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

6. **Submit Changes**
   - Submit the changes marking the mission as `DONE` and PR.
