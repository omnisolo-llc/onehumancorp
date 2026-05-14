# Memory Consolidation Layer Architecture

The Memory Consolidation Layer serves as the central nervous system for OHC AI Agents, providing persistent long-term context across independent agent sessions, departments, and user interactions. The system acts autonomously, continually resolving conflicts and pruning stale context to maintain high-quality vector retrieval.

## 1. Persistent Memory Layer
The layer supports embedding storage via an agnostic `VectorRepository`. In Cloud mode, it leverages a high-performance PostgreSQL `pgvector` store with HNSW/IVFFlat indices. In Standalone mode, it gracefully falls back to a SQLite FTS/Vector representation securely stored locally.

### Tenant Isolation
Every operation within the Vector Repository is strictly scoped to the `tenant_id`. Whether retrieving cross-department context or pruning background noise, operations ensure data leakage never occurs between business owners.

## 2. Cross-Department Context Sharing
Memory is not siloed. When the Sales Agent stores an insight about a "vegan cake order" and a "budget constraint," the Operations and Advisory departments will natively retrieve this context if semantically relevant to their session. This guarantees Maya's bakery operates cohesively regardless of the entry point into the system.

## 3. Auto Conflict Resolution
Background workers routinely scan the database (per tenant) for semantically identical embeddings (< 0.05 distance) representing the same logical fact but containing disparate data.
The conflict resolution strategy:
1.  **Explicit Owner Override:** Manual business owner overrides always win.
2.  **Reliability Score:** Internal metrics of source certainty dictate priority.
3.  **Recency:** The newest fact prevails over the stale assumption.
The winner inherits the loser's reference count, consolidating knowledge seamlessly.

## 4. Conservative Stale Context Pruning
To prevent unbounded memory growth and unhelpful legacy results, the background pruning workers conservatively clean the store. Any `TASK_SUMMARY` older than the threshold (default 180 days) that has not been referenced 5 times, or any highly unreliable context, is safely deleted unless explicitly overridden by the owner.