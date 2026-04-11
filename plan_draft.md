1. **Explore & Define the Schema Updates**
   - The primary context table for RAG appears to be `swarm_memory_embeddings` (since it holds the context and `vector_embedding`), and maybe `swarm_memory`. We will add the sync columns to `swarm_memory_embeddings` as it most closely matches the prompt's `RAGSyncRecord`.
   - The new columns needed are:
     `sync_status VARCHAR(50) DEFAULT 'pending'`
     `last_sync_at TIMESTAMP NULL`
   - Create a new migration file `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` extending `swarm_memory_embeddings`.
   - Update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.

2. **Implement Go Interfaces & Service (`srcs/server/hub/rag_sync.go`)**
   - Wait, `srcs/server/hub` directory doesn't exist yet! We need to create it. `mkdir -p srcs/server/hub`.
   - Create `srcs/server/hub/rag_sync.go`.
   - Define `SyncStatus`, constants, `RAGSyncRecord` and `RAGSyncService` interface as requested.
   - Implement the service struct `RAGSyncServiceImpl` that satisfies the interface.
   - Inject the database provider (`db.Provider`).
   - `FetchPendingSyncs`: Query `swarm_memory_embeddings` where `sync_status = 'pending'`, limit `limit`. Note that vector in db is BYTEA/BLOB, map it to `[]byte` then convert back to `[]float32` if needed, but per memory: "When mapping the vector_embedding field from the swarm_memory_embeddings table in Go, strictly use []byte to properly interface with the database's BYTEA (PostgreSQL) or BLOB (SQLite) types, rather than []float32." Wait, the prompt says: `Vector []float32 // Convert to string internally for SQLite compat if needed`. I will stick to what the prompt interface specifies for the struct definition, but when querying/saving, handle the DB's `[]byte`.
   - `MarkSynced`: Update `swarm_memory_embeddings` SET `sync_status = 'synced'`, `last_sync_at = CURRENT_TIMESTAMP` WHERE `memory_id` in `ids`.
   - `ProcessIncomingSync`: Upsert incoming records into `swarm_memory_embeddings`. Make sure to use UPSERT logic: `INSERT ... ON CONFLICT (memory_id) DO UPDATE SET ...`

3. **Metrics & Observability**
   - Modify `srcs/server/telemetry/telemetry.go` to add `RAGRecordsSyncedTotal` and `RAGSyncErrorsTotal` as OpenTelemetry Int64Counters.
   - Ensure they are exported.
   - Initialize them correctly in `InitWithMeter` *after* the `var errs []error` declaration.
   - Use these metrics in `rag_sync.go`.

4. **Unit Tests**
   - Create `srcs/server/hub/rag_sync_test.go`.
   - Write tests mocking the DB or using SQLite memory DB via `NewSqliteProvider` to test the methods.

5. **Mark Mission as DONE & Status/Memory Files**
   - Mark `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` as DONE.
   - Create `.agent-task/memory/{timestamp}.yml` and `.agent-task/status/{timestamp}.yml`.

6. **Pre-commit Checks**
   - Run `pre_commit_instructions` tool to get the pre-commit steps.

7. **Submit**
   - Run Bazel updates (`bazelisk run //:gazelle -- update srcs/server/hub` and `bazelisk run //:gazelle -- update srcs/server/telemetry` if needed).
   - Run tests `bazelisk test //srcs/server/hub/...`.
   - Submit PR.
