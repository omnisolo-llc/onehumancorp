---
status: PENDING
priority: P0
scope: Large
title: "KAIROS: Architect Sub-Agent Orchestration & Queuing"
---

# Title: Architect Sub-Agent Orchestration & Queuing

## Problem Statement
To scale execution in the OHC platform, KAIROS Orchestrator needs to implement scalable background queuing logic to spawn isolated sub-agents in a production environment. This is critical to fulfilling the "Sub-Agent Orchestration" requirement in the Autonomous Task Definition.

## Research Report
- Standalone mode requires an SQLite-backed or in-memory queuing solution that minimizes memory overhead and prevents `SQLITE_BUSY` errors.
- Cloud-Native mode requires a robust distributed queue system (like BullMQ or Celery) backed by Redis.
- A unified Queue interface needs to be defined for the sub-agent spawning mechanism to seamlessly transition between both environments.

## Design Doc
1.  **Interface Definition:**
    - `SubAgentQueue` interface with methods: `Enqueue(job Job)`, `Dequeue() Job`, `Ack(jobID string)`, `Nack(jobID string)`.
2.  **Database/Redis Structure:**
    - SQLite: A `sub_agent_jobs` table representing the queue, using SQLite transaction locks.
    - Redis: Using Redis Lists or standard Pub/Sub robust messaging (e.g. BullMQ pattern).
3.  **Job Spawner:**
    - Worker pool that continuously checks the queue via the abstract interface and forks isolated sub-agent processes.

## Implementation Prompt
- Create the `SubAgentQueue` interface in `srcs/server/orchestration/queue.go`.
- Implement `SQLiteQueue` tailored for local mode with explicit locks.
- Implement `RedisQueue` tailored for cloud mode using Redis primitives.
- Create tests ensuring >90% coverage for the queuing behavior in both modes.
