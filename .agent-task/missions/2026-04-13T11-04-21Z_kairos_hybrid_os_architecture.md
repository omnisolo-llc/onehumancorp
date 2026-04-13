<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: OHC Hybrid Agentic OS: Shared Task List, Teammate Mesh, and autoDream Architecture

## Problem Statement
The OHC swarm currently lacks a standardized, hybrid-compatible (Cloud, Standalone, Thin Client) architecture for task delegation, real-time agent coordination, and long-term memory consolidation. Without a robust Shared Task List, a low-latency Teammate Mesh, and a durable autoDream pipeline, agents cannot efficiently coordinate complex workflows or learn from past executions.

## Research Report
*   **Market Analysis**: Competitor agentic systems (e.g., AutoGPT, BabyAGI) often struggle with state loss and poor intra-agent communication. OHC's unique "Hybrid Agentic OS" requires a system that scales from a single SQLite file (Standalone) to distributed PostgreSQL/Redis clusters (Cloud).
*   **Teammate Mesh**: To ensure real-time coordination, WebSockets combined with Redis Pub/Sub (for Cloud) and in-memory event buses (for Standalone) offer the best balance of latency and fallback capability.
*   **Memory Consolidation**: OHC needs an `autoDream` process. Leveraging `pgvector` for cloud deployments and local vector storage (e.g., FAISS or local SQLite extensions) ensures agents can consolidate experiences into embeddings for long-term retrieval.

## Design Doc

### 1. Phase 1: Shared Task List (Database Design & Sequence)
**Schema Overview (PostgreSQL / SQLite)**:
*   `tasks`: `id`, `parent_id`, `title`, `description`, `status` (PENDING, IN_PROGRESS, COMPLETED, FAILED), `assigned_agent_role`, `created_at`, `updated_at`.
*   `task_dependencies`: `task_id`, `depends_on_task_id`.

**Sequence**:
1. KAIROS agent decomposes a user prompt into sub-tasks.
2. Sub-tasks are inserted into the `tasks` table with appropriate dependencies.
3. Sub-agents poll or subscribe to task creation events.
4. Agents lock tasks (using Redis locks or DB transactions) before execution.

### 2. Phase 2: Teammate Mesh APIs
**Communication Layer**:
*   **gRPC / WebSockets**: Agents communicate state changes and negotiation requests via a central realtime API.
*   **Redis Pub/Sub**: In Cloud Mode, channels like `ohc.mesh.agent_events` distribute messages across the K8s cluster.
*   **API Contracts**:
    *   `POST /api/v1/mesh/broadcast`: Sends a message to a specific agent or broadcast channel.
    *   `GET /wss/v1/mesh/stream`: WebSocket connection for an agent to receive real-time updates.

### 3. Phase 3: autoDream Data Pipelines
**Vector DB Consolidation**:
*   **Pipeline**: `Task Completion Event` -> `Summarizer LLM` -> `Embedding Model` -> `pgvector` (Cloud) / Local Storage (Standalone).
*   **Schema (`agent_memories`)**: `id`, `task_id`, `summary_text`, `embedding` (VECTOR(1536)), `created_at`.
*   **Execution**: A scheduled background job (`autoDream` worker) periodically batches completed tasks, generates embeddings, and inserts them into the vector database for future context retrieval.

## Implementation Prompt
**Role**: Implementer Agent
**Task**: Implement the foundational backend structure for the Shared Task List, Teammate Mesh APIs, and autoDream pipeline.
**Instructions**:
1.  **Shared Task List**: Create the database migration (PostgreSQL and SQLite compatible) for `tasks` and `task_dependencies` in `srcs/server/db/migrations/`. Implement the Go models and repository layer in `srcs/server/repository/`. Ensure tenant isolation using `OrganizationIDFromContext(ctx)`.
2.  **Teammate Mesh**: Implement a WebSocket handler in `srcs/server/api/mesh.go` that allows agents to subscribe to updates. Add a Redis Pub/Sub implementation for Cloud Mode.
3.  **autoDream**: Create a background worker stub in `srcs/server/workers/autodream.go` that queries completed tasks and prepares them for embedding.
4.  **Tests**: Write comprehensive unit tests for the repository and WebSocket handlers. Use `auth.ClaimsContextKeyForTest` for context injection.
**Acceptance Criteria**:
*   Migrations apply cleanly.
*   WebSocket API handles basic connection and message broadcasting.
*   Test coverage for new packages is >90%.

## Priority
P0

## Estimated Scope
Large

</div>
