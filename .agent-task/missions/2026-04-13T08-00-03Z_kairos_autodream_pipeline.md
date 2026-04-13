---
title: "KAIROS: autoDream Memory Consolidation Pipeline (Phase 3)"
status: PENDING
priority: P0
scope: Medium
agent: Implementer
---

# Title: KAIROS: autoDream Memory Consolidation Pipeline (Phase 3)

## Problem Statement
While the `AutoDreamWorker` (`srcs/server/orchestration/autodream.go`) exists, it is currently a collection of heuristic background tasks. We need to formalize this into a structured "Memory Consolidation Pipeline" that ensures all `DONE` tasks and episodic agent session data are synthesized into the global `swarm_truth_embeddings` with high semantic fidelity.

## Research Report
- Current `ingestCompletedTasks` logic is basic and lacks robust error recovery if the LLM/Embedding API fails.
- Hybrid architecture requires a reliable way to sync local SQLite memories to Cloud pgvector (Hybrid MCP RAG Sync).
- The "Synthesis" phase (compressing raw logs) is the most expensive and needs better batching.

## Design Doc
1. **Pipeline Stages**:
   - **Extraction**: Efficiently sweep `shared_tasks` (status=DONE) and `agent_session_data`.
   - **Synthesis**: Group related tasks and use `CachedMinimaxClient` to generate a "Learned Insight" summary.
   - **Embedding**: Generate vectors for insights.
   - **Consolidation**: Upsert into `swarm_truth_embeddings` and `autodream_memories`.
2. **Hybrid RAG Sync**:
   - Implement a "Sync Status" flag on memories to track what has been pushed from Standalone to Cloud.
3. **Conflict Resolution**:
   - Enhance the `resolveConflicts` logic to use semantic similarity thresholds to trigger "Memory Merging" cycles.

## Implementation Prompt
Refactor and finalize the `AutoDreamWorker` in `srcs/server/orchestration/autodream_pipeline.go`.
1. Formalize the pipeline using a worker-pool pattern to handle Synthesis and Embedding in parallel.
2. Implement a `SyncToCloud` method that identifies local memories not yet present in the remote pgvector store (using `Hybrid MCP RAG` protocols).
3. Improve the `ConsolidateEpoch` method to create a "Global Knowledge Graph" update by linking new insights to existing memory IDs.
4. Add metrics: `ohc_autodream_records_processed_total`, `ohc_autodream_synthesis_duration_seconds`.
5. Ensure graceful degradation: If `MINIMAX_API_KEY` is missing, fallback to standard FTS (Full Text Search) indexing in SQLite.
6. Write coverage tests (>90%) for the full pipeline lifecycle.

## Priority
P0

## Estimated Scope
Medium
