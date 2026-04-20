<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AutoDream Pipelines

The AutoDream Pipelines consolidate short-term agent episodic memory into long-term embedded knowledge.

## Pipeline Flow

```mermaid
sequenceDiagram
    participant Worker as Agent Worker
    participant Storage as Short-term Buffer
    participant LLM as Embedding Model
    participant DB as Vector Database

    Worker->>Storage: Store Episodic Memory
    Storage->>LLM: Batch Process Episodes
    LLM-->>Storage: Vector Embeddings
    Storage->>DB: Upsert into Consolidated Memory
```

## Integrations
Seamlessly syncs with KAIROS orchestration components.

</div>
