# OHC AI Agent Context Consolidation System

## 1. Overview
The Memory Consolidation Layer enables AI departments to retain knowledge across sessions. It supports the storage, semantic search, conflict resolution, and pruning of business context. The system is designed to work seamlessly in both Cloud (PostgreSQL with `pgvector`) and Standalone (SQLite with vector extensions) environments, with strict tenant-isolation applied.

## 2. Architecture Components

### 2.1 Persistent Memory Layer (`VectorRepository`)
The `VectorRepository` acts as the primary interface for memory operations, interacting with the `consolidated_memory` table.
- **Storage Strategy:** Stores agent contexts as vector embeddings (1536 dimensions) along with metadata like `tenant_id`, `agent_id`, `source_type`, and timestamps.
- **Semantic Search:** Facilitates cross-department context sharing. A query embedding is generated and compared against stored embeddings using cosine distance (`<=>` in Postgres, `vec_distance_cosine` in SQLite) scoped strictly by `tenant_id`.

### 2.2 Conflict Resolution (`auto_resolve_conflicts`)
Conflicts occur when the same semantic fact is stored with varying details (identified when cosine distance < 0.05).
- **Rules Engine:**
  1. `owner_override`: Explicit user overrides take precedence.
  2. `reliability_score`: Higher confidence sources win.
  3. Recency: Newer entries overwrite older ones.
- **Merging:** The "winning" record absorbs the reference counts of the "losing" record to signify its strengthened validity, while the loser is deleted.

### 2.3 Stale Context Pruning (`prune_stale`)
To prevent unbounded memory growth, background pruning processes remove outdated context.
- **Conservative Approach:** Only deletes records older than 180 days (`last_referenced_at`), where `owner_override = FALSE`, and `reference_count < 5`. This ensures valuable, actively referenced business history is retained.

### 2.4 Asynchronous Background Worker (`MemoryConsolidationWorker`)
The `MemoryConsolidationWorker` is a `tokio::spawn` background task that prevents memory operations from blocking the main AI request path. It polls every hour (3600s) to run the `prune_stale` and `auto_resolve_conflicts` pipelines.

```mermaid
graph TD
    A[AI Agent] -->|Store Context| B(VectorRepository)
    A -->|Retrieve Context| B
    B -->|Upsert/Query| C[(consolidated_memory)]
    D[MemoryConsolidationWorker] -->|Background Tick Hourly| E{Maintenance Tasks}
    E -->|prune_stale| C
    E -->|auto_resolve_conflicts| C
```

## Security and Privacy in the Memory Layer

### 13. Differential Privacy in Memory Aggregation
While strict tenant isolation is required for raw data, OHC can leverage aggregated, anonymized memory to improve the underlying foundation models. We must employ Differential Privacy techniques when aggregating data (e.g., determining the average price of a plumbing service in Ohio) to ensure that no individual business's proprietary pricing data can ever be reverse-engineered from the aggregated model.

### 14. Ephemeral Context Windows
Not all memory needs to be stored permanently. During a live chat session, the agent needs a deep understanding of the current conversation, but that granular detail (e.g., "The customer asked if the blue shirt comes in red") is usually irrelevant a month later. The architecture must support "Ephemeral Context Windows" (e.g., using Redis) for short-term session state, preventing the primary vector database from becoming bloated with conversational noise.

### 15. Automated PII Redaction Before Embedding
Before any text is passed to an embedding model to be stored in the vector database, it must pass through a strict PII (Personally Identifiable Information) redaction pipeline. Credit card numbers, social security numbers, and specific addresses must be scrubbed or tokenized to ensure the memory layer does not become a toxic liability in the event of a breach.


### 16. Contextual Reinforcement Learning
Feedback from user approvals or rejections in the Action Feed must directly influence the weights in the vector database. Approved actions should have their contextual memory pathways strengthened, while rejected actions should trigger a localized pruning or down-weighting mechanism to ensure agents continually adapt to the owner's preferences.
### 17. Embedding Distance Metrics
### 18. Document Chunking Strategies
### 19. Index Warm-Up Protocols
### 20. Tokenization Rate Limits
### 21. Embedding Distance Metrics
### 22. Document Chunking Strategies
### 23. Index Warm-Up Protocols
### 24. Tokenization Rate Limits
