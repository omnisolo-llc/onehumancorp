<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 24px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); color: #fff;">

# Title: KAIROS Orchestrator: Shared Task List, Teammate Mesh, AutoDream, and Sub-Agent Queue

## Problem Statement
The OHC Hybrid Agentic OS requires a central autonomous orchestrator (KAIROS) to decompose high-level feature requests into a shared task list for the agent team. The OHC swarm requires a distributed Shared Task List, a realtime Teammate Mesh, an AutoDream vector memory pipeline, and a Sub-Agent Queue for true autonomous orchestration in both cloud-native and standalone deployments. We need to formalize these backend interfaces and database schemas for sub-agents.

## Research Report
The existing system relies on fragmented designs across numerous architecture documents. We need to consolidate the KAIROS Orchestrator implementation into concrete backend API interfaces and database schemas.
- The **Shared Task List** requires a database table (`shared_tasks_decomposition`) using Postgres `SKIP LOCKED` and SQLite `sync.Mutex`/transactions.
- The **Teammate Mesh** needs an API (`POST /api/mesh/broadcast`) to validate and route agent coordination events over Redis or Go channels.
- The **AutoDream Pipeline** needs a background worker to embed YAML files from `.agent-task/memory/` and insert them into the `autodream_memories` vector table.
- The **Sub-Agent Queue** requires a queue interface (`EnqueueJob`, `DequeueJob`) to manage sub-agent backgrounds tasks.

## Design Doc
See `docs/architecture/KAIROS_AI_OS_HYBRID_ORCHESTRATION_MASTER.md` for the comprehensive, unified design containing sequence diagrams and database schemas.

## Implementation Prompt
Hello Implementer agent! Your mission is to implement the KAIROS Orchestrator backend features.

1. **Shared Task List**: Add a new `055_kairos_shared_tasks.sql` migration in `srcs/server/db/migrations/` creating the `shared_tasks_decomposition` table. Create `srcs/server/orchestration/shared_tasks.go` mapping to this schema with `ClaimTask(ctx)` and `TransitionTask(ctx)` functions. Use Postgres `SKIP LOCKED` / SQLite `sync.Mutex`. Write `TestClaimTask` in `srcs/server/orchestration/shared_tasks_test.go` confirming 100% test coverage. Update `srcs/server/db/BUILD.bazel`.
2. **Teammate Mesh API**: Implement `POST /api/mesh/broadcast` in `srcs/server/orchestration/mesh/server.go`. Payload must enforce `MeshEvent` JSON validation (`agent_id`, `action`, `status`). Connect it to Redis Pub/Sub via `rueidis` when `OHC_MULTITENANT=true` is set, fallback to Go channels. Write tests verifying payload compliance. Wrap the handler with `mesh.ValidationMiddleware` and `auth.RequireRole("system", ...)` in `server.go`.
3. **AutoDream Pipeline**: Build the background worker in `srcs/server/orchestration/autodream.go`. It should scan `.agent-task/memory/*.yml`, embed using `srcs/server/agents/local/llm.go`, and insert into `autodream_memories`. Verify using `db.NewTestProvider(t)` mock DB.
4. **Sub-Agent Queue**: Create `srcs/server/orchestration/queue/queue.go` with functions `EnqueueJob` and `DequeueJob`. In cloud, back this with Redis ZSETs. In Standalone, back it with a new `sub_agent_jobs` SQLite table. Test enqueueing and dequeueing with mock payloads.

## Priority
P0

## Estimated Scope
Large

</div>
