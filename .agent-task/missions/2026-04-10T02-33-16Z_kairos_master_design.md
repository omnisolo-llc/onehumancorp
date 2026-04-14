---
status: DONE
agent: Jules
priority: P0
---

# Title: KAIROS Orchestration: Master Design for Shared Task List, Teammate Mesh, and AutoDream

## Problem Statement
The OHC Hybrid Architecture requires a robust orchestration engine (KAIROS) to distribute tasks among the swarm, coordinate state changes in realtime, and consolidate long-term memory. We must define the architectural blueprints for the Shared Task List, the Realtime Teammate Mesh APIs, and the AutoDream Data Pipelines to ensure seamless scaling from Standalone SQLite to Cloud-Native PostgreSQL.

## Research Report
1.  **Shared Task List**: Needs strict row-level locks (`FOR UPDATE SKIP LOCKED`) in PostgreSQL to prevent worker collision during concurrent execution, while degrading to simple transaction isolation in SQLite for Standalone Mode.
2.  **Teammate Mesh**: Requires a low-latency, highly available pub/sub layer. For Cloud-Native, Redis Pub/Sub (`rueidis`) combined with `CentrifugeNode` is mandatory. For Standalone, a local memory mesh is sufficient.
3.  **AutoDream Pipeline**: Ephemeral session logs must be periodically swept and embedded into `pgvector` for exact Nearest Neighbor search in the cloud. In Standalone mode, SQLite JSON blobs are the fallback.

## Design Doc
### 1. Shared Task List (PostgreSQL & SQLite)
- **Schema `shared_tasks`**:
  ```sql
  CREATE TABLE shared_tasks (
      id TEXT PRIMARY KEY,
      organization_id TEXT NOT NULL,
      title TEXT NOT NULL,
      status TEXT NOT NULL DEFAULT 'PENDING',
      dependencies JSONB
  );
  ```
- **Claim Logic**: `SELECT id FROM shared_tasks WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED` (Postgres only).

### 2. Teammate Mesh APIs
- **Protocol**: Add `MeshEvent` to `srcs/proto/hub.proto`.
- **Transports**: Implement `RedisMeshTransport` and `MemoryMeshTransport` under the `MeshTransport` interface in `srcs/server/orchestration/hub.go`.

### 3. AutoDream Data Pipelines
- **Schema `autodream_memories`**:
  ```sql
  CREATE EXTENSION IF NOT EXISTS vector;
  CREATE TABLE autodream_memories (
      id TEXT PRIMARY KEY,
      embedding vector(1536),
      content TEXT NOT NULL
  );
  ```
- **Daemon**: `AutoDreamWorker` periodically polls `.agent-task/memory/` and `agent_session_data`, generating embeddings via the Minimax/Local LLM clients, and upserting into the vector DB.

## Implementation Prompt
Hello Implementer agent!
1. **Shared Task List**: Create the database migrations for `shared_tasks` and implement the `ClaimTask` DAL logic with the `FOR UPDATE SKIP LOCKED` requirement for PostgreSQL in `srcs/server/orchestration/tasks_db.go`.
2. **Teammate Mesh**: Update the proto files, generate the stubs, and implement `RedisMeshTransport` using `rueidis`.
3. **AutoDream**: Create the `autodream_memories` migration and the background worker (`autodream_pipeline.go`) to batch process ephemeral logs into vectors.
4. Ensure all designs pass architectural linting with >90% test coverage.

## Priority
P0

## Estimated Scope
Large
