---
title: "KAIROS Master Orchestration Interfaces"
status: DONE
agent: jules
priority: P0
estimated_scope: Large
agent: "Principal Product Architect & KAIROS Orchestrator (L7)"
---

# Title: KAIROS Master Orchestration Interfaces

## Problem Statement
The KAIROS orchestrator is required to decompose high-level feature requests into a shared task list for the agent team. The OHC swarm requires a distributed Shared Task List, a realtime Teammate Mesh, an AutoDream vector memory pipeline, and a Sub-Agent Queue for true autonomous orchestration in both cloud-native and standalone deployments. We need to formalize these interfaces for sub-agents.

## Research Report
The existing designs and plans need to be consolidated and built into concrete backend API interfaces and database schemas. The swarm relies on database locks (`FOR UPDATE SKIP LOCKED`) in PostgreSQL and `sync.Mutex`/explicit transactions in SQLite for safe parallel task execution. The messaging layer must enforce OHC-SIP payload compliance.

## Design Doc
See `docs/architecture/KAIROS_AI_OS_HYBRID_ORCHESTRATION_MASTER.md` for the unified design.

## Implementation Prompt
Hello Implementer agent!

Your mission is to implement the KAIROS Orchestrator feature set.

1. **Shared Task List**: Build upon `srcs/server/orchestration/shared_tasks.go` and the `SharedTaskDecomposition` struct. Ensure task claiming uses Postgres `SKIP LOCKED` / SQLite `sync.Mutex` appropriately based on the database provider.
2. **Teammate Mesh API**: Implement broadcast capabilities via `HandleBroadcast` in `srcs/server/orchestration/mesh/server.go`. Utilize Redis Pub/Sub via `rueidis` when running in cloud mode.
3. **AutoDream Pipeline**: Expand upon the `AutoDreamPipeline` background worker in `srcs/server/orchestration/autodream_pipeline.go` to generate embeddings via `client.GenerateEmbedding` and consolidate episodic memories.
4. **Sub-Agent Queue**: Utilize the `TaskQueue` interface in `srcs/server/orchestration/queue/queue.go` with functions `Enqueue` and `Dequeue` for orchestrating sub-agent execution tasks.
