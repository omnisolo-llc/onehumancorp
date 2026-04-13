<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS: Phase 3 - Architect autoDream Data Pipeline

## Problem Statement
OHC Agents execute complex tasks but do not currently learn or retain execution history in a queryable format. We need an `autoDream` data pipeline to consolidate completed tasks into a Vector DB for long-term "semantic memory".

## Research Report
*   **Vector Search**: `pgvector` provides seamless integration with Postgres for Cloud Mode. SQLite `vss` or a similar local extension can be used for Standalone Mode.
*   **Pipeline Flow**: Completed Task Data -> LLM Summarization -> Text Embedding Model -> Vector Storage.

## Design Doc
### autoDream Pipeline Architecture
**Schema `agent_memories`**:
*   `id` (UUID)
*   `organization_id` (UUID)
*   `task_id` (UUID, FK to `tasks.id`)
*   `raw_content` (TEXT)
*   `summary_embedding` (VECTOR(1536))
*   `created_at` (TIMESTAMP)

**Worker Job**:
1.  A background CRON job (`AutoDreamWorker`) polls `tasks` where `status = 'COMPLETED'` and `auto_dreamed = false`.
2.  Invokes the Minimax API to generate a summary of the task execution logs.
3.  Invokes the Embeddings API to generate a 1536-dimensional vector.
4.  Upserts into `agent_memories` and sets `tasks.auto_dreamed = true`.

## Implementation Prompt
**Role**: Implementer Agent
**Task**: Implement the autoDream Vector DB consolidation pipeline.
**Instructions**:
1. Add a DB migration for the `agent_memories` table with `pgvector` support (Cloud) and fallback for Standalone.
2. Implement the `AutoDreamWorker` in `srcs/server/workers/autodream.go`.
3. Integrate the Embedding API call and the `pgvector` insert logic.
4. Schedule the worker to run periodically using the existing scheduler framework.
**Acceptance Criteria**:
*   Migrations apply cleanly.
*   The worker successfully identifies completed tasks and processes mock embeddings.
*   Unit tests cover the worker loop and DB operations.

## Priority
P1

## Estimated Scope
Medium

</div>
