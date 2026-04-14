---
status: FAILED
priority: P0
scope: Large
title: "KAIROS: Architect Shared Task List, Teammate Mesh, and autoDream"
---
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Mission: KAIROS Orchestration Design

**Title**: KAIROS: Architect Shared Task List, Teammate Mesh, and autoDream

**Problem Statement**:
The OHC swarm lacks a robust, distributed orchestration framework for complex task decomposition, high-availability inter-agent communication, and long-term memory consolidation across Cloud and Standalone modes.

**Research Report**:
Competitors force users into binary choices: pure cloud orchestration or local ephemeral execution. OHC's "Unfair Advantage" is its Hybrid Architecture. We must build a system where the Shared Task List orchestrates DAG dependencies safely, the Teammate Mesh handles local-to-cloud IPC seamlessly, and autoDream consolidates task memory into pgvector/SQLite.

**Design Doc**:
See `docs/architecture/kairos/master_design_doc.md` for exact API contracts and schemas.
- **Database**: `shared_tasks` and `consolidated_memory`.
- **Channels**: `mesh:tasks`, `mesh:coordination`.

**Implementation Prompt**:
You are an Implementer agent. Execute the following:
1. Implement the Teammate Mesh utilizing `rueidis` (Redis) and Go channels (SQLite fallback).
2. Create the `shared_tasks` queueing logic with `FOR UPDATE SKIP LOCKED` and fallback mutexes.
3. Build the AutoDream pipeline to embed `DONE` tasks into `consolidated_memory`.
Ensure >90% test coverage and full OpenTelemetry exposure.

**Priority**: P0
**Estimated Scope**: Large

</div>
