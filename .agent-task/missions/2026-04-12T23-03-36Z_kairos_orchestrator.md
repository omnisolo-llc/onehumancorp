---
status: PENDING
priority: P0
scope: Large
---
# Architect and Implement the KAIROS Teammate Mesh and Shared Task List

## Problem Statement
The OHC Hybrid OS requires a highly concurrent Teammate Mesh and robust Shared Task List. Currently, agents lack scalable realtime synchronization across Standalone and Cloud nodes.

## Research Report
- Benchmarks indicate Redis Pub/Sub provides sub-millisecond coordination latency.
- Standalone degradation to SQLite requires a unified queue interface.
- See Design Doc: docs/kairos/hybrid_ai_os_orchestration.md

## Design Doc
Architecture components:
- API endpoints for enqueue and claim.
- Unified queue interface supporting PostgreSQL (FOR UPDATE SKIP LOCKED) and SQLite.
- autoDream background sync to pgvector.

## Implementation Prompt
Dear Implementer,
1. Implement unified queue logic in the appropriate `srcs/server/orchestration/` files based on `provider.IsSQLite()`.
2. Add gRPC / WebSocket endpoints for the Mesh in the server directory (create the directory if it does not exist).
3. Ensure >90% test coverage via `bazelisk test //...`.
