# Title
Long-Term Memory and Context Consolidation System for OHC AI Agents

# Problem Statement
Currently, AI agents lack persistent context across sessions. When a business owner (e.g., Maya) reopens a session the next day, the AI forgets her preferences, popular products, and customer history. There's also no mechanism to resolve conflicting facts or prune stale data, and knowledge is siloed within individual departments.

# Research Report
The system must be tenant-scoped to ensure strict data privacy. The persistence layer requires vector embeddings to support semantic search ("vegan cake orders"). It needs to be operational in both Cloud mode (PostgreSQL with pgvector) and Standalone mode (SQLite with sqlite-vec). Conflict resolution logic needs to weigh recency, source reliability, and explicit owner overrides. Pruning must be conservative to preserve business history. A shared context layer is required to break down inter-departmental silos.

# Design Doc
## Persistent Memory Layer
- Vector storage (PostgreSQL/SQLite) for semantic search.
- Tenant-scoped data isolation.

## Conflict Resolution
- Recency-based overrides.
- Source reliability weighting.
- Explicit owner overrides.

## Stale Context Pruning
- Background workers (async).
- Pruning based on time since last reference and event type.
- Conservative pruning strategy.

## Cross-Department Context Sharing
- Centralized, tenant-scoped memory bus accessible by all agents.

# Implementation Prompt
Implement the architecture described above.

# Priority
High

# Estimated Scope
Large
