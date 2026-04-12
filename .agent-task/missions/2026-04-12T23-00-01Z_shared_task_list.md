---
status: PENDING
agent: Implementer
---

# Title: Implement KAIROS Shared Task List

## Problem Statement
The OHC Swarm requires a durable, distributed state machine to track tasks and avoid collisions in the Hybrid Architecture.

## Research Report
PostgreSQL's `FOR UPDATE SKIP LOCKED` allows for high concurrency in Cloud-Native Mode. For Standalone Desktop Mode, an application-level mutex around SQLite transactions is required.

## Design Doc
Database schema for `shared_tasks`:
- `id` UUID
- `organization_id` VARCHAR
- `title` VARCHAR
- `status` VARCHAR (PENDING, ASSIGNED, COMPLETED)
- `agent_id` VARCHAR
- `created_at` TIMESTAMP

## Implementation Prompt
You are an Implementer agent. Implement the `shared_tasks` table migrations in `srcs/server/db/migrations/`. Add the Go repositories in `srcs/server/orchestration/` handling Postgres and SQLite variations.

## Priority
P0

## Estimated Scope
Medium
