# AI Memory Consolidation Layer Architecture

The Memory Consolidation Layer serves as the long-term, cross-department memory system for the OHC AI agents. It ensures that business context learned in one interaction (e.g., a customer's product preference) is preserved and surfaced in future interactions across all AI departments.

## Core Components

1. **Persistent Vector Storage** (`VectorRepository`):
   - Stores memories (`EmbeddingRecord`) using dense vector embeddings (1536 dims).
   - Supports both `PostgreSQL` (via `pgvector`) for Cloud Mode and `SQLite` (via `sqlite-vec` or raw bytes) for Standalone Mode.
   - All operations are strictly tenant-scoped (`tenant_id`) ensuring data isolation.

2. **Cross-Department Context Sharing** (`CrossDepartmentMemoryLayer`):
   - Provides a high-level API for departments (e.g., Business Advisory, Operations, Customer Success) to read and write memories.
   - When querying memory via `semantic_search`, it filters by `tenant_id` but *does not* filter by `agent_id`, enabling agents to benefit from insights gathered by other departments.

3. **Conflict Resolution Strategy** (`auto_resolve_conflicts`):
   - Detects when conflicting facts are stored (e.g., differing prices for a product).
   - Resolution order of precedence:
     1. **Explicit Owner Override**: If one memory was explicitly flagged by the business owner, it wins.
     2. **Source Reliability Score**: If one source is more reliable, it wins.
     3. **Recency**: The newest memory wins.
   - Handled via a background worker (`MemoryConsolidationWorker`).

4. **Stale Context Pruning** (`prune_stale`):
   - Conservatively removes irrelevant context (e.g., unreferenced interactions older than 180 days).
   - Exempts memories that have an active `owner_override` or a high `reference_count`.
   - Also executed automatically by the `MemoryConsolidationWorker`.

## Design Constraints Addressed

- **Tenant Isolation**: Handled seamlessly by forcing `tenant_id` in all operations.
- **Background Execution**: The `MemoryConsolidationWorker` runs on a non-blocking background tokio task.
- **Environment Parity**: The system fully operates in both SQLite and Postgres.