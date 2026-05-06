# Memory Consolidation Architecture

## 1. Persistent Memory Layer
The Persistent Memory Layer allows every AI department to retain knowledge across sessions.
When an AI department processes a business event, the relevant context is embedded using a 1536-dimensional vector and stored.

**Schema:**
The consolidated memory is stored in the `consolidated_memory` table, which includes:
- `id`: Unique identifier for the memory record.
- `tenant_id`: Scope for all operations (multi-tenant isolation).
- `agent_id`: Identifies which agent generated the context.
- `content`: The raw textual context.
- `embedding`: A 1536-dimensional vector representing the semantic meaning of the content.
- `source_type`: Type of the memory (e.g., "TASK_SUMMARY").
- `created_at`: The timestamp when the memory was initially created.
- `last_referenced_at`: The timestamp of the last time this memory was matched or accessed.
- `reference_count`: The number of times the memory was referenced in subsequent queries.
- `reliability_score`: Heuristic score (0-100) based on source certainty.
- `owner_override`: Boolean flag indicating if the business owner explicitly provided or verified this context.
- `metadata`: Optional JSON string for extra info.

**Storage Modes:**
- **Cloud Mode:** Uses PostgreSQL with the `pgvector` extension for efficient cosine distance querying (`<=>`).
- **Standalone Mode:** Uses SQLite, implementing the `vec_distance_cosine` extension to ensure parity in semantic search queries. Both models ensure isolated, tenant-scoped operations.

## 2. Cross-Department Context Sharing
Memory is not siloed. While the `agent_id` is tracked, the fundamental semantic search is performed by querying across the `tenant_id`:

```sql
SELECT id, tenant_id, agent_id, content, embedding, ...
FROM consolidated_memory
WHERE tenant_id = $1
ORDER BY embedding <=> $2::vector
LIMIT $3
```
This enables an agent in the "Business Advisory" department to retrieve contexts originally inserted by an agent in the "Customer Success" department.
Whenever a record is accessed, its `last_referenced_at` and `reference_count` fields are automatically incremented.

## 3. Conflict Resolution
Duplicate or overlapping context entries can occur when facts evolve (e.g., "$50" vs "$55" for a cake).
The background worker `MemoryConsolidationWorker` periodically identifies potential conflicts by locating memory pairs within the same tenant that have a cosine distance < 0.05.

The logic resolves conflicts with the following precedence:
1. `owner_override` (Explicit human overrides win)
2. `reliability_score` (Highly reliable sources win over uncertain ones)
3. `created_at` (Recency wins)

When resolving a conflict, the winning record absorbs the `reference_count` of the loser to retain its historical importance, and the losing record is securely deleted.

## 4. Stale Context Pruning
To prevent context windows from being cluttered with outdated info, the system automatically prunes stale memories.
The `MemoryConsolidationWorker` deletes context if it meets all of the following conservative criteria:
- **Age**: Not referenced in the last 180 days (`last_referenced_at < NOW() - 180 days`).
- **Override Status**: No human override (`owner_override = FALSE`).
- **Significance**: Rarely used (`reference_count < 5`).
- **Type Restrictions**: Limited to specific types like `"TASK_SUMMARY"`, ensuring critical data is retained.

The background worker handles both pruning and conflict resolution asynchronously to prevent blocking the main AI request path.