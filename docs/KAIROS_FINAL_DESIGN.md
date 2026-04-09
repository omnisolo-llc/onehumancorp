<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# KAIROS Orchestration: Final Premium Design Document
**Author:** Principal Product Architect & KAIROS Orchestrator (L7)
**Status:** Approved
**Version:** 3.0.0

## Executive Summary
This design document synthesizes the architectural vision for the OHC "Hybrid Agentic OS". KAIROS serves as the central orchestrator, decomposing complex feature requests into a distributed state machine (the Shared Task List), coordinating sub-agents via the Teammate Mesh, and consolidating long-term memory via the AutoDream vector data pipelines.

## 1. Task Decomposition & The Shared Task List (Phase 1)
To allow agents to collaborate without stepping on each other, high-level features are decomposed via the **UltraPlan Engine**. This LLM-backed engine formulates a Directed Acyclic Graph (DAG) of sub-tasks.
These tasks are tracked in the `shared_tasks` table.
- **Cloud Mode:** Relies on PostgreSQL `FOR UPDATE SKIP LOCKED` for lock-free concurrency.
- **Standalone Mode:** Uses SQLite transactions and application-level `sync.Mutex` locks.

## 2. Realtime Teammate Mesh APIs (Phase 2)
Agents broadcast state changes and coordination intents via the Teammate Mesh.
- **Cloud Mode:** `RedisMeshTransport` routes events through `rueidis` Redis Pub/Sub.
- **Standalone Mode:** `MemoryMeshTransport` handles localized IPC via Go channels.
These events stream downstream to the Human CEO dashboard via Centrifuge WebSockets.

## 3. Sub-Agent Queue Orchestration
To spawn isolated execution contexts, tasks designated for `IMPLEMENTER` agents are pushed to a background queue.
- A `SubAgentSpawner` worker loops through `DELEGATED` tasks, spawns isolated contexts, and handles retries and exponential backoffs.

## 4. Distributed State Machine Tracking
A rigid distributed state machine governs task lifecycles (e.g., `PENDING` -> `EXECUTING` -> `REVIEW` -> `COMPLETED`). State transitions are logged in `state_machine_transitions` and gated by distributed Redis locks (or SQLite transactions), guaranteeing consistency even if worker pods crash mid-execution.

## 5. AutoDream Vector Data Pipelines (Phase 3)
Agents write their ephemeral context and thoughts to local `.agent-task/memory/*.yml` files. The **AutoDream Pipeline** asynchronously digests these artifacts, generates LLM embeddings, and upserts them into `autodream_memories`.
- **Cloud Mode:** `pgvector` enables exact Nearest Neighbor searches (`ORDER BY embedding <-> $1`).
- **Standalone Mode:** Degrades to full-text search strategies.

## Visual Excellence Mandate
This architecture enforces the OHC aesthetic standards for all orchestrator interfaces:
- `backdrop-filter: blur(20px) saturate(200%)`
- `background: rgba(255, 255, 255, 0.03)`
- `font-family: 'Outfit', 'Inter', sans-serif`

</div>
