1. Create `srcs/server/db/migrations/032_hybrid_mcp_rag_sync.sql`
   - Use `ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';`
   - Use `ALTER TABLE autodream_memories ADD COLUMN last_sync_at TIMESTAMP NULL;`
   - These are compatible with both PostgreSQL and SQLite. (Since `IF NOT EXISTS` is not supported for `ADD COLUMN` in SQLite, we won't use it).
2. Update `srcs/server/db/BUILD.bazel` to include `migrations/032_hybrid_mcp_rag_sync.sql` in `embedsrcs`.
3. Create `srcs/server/hub/rag_sync.go` with `RAGSyncRecord` and `RAGSyncService` interfaces, as well as `SyncStatus` constants.
4. Add basic telemetry inside `srcs/server/hub/rag_sync.go` (if feasible or keep as interface-only, but the mission asks for counters so we might need a small implementation wrapper or just the metrics definition). We'll add the global metric definitions `rag_records_synced_total` and `rag_sync_errors_total` using OpenTelemetry in `srcs/server/hub/rag_sync.go` or a mock file. Wait, the mission says: "In `srcs/server/hub/rag_sync.go` or a dedicated telemetry file, add OpenTelemetry counters...". I will add them in `rag_sync.go`.
5. Create `srcs/server/hub/rag_sync_test.go` to mock the interface and verify data flow logic. I'll need to define a mock service and write a basic test.
6. Create `srcs/server/hub/BUILD.bazel` to build the new package `hub`. Include dependencies to `@io_opentelemetry_go_otel_metric//:metric` and `@io_opentelemetry_go_otel//:otel` for the metrics.
7. Change the mission status to `DONE` in `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done (using `bazelisk test //srcs/server/hub/...` and `bazelisk build //srcs/server/db/...`).
9. Submit the code.
