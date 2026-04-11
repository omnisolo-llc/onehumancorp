1. **Implement Database Migration:**
   - Create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` to add `sync_status` and `last_sync_at` columns. Ensure separate ALTER TABLE statements are used as per SQLite compatibility requirements.
2. **Implement Go Interface:**
   - Create `srcs/server/hub/rag_sync.go` with the required `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` interfaces based on the prompt's specifications.
3. **Add Telemetry:**
   - Create `srcs/server/telemetry/rag_sync_metrics.go` setting up `ragRecordsSyncedTotal` and `ragSyncErrorsTotal` using OpenTelemetry.
   - Update `srcs/server/telemetry/telemetry.go` to invoke metric initialization for the sync logic in `InitWithMeter`.
4. **Unit Tests:**
   - Add tests for interface functionality in `srcs/server/hub/rag_sync_test.go` and for telemetry in `srcs/server/telemetry/telemetry_extra_test.go`.
5. **Update Mission File:**
   - Implement append-only semantic update on `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` switching `status: PENDING` to `status: DONE` and recording agent details.
6. **Pre-commit and Submit:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
