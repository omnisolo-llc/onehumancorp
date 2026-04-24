1. **Shared Task List: DAG Dependency Resolution & DB Locking**
   - Update `src/server/orchestration/tasks_db.go`:
     - Fix the fallback to `mu.Lock()`/`mu.Unlock()` for SQLite to wrap the entire transaction in `ClaimTask` and `ClaimPendingTask` (acquired before `db.BeginTx` and released via `defer` after `Commit` / `Rollback`) to prevent race conditions.
     - Ensure `FOR UPDATE SKIP LOCKED` is used properly for Postgres.
   - Update `src/server/orchestration/task_orchestrator.go`:
     - Wrap SQLite transactions with the mutex in `AcquireReadyTask`, similar to `tasks_db.go`.
     - Confirm query handles dependencies and `SKIP LOCKED`.

2. **Teammate Mesh: APIs with Redis Pub/Sub**
   - Update `src/server/orchestration/service_mesh.go`:
     - Add required API contracts: `POST /v1/orchestration/mesh/broadcast` and `GET /v1/orchestration/tasks/stream` (SSE/WebSocket), using `MeshTransport`.
     - In the teammate mesh communication layer (`interop.TeammateMesh`), ensure agents broadcast active state changes.

3. **AutoDreamWorker Data Pipeline**
   - Update `src/server/orchestration/autodream_pipeline.go`:
     - Modify `AutoDreamWorker` data pipeline (`process` function) to passively scan `COMPLETED` tasks from `shared_tasks_master` or `swarm_tasks` and vectorize them into `autodream_memories`.

4. **SQL Migrations**
   - Verify `autodream_memories` table schema matches the required specification in `src/server/db/migrations/20260418000000_autodream_pgvector_memories_pg.sql`.
   - Update `src/server/db/BUILD.bazel` to correctly `embedsrcs = glob(["migrations/*.sql", "migrations/*.go"])`. (Done)

5. **Complete pre commit steps**
   - Ensure proper testing, verification, review, and reflection are done.
   - `bazelisk test //src/server/orchestration/...` to achieve >95% test coverage.

6. **Submit the change**
