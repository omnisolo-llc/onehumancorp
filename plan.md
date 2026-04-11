1. **Mark Mission In-Progress**:
   Use the `run_in_bash_session` tool to execute a `sed` command to simultaneously update the frontmatter of `.agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md` to `status: IN_PROGRESS` and `agent: Guide`.

2. **Database Migration**:
   Use the `run_in_bash_session` tool to execute a heredoc to create `srcs/server/db/migrations/032_hybrid_sync_metadata.sql` with the following content:
   ```sql
   -- 032_hybrid_sync_metadata.sql
   ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
   ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
   ```
   Followed immediately by `cat srcs/server/db/migrations/032_hybrid_sync_metadata.sql` and `ls -la srcs/server/db/migrations/` to verify.

3. **Go Interface Definition and Metrics**:
   Use the `run_in_bash_session` tool to execute a heredoc to create `srcs/server/hub/rag_sync.go` with the requested struct (`RAGSyncRecord` where Vector is `[]byte` due to memory rule constraint), `SyncStatus` types, `RAGSyncService` interface, and the two requested OpenTelemetry metrics (`rag_records_synced_total` and `rag_sync_errors_total`). Followed immediately by `cat srcs/server/hub/rag_sync.go` to verify.

4. **Testing**:
   Use the `run_in_bash_session` tool to execute a heredoc to create `srcs/server/hub/rag_sync_test.go` with unit tests that mock the `RAGSyncService` interface to verify the basic data flow logic, aiming for high test coverage of the new definitions. Followed immediately by `cat srcs/server/hub/rag_sync_test.go` to verify.

5. **Test Execution**:
   Run the project-wide tests using the `run_in_bash_session` tool: `bazelisk test //...`. Ensure all tests pass.

6. **Pre-commit Steps**:
   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7. **Submit Changes**:
   Use the `submit` tool to create a Pull Request with the exact title `🗺️ Guide: Hybrid MCP RAG Protocol Sync Interface` and description `Implemented the Offline-to-Cloud State Sync interface, database migration, and metrics for the Hybrid MCP RAG Protocol.`.
