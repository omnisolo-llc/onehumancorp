# AutoDream Data Pipelines Design

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

## Pipeline Architecture
To synthesize the agent's long-term learnings, AutoDream extracts structural state into a pgvector store.
1. **Extraction**: Cron-driven jobs extract finalized UltraPlans and resolved Tasks.
2. **Embedding**: Payloads sent to LLM for dense vector embedding generation.
3. **Storage**: Vectors upserted into `pgvector` indexed tables.

```sql
CREATE TABLE knowledge_embeddings (
    id UUID PRIMARY KEY,
    content TEXT,
    embedding VECTOR(1536)
);
CREATE INDEX ON knowledge_embeddings USING ivfflat (embedding vector_cosine_ops);
```

</div>
