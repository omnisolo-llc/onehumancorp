1. **Explore & Verify Mission Context:**
   - Understand the current state of RAG memories and what table acts as "rag_memories". We have `autodream_memories` and `consolidated_memory`, which contains `embedding` / `vector_embedding` fields. The instruction mentions "assuming such a table exists, or the primary context table". I'll use `consolidated_memory` or `autodream_memories` or create a new table, but let's just add the columns `sync_status` and `last_sync_at` to `consolidated_memory`.
   Wait, the prompt says "assuming such a table exists, or the primary context table". The main tables with vectors are `swarm_memory_embeddings`, `autodream_memories`, and `consolidated_memory`. Let's use `autodream_memories` or `consolidated_memory`. Let's create `032_hybrid_rag_sync_metadata.sql` and modify `consolidated_memory` as it seems like the central table, but actually `autodream_memories` might be better or I can modify both. Let's just modify `consolidated_memory`. Let's look at `consolidated_memory` schema again. It's in `027_consolidated_memory.sql`.

   Let's check `027_consolidated_memory.sql`:
   ```sql
   CREATE TABLE IF NOT EXISTS consolidated_memory (
       id TEXT PRIMARY KEY,
       organization_id TEXT NOT NULL,
       agent_id TEXT,
       content TEXT NOT NULL,
       embedding VECTOR(1536),
       source_type TEXT NOT NULL,
       created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
   );
   ```
   So I will add to it:
   ```sql
   ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
   ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
   ```

2. **Step 1: Database Migration:**
   - Create `srcs/server/db/migrations/032_hybrid_rag_sync_metadata.sql` containing:
     ```sql
     ALTER TABLE consolidated_memory ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
     ALTER TABLE consolidated_memory ADD COLUMN last_sync_at TIMESTAMPTZ NULL;
     ```

3. **Step 2: Go Interface Definition:**
   - Create `srcs/server/hub/rag_sync.go` with the exact interface and structures provided in the mission.
   - Also add OTel counters `rag_records_synced_total` and `rag_sync_errors_total` to `srcs/server/telemetry/telemetry.go`. Create helper functions like `RecordRagRecordsSynced(ctx, count)` and `RecordRagSyncError(ctx)`.

4. **Step 3: Metrics & Observability:**
   - Update `srcs/server/telemetry/telemetry.go` to define and initialize the new metrics in `InitWithMeter`.
   - Use the new metrics in `srcs/server/hub/rag_sync.go` (if implementing the service) or just ensure the metrics are defined.
   - The mission says: "In `srcs/server/hub/rag_sync.go` or a dedicated telemetry file, add OpenTelemetry counters...". Since telemetry is centralized in `telemetry.go`, I'll put them there and add wrapper functions.

5. **Step 4: BUILD files and Verification:**
   - Create `srcs/server/hub/BUILD.bazel`.
   - Create `srcs/server/hub/rag_sync_test.go` to mock the interface and verify the basic data flow logic. It should test a mock service that uses the metrics to ensure they are tracked.
   - Update `WORKSPACE` or run Bazel test command `bazelisk test //srcs/server/hub/... //srcs/server/telemetry/...` to verify.

6. **Pre-commit:**
   - Call `pre_commit_instructions` tool to run verification and tests.

7. **Submit:**
   - Change mission status to DONE in `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`.
   - Submit the PR.
