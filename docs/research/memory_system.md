# AI Agent Context Consolidation Architecture

## Overview
This document outlines the design and implementation of the long-term memory and context consolidation system for OHC AI agents.

## Key Components

1. **Persistent Memory Layer (`VectorRepository`)**:
   - Implements multi-tenant scalable memory storage using `consolidated_memory` SQL tables.
   - Dual-mode compatible: Integrates natively with `pgvector` in cloud (PostgreSQL) and falls back to `sqlite-vec` or manual cosine distance checking in standalone mode (SQLite).
   - `search` and `retrieve` methods implemented to enable sharing context cross-department.

2. **Conflict Resolution (`MemoryConsolidationWorker`)**:
   - Periodically computes pairwise semantic conflicts in background threads.
   - Detects competing facts (using vector `<=>` operator to check similarity > 0.95).
   - Resolves conflicts logically via a rule-based priority system (owner override, reliability score, and recency).

3. **Stale Context Pruning (`prune_stale`)**:
   - Aggressively purges unreferenced memory to ensure relevant token budgets remain stable.
   - Preserves high-reliability and owner-overridden facts regardless of recency.

## Next Actions
This design ensures all OHC departments leverage shared context cross-functionally and serves as the backbone for multi-agent proactive task execution.
