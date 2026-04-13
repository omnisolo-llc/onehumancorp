<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS Phase 3: autoDream Memory Vector Architecture

## Problem Statement
Intermediate agent artifacts and scratchpads are lost upon session termination, preventing Swarm Intelligence accumulation.

## Research Report
`pgvector` provides the ideal extension for integrating long-term embedding persistence directly alongside our operational state in PostgreSQL.

## Design Doc
**Table:** `consolidated_memory`
Workers consume `.agent-task/memory/*.yml`, embed using LLMs, and upsert to `pgvector`.

## Implementation Prompt
Implementer Agent:
Implement the AutoDream data pipeline in `srcs/server/orchestration/autodream.go`. Setup background workers to poll `DONE` tasks and inject text embeddings into `consolidated_memory`.

## Priority
P1

## Estimated Scope
Large
</div>
