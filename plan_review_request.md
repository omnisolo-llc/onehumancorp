The task requires implementing the KAIROS Master Orchestration DB schemas and Mesh API.
Here's the plan:
1.  **Database Migration**:
    *   Create a new Go file in `srcs/server/db/migrations/` to implement the required schema changes: `kairos_shared_tasks`, `kairos_state_transitions`, `kairos_sub_agent_jobs`, and `autodream_vector_memories`.
    *   The file will implement graceful degradation to SQLite (using `tx.QueryRowContext(ctx, "SELECT sqlite_version()")` to branch the logic).
    *   For Postgres, I will use `JSONB` for payload and dependencies, and `UUID PRIMARY KEY DEFAULT gen_random_uuid()`, along with `vector(1536)` for embeddings where appropriate (or similar, per instructions, though pgvector extension might need to be enabled if not already).
    *   For SQLite, I will use `TEXT` for JSON fields and `TEXT` for UUID primary keys.
    *   Update `srcs/server/db/BUILD.bazel` to include this new migration file in the `embedsrcs` or `srcs` of the `db` library, as required by the setup. (Since it's a `.go` file, it goes into `srcs`).
2.  **Mesh API Update**:
    *   The `ValidationMiddleware` in `srcs/server/api/mesh/middleware.go` is checking for `agent_id`, `channel`, `event_type`, and `data`. This already exists and seems to enforce OHC-SIP compliance. Let's verify `HandleBroadcast` and `HandleMeshV1Broadcast` in `srcs/server/orchestration/mesh_api.go` and `srcs/server/dashboard/server.go`.
    *   In `srcs/server/orchestration/mesh_api.go`, `HandleBroadcast` doesn't enforce the strict payload, but it parses the body into `map[string]interface{}`. I'll update it to use a structured schema and validate the fields. The task says: *Ensure `POST /api/mesh/broadcast` validates payload structures against OHC-SIP specifications (`agent_id`, `channel`, `event_type`, `data`).* Wait, if `ValidationMiddleware` is already doing it for `/api/mesh/broadcast`, maybe we just need to ensure the route is using it? In `srcs/server/orchestration/mesh_api.go`, the route is `mux.HandleFunc("/api/mesh/broadcast", api.HandleBroadcast)`. It doesn't wrap with `ValidationMiddleware`. I'll update `RegisterRoutes` in `mesh_api.go` to wrap the handlers with the `ValidationMiddleware` or implement validation directly inside `HandleBroadcast`.
3.  **Tests**:
    *   Add unit tests in `srcs/server/db/migrations/` for the new schema or verify it via a test.
    *   Add unit tests in `srcs/server/orchestration/mesh_api_test.go` to ensure `/api/mesh/broadcast` validates payload structures correctly.
4.  **Run pre-commit and tests**.
