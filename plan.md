1. **Shared Task List (Database Schema)**
   - Create a migration in `srcs/server/db/migrations/018_tasks.sql` (if not fully matching requirements) or a new one to ensure `tasks` table has `metadata` (JSONB for Postgres, TEXT for SQLite) and mode-dependent `id` column.
     *Wait, `018_tasks.sql` already exists. I will check its content and update it to match the requirements.*
2. **Teammate Mesh APIs**
   - Add `POST /api/mesh/direct` and `GET /api/mesh/mailbox` handlers in `srcs/server/dashboard/server.go`. (Note: The prompt asks to implement REST endpoints in `srcs/server/orchestration/`. Let me check if there's a file for API handlers in `orchestration/`, or if they should be in `server.go` calling `orchestration.`).
   - Implement the actual business logic in `orchestration` (e.g. `orchestration/mesh.go`).
   - Ensure `db.Provider` is used to filter by `orgID` if appropriate.
   - Wait, `mesh.go` has `TeammateMesh` interface. I'll add the new endpoints to `srcs/server/dashboard/server.go` and logic in `orchestration/mesh.go`.
3. **autoDream Engine**
   - Implement `AutoDreamSyncEngine` daemon in `srcs/server/sync/autodream_sync.go` (or `orchestration/autodream.go`). It should conditionally check `dbWrapper.IsSQLite()`.
   - Implement the background worker to process `DONE` tasks and generate embeddings using `orchestration.MinimaxClient`.
4. **Metrics**
   - Instrument with OpenTelemetry (`swarm_tasks_created` etc.?).
   - Update `deploy/docker/grafana/provisioning/dashboards/` JSON files.
5. **Testing**
   - Write tests for the endpoints and autoDream Engine. Ensure `defer ClearSemaphore()` is used if applicable.
