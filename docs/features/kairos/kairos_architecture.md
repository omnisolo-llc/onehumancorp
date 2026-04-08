<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# KAIROS Orchestration Engine: Core Architecture

The **KAIROS Orchestration Engine** is the central nervous system of the One Human Corp (OHC) "Hybrid Agentic OS." It transcends basic LLM wrapper scripts by providing a durable, distributed, and highly observable platform for managing the OHC Swarm.

This document consolidates the structural and aesthetic vision for KAIROS, uniting its foundational pillars into a cohesive framework.

## 1. The Vision

KAIROS acts as the L7 "Principal Product Architect" routing requests. Its mandate is twofold:

1.  **Decompose Complexity:** Transform high-level, ambiguous user intent into a precise, actionable graph of sub-tasks via UltraPlan Deliberation.
2.  **Ensure Reliability:** Guarantee that tasks are executed flawlessly across a distributed swarm of specialized agents, resilient against node failures, network partitions, and process crashes.

### Aesthetic Mandate (The "Premium Feel")

KAIROS interfaces and documentation strictly adhere to the OHC Premium Visual Standard:
- Typography: strictly `Outfit` and `Inter`.
- UI Elements: Translucent glassmorphism (`backdrop-filter: blur(20px) saturate(200%)`).
- Backgrounds: Subtle transparency (`background: rgba(255, 255, 255, 0.03)`).

## 2. Core Pillars of KAIROS

KAIROS manages the swarm via four distinct, tightly integrated subsystems. Detailed documentation for each is linked below.

### I. The Shared Task List (Goal Decomposition)
The central objective tracker. KAIROS translates requests into a recursive hierarchy of tasks stored persistently. This backlog is visible to all swarm members, allowing specialized agents (Researchers, Implementers) to claim specific nodes in the execution graph.
*   **Documentation:** [Shared Task List Design Doc](./shared_task_list.md)
*   **Mechanism:** PostgreSQL `uuid` relationships (Cloud) / SQLite (Standalone).

### II. Distributed State Machine Tracker (Deterministic Execution)
Ensures that the state of tasks within the Shared Task List progresses deterministically. It enforces valid transitions (e.g., `PENDING -> IN_PROGRESS -> COMPLETED`) across multiple concurrent pods.
*   **Documentation:** [State Machine Tracker](./state_machine.md)
*   **Mechanism:** Distributed locking via Redis `SET NX EX` (Cloud) / SQLite Mutex (Standalone).

### III. Teammate Mesh APIs (Real-time Coordination)
The ultra-low latency signaling layer. While the Sub-Agent Queue manages the *what*, the Teammate Mesh manages the *now*. It enables agents to broadcast state transitions, share rapid context, and trigger immediate reactions in sibling processes without hitting the database.
*   **Documentation:** [Teammate Mesh APIs](./teammate_mesh_apis.md)
*   **Mechanism:** Redis Pub/Sub (Cloud) / In-Memory Channel Multiplexer (Standalone).

### IV. AutoDream Pipeline (Memory Consolidation)
The swarm's long-term intelligence. It asynchronously sweeps ephemeral agent session data, chunks it, and generates vector embeddings. This prevents context starvation on long-running projects.
*   **Documentation:** [AutoDream Data Pipeline](./autodream_pipeline.md)
*   **Mechanism:** `pgvector` nearest neighbor search (Cloud) / SQLite JSON Blob fallback (Standalone).

### V. Sub-Agent Orchestration Queue (Task Distribution)
Handles the horizontal scaling of agent execution. When tasks are created in the Shared Task List, they are enqueued here for worker pods to consume, complete with retry logic and backoffs.
*   **Documentation:** [Sub-Agent Queue](./sub_agent_queue.md)
*   **Mechanism:** Redis ZSETs (Cloud) / SQLite explicit transactions (Standalone).

## 3. The Master Loop: Think → Act → Observe → Decide

The entire KAIROS architecture operates continuously on this master loop:

1.  **Think:** KAIROS receives intent, initiates an "UltraPlan" deep-deliberation cycle using LLMs, and populates the **Shared Task List**.
2.  **Act:** Tasks are pushed to the **Sub-Agent Orchestration Queue**. Workers dequeue tasks and update the **Distributed State Machine**.
3.  **Observe:** Workers utilize the **Teammate Mesh** to broadcast real-time telemetry and state changes back to KAIROS.
4.  **Decide:** Background workers execute the **AutoDream Pipeline** to consolidate the resulting artifacts into persistent vector memory, informing future "Think" cycles.

</div>
