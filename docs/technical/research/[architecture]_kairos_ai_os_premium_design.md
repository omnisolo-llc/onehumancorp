# Title: KAIROS AI OS Premium Design Blueprint

## Problem Statement
The One Human Corp (OHC) Hybrid Agentic OS requires a fluid and consistent architecture across completely different operating models. We need to formalize the technical spec for the "Standalone" wrapper (Local First) and "Thin Client" API definitions, ensuring that the OHC "Premium Feel" and robust multi-agent orchestration are indistinguishable regardless of deployment tier. Currently, we lack a single, unified premium design document that covers the master implementation loop (Think -> Act -> Observe -> Decide).

## Research Report
The KAIROS orchestrator must handle:
1.  **Task Decomposition**: Managing a shared distributed task list to decouple task submission from execution.
2.  **Sub-Agent Orchestration**: Managing a scalable background queuing logic needed to spawn isolated sub-agents in a production environment with proper VRAM/Token quotas.
3.  **Teammate Mesh**: Providing highly available realtime communication via Redis Pub/Sub (Cloud) or Go Channels (Standalone) to avoid race conditions.
4.  **AutoDream pipelines**: Consolidating memory by extracting episodic data and converting it to Vector Embeddings.
5.  **UltraPlan Deliberation**: Implementing a state machine tracker using robust distributed locks.

These needs must be packaged into a premium aesthetic presentation for internal and external consumption.

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

**KAIROS Master Execution Playbook**

*   **Phase 1 (UltraPlan/Decomposition):** Create backend database designs and sequence diagrams for the "Shared Task List" feature. This handles the complex DAG dependencies. `shared_tasks` uses `FOR UPDATE SKIP LOCKED` in Postgres and mutexed transactions in SQLite.
*   **Phase 2 (Orchestration):** Design the Realtime Teammate Mesh APIs so other feature agents can implement them in production. Expose `POST /api/mesh/v2/broadcast`. This ensures agents coordinate without delays.
*   **Phase 3 (autoDream):** Architect the data pipelines (e.g., pgvector, LLM embeddings) for OHC's long-term state consolidation system. The system watches the runtime memory, chunks and embeds it, and stores it in `consolidated_memory`.
*   **Phase 4 (Finalize):** Sub-Agent Queue background logic with VRAM/Token quota enforcement and exponential backoff retry logic.

</div>

## Implementation Prompt
Implement the entire KAIROS Master Execution Playbook. Set up the `shared_tasks` table and implement the DAG logic. Create the Realtime Teammate Mesh APIs under `src/server/orchestration/mesh/` handling pub/sub over Redis or in-memory Go channels. Build the AutoDream data pipelines, adding `consolidated_memory` schema utilizing pgvector `vector(1536)` columns with local fallback to SQLite Vector/FTS. Complete the sub-agent queue with strict token and quota checks and exponential backoff retry mechanisms. All UI interfaces associated must inherit the OHC Premium CSS tokens strictly.

## Priority
P0

## Estimated Scope
Large
