---
status: IN_PROGRESS
agent: jules
---

# Title: Implement KAIROS Master Design (Phase 4)

# Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. KAIROS currently lacks a central Master Design that unifies the Shared Task List, Teammate Mesh APIs, and AutoDream pipelines into a single coherent Swarm Intelligence OS. Without this, complex architectural missions cannot be reliably decomposed and executed by the swarm.

# Research Report
Based on a comprehensive review of the `docs/features/kairos/` documentation, `.agent-task/missions/`, and existing KAIROS Orchestration specifications, the current system has fragmented documentation for:
- The **Shared Task List**: Durable distributed state machine using PostgreSQL `FOR UPDATE SKIP LOCKED` (Cloud) or SQLite (Standalone).
- The **Teammate Mesh**: Real-time coordination via Redis Pub/Sub (`rueidis`) and `CentrifugeNode`.
- The **AutoDream Data Pipelines**: Ephemeral session consolidation to long-term semantic embedded memory (`pgvector`).

These elements need to be unified under a "KAIROS Orchestrator" paradigm that manages Sub-Agent Queuing, DAG dependencies, and strict aesthetic guidelines (Visual Excellence Mandate).

# Design Doc
See `docs/features/kairos/master_design.md` for the complete, premium architectural overview of the unified KAIROS Orchestration layer.

# Implementation Prompt
You are an Implementer agent. Your task is to execute the KAIROS Master Design (Phase 4).
1. Read the comprehensive architecture defined in `docs/features/kairos/master_design.md`.
2. Ensure that any backend task logic in `srcs/server/orchestration/` strictly implements the unified KAIROS Triad: Task Queues, Mesh Pub/Sub broadcast, and AutoDream memory writes.
3. Validate that the frontend integration strictly follows the Visual Excellence Mandate: `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;`.
4. Ensure robust tests are added for all new orchestration endpoints and state transitions. Run `bazelisk test //...` to verify.
5. Create a descriptive PR with a title such as "🚀 Nova: KAIROS Master Design Implementation".

# Priority
P0

# Estimated Scope
Large
