---
Title: "KAIROS Orchestration: Unifying Shared Task List, Teammate Mesh, and AutoDream"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
To realize the absolute autonomy of the OHC Swarm, we need robust backend architectural systems that facilitate high-throughput, low-latency agent coordination and memory consolidation. Specifically, agents need a robustly shared task queue (Shared Task List), a real-time messaging fabric (Teammate Mesh), and a durable vectorized memory layer (autoDream pgvector pipelines). While fragments exist, they are not architecturally integrated into a scalable queue and event bus.

# Research Report
1. **Shared Task List**: `swarm_tasks` currently has basic CRUD but lacks isolated queue mechanics across Kubernetes pods for `PENDING` -> `READY` state transitions. Need scalable BullMQ-style or Celery-style background queuing logic, utilizing Postgres `FOR UPDATE SKIP LOCKED` for Multi-tenant isolation.
2. **Teammate Mesh**: Needs real-time WebSockets/gRPC overlay via `Redis Pub/Sub` for agents to broadcast and listen to state changes actively (the "Mailbox" concept).
3. **AutoDream**: Ephemeral contexts mapped in `agent_session_data` must be compressed via LLM (e.g., Minimax) and vectorized into `autodream_memories` using `pgvector` for semantic truth-seeking.

# Design Doc
- **Microservices Boundary**: Create a unified interface `OrchestrationHub` orchestrating the DAG of `swarm_tasks` dependencies and triggering `Mesh` broadcasts when status changes.
- **API Contracts**:
  - `POST /v1/orchestration/mesh/broadcast`
  - `GET /v1/orchestration/tasks/stream` (SSE/WebSocket)
- **Database Schema Updates**:
  ```sql
  CREATE EXTENSION IF NOT EXISTS vector;
  CREATE TABLE IF NOT EXISTS autodream_memories (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      organization_id TEXT NOT NULL,
      agent_id TEXT,
      content TEXT NOT NULL,
      embedding vector(1536),
      source_type TEXT NOT NULL,
      created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
  );
  CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
  ```

# Implementation Prompt
You are an Implementer agent executing Phase 1-3 of the KAIROS Orchestration playbook.
1. Update `srcs/server/orchestration/tasks_db.go` and `task_orchestrator.go` to implement robust DAG dependency resolution for the Shared Task List, ensuring Postgres `SKIP LOCKED` and SQLite Mutex isolation.
2. Implement the Realtime Teammate Mesh APIs in `srcs/server/orchestration/service_mesh.go` utilizing Redis Pub/Sub channels (via `rueidis`).
3. Build the `AutoDreamWorker` data pipeline (`srcs/server/orchestration/autodream_pipeline.go`) to passively scan `COMPLETED` tasks and vectorize them into `autodream_memories`.
4. Create the necessary SQL migrations for `autodream_memories` and update `srcs/server/db/BUILD.bazel`.
5. Achieve >95% test coverage for `//srcs/server/orchestration/...`.

# Visual Excellence Guidelines
Any internal dashboards or UI exposing KAIROS logs must adhere to the Premium Feel:
`backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);`
