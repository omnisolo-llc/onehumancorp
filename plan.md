1. **Database Migration**
   - Read the existing migrations to determine the next number. `031_agent_missions_updated_at.sql` is the last one, so I'll create `032_add_hybrid_sync_metadata.sql`.
   - The migration will add `sync_status` and `last_sync_at` to the `autodream_memories` table using standard SQL `ALTER TABLE ADD COLUMN`.

2. **Go Interface Definition**
   - Create the `srcs/server/hub/rag_sync.go` file.
   - Define `SyncStatus`, `RAGSyncRecord`, and the `RAGSyncService` interface as specified in the mission document.

3. **Metrics & Observability**
   - Add OpenTelemetry metrics for `RAGRecordsSyncedTotal` and `RAGSyncErrorsTotal` in `srcs/server/telemetry/telemetry.go`. Both should be `metric.Int64Counter`.
   - Update `InitWithMeter` in `telemetry.go` to initialize these counters.

4. **Verification**
   - Create `srcs/server/hub/rag_sync_test.go` to test basic data flow logic using a mock implementation.
   - Run `bazelisk test //srcs/server/...` to make sure all tests pass.

5. **Pre-commit step**
   - I will call `pre_commit_instructions` to ensure proper testing, verification, review, and reflection are done.

6. **Submit**
   - Update mission status to `DONE`.
   - Commit and submit.
