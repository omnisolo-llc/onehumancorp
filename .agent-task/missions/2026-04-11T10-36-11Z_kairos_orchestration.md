---
title: "Implement Shared Task List & Omni-Context Sub-agent Routing"
status: PENDING
priority: P0
estimated_scope: Large
---

# Title: Implement Shared Task List & Omni-Context Sub-agent Routing

## Problem Statement
Complex agentic workflows cannot be handled sequentially by a single monolithic model call. High-level requests must be decomposed into isolated, concurrent sub-tasks. Currently, sub-agents spawning lack zero-latency context retrieval, leading to hallucination, alignment drift, and token bloat from explicit file-reading tool calls.

## Research Report
The market audit against OpenClaw, Claude Code, and OpenCode identifies OHC's "Unfair Advantage" as Omni-Context Sub-agent Routing. Instead of ad-hoc spawning and file-based grounding files (`CLAUDE.md`, `AGENTS.md`) requiring explicit read calls, OHC must build Swarm-as-Code. A shared task list combined with the `autodream_memories` vector embeddings pipeline ensures zero-latency grounding and perfect architectural alignment across worker nodes.

## Design Doc
See `docs/features/kairos/shared_task_list.md` for full architectural designs, encompassing:
1. `kairos_tasks` database schema (UUID, parent tasks, status tracking).
2. Distributed State Machine tracking and Teammate Mesh Pub/Sub integration.
3. Omni-Context AutoDream pgvector integration, ensuring database-injected contextual `[SYSTEM GROUNDING]` payloads during sub-agent delegation.

## Implementation Prompt
You are the Implementer Agent.
1. Implement the `kairos_tasks` schema in `srcs/server/db/migrations/` (check the existing latest prefix and add the next sequence).
2. Implement the `KairosTaskProvider` interface in `srcs/server/db/kairos.go` with functions to `CreateTask`, `UpdateTaskStatus`, and `GetTasksForAgent`.
3. In `srcs/server/orchestration/sip.go`, modify the `DelegateMission` method. Have it retrieve relevant context from the `autodream_memories` table (via `dbWrapper`) and append it under the `[SYSTEM GROUNDING]` namespace within the sub-agent's starting mission payload.
4. Write comprehensive tests in `srcs/server/orchestration/sip_test.go` and ensure over 90% coverage.
5. Run `bazelisk test //...` to ensure stability.
6. Ensure no `CLAUDE.md` fallback behaviors are required for spawned agents.

## Priority
`P0` (Critical - Required for Swarm Intelligence Protocol scaling)

## Estimated Scope
Large
