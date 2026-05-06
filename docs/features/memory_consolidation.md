# Memory Consolidation Architecture

The Long-Term Memory and Context Consolidation System provides AI agents with persistent, shared memory across sessions. This allows every AI department to retain knowledge, ensuring a continuous and context-aware user experience. The system is designed to work seamlessly in both Cloud (PostgreSQL + pgvector) and Standalone (SQLite + sqlite-vec) environments.

## Persistent Memory Layer
The Persistent Memory Layer is responsible for storing and retrieving embedded AI agent context long-term.
- **Storage Backend**: Contexts are embedded into vector format (1536 dimensions) and stored in the `consolidated_memory` table.
- **Tenant Scoping**: All operations enforce strict tenant isolation using `tenant_id` and Row-Level Security (RLS) in PostgreSQL, ensuring that business-specific contexts never bleed across organizations.
- **Semantic Search**: Retrieval is performed via vector cosine distance, allowing the system to find conceptually related context (e.g., matching "vegan cake" inquiries with past "plant-based dessert" notes).

## Conflict Resolution Strategy
As memory accumulates, agents might ingest contradictory facts (e.g., "$50" vs "$55" for pricing).
- **Detection**: Background workers periodically scan for context blocks with high vector similarity (cosine distance < 0.05).
- **Resolution Matrix**:
  1.  **Owner Override**: Human-verified facts explicitly marked by the owner (`owner_override = true`) always win over auto-ingested context.
  2.  **Reliability Score**: If no override exists, the context with the higher reliability score (based on the source plugin or event type) wins.
  3.  **Recency**: If scores are identical, the most recently created context wins.
- **Consolidation**: The losing context is pruned, but its `reference_count` is merged into the winner to reflect the fact's historic importance.

## Stale Context Pruning
A dedicated `MemoryConsolidationWorker` runs background loops (every hour) to aggressively clean up outdated or irrelevant information.
- **Pruning Criteria**: Contexts that have not been referenced (`last_referenced_at`) in over 180 days are considered stale.
- **Conservation Rules**: Stale contexts are only purged if:
  - They are NOT marked as an `owner_override`.
  - Their `reference_count` is less than 5.
  - Their `source_type` is `TASK_SUMMARY` (protecting explicit user inputs or structural facts).

## Cross-Department Context Sharing
Memory is not siloed to the agent that created it. Context ingested by one department (e.g., Customer Success logging a support issue) is globally embedded into the tenant's consolidated memory.
- When an agent in another department (e.g., Business Advisory) performs a semantic search during planning, the relevant context is retrieved regardless of the originating `agent_id`.
- This creates a unified "brain" for the business, ensuring holistic awareness across all interactions.
