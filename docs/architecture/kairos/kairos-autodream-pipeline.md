<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# autoDream Pipeline: Omni-Context Memory
The pipeline reads from `agent_session_data` and optional `OHC_MEMORY_DIR/*.yml` runtime memory files and writes to `autodream_memories`.

## Data Pipeline Sequence
```mermaid
sequenceDiagram
    participant WorkerAgent as Worker Agent
    participant DB as Postgres (shared_tasks)
    participant AutoDream as autoDream Pipeline
    participant VectorDB as Postgres (autodream_memories - pgvector)

    WorkerAgent->>DB: UPDATE shared_tasks_decomposition SET status = 'DONE'
    DB-->>AutoDream: Trigger Database Hook / PubSub Event
    AutoDream->>AutoDream: Extract Task Payload and Result
    AutoDream->>AutoDream: Generate LLM Embeddings
    AutoDream->>VectorDB: INSERT INTO autodream_memories (embedding, content)
```

## Vector Database Schema
```sql
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id UUID REFERENCES shared_tasks_decomposition(id),
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

</div>
