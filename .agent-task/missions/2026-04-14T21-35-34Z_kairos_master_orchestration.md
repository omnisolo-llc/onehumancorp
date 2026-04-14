---
title: "KAIROS Master Orchestration Interfaces"
priority: P0
estimated_scope: Large
status: PENDING
---

# Title: KAIROS Master Orchestration Interfaces

## Problem Statement
The KAIROS orchestrator is required to decompose high-level feature requests into a shared task list for the agent team. The OHC swarm requires a distributed Shared Task List, a realtime Teammate Mesh, an AutoDream vector memory pipeline, and a Sub-Agent Queue for true autonomous orchestration in both cloud-native and standalone deployments. We need to formalize these interfaces for sub-agents.

## Research Report
The existing designs and plans need to be consolidated and built into concrete backend API interfaces and database schemas. The swarm relies on database locks (`FOR UPDATE SKIP LOCKED`) in PostgreSQL and `sync.Mutex`/explicit transactions in SQLite for safe parallel task execution. The messaging layer must enforce OHC-SIP payload compliance.

## Design Doc
See `docs/architecture/KAIROS_AI_OS_HYBRID_ORCHESTRATION_MASTER.md` for the unified design.

## Implementation Prompt
Hello Implementer agent!

Your mission is to implement the KAIROS Orchestrator feature set.

1. **Shared Task List**: Add a new `055_kairos_shared_tasks.sql` migration in `srcs/server/db/migrations/` creating `shared_tasks_decomposition`. Create `srcs/server/orchestration/shared_tasks.go` mapping to this schema with `ClaimTask(ctx)` and `TransitionTask(ctx)` functions using Postgres `SKIP LOCKED` / SQLite `sync.Mutex`. Write `TestClaimTask` in `srcs/server/orchestration/shared_tasks_test.go` confirming 100% test coverage. Update `srcs/server/db/BUILD.bazel`.
2. **Teammate Mesh API**: Implement `POST /api/mesh/broadcast` in `srcs/server/orchestration/mesh/server.go`. Payload must enforce `MeshEvent` JSON validation (`agent_id`, `action`, `status`). Connect it to Redis Pub/Sub via `rueidis` when `OHC_MULTITENANT=true` is set, fallback to Go channels. Write tests verifying payload compliance. Wrap the handler with `mesh.ValidationMiddleware` and `auth.RequireRole("system", ...)` in `server.go`.
3. **AutoDream Pipeline**: Build the background worker in `srcs/server/orchestration/autodream.go`. It should scan `.agent-task/memory/*.yml`, embed using `srcs/server/agents/local/llm.go`, and insert into `autodream_memories`. Verify using `db.NewTestProvider(t)` mock DB.
4. **Sub-Agent Queue**: Create `srcs/server/orchestration/queue/queue.go` with functions `EnqueueJob` and `DequeueJob`. In cloud, back this with Redis ZSETs. In Standalone, back it with a new `sub_agent_jobs` SQLite table. Test enqueueing and dequeueing with mock payloads.
