---
status: "PENDING"
agent: "jules"
Title: "KAIROS Orchestration Layer Design"
Problem Statement: "The OHC Swarm lacks a central KAIROS orchestration layer to decompose complex feature requests into a shared task list for the agent team."
Priority: "P0"
Estimated Scope: "Medium"
---

# Research Report
The One Human Corp (OHC) platform needs to evolve its multi-agent capabilities. As agents tackle increasingly complex requests, they require a coordinated approach to task management. Currently, agents operate in silos, leading to duplicated effort or missed dependencies. We need a "Shared Task List" acting as a distributed state machine to manage these decomposed feature requests. The system must support both Cloud-Native (PostgreSQL + Redis Pub/Sub) and Standalone Desktop (SQLite + in-memory buses) modes.

# Design Doc

## Architecture
The KAIROS Orchestration layer comprises three main components:
1.  **Shared Task List:** A durable database schema (PostgreSQL `FOR UPDATE SKIP LOCKED` / SQLite transactions) tracking decomposed sub-tasks, their status, assigned agents, and dependencies.
2.  **Teammate Mesh:** A real-time communication layer (Redis Pub/Sub / In-Memory) allowing agents to broadcast their availability, claim tasks, and signal completion.
3.  **AutoDream Pipelines:** The long-term persistence layer storing memories of task execution in `pgvector` for semantic retrieval.
4. **Sub-Agent Orchestration:** Background queuing logic (like BullMQ/Celery) to spawn isolated sub-agents in a production environment safely bounded by the distributed state machine.

## Aesthetic Mandate
Any UI built to monitor or manage the KAIROS Orchestration layer MUST adhere to the OHC Premium Feel:
-   `backdrop-filter: blur(20px) saturate(200%)`
-   `background: rgba(255, 255, 255, 0.03)`
-   `font-family: 'Outfit', 'Inter', sans-serif`

# Implementation Prompt
As an Implementer agent, your task is to build the backend infrastructure for the KAIROS Orchestration layer.
1.  **Database Schema:** Implement migrations for the `shared_tasks` table, including columns for `id`, `parent_task_id`, `status` (PENDING, IN_PROGRESS, DONE, BLOCKED), `assigned_agent_id`, `payload`, `dependencies`, and timestamps. Ensure standard SQL compatibility for both PostgreSQL and SQLite.
2.  **State Machine:** Implement the task claiming logic using `FOR UPDATE SKIP LOCKED` in PostgreSQL and appropriate transaction locking in SQLite to prevent race conditions.
3.  **Teammate Mesh Integration:** Connect the task claiming/completion lifecycle to the existing Teammate Mesh APIs (Redis/Centrifugo or in-memory equivalents) to broadcast state changes.
4.  **Observability:** Expose OpenTelemetry metrics for task claiming latency, task completion rates, and queue depth.
