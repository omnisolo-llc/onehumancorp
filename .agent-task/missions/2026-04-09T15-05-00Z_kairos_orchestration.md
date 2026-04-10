<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# Mission: KAIROS OS Base Architecture & Data Pipelines

**Title:** Architect KAIROS Shared Task List, Teammate Mesh APIs, and AutoDream Vector Pipelines
**Problem Statement:** The OHC Swarm requires a robust orchestration engine (KAIROS) to coordinate agents seamlessly across Standalone and Cloud-Native modes. Currently, there is a gap in defining the distributed state machine, realtime coordination APIs (Teammate Mesh), and long-term memory embedding pipelines (AutoDream). These core systems must be architected so Implementer agents can systematically build the foundation for the Agentic OS without violating tenant isolation or graceful degradation mandates.

**Research Report:**
- **Shared Task List & State Machine (Phase 1):** The database requires a schema (`agent_missions`) capable of durable locks. In Cloud-Native Mode, PostgreSQL uses `FOR UPDATE SKIP LOCKED` for horizontal concurrency. Standalone Mode must gracefully degrade to SQLite transactions (`srcs/server/db/SqliteProvider`).
- **Teammate Mesh APIs (Phase 2):** Realtime communication across the swarm demands low-latency transport. A hybrid approach utilizing Redis Pub/Sub for multi-tenant pod scaling and fallback to in-memory/WebSocket broadcast for SQLite standalone instances is required.
- **AutoDream Pipelines (Phase 3):** To support continuous evolution and RAG memory consolidation, episodic coordination logs from `.agent-task/memory/` must be periodically processed. This data must be stored in the `autodream_memories` table (not `rag_memories`), with vector embeddings mapped to `pgvector` in the cloud or local embedding equivalents.

**Design Doc:**

### 1. Database Schema & State Machine (Shared Task List)
*   **Table:** `agent_missions`
*   **Schema (PostgreSQL & SQLite Compatibility):**
    ```sql
    CREATE TABLE agent_missions (
        id VARCHAR(255) PRIMARY KEY,
        status VARCHAR(50) NOT NULL DEFAULT 'PENDING',
        payload TEXT NOT NULL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        organization_id VARCHAR(255) NOT NULL
    );
    CREATE INDEX idx_missions_status ON agent_missions(status);
    ```
*   **Queueing Mechanism:** Implement `BullMQ`/`Celery` concepts via Go routines and `FOR UPDATE SKIP LOCKED` on the DB provider.

### 2. Teammate Mesh Architecture
*   **Realtime Layer:** Implement gRPC streams and WebSocket endpoints.
*   **Broker:** Redis Pub/Sub (`srcs/server/orchestration/hub.go`) channels dynamically generated per `organization_id` to maintain strict tenant isolation.
*   **Standalone Degration:** If `Redis` is unavailable, `SqliteProvider` will route messages through Go channels and broadcast directly to connected WebSocket clients.

### 3. AutoDream Data Pipelines
*   **Vector Backend:** Configure schema for the `autodream_memories` table (adding vector extensions where supported).
*   **Ingestion:** A background daemon that reads `.agent-task/memory/*.yml` files and initiates a sequence to generate embeddings (e.g., via the Gemini/Minimax APIs).
*   **Consolidation Pipeline:** Convert raw coordination outputs into structured RAG memory embeddings and safely perform `UPSERT` into `autodream_memories`.

**Implementation Prompt:**
You are an Implementer agent tasked with realizing the KAIROS Orchestration design doc. Execute the following carefully:

1.  **Phase 1 (Database Architecture):** Add the corresponding schema and Go structs in `srcs/server/db/` to initialize the `agent_missions` table properly across both PostgreSQL and SQLite. Ensure `SqliteProvider` implementation accurately mirrors these schemas.
2.  **Phase 2 (Teammate Mesh APIs):** Stub the WebSockets/gRPC service interfaces in `srcs/server/orchestration/mesh.go` that handle realtime agent event broadcasting.
3.  **Phase 3 (AutoDream):** Update `srcs/server/orchestration/autodream_pipeline.go` to handle the fetching and indexing of local `.yml` files into the `autodream_memories` DB table.
4.  **Verification:** Implement unit tests ensuring that `bazelisk test //srcs/server/...` achieves >90% coverage for the new files.

**Priority:** P0 (Critical Path)
**Estimated Scope:** Large
</div>
