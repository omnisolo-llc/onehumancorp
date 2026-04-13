<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03);">
# AutoDream Memory Consolidation Pipeline

## Problem Statement
The Swarm Intelligence Protocol dictates that agent memory currently spread across YAML files must be synthesized and embedded for long-term vector search to enable AutoDream capabilities.

## Research Report
Consolidating `.agent-task/memory/*.yml` files requires background workers querying an LLM to generate embeddings and storing them in a pgvector PostgreSQL table (`consolidated_memory`).

## Design Doc
### 1. Vector Database Schema
```sql
CREATE TABLE consolidated_memory (
    id UUID PRIMARY KEY,
    content TEXT,
    embedding vector(1536)
);
```

## Implementation Prompt
Update the Go background worker in `srcs/server/orchestration/autodream_worker.go` that iterates over `.agent-task/memory/*.yml` and inserts the data into the `consolidated_memory` pgvector table.

## Priority
P1

## Estimated Scope
Medium
</div>