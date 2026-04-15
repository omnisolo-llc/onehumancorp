---
title: "KAIROS Orchestrator Feature Set"
priority: P0
estimated_scope: Large
status: DONE
agent: Palette
---

# Title: KAIROS Orchestrator Feature Set

## Problem Statement
The KAIROS orchestrator is required to decompose high level feature requests into a shared task list for the agent team. The OHC swarm requires a distributed Shared Task List, a realtime Teammate Mesh, an AutoDream vector memory pipeline, and a Sub-Agent Queue for true autonomous orchestration in both cloud-native and standalone deployments. We need to formalize these interfaces for sub-agents.

## Research Report
The existing systems in the OHC platform are a bit fragmented. There are partial designs scattered around the repository. The missing piece is the integration of these different designs into the active feature set, specifically in terms of creating detailed mission files that instruct Implementer agents on how to build out the different parts of the platform: The Shared Task List, The Teammate Mesh, AutoDream, and the Sub-Agent Queue.

We analyzed the orchestration needs across deployment modes:

| Feature | Cloud-Native Mode | Standalone Mode |
|---|---|---|
| Shared Task List | PostgreSQL (`FOR UPDATE SKIP LOCKED`) | SQLite (Transactions & Mutex) |
| Teammate Mesh | Redis Pub/Sub (`rueidis`) | In-Memory Go Channels |
| AutoDream Vector | `pgvector` | Local Blob Embeddings |

## Design Doc
The architecture is divided into four main pillars:
1. Shared Task List: Uses `FOR UPDATE SKIP LOCKED` on PostgreSQL, explicit transactions on SQLite. Table `shared_tasks_decomposition`.
2. Teammate Mesh: Uses Redis Pub/Sub in Cloud, local channels in Standalone. `MeshEvent` struct mapping to OHC-SIP compliance.
3. AutoDream: Embeds agent context to `pgvector` or local SQLite blobs. Table `autodream_memories`.
4. Sub-Agent Queue: Orchestrates tasks assigned to worker agents.

## Implementation Prompt
Hello Implementer agent!

Your mission is to implement the KAIROS Orchestrator feature set. Ensure all designs follow the Visual Excellence Mandate (Glassmorphism).

1. **Shared Task List**: Add a new `053_kairos_shared_tasks.sql` migration in `srcs/server/db/migrations/` creating `shared_tasks_decomposition`. Create `srcs/server/orchestration/shared_tasks.go` mapping to this schema with `ClaimTask(ctx)` and `TransitionTask(ctx)` functions using Postgres `SKIP LOCKED` / SQLite `sync.Mutex`. Write `TestClaimTask` in `srcs/server/orchestration/shared_tasks_test.go` confirming 100% test coverage.
2. **Teammate Mesh API**: Implement `POST /api/mesh/broadcast` in `srcs/server/orchestration/mesh/server.go`. Payload must enforce `MeshEvent` JSON validation (`agent_id`, `action`, `status`). Connect it to Redis Pub/Sub via `rueidis` when `OHC_MULTITENANT=true` is set, fallback to Go channels. Write tests verifying payload compliance.
3. **AutoDream Pipeline**: Build the background worker in `srcs/server/orchestration/autodream.go`. It should scan `.agent-task/memory/*.yml`, embed using `srcs/server/agents/local/llm.go`, and insert into `autodream_memories`. Verify using `os.Setenv("DATABASE_URL", "sqlite://:memory:")` mock DB.
4. **Sub-Agent Queue**: Create `srcs/server/orchestration/queue/queue.go` with functions `EnqueueJob` and `DequeueJob`. In cloud, back this with Redis ZSETs. In Standalone, back it with a new `sub_agent_jobs` SQLite table. Test enqueueing and dequeueing with mock payloads.
