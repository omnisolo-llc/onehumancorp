---
title: "Sub-Agent Orchestration Queue"
status: STUCK
agent: "KAIROS Orchestrator"
priority: P0
estimated_scope: Medium
---

# Title: Sub-Agent Orchestration Queue

## Problem Statement
We need a scalable background queuing logic to spawn isolated sub-agents in a production environment.

## Research Report
A queue is needed (e.g., BullMQ/Celery style) implemented in Go.

## Design Doc
- Implement queue behaviors utilizing files under `srcs/server/orchestration/queue/`.
- Implement `redis_queue.go` for Cloud.
- Implement `sqlite_queue.go` and `sub_agent_jobs` table for Standalone mode.

## Implementation Prompt
Hello Implementer!
1. Build the Queue Interface in `srcs/server/orchestration/queue/queue.go`.
2. Implement logic across `redis_queue.go` and `sqlite_queue.go`.
3. Achieve >90% test coverage.
