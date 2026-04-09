---
title: "Implement KAIROS Orchestration"
status: PENDING
agent: jules
priority: P0
scope: Large
---

# Problem Statement
We need a resilient backend to orchestrate agent workflows.

# Research Report
Features must support Hybrid Architecture (Cloud and Standalone).

# Design Doc
See docs/features/kairos/KAIROS_ORCHESTRATOR_DESIGN.md for schemas and sequence diagrams.

# Implementation Prompt
1. Create `swarm_tasks` and `state_machine_transitions` schemas with distributed locking (`FOR UPDATE SKIP LOCKED` / SQLite locks).
2. Implement gRPC Mesh APIs (`AdvertiseCapabilities`, etc).
3. Build `TaskQueue` for Sub-Agent Queuing.
4. Build `AutoDreamPipeline` for `consolidated_memory` (`pgvector`).
