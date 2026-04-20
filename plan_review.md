1. **Create Database Migration:**
   - I will use `run_in_bash_session` to create `srcs/server/db/migrations/20260422000000_crdt_deltas.sql` using a `cat << 'EOF' > ...` block containing:
   ```sql
   -- +goose Up
   -- +goose StatementBegin
   CREATE TABLE IF NOT EXISTS crdt_deltas (
       id VARCHAR(255) PRIMARY KEY,
       entity_id VARCHAR(255) NOT NULL,
       data TEXT NOT NULL,
       updated_at TIMESTAMP NOT NULL,
       synced_to_cloud BOOLEAN DEFAULT FALSE
   );
   -- +goose StatementEnd

   -- +goose Down
   -- +goose StatementBegin
   DROP TABLE IF EXISTS crdt_deltas;
   -- +goose StatementEnd
   ```

2. **Update MCP Interface and Tools:**
   - I will run a python script to modify `srcs/server/tools/statesyncmcp/mcp.go` to add `crdt_pull` and `crdt_push` to `ListTools` and update the `StateSyncProvider` and `CallTool` methods to include `CRDTPush` and `CRDTPull`.

3. **Implement MCP Provider Logic:**
   - I will run a python script to modify `srcs/server/tools/statesyncmcp/provider.go` to implement `CRDTPull` and `CRDTPush`. `CRDTPush` will fetch unsynced deltas from `crdt_deltas`, send them to `/api/v1/sync/mcp-deltas`, and mark them as synced. `CRDTPull` will fetch from the same API and update the local DB.

4. **Update MCP Tests:**
   - I will run a python script to append comprehensive unit tests for `CRDTPull` and `CRDTPush` into `srcs/server/tools/statesyncmcp/mcp_test.go` and `srcs/server/tools/statesyncmcp/provider_test.go`.

5. **Create Cloud API Handlers:**
   - I will use a `cat << 'EOF' > ...` block to create `srcs/server/api/sync_crdt_handler.go` which contains `HandleCRDTSync` to ingest deltas and resolve conflicts, and `HandleCRDTPull`.
   - I will use a `cat << 'EOF' > ...` block to create `srcs/server/api/sync_crdt_handler_test.go` to test these handlers.

6. **Update API BUILD Configuration:**
   - I will run `bazelisk run //:gazelle -- srcs/server/api` to update `BUILD.bazel`.

7. **Implement Background Sync Worker:**
   - I will use a python script to append `CRDTSyncWorker` logic into `srcs/server/orchestration/hybrid_sync/hybrid_sync.go` and its tests into `srcs/server/orchestration/hybrid_sync/hybrid_sync_test.go`.

8. **Ensure Full Verification:**
   - I will run `bazelisk test //...` to ensure everything passes and code is building correctly.

9. **Ensure Pre-Commit and Verifications:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

10. **Submit PR:**
    - Submit the complete feature using the `submit` tool, explicitly including `issue_id: 5880`.
