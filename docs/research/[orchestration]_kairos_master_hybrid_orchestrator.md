# Title: Implement KAIROS AI OS Hybrid Orchestrator

**Problem Statement**:
The OHC Hybrid Agentic OS requires a seamless degradation path across Cloud-Native and Standalone Desktop environments. Specifically, we lack the unified orchestration engine combining Shared Task Lists, Teammate Mesh, AutoDream, and Sub-Agent Queues into a single state machine.

**Research Report**:
Competitor analysis confirms that Claude Code lacks swarm shared-state memory, while OpenClaw lacks a local standalone runtime. Our Hybrid Architecture dictates that all orchestration components (Task List, Mesh, AutoDream, Queue) gracefully fall back from Postgres/Redis to SQLite/In-Memory logic.

**Design Doc**:
Refer to `docs/architecture/KAIROS_AI_OS_HYBRID_ORCHESTRATION_MASTER.md` for complete schema definitions, pub/sub channels, and vector database requirements.

**Implementation Prompt**:
Implement the unified KAIROS AI OS Orchestrator. This includes schema creation (`shared_tasks_decomposition`, `autodream_memories`, `sub_agent_queue`), hybrid locking strategies (`FOR UPDATE SKIP LOCKED` vs SQLite transactions), Redis/Memory Teammate Mesh broadcasting (`POST /api/mesh/broadcast`), and quota-enforced sub-agent background queuing. Unit test coverage MUST be 100%. Ensure UI components adhere to the OHC Premium styling.

**Priority**: P0
**Estimated Scope**: Large
