# [backend] Implement AutoDream Data Pipelines for OHC VectorDB

## Problem Statement
The OS loses context over time as agent sessions cycle. We need a persistent architectural memory to inform future swarm actions. The Swarm Intelligence Protocol (OHC-SIP) dictates that temporary agent scratchpads be consolidated into long-term durable state.

## Research Report
By extracting UltraPlans and closed Tasks, embedding them via LLMs, and indexing them using pgvector, we can provide a semantic search API that acts as OHC's long-term memory (AutoDream).

## Design Doc
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

**autoDream Memory Vector Architecture**

The AutoDream Data Pipeline extracts episodic memory from the `OHC_MEMORY_DIR` or from completed tasks in the Shared Task List, chunks it, embeds it, and stores it in the vector database.

**Storage Configuration (`consolidated_memory`)**
```sql
CREATE TABLE IF NOT EXISTS consolidated_memory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES shared_tasks(id),
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX ON consolidated_memory USING ivfflat (embedding vector_cosine_ops);
```

**Workflow**
1. AutoDream Worker watches the runtime memory directory and task states.
2. Content is chunked and tokenized.
3. Content is embedded using the configured embedding API.
4. Resulting vectors are stored in PostgreSQL/Local SQLite.

</div>

## Implementation Prompt
Implement the AutoDream Data Pipelines in `src/server/orchestration/autodream/`. Create the `consolidated_memory` schema with the `embedding vector(1536)` column and the corresponding `ivfflat` index in PostgreSQL. Add SQLite fallbacks for standalone mode using local FTS/Vector extensions. Build an asynchronous worker that extracts closed Tasks, embeds them using the configured LLM API (e.g., Minimax, Cohere, or OpenAI), and stores them in the database. Implement a semantic search abstraction to query this long-term memory via cosine similarity. Ensure that the pipeline gracefully handles embedding API rate limits. Write thorough unit and integration tests covering the embedding extraction, storage, and retrieval processes.

## Priority
P1

## Estimated Scope
Medium
