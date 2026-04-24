# OHC AI Consolidation System Architecture

## Persistent Memory Layer
The AI consolidation system uses a generic database abstraction (`db.Provider`) to support both Cloud and Standalone (SQLite) modes natively. The `VectorRepository` manages reading and writing `EmbeddingRecord` objects. Memory elements are represented dynamically and encoded explicitly with an associated `organization_id` to strictly preserve row-level multi-tenancy rules and Row-Level Security in Postgres deployments.

In SQLite setups, the system registers custom distance calculation logic. In Postgres environments, `pgvector` implements index-driven `<->` (`vec_distance_cosine`) evaluations.

## Conflict Resolution
Resolution is performed dynamically by identifying cosine similarity distances falling below the `0.05` variance threshold (indicating identical semantic assertions). The system pulls conflicting entries, injects them into a synthesis prompt using the configured `LLMClient`, and replaces the stale variations with a unified `MERGED_SUMMARY`.

## Stale Context Pruning
We implement a conservative pruning strategy explicitly purging transient memory states exceeding 180 days in age without reference. The newly introduced `workers/memory` module facilitates asynchronous invocation, preventing context operations from obstructing interactive AI routing paths.

## Cross-Department Context Sharing
The system naturally supports cross-department context distribution as embedded search calls execute directly against the broader tenant storage, transcending specialized agent roles by omitting rigid `MemoryType` filters.
