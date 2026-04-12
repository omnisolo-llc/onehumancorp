<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03); color: #fff;">

# KAIROS Orchestration: Shared Task List, Teammate Mesh, & AutoDream

## 1. Vision
The KAIROS Orchestrator serves as the multi-tenant "Hybrid Agentic OS" kernel. It breaks down complex user intents into an executable, distributed Directed Acyclic Graph (DAG) for the swarm.

## 2. Core Architecture

### 2.1 Shared Task List (State Machine)
The `SharedTaskOrchestrator` implements a distributed DAG with pessimistic locking.
- **Cloud-Native**: PostgreSQL `FOR UPDATE SKIP LOCKED`.
- **Standalone Desktop**: SQLite single-thread Mutex application-level locks.
- **Schema**: `shared_tasks` (id, status, payload, dependencies) to model pending vs completed tasks.

### 2.2 Teammate Mesh (Realtime RPC)
Agents coordinate sub-millisecond status updates over the mesh.
- **Cloud-Native**: Redis Pub/Sub channels (e.g. `mesh:tasks`).
- **Standalone**: Mocked event bus to degrade gracefully.

### 2.3 AutoDream (Memory Consolidation)
Episodic task completion logs are periodically compressed via LLM and vectorized.
- **Cloud-Native**: pgvector `VECTOR(1536)` in `[autodream_table]`.
- **Standalone**: local embedding vector file sync or SQLite vss.

## 3. Delegation & Sub-Agents
The `TaskManager.DelegateSubTask` enqueues high-concurrency background sub-agents via Redis lists/sets or local mutexed SQLite tables to prevent race conditions during execution.

</div>
