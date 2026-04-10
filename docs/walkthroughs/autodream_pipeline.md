<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Data Pipeline Walkthrough

Welcome to the interactive walkthrough for the **AutoDream Data Pipeline**, the core component of the KAIROS Orchestrator that powers long-term memory consolidation for the OHC Swarm.

## 1. Why AutoDream?

Agents generate massive amounts of ephemeral data during task execution. Without a mechanism to distill this information, agent context windows would quickly overflow. AutoDream prevents this by continuously sweeping session data and consolidating it into durable vector embeddings.

## 2. Memory Consolidation Workflow

The AutoDream pipeline operates asynchronously to ensure it never blocks primary agent execution. It follows a rigorous sequence to convert raw text into highly queryable vectors.

```mermaid
sequenceDiagram
    participant Worker as Agent (Worker)
    participant Mesh as Teammate Mesh (Redis/Local)
    participant AutoDream as AutoDreamWorker (Background)
    participant Embed as LLM Embedding API
    participant DB as PgVector/SQLite

    Worker->>Mesh: 1. Broadcast "Task Started" (mesh:tasks)
    Worker->>Mesh: 2. Share Findings (mesh:coordination)
    Worker->>Worker: 3. Complete Task & write to .agent-task/memory
    Worker->>Mesh: 4. Broadcast "Task Completed" (mesh:tasks)
    AutoDream->>Worker: 5. Wake up & Read .agent-task/memory/*.yml
    AutoDream->>Embed: 6. Request Context Compression (Tokens -> Vector)
    Embed-->>AutoDream: 7. Return 1536-dim Vector
    AutoDream->>DB: 8. Upsert to agent_memories (pgvector)
    AutoDream->>Worker: 9. Prune stale agent_session_data (>24h)
```

## 3. Storage Adapters

AutoDream is built for the OHC Hybrid Architecture (OHC-HA) and adapts its storage engine automatically:

- **Cloud-Native Mode:** Uses **PostgreSQL with the `pgvector` extension** for Exact Nearest Neighbor search across multi-tenant data on 1536-dimensional embeddings.
- **Standalone Mode:** Degrades gracefully to **SQLite**.

For a deeper look into the exact schema and locking mechanisms, check out the [AutoDream Feature Guide](../features/kairos/autodream_pipeline.md).

</div>
