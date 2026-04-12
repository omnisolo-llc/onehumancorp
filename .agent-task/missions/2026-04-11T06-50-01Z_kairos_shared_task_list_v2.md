---
status: "PENDING"
Title: "KAIROS Phase 1: Shared Task List Backend Database Design"
Priority: "P0"
Estimated Scope: "Medium"
---
# Problem Statement
The Swarm lacks a central orchestration layer with a shared task list. Agents need a durable database schema in Cloud-Native (PostgreSQL) and Standalone Desktop (SQLite) modes.

# Research Report
OHC operates in Hybrid Architecture. The `shared_tasks` table is critical for representing a global queue. We need robust locking: PostgreSQL row-level locks (`FOR UPDATE SKIP LOCKED`) in cloud mode, and application-level mutexes in SQLite standalone mode.

# Design Doc
**Schema:** Create `shared_tasks` table with ID, organization ID, status, assigned agent, and JSONB dependencies.

# Implementation Prompt
You are an Implementer agent. Implement the backend database designs for the "Shared Task List" feature.
1. Create SQL migration for `shared_tasks` in `srcs/server/db/migrations/`.
2. Create data access layer in `srcs/server/orchestration/` with a task claiming method.
3. Use `FOR UPDATE SKIP LOCKED` for Postgres, and application mutexes for SQLite.
4. Achieve >90% test coverage.

# Visual Excellence Guidelines
Apply `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;` for UI.
