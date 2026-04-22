# Architect AutoDream Vector Data Pipelines for State Consolidation

## Problem Statement
OHC agents need a long-term memory system to consolidate complex state from `shared_tasks` and `agent_mesh_messages` into vectorized knowledge, preventing amnesia between Cloud-Native and Standalone execution modes.

## Research Report
* **Current Gap:** Agents discard execution context once KAIROS tasks complete. This lack of persistent context limits Swarm Intelligence.
* **Analysis:** Modern Agentic frameworks utilize RAG (Retrieval-Augmented Generation) combined with PGVector for efficient state retrieval. By polling completed tasks and mesh logs, we can construct episodic memory.
* **Competitive Landscape:** CrewAI relies on ephemeral memory loops; OHC can achieve true persistent state across Hybrid modes by embedding state logs into vector databases.

## Design Doc
### 1. Architecture
* **AutoDream Pipeline:** A background worker running every 5 minutes.
* **Database Schema:** `autodream_memories` table with a `vector(1536)` column (using `pgvector` in Cloud-Native mode or a fallback stub in SQLite).
* **Integration:** Hook into `TaskQueueService.CompleteTask` to enqueue tasks for AutoDream consolidation.

### 2. Sequence Diagram
```mermaid
classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
sequenceDiagram
    participant TaskQueue
    participant AutoDreamWorker
    participant VectorDB
    TaskQueue->>AutoDreamWorker: Poll COMPLETED tasks
    AutoDreamWorker->>VectorDB: Generate & Store Embeddings
```

## Implementation Prompt
**Context:** You are implementing the AutoDream data pipeline in `srcs/server/orchestration/autodream_worker.go`.
**Instructions:**
1. Use `pgvector` for PostgreSQL compatibility. For SQLite, provide a graceful fallback that stores the raw embeddings as JSON arrays.
2. Ensure the worker is idempotent. If a lock cannot be acquired (`FOR UPDATE SKIP LOCKED`), it should gracefully skip until the next cycle.
3. Integrate with the existing Teammate Mesh to broadcast a `CONSOLIDATED` event once processing succeeds.
