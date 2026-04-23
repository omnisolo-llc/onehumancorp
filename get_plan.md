1. **Schema Migration**
   - Create `srcs/server/db/migrations/20260429000000_kairos_shared_tasks_schema_update.go`. Wait, `write_file` is better done via bash script to avoid format issues. Wait, writing files with goose migrations:
     I will use `write_file` to create `srcs/server/db/migrations/20260429000000_kairos_shared_tasks_add_columns_pg.sql` and `..._sqlite.sql` with correct goose annotations. I will use `run_in_bash_session` to `cat` them and verify they are correctly written.
2. **Teammate Mesh**
   - Read `srcs/server/orchestration/kairos/mesh.go` completely using `cat`.
   - Use `replace_with_git_merge_diff` to update `LocalTeammateMesh` to wrap `TeammateMesh` rather than `*MemoryMesh`, and update `NewLocalTeammateMesh` to accept `*redis.Client` to conditionally create `RedisMesh` or `MemoryMesh`.
   - Use `run_in_bash_session` to `cat srcs/server/orchestration/kairos/mesh.go` to verify.
3. **AutoDream Pipeline**
   - Read `srcs/server/orchestration/autodream.go` completely using `cat`.
   - Use `replace_with_git_merge_diff` to update `ingestAgentMemories` in `srcs/server/orchestration/autodream.go`. If `OHC_MEMORY_DIR` is empty, set it to `.agent-task/memory`. Also change the duplicate check to query `consolidated_memory` using `id` instead of `autodream_memories` using `source_mission_id`. Ensure `source_type` is 'autodream'. Add `agent_id` parameter to the query so it matches the expected schema. Wait, the insertion query `INSERT INTO consolidated_memory (id, organization_id, content, embedding, source_type)` has 5 columns. `autodream_pipeline.go` uses 6 columns `(id, organization_id, agent_id, content, embedding, source_type)`. So I'll update it to insert `agent_id` as well.
   - Use `run_in_bash_session` to `cat srcs/server/orchestration/autodream.go` to verify.
4. **Run Tests**
   - Use `run_in_bash_session` to `bazelisk test //...`
5. **Pre commit**
   - Use `pre_commit_instructions` tool and complete steps.
