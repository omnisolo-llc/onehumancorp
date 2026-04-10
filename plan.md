1. **Database Migration**
   - Create a new migration file `srcs/server/db/migrations/032_hybrid_sync_metadata.sql`.
   - In this file, add the following queries to add `sync_status` and `last_sync_timestamp` columns to `autodream_memories`:
     ```sql
     ALTER TABLE autodream_memories ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
     ALTER TABLE autodream_memories ADD COLUMN last_sync_timestamp TIMESTAMPTZ NULL;
     ```
   - Make sure to update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.

2. **Go Interface Definition**
   - Create a new file `srcs/server/hub/rag_sync.go` (and create the `hub` directory if it doesn't exist).
   - In this file, define the `SyncStatus` constants, `RAGSyncRecord` struct, and `RAGSyncService` interface as described in the mission.
   - For `Vector []float32`, to be fully compatible with SQLite as per memory ("When querying vector embeddings... cast the embedding to text using CAST(embedding AS TEXT) ... When converting float arrays... to string representations for database storage, use encoding/json"), I will keep it as `[]float32` in the struct but it will be handled as JSON string in SQL queries. In this file I'm only defining the structs, so `Vector []float32` is perfect.
   - Add OpenTelemetry metrics counters for `rag_records_synced_total` and `rag_sync_errors_total`. Initialize them using the global meter instance from `go.opentelemetry.io/otel/metric`.

3. **Metrics & Observability**
   - In `rag_sync.go`, include the initialization of the telemetry metrics `rag_records_synced_total` and `rag_sync_errors_total`. I can use `otel.Meter("hub")` for this.

4. **Verification & Testing**
   - Create `srcs/server/hub/rag_sync_test.go` with unit tests mocking the interface to verify basic data flow logic. For metrics, use `go.opentelemetry.io/otel/metric/noop` to mock metrics if necessary.
   - Create `srcs/server/hub/BUILD.bazel` for the new `hub` package. Include `rag_sync.go` in `go_library` and `rag_sync_test.go` in `go_test`. Ensure correct dependencies.
   - Run `bazelisk test --config=local //srcs/server/hub/... //srcs/server/db/...` to verify changes pass.

5. **Pre-commit and Submit**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
   - Create a PR with `submit` using the branch name `feat/hybrid-mcp-rag-protocol`, commit message `feat: implement Hybrid MCP RAG Protocol sync structures`, and a detailed description matching the mission.
