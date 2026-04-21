<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Orchestrator Architecture Master Design

## Phase 1: Shared Task List Decomposition
**Architecture:**
Uses a PostgreSQL backend for Cloud-Native environments with `FOR UPDATE SKIP LOCKED` for task claiming. For Standalone mode, uses SQLite with mutex-based coordination.
**Schema:**
`shared_tasks_decomposition` tracking status (PENDING, IN_PROGRESS, COMPLETED) and DAG dependencies.

**Sequence:**
1. Agent queries `shared_tasks_decomposition`
2. Locks row & sets status to `IN_PROGRESS`
3. Executes task, updates state to `COMPLETED`

## Phase 2: Realtime Teammate Mesh APIs
**Architecture:**
Pub/Sub system for state propagation. Uses Redis Pub/Sub in Cloud-Native and an in-memory event bus for Standalone.
**Endpoint:**
`POST /api/mesh/broadcast` distributes state machine transitions (e.g. Task completed -> Dependencies unblocked).

## Phase 3: AutoDream Data Pipelines
**Architecture:**
Background daemon converts episodic memory to semantic vectors.
**Storage:**
Uses pgvector (`vector(1536)`) within the `autodream_memories` table in PostgreSQL.

## Phase 4: Sub-Agent Orchestration
**Architecture:**
Background queue (e.g. BullMQ pattern) with strict VRAM/Token quota enforcement per sub-agent. Degrades gracefully to local `sub_agent_queue` table in SQLite.

</div>
