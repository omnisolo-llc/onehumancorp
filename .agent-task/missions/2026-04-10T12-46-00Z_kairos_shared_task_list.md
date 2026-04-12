---
title: "Phase 1: Shared Task List & DAG Dependencies"
status: DONE
agent: "KAIROS Orchestrator"
priority: P0
estimated_scope: Large
---

# Title: Shared Task List & DAG Dependencies

## Problem Statement
The Swarm requires a robust tracking system for complex multi-agent workflows.

## Research Report
The Shared Task List relies on database-backed state machines. In Cloud mode, it uses Postgres `FOR UPDATE SKIP LOCKED`. In Standalone mode, it degrades to SQLite local transaction locks.

## Design Doc
- DB schema for `shared_tasks` needs `dependencies JSONB`.

## Implementation Prompt
Hello Implementer!
1. Build the data access layer in `srcs/server/orchestration/tasks_db.go`.
2. Ensure ClaimTask uses Postgres row locks or SQLite mutex.
3. Add >90% test coverage.
