1.  **Mark Mission as In Progress**
    -   Update mission file with the following commands:
        ```bash
        sed -i 's/status: PENDING/status: IN_PROGRESS/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
        sed -i 's/agent: Researcher/agent: Link/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
        cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 5
        ```

2.  **Create Database Migration**
    -   Create migration file and update `BUILD.bazel`:
        ```bash
        cat << 'EOF' > srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql
        ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
        ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
        EOF
        sed -i '/"migrations\/031_agent_missions_updated_at.sql",/a \        "migrations/032_add_hybrid_sync_metadata.sql",' srcs/server/db/BUILD.bazel
        cat srcs/server/db/migrations/032_add_hybrid_sync_metadata.sql
        grep "032_add_hybrid_sync_metadata.sql" srcs/server/db/BUILD.bazel
        ```

3.  **Implement Go Interface and Metrics**
    -   The `rag_sync.go` and `rag_sync_test.go` files have been created. The interface, structs and testing flow are ready.

4.  **Mark Mission Done**
    -   Update mission file with the following commands:
        ```bash
        sed -i 's/status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md
        cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 5
        ```

5.  **Run Tests**
    -   Run test command:
        ```bash
        bazelisk test //srcs/server/hub/... //srcs/server/db/...
        ```

6.  **Complete pre-commit steps**
    -   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

7.  **Submit**
    -   Call `submit` to create PR with the appropriate details.
