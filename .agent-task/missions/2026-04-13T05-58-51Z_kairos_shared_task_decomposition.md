---
status: STUCK
priority: P0
scope: Large
title: "KAIROS: Architect Shared Task List Decomposition"
---

# Title: KAIROS: Architect Shared Task List Decomposition

## Problem Statement
The KAIROS Orchestrator needs to seamlessly decompose high-level goals into parallel agentic workflows. Without a structured "Shared Task List" utilizing a distributed state machine, agents cannot safely orchestrate complex DAG (Directed Acyclic Graph) dependencies across the OHC Hybrid Architecture (Cloud-Native PG/Redis vs. Standalone SQLite).

## Research Report
- Current multi-step workflows suffer from isolation and potential deadlocks without a centralized tracking mechanism.
- We must design a durable database schema (e.g., `shared_tasks`) capable of handling task dependencies and status transitions.
- The state machine must leverage `SELECT FOR UPDATE SKIP LOCKED` on PostgreSQL to avoid pod-level worker collisions in the cloud, while cleanly falling back to SQLite and application-level mutexes in Standalone Desktop mode.

## Design Doc
1. **Database Schema**:
   - `shared_tasks`: `id`, `organization_id`, `title`, `status` (PENDING, IN_PROGRESS, DONE, BLOCKED), `dependencies` (JSONB/Text), `created_at`.
2. **State Machine Tracking**:
   - `state_machine_transitions`: Log state changes to facilitate resumption and distributed tracing.
3. **Task Decomposer Engine**:
   - A sub-agent queueing system inside `srcs/server/orchestration/tasks_db.go` that monitors the DAG and exposes available tasks to the Teammate Mesh.

## Implementation Prompt
- Implement the `Shared Task List` database migrations (`shared_tasks` and `state_machine_transitions`).
- Build the task assignment and claiming logic in `srcs/server/orchestration/tasks_db.go` implementing the Cloud/Standalone degradation strategy (PG locking vs SQLite application mutexes).
- Provide unit tests ensuring >90% coverage for the assignment engine.
