---
title: "KAIROS Phase 1: Shared Task List Database Schema"
agent: Researcher
status: PENDING
---

# Title
KAIROS Phase 1: Shared Task List Database Schema

# Problem Statement
The One Human Corp (OHC) Swarm requires a durable, distributed state machine to decompose feature requests and share tasks. Without a Shared Task List, agents cannot safely orchestrate complex workflows across Cloud (PostgreSQL) and Standalone (SQLite) modes without race conditions.

# Research Report
- Must leverage `FOR UPDATE SKIP LOCKED` in Postgres to prevent worker collisions.
- Must gracefully degrade to SQLite transactions for Standalone mode.
- Needs an `UltraPlan` abstraction for deep-deliberation cycles (DAG dependencies).

# Design Doc
- Database Schema: Implement `shared_tasks` table with fields for `id`, `organization_id`, `parent_plan_id`, `status`, and JSONB `dependencies`.
- Implement `PeekTasks` with strict row-level locks.
- UI representation must enforce the Premium Feel: `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`.

# Implementation Prompt
You are an Implementer agent.
1. Add/modify DB migration scripts in `srcs/server/db/migrations/` for `shared_tasks`.
2. Update `srcs/server/orchestration/tasks_db.go` with `PeekTasks` using appropriate locks based on `dbProvider.IsSQLite()`.
3. Add tests in `srcs/server/orchestration/` ensuring >95% coverage.

# Priority
P0

# Estimated Scope
Medium
