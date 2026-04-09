---
agent: kairos
title: "KAIROS: Shared Task List Database Schema"
problem: "The Swarm needs a durable Shared Task List database schema to decompose complex feature requests in both Cloud and Standalone modes."
priority: P1
scope: Medium
---

# Title
KAIROS: Shared Task List Database Schema

# Problem Statement
The OHC platform lacks a central "KAIROS" orchestration layer with a shared task list. To decompose complex feature requests for the Swarm, agents need a durable database schema in both Cloud-Native (PostgreSQL) and Standalone Desktop (SQLite) modes to track the Shared Task List. Without this, complex architectural missions cannot be decomposed or shared securely among agents.

# Research Report
Based on OHC Hybrid Architecture:
- The Shared Task List acts as the brain, mapping high-level requests into decomposed sub-tasks.
- It must support horizontal pod concurrency in the cloud (`FOR UPDATE SKIP LOCKED` equivalent in Postgres) and fallback to SQLite transactions in Standalone mode.
- Distributed state tracking requires robust dependency management within the schema.

# Design Doc
- **Schema**:
    - `tasks`: `id`, `mission_id`, `title`, `status`, `assigned_to`, `created_at`, `updated_at`.
    - `task_dependencies`: `task_id`, `depends_on_task_id`.
- The schema will track task completion, assignment, and blockers to form a DAG of agent tasks.

# Implementation Prompt
- Implement the database migration files for the Shared Task List (`srcs/server/db/migrations/...`).
- Ensure the changes support PostgreSQL and SQLite gracefully.
- Do NOT implement the actual Go backend logic, only output the SQL schema designs as instructed in the orchestrator role.

# Priority
P1

# Estimated Scope
Medium
