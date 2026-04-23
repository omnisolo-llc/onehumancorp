1. **Schema Migration**
   - Use `write_file` to create `srcs/server/db/migrations/20260429000000_kairos_shared_tasks_add_columns_pg.sql` containing standard Goose Up/Down migrations adding `agent_id VARCHAR(255)`, `priority INTEGER`, and `payload JSONB` to `shared_tasks`.
   - Use `write_file` to create `srcs/server/db/migrations/20260429000000_kairos_shared_tasks_add_columns_sqlite.sql` containing standard Goose Up/Down migrations adding `agent_id TEXT`, `priority INTEGER`, and `payload TEXT` to `shared_tasks`.
   - Use `run_in_bash_session` to execute `cat` on both `.sql` files to confirm they were written successfully.

2. **Teammate Mesh**
   - Use `replace_with_git_merge_diff` to modify `srcs/server/orchestration/kairos/mesh.go`. Change `LocalTeammateMesh` struct to have field `mesh TeammateMesh`. Update `NewLocalTeammateMesh(redisClient *redis.Client)` to wrap `NewTeammateMesh(redisClient)`.
   - Use `replace_with_git_merge_diff` to modify `srcs/server/orchestration/kairos/mesh_test.go` to update `TestLocalTeammateMesh_PublishSubscribe` to call `NewLocalTeammateMesh(nil)` and add `TestLocalTeammateMesh_Redis`.
   - Use `run_in_bash_session` to execute `cat` on both modified files to verify the changes.

3. **AutoDream Pipeline**
   - Use `replace_with_git_merge_diff` to modify `srcs/server/orchestration/autodream.go`. Inside `ingestAgentMemories`, update `memoryDir := os.Getenv("OHC_MEMORY_DIR")` to fallback to `".agent-task/memory"` if empty. Update `checkQuery` to query `SELECT 1 FROM consolidated_memory WHERE id = ? LIMIT 1` (and `$1` for PG) using `memoryID`. Update `insertQuery` for both SQLite and PG to target `consolidated_memory` using `(id, organization_id, agent_id, content, embedding, source_type)` with values like `(?, 'system', 'system', ?, ?, 'autodream')` (and `$1`, `$2`, `$3::vector` for PG).
   - Use `run_in_bash_session` to execute `cat srcs/server/orchestration/autodream.go` to verify the changes.

4. **Run Tests**
   - Use `run_in_bash_session` to execute `bazelisk test //...` to ensure everything is working correctly.

5. **Pre Commit Steps**
   - Call `pre_commit_instructions` tool to make sure proper testing, verifications, reviews and reflections are done.
