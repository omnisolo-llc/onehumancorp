1. **Migrations**: Create a new explicit SQL migration `062_autodream_phase3.sql` in `srcs/server/db/migrations/` that ensures `autodream_memories` table exists with `embedding vector(1536)` (Postgres) and `embedding TEXT` (SQLite). This uses `CREATE TABLE IF NOT EXISTS` to safely assert the schema. Update `srcs/server/db/BUILD.bazel` to include it in `embedsrcs`.

2. **Verify Migrations**: Use `cat` to verify the newly created `062_autodream_phase3.sql` file and the updated `embedsrcs` block in `srcs/server/db/BUILD.bazel`.

3. **AutoDreamPipeline Logic**: Modify `srcs/server/orchestration/autodream_pipeline.go`.
   - Update `process(ctx)` logic to insert into `autodream_memories` instead of `consolidated_memory` or `swarm_long_term_memory`.
   - Change references to `consolidated_memory` and `swarm_long_term_memory` to instead insert into `autodream_memories` matching its schema.
   - Add a step to `process(ctx)` to sweep `shared_tasks` where `status = 'COMPLETED'`, filter out tasks that have already been inserted into `autodream_memories` by checking their `task_id` (e.g. `AND id NOT IN (SELECT task_id FROM autodream_memories WHERE task_id IS NOT NULL)`), generate embeddings for their content (title/payload), and insert into `autodream_memories`.

4. **Verify Pipeline Logic**: Use `cat` to verify the changes made to `srcs/server/orchestration/autodream_pipeline.go`.

5. **Exact-Neighbor Queries**: Modify `srcs/server/orchestration/autodream_worker.go`. On line 305, it currently says `embedding <=> $1::vector as distance`. Replace `<=>` with `<->` to meet the exact string requirement `ORDER BY embedding <-> $1` from the prompt.

6. **Verify Exact-Neighbor Queries**: Use `cat` to verify the changes made to `srcs/server/orchestration/autodream_worker.go`.

7. **Testing and Verification**: Run `bazelisk test //srcs/server/orchestration/...` and `bazelisk test //srcs/server/db:db_test` to verify code changes are functional and pass tests.

8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. **Finalize**: Output a final unstructured message containing issue_id.
