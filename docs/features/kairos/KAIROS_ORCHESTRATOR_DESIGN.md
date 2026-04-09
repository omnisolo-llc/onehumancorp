<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# KAIROS Orchestrator: Master Design Document

The **KAIROS Orchestrator** is the central nervous system of the One Human Corp (OHC) Agentic Operating System. It enables complex, multi-agent workflows through deep deliberation, task decomposition, and memory consolidation, adhering to the OHC Hybrid Architecture (OHC-HA).

## 1. Executive Summary

To achieve absolute autonomy, agents must coordinate efficiently without stepping on each other's toes or running out of memory context. The KAIROS Orchestrator solves this via three distinct subsystems:
1.  **Shared Task List:** A durable, distributed task queue.
2.  **Teammate Mesh:** A realtime communication and locking protocol.
3.  **AutoDream Pipelines:** A long-term vector memory consolidation engine.

## 2. Shared Task List (Phase 1)

The Shared Task List acts as the global backlog. When a complex feature request is submitted, a `Planner` agent decomposes it into a DAG (Directed Acyclic Graph) of sub-tasks.

### Database Design
The `shared_tasks` table is designed to support both PostgreSQL (Cloud) and SQLite (Standalone):
-   **Locking:** In PostgreSQL, worker agents claim tasks using `SELECT ... FOR UPDATE SKIP LOCKED` to prevent concurrent assignment collisions. In SQLite, the application uses in-memory mutexes.
-   **Dependencies:** Task dependencies are stored in a compressed `JSONB` array to minimize storage footprint.

## 3. Teammate Mesh APIs (Phase 2)

The Teammate Mesh is the realtime heartbeat of the Swarm.

### Architecture
-   **Cloud-Native Mode:** Built on `Centrifuge` and backed by `Redis`. It utilizes Redis Pub/Sub for routing messages between agent pods and `rueidis` for distributed locking (`SET NX EX`).
-   **Standalone Mode:** Degrades to an in-memory `Centrifuge` hub and SQLite table-level locks, ensuring full functionality on a local machine without heavy dependencies.

## 4. AutoDream Data Pipelines (Phase 3)

The AutoDream pipeline guarantees the Swarm Intelligence Protocol (OHC-SIP) mandate for shared, persistent memory.

### Workflow
1.  **Sweep:** The `AutoDreamWorker` daemon periodically scans `agent_session_data` for inactive or completed sessions.
2.  **Consolidate:** Session data is chunked and summarized using the available LLM (OpenAI/Anthropic/Gemini).
3.  **Embed:** Summaries are converted into 1536-dimensional vectors.
4.  **Persist:** Vectors are stored in the `autodream_memories` table (using `pgvector` in Cloud-Native mode or JSON blobs in SQLite).

## 5. Visual Excellence

All UI representations of KAIROS components (e.g., Task Boards, Mesh Status, Memory Graphs) MUST adhere to the OHC Premium Feel:
-   `backdrop-filter: blur(20px) saturate(200%)`
-   `background: rgba(255, 255, 255, 0.03)`
-   Typography: 'Outfit' or 'Inter'

</div>
