---
title: "KAIROS Master Orchestration"
status: PENDING
agent: Implementer
priority: P0
estimated_scope: Large
---
# Title: KAIROS Master Orchestration
## Problem Statement
The One Human Corp (OHC) platform lacks a central "KAIROS" orchestration layer. We need to implement the full KAIROS Orchestration framework encompassing the Shared Task List, Teammate Mesh, AutoDream Pipeline, and Sub-Agent Queue to enable autonomous execution and coordination.
## Research Report
- Based on CLAUDE_OHC.md and README.md, OHC operates in a "Hybrid Architecture" (OHC-HA).
- We need robust locking: PostgreSQL row-level locks (FOR UPDATE SKIP LOCKED) in cloud mode, and application-level semaphores or simple transaction isolation in SQLite standalone mode to prevent worker collision when claiming tasks.
- Cloud-Native mode requires Redis Pub/Sub driving Centrifuge WebSocket hubs (mesh:tasks, mesh:coordination).
- OHC uses Vector DBs (e.g., pgvector, Pinecone) for long-term memory.
## Design Doc
See docs/architecture/KAIROS_AI_OS_ARCHITECTURE.md for full context.
- **Shared Task List Schema:** 033_master_shared_tasks.sql
- **Teammate Mesh:** srcs/server/orchestration/mesh.go
- **AutoDream Schema:** 034_master_autodream.sql
- **Sub-Agent Queue Schema:** 035_master_sub_agent_queue.sql
## Implementation Prompt
Dear Implementer,
Please execute the following technical changes to enable the KAIROS Orchestration framework:
1. Shared Task List: Create 033_master_shared_tasks.sql with shared_tasks table. Implement data access layer in srcs/server/orchestration/tasks_db.go.
2. Teammate Mesh: Implement LocalTeammateMesh in srcs/server/orchestration/mesh.go. Update srcs/server/orchestration/centrifuge_hub.go.
3. AutoDream Pipeline: Create 034_master_autodream.sql with autodream_memories table. Implement background worker in srcs/server/orchestration/autodream_worker.go.
4. Sub-Agent Queue: Create 035_master_sub_agent_queue.sql with sub_agent_queue table. Implement queuing logic in srcs/server/orchestration/queue/queue.go.
5. Testing: Write >90% coverage Go tests for all new files. Add migrations to Bazel embedsrcs. Test thoroughly with ~/go/bin/bazelisk test //srcs/server/orchestration/... .
Make sure to strictly adhere to OHC security protocols (SPIFFE/SPIRE).
