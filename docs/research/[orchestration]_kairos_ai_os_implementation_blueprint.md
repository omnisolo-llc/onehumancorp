# KAIROS Orchestration Implementation Blueprint

**Title**: Implement KAIROS Orchestration Backend and Queue Infrastructure

**Problem Statement**:
The One Human Corp Swarm requires a robust, autonomous, and self-healing orchestration backend. We currently lack a unified database-backed state machine for task claiming (Shared Task List), a real-time Teammate Mesh for zero-friction agent coordination, and an autoDream memory vector architecture to consolidate temporary agent scratchpads into durable semantic memory. Without these components, agents risk race conditions during task claiming and miss out on long-term context retention.

**Research Report**:
Competitor analysis and our internal audits (`ohc_hybrid_competitive_analysis.md`) indicate that autonomous agent operating systems require highly available task queues and seamless inter-agent communication. Our hybrid architecture dictates that this orchestration layer must gracefully degrade from a robust PostgreSQL/Redis cloud implementation to a resource-constrained SQLite/in-memory Standalone mode. The orchestration relies heavily on distributed state machine tracking (e.g., `PENDING` -> `IN_PROGRESS` -> `SUCCESS`) and requires strict token/VRAM quotas for sub-agent queuing.

**Design Doc**:
Refer to the detailed implementation blueprint at `docs/architecture/KAIROS_ORCHESTRATION_IMPLEMENTATION_BLUEPRINT.md` for full schema definitions, Teammate Mesh API payloads, and autoDream data pipeline specifications.

**Implementation Prompt**:
As an Implementer agent, you are tasked with implementing the KAIROS Orchestration Backend.
1. Implement the `shared_tasks_decomposition` database schema in PostgreSQL, including the `FOR UPDATE SKIP LOCKED` logic for task claiming.
2. Build the Teammate Mesh broadcast API endpoint (`POST /api/mesh/broadcast`) to distribute state machine transitions.
3. Scaffold the `autodream_memories` table using pgvector for long-term semantic memory storage.
4. Implement a background queue manager (similar to BullMQ/Celery) that integrates with the task list, enforcing resource quotas per sub-agent.
Unit test coverage MUST be 100%.

**Priority**: P0 (critical)
**Estimated Scope**: Large
