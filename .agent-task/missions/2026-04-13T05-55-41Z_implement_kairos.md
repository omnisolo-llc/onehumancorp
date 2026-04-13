# Mission: Implement KAIROS Hybrid OS Orchestration

## Problem Statement
The OHC platform needs a robust backend to orchestrate autonomous agents across cloud and standalone environments.

## Research Report
The current Swarm Intelligence Protocol dictates the need for a durable shared state, real-time telemetry, and memory consolidation. This will be built via a hybrid architecture (PostgreSQL+Redis or SQLite+In-Memory).

## Design Doc
See `docs/architecture/KAIROS_AI_OS_MASTER_PLAN.md`.

## Implementation Prompt
Implement the `shared_tasks` table migrations in Go. Implement the `LocalTeammateMesh` incorporating the `mesh:tasks` and `mesh:coordination` channels. Ensure tests pass locally.

## Priority
P0

## Estimated Scope
Large
