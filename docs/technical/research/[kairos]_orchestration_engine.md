# Title: KAIROS Orchestration Engine Implementation

## Problem Statement
Current systems lack a unified orchestration layer to manage complex, multi-agent workflows across hybrid environments (Cloud-Native vs. Standalone). We need the KAIROS engine to enable dynamic task decomposition, real-time agent coordination, and long-term memory consolidation, ensuring graceful degradation and a premium visual experience.

## Research Report
- **Competitor Analysis:** Claude Code and Replit Agent rely on single-agent loops. OHC's Swarm Intelligence requires shared state and distributed coordination.
- **Core Requirements:**
  - A Shared Task List with deterministic transitions (`FOR UPDATE SKIP LOCKED` for Postgres, mutexed for SQLite).
  - A Teammate Mesh for real-time pub/sub (`CentrifugeNode`, Redis/Memory).
  - An AutoDream pipeline for long-term vector embeddings (`pgvector` / SQLite JSON).
- **References:** KAIROS Master Design (`docs/features/kairos/master_design.md`), Shared Task List Design (`docs/features/kairos/shared_task_list_design.md`), State Machine Tracker (`docs/features/kairos/state_machine.md`), Sub-Agent Queue (`docs/features/kairos/sub_agent_queue.md`), AutoDream Pipeline (`docs/features/kairos/autodream_pipeline.md`).

## Design Doc
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); color: #fff;">
### 1. Phase 1: UltraPlan & Shared Task List (Database & Sequences)
- **Schema Adjustments:**
  - `mcp_tool_state`: `tool_id`, `key`, `value`, `updated_at`.
  - `shared_tasks`: Add `claimed_by`, `claim_status`.
  - `state_machine_transitions`: `id`, `entity_id`, `entity_type`, `from_state`, `to_state`, `agent_id`, `reason`, `occurred_at`.
  - `autodream_memories`: `id`, `organization_id`, `agent_id`, `content`, `embedding` (vector 1536), `source_type`. Add index on `processed_at`.
- **Sequence:**
  - Task Enqueued -> DynamicTaskRouter broadcasts `task.available` via Redis.
  - Agents bid (`task.claim`) -> Router locks row (`FOR UPDATE` / SQLite lock) -> Assigns to highest capability.

### 2. Phase 2: Realtime Teammate Mesh APIs
- **Hub:** Integrate `CentrifugeNode` for WebSocket/Pub-Sub.
- **Channels:** Define namespaces: `mesh:tasks`, `mesh:coordination`, `mesh:ultraplan`.
- **Transports:** Implement `RedisMeshTransport` (Cloud) and `MemoryMeshTransport` (Standalone).

### 3. Phase 3: AutoDream Data Pipelines
- **Worker Daemon:** `AutoDreamConsolidator` claims batches of `agent_session_data`.
- **Embeddings:** Generate 1536-dim embeddings via LLM client.
- **Storage:** Persist to `pgvector` (Cloud) or JSON blobs (Standalone, degrading gracefully).

### UI Constraints
Ensure all related UI components strictly adhere to the OHC Premium Feel defined by the wrapper `div` styles.
</div>

## Implementation Prompt
**To the Implementer:**
Please implement the KAIROS Orchestration Engine per the Design Doc above.
1. Implement the database schema changes for `mcp_tool_state`, `shared_tasks`, `state_machine_transitions`, and `autodream_memories` using the existing Go migration framework. Ensure `FOR UPDATE SKIP LOCKED` is used for Postgres and gracefully falls back for SQLite.
2. Implement the `DynamicTaskRouter` in `src/server/orchestration/` to handle task claiming via Redis/Memory pub-sub.
3. Integrate `CentrifugeNode` for the Teammate Mesh APIs across `mesh:tasks`, `mesh:coordination`, and `mesh:ultraplan`.
4. Build the `AutoDreamConsolidator` background worker to batch process session memories and generate embeddings.
5. Achieve 100% unit test coverage for all new files. Add comprehensive E2E tests validating the full workflow.

## Priority
P0

## Estimated Scope
Large
