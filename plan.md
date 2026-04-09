1.  **Read Target Mission and Update State**
    - The mission we are tackling is `2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
    - We will update the mission's frontmatter to change `status: PENDING` to `status: IN_PROGRESS` and `agent: Researcher` to `agent: Jules`.

2.  **Database Migration (Step 1)**
    - We need to create a new SQL migration file `srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql`.
    - We will add `sync_status VARCHAR(50) DEFAULT 'pending'` and `last_sync_at TIMESTAMPTZ NULL` to the `autodream_memories` table since that acts as the primary context/RAG memory table (based on recent migrations and instructions).
    - We'll use simple, separate `ALTER TABLE ... ADD COLUMN ...` statements to ensure compatibility with SQLite as mentioned in memory guidelines.

3.  **Go Interface Definition (Step 2)**
    - We will create a new Go file at `srcs/server/hub/rag_sync.go`.
    - We will define `SyncStatus`, `RAGSyncRecord`, and `RAGSyncService` interface exactly as requested in the mission implementation prompt.

4.  **Metrics & Observability (Step 3)**
    - We need to update `srcs/server/telemetry/telemetry.go` to add OpenTelemetry counters for `rag_records_synced_total` and `rag_sync_errors_total`.
    - We will add global variables for these counters.
    - We will update `InitWithMeter` to initialize these metrics using `m.Int64Counter`.
    - We will add recording functions `RecordRAGRecordsSynced` and `RecordRAGSyncErrors` to increment the counters. We will check if the counter is non-nil before incrementing, as required by the guidelines.

5.  **Verification (Step 3)**
    - We will write a unit test in `srcs/server/hub/rag_sync_test.go` to mock the `RAGSyncService` interface and verify basic data flow logic.

6.  **Run All Tests**
    - `~/go/bin/bazelisk test //srcs/server/... //srcs/app/... --test_output=errors` to ensure everything is functional and compiling correctly.
    - If needed we'll also run `cd srcs/server && go test ./...`.

7.  **Complete Mission**
    - We will mark the mission file `2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` as `status: DONE`.

8.  **Pre-Commit Steps**
    - Call `pre_commit_instructions` to ensure proper testing, verification, review, and reflection are done.

9.  **Submit Pull Request**
    - Use `submit` to create a PR with our changes.
