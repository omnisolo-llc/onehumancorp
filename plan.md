1. **Create the SQL Migration (`032_hybrid_rag_sync.sql`)**
   - We need to add the synchronization fields (`sync_status`, `last_sync_at`) to the memory table. Since we don't have a specific `rag_memories` table but we do have `autodream_memories` serving as the persistent vector database, we'll apply these columns to `autodream_memories`.
   - Update `srcs/server/db/BUILD.bazel` to include this migration.

2. **Define Go Interfaces (`srcs/server/hub/rag_sync.go`)**
   - Create `srcs/server/hub/rag_sync.go` and implement the `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` interfaces as defined in the mission file.
   - We will need to add an implementation `ragSyncServiceImpl` for `RAGSyncService` that interacts with the `db.Provider` database instance.

3. **Metrics & Observability**
   - Use the `go.opentelemetry.io/otel/metric` package to add `rag_records_synced_total` and `rag_sync_errors_total` metrics into the implementations logic.
   - Note: In Cloud-Native mode, `telemetry.BufferMetricFunc` is nil. Ensure that synchronous API endpoints and telemetry functions (like latency recorders) check for this and route metrics directly to OpenTelemetry when the buffer is unavailable. (From Memory)

4. **Add Unit Tests (`srcs/server/hub/rag_sync_test.go`)**
   - Write tests validating that the implementation successfully queries pending records, marks them as synced, and appropriately processes incoming sync payloads.

5. **Run Pre-Commit steps**
   - Execute all requisite bazelisk commands to format code and run the tests.

6. **Submit**
   - Submit the PR once all testing passes and the `.agent-task/missions/` file is marked `DONE`.
