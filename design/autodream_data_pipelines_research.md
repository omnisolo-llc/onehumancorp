Parent: #4909

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# [research] Architect AutoDream Data Pipelines for OHC VectorDB

## Problem Statement
To achieve true Swarm Intelligence, OHC agents must possess long-term memory. The AutoDream system must periodically consolidate execution logs and task states into dense vector embeddings for semantic recall. Currently, we lack the data pipeline architecture connecting the execution layer to our pgvector database.

## Research Report
- **Embedding Models**: Using standard 1536-dimensional embeddings (e.g., OpenAI or local equivalents) offers a good balance of semantic richness and storage cost.
- **Storage**: pgvector extension in our PostgreSQL cluster allows hybrid SQL/Vector queries, crucial for joining memories with structured task data.
- **Pipeline**: A batch job should poll for COMPLETED tasks, generate embeddings from their execution logs, and insert them into the VectorDB.

## Design Doc
1. **Data Model**: Create `autodream_memories_master` table with an `embedding vector(1536)` column.
2. **Pipeline Architecture**:
```mermaid
sequenceDiagram
    participant Worker Agent
    participant DB as Shared Tasks
    participant AutoDream Worker
    participant LLM as Embedding Model
    participant VectorDB as pgvector

    Worker Agent->>DB: Marks task COMPLETED
    AutoDream Worker->>DB: Poll for recently completed tasks
    AutoDream Worker->>LLM: Generate vector embedding from execution log
    LLM-->>AutoDream Worker: Returns Vector[1536]
    AutoDream Worker->>VectorDB: INSERT into autodream_memories_master
```
3. **Component**: Implement `AutoDreamConsolidator` in `srcs/server/orchestration/autodream_worker.go`.

## Implementation Prompt
Hello Implementer!
1. Add Postgres migration scripts to create the `autodream_memories_master` table using the `pgvector` extension.
2. Implement the `AutoDreamConsolidator` pipeline in `srcs/server/orchestration/autodream_worker.go` that fetches COMPLETED tasks and generates embeddings using `lib/llm/embeddings.go`.
3. Implement the Postgres insert logic using `pgx` to store the generated embeddings.
4. Write tests to verify the batching logic and ensure `bazel test //srcs/server/orchestration/autodream/...` passes.

## Priority
P0

## Estimated Scope
Large

</div>
