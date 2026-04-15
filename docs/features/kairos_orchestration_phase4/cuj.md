<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; font-family: 'Outfit', 'Inter', sans-serif;">

# KAIROS Orchestration: Unified Architecture (Phase 4)

## Critical User Journeys (CUJ)

1. **System Startup:** Operator spins up the OHC cluster with KAIROS layer.
2. **Task Registration:** A sub-agent creates a new task within the state machine using Postgres `FOR UPDATE SKIP LOCKED`.
3. **Agent Delegation:** The task is claimed by another sub-agent dynamically via the Teammate Mesh APIs.
4. **Knowledge Retrieval:** Post-task completion, AutoDream embeddings are generated, enabling semantic search querying.

</div>
