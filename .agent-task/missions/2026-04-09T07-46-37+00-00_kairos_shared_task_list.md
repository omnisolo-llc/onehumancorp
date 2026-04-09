---
status: "PENDING"
Title: "Implement Shared Task List (KAIROS Orchestration Phase 1)"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. We lack a robust distributed state machine to track asynchronous tasks (`swarm_tasks` and `shared_tasks`) across the swarm with exact sequence and DAG dependencies.

# Research Report
Based on the KAIROS Orchestration Design Doc:
- We must utilize `swarm_tasks` for mission-critical steps and `shared_tasks` for inter-agent delegation.
- In Cloud Mode (PostgreSQL Native), we need to rely on `FOR UPDATE SKIP LOCKED` for lock-free concurrency and zero TOCTOU race conditions.
- In Standalone Mode (SQLite Fallback), we degrade gracefully utilizing local table locks.
- DAG Dependencies must enforce sequence and parallel task unblocking.

# Design Doc
1. **Schema Updates:** Ensure `swarm_tasks` and `shared_tasks` are created or updated correctly.
2. **Go Models:** Define DAG dependency logic and task struct updates to enforce sequence blocking.
3. **Provider:** Add PostgreSQL-specific logic utilizing `FOR UPDATE SKIP LOCKED` inside explicit transactions to avoid releasing row locks too early.
4. **Fallback:** Degrade gracefully and apply SQLite single-node concurrency mechanisms.

# Implementation Prompt
1. Search for the relevant database migrations directory to ensure `swarm_tasks` and `shared_tasks` exist. If not, create them.
2. Search for task orchestration files and implement logic ensuring you open a transaction for PostgreSQL locks.
3. Ensure SQLite mode handles task locking appropriately using `UPDATE ... RETURNING`.
4. Implement DAG blocking/unblocking logic so dependent tasks only run when parent tasks complete.
