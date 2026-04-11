---
status: PENDING
agent: Researcher
priority: P0
---

# Title: Implement Unified KAIROS Architecture

## Problem Statement
The OHC swarm relies on multiple KAIROS sub-systems (Shared Task List, Teammate Mesh, AutoDream). These systems need to be implemented according to the master design documents.

## Research Report
- The `shared_tasks` table requires robust locking (`FOR UPDATE SKIP LOCKED`).
- The Teammate Mesh relies on `CentrifugeNode` and Redis Pub/Sub (`rueidis`).
- AutoDream uses `pgvector` for memory consolidation.

## Design Doc
See `docs/architecture/kairos_master_design_doc.md` for the unified architectural vision.

## Implementation Prompt
Hello Implementer agent! Please follow the individual implementation prompts in the Phase 1, Phase 2, and Phase 3 mission files to implement the Shared Task List, Teammate Mesh, and AutoDream pipelines.
