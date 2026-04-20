1. **Create Database Migration:**
   - I will use `run_in_bash_session` to create `srcs/server/db/migrations/20260422000000_crdt_deltas.sql` using a `cat << 'EOF' > ...` block. It will define the `crdt_deltas` table with `id`, `entity_id`, `data`, `updated_at`, and `synced_to_cloud` columns. Then I will run `ls srcs/server/db/migrations/*crdt_deltas.sql` to verify its creation.

2. **Update MCP Interface and Tools:**
   - I will use `run_in_bash_session` to run a python script to modify `srcs/server/tools/statesyncmcp/mcp.go`. I will add `crdt_pull` and `crdt_push` to `ListTools` and update the `CallTool` and `StateSyncProvider` interfaces to include `CRDTPush` and `CRDTPull`. I will then `cat` the file to verify the changes.

3. **Implement MCP Provider Logic:**
   - I will use `run_in_bash_session` with a python script to modify `srcs/server/tools/statesyncmcp/provider.go`. I will add `CRDTPull` and `CRDTPush` methods that query the SQLite database (`crdt_deltas` table) and send/receive the data from the cloud API endpoint `/api/v1/sync/mcp-deltas`. I will `cat` the file to verify the changes.

4. **Update MCP Tests:**
   - I will use `run_in_bash_session` with a python script to append comprehensive unit tests for the new CRDT pull/push capabilities into `srcs/server/tools/statesyncmcp/mcp_test.go` and `srcs/server/tools/statesyncmcp/provider_test.go`. Then I will run `bazelisk test //srcs/server/tools/statesyncmcp/...` to verify the tests pass.

5. **Create Cloud API Handlers:**
   - I will use `run_in_bash_session` with a `cat << 'EOF' > ...` block to create `srcs/server/api/sync_crdt_handler.go`. It will contain `HandleCRDTSync` (POST) to handle the `/api/v1/sync/mcp-deltas` endpoint, ingest deltas into Postgres, and resolve conflicts via `updated_at`. I will also create `HandleCRDTPull` (GET). I will verify creation with `ls`.
   - I will use `run_in_bash_session` with a `cat << 'EOF' > ...` block to create `srcs/server/api/sync_crdt_handler_test.go` to test these handlers. I will verify creation with `ls`.

6. **Update API BUILD Configuration:**
   - I will run `bazelisk run //:gazelle -- srcs/server/api` to update `srcs/server/api/BUILD.bazel` to include `sync_crdt_handler.go` and `sync_crdt_handler_test.go`. I will run `git diff srcs/server/api/BUILD.bazel` to verify. Then I will run `bazelisk test //srcs/server/api/...` to verify the tests pass.

7. **Implement Background Sync Worker:**
   - I will use `run_in_bash_session` with a python script to modify `srcs/server/orchestration/hybrid_sync/hybrid_sync.go` and append `CRDTSyncWorker` that polls the `crdt_deltas` table and pushes unsynced deltas to the cloud API, and add it to `Start()`. I will modify `hybrid_sync_test.go` to add tests for this worker. I will run `bazelisk test //srcs/server/orchestration/hybrid_sync/...` to verify the tests pass.

8. **Ensure Full Verification:**
   - I will run `bazelisk test //...` to ensure everything passes and code is building correctly.

9. **Ensure Pre-Commit and Verifications:**
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

10. **Submit PR:**
    - Submit the complete feature using the `submit` tool, explicitly including `issue_id: 5880`.
