# KAIROS Orchestrator - Master Design Document

## 1. Executive Summary
The KAIROS Orchestrator acts as the "brain" of the OHC Hybrid Agentic OS, responsible for decomposing complex high-level directives into manageable tasks. It utilizes a distributed `shared_tasks` state machine to maintain execution order and prevent duplicate efforts, broadcasting transitions over the realtime Teammate Mesh, and storing learnings in the autoDream vector database pipeline.

## 2. Architecture Overview
KAIROS relies on three core pillars:
*   **Shared Task List (Phase 1)**: Robust database layer tracking task state (`PENDING`, `ASSIGNED`, `COMPLETED`), priorities, and dependencies.
*   **Teammate Mesh APIs (Phase 2)**: The realtime nervous system (Redis Pub/Sub) notifying the Swarm when a task transition occurs.
*   **autoDream Data Pipelines (Phase 3)**: Long-term memory consolidation running background jobs to extract, embed, and store insights.

## 3. Database Schema (PostgreSQL & SQLite Compatibility)
### 3.1 `shared_tasks` Table Improvements
*   Added `parent_plan_id` to correlate sub-tasks to a parent objective.
*   Added `dependencies` (JSONB) to enforce DAG ordering.
*   `status` enum tracks the state machine.

### 3.2 State Machine Transitions
*   A new `state_machine_transitions` log will act as the source of truth for the Teammate Mesh.
*   **SQLite Note**: Handled via `mu sync.Mutex` in code to prevent concurrency leaks during `ClaimTask`.
*   **Postgres Note**: `SELECT FOR UPDATE SKIP LOCKED` guarantees high throughput.

## 4. Teammate Mesh Integration (Redis Pub/Sub)
KAIROS agents will subscribe to `ohc:tasks:transitions`. When an agent claims a task, the `TaskOrchestrator` publishes an event.

## 5. Security & Isolation
*   All queries enforce `organization_id` derived from the SPIFFE/SPIRE JWT context (`claims.OrganizationID`).
*   No implicit tenant-crossing allowed.

## 6. Observability (Full-Spectrum)
*   Every `ClaimTask` and `UpdateTask` method is wrapped in an OpenTelemetry span.
*   Prometheus counters monitor transition throughput and error rates.

## 7. Conclusion
This architecture guarantees that the OHC Swarm can scale horizontally in Cloud Mode while remaining resource-efficient in Standalone Mode, maintaining absolute autonomy across the network.
