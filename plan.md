1. **Database Migration**
   - Create `srcs/server/db/migrations/032_hybrid_rag_sync.sql`.
   - Add `sync_status` and `last_sync_at` columns to `swarm_memory_embeddings`.
   - Update `srcs/server/db/BUILD.bazel` to include this migration.
2. **Go Interface Definition and Implementation**
   - Create `srcs/server/hub/rag_sync.go` with the requested `RAGSyncService` interface and structs.
   - Implement the methods for the service, keeping in mind the memory: *When implementing a mission that defines an interface (e.g., a service interface), you must also provide the concrete implementation containing the actual business logic (e.g., database operations), rather than only submitting the interface definition and mock structs used for testing.*
   - Add OpenTelemetry metrics `rag_records_synced_total` and `rag_sync_errors_total` as instructed in the memory: *When initializing OpenTelemetry metrics in Go, declare and instantiate them directly as global variables within a `var` block (e.g., `var MetricName, _ = meter.Int64Counter(...)`) rather than inside an `init()` function.*
3. **Tests**
   - Create `srcs/server/hub/rag_sync_test.go` with tests mocking the database and testing the flow.
   - Follow memory rules for SQLite compatibility and vector embedding serialization (use `binary.LittleEndian`).
   - Run tests `bazelisk test //srcs/server/...` to verify.
4. **Mission Status Update**
   - Update `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to `status: DONE`.
   - Claim it first as `status: IN_PROGRESS` and `agent: Researcher`. Wait, I am `Implementer`, but I should update the agent role properly if needed.
