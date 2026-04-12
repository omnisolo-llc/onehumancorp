---
status: PENDING
priority: P1
agent: Implementer
---

# Title: KAIROS Phase 1: Shared Task List, DAG Dependencies & State Machines

## Problem Statement
The OHC Swarm requires a robust distributed system to decompose complex feature requests and share tasks safely across agents without race conditions, especially across multi-pod cloud workers.

## Research Report
Based on OHC Hybrid Architecture (OHC-HA), the shared task list must function in a Cloud-Native mode (PostgreSQL multi-tenant) with horizontal concurrency using `FOR UPDATE SKIP LOCKED`. For Standalone Desktop mode, it must gracefully degrade using SQLite table transactions.

## Design Doc
- **Schema Updates**: Design a robust `swarm_tasks` and `state_machine_transitions` schema.
- **DAG Dependencies**: Introduce a `dependencies` JSONB field or a relation table to enforce sequence blocking and unblocking.
- **Microservices Mapping**: Define the Go API `ClaimTask` handlers to support atomic acquisition.

## Implementation Prompt
Hello Implementer agent!
1. Check `srcs/server/db/migrations/` to ensure `swarm_tasks` and `state_machine_transitions` tables correctly map sub-agent orchestration (`parent_plan_id`). If not, add/update migrations and include in `embedsrcs` within `srcs/server/db/BUILD.bazel`.
2. Implement robust locking in `ClaimTask` using `FOR UPDATE SKIP LOCKED` for Postgres. Disable Postgres-specific locks using `dbWrapper.Provider().IsSQLite()` fallback logic.
3. Incorporate DAG blocking constraints in the query logic.
4. Verify with Bazel and ensure all code maintains >90% coverage.

## Priority
P0

## Estimated Scope
Large
