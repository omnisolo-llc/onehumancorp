---
Title: "KAIROS Orchestration: Unified Hybrid State & Teammate Mesh"
Priority: P0
Estimated Scope: Large
status: PENDING
---

# Problem Statement
The One Human Corp (OHC) Swarm requires a robust orchestration engine (KAIROS) capable of decomposing complex features into isolated sub-agent tasks. Without a unified Distributed State Machine and a resilient Teammate Mesh, complex multi-agent workflows degrade into race conditions, resulting in duplicate task assignments and fragmented episodic memory.

# Research Report
- **Architecture Context**: OHC operates in a Hybrid Architecture (`OHC-HA`). Shared state must live in PostgreSQL for horizontal scaling, but gracefully degrade to SQLite for standalone standalone nodes.
- **Teammate Mesh**: Inter-agent communication is hindered by a lack of realtime WebSockets/Redis PubSub. Agents must coordinate via a durable `CentrifugeNode` and `rueidis`.
- **Durable Memory**: Ephemeral workflow outputs are lost upon session termination. AutoDream pipelines must consolidate this into `pgvector` indexes for long-term semantic reuse.
- **Git-Lock Coordination**: Multi-agent file edits require checking production distributed Redis locks before modifying files. Wait if locked.

# Design Doc
**1. Shared Task List (PostgreSQL / SQLite)**
- **Schema**: A central `shared_tasks` table containing `status`, `assigned_agent_id`, and `dependencies`.
- **Locking**: Implements `FOR UPDATE SKIP LOCKED` for Postgres to prevent concurrent reads, ensuring deterministic sub-agent assignments. SQLite falls back to `sync.Mutex` or immediate transaction locking.

**2. Distributed State Machine & Sub-Agent Queuing**
- **State Flow**: `PENDING -> CLAIMED -> PROCESSING -> AUTO_DREAM -> DONE`.
- **Queueing**: A scalable background queue (e.g., BullMQ paradigm adapted for Go via Rueidis) to spawn and manage isolated sub-agent pods.

**3. Teammate Mesh Architecture**
- **Transport**: `RedisMeshTransport` leveraging `github.com/redis/rueidis` to broadcast `TaskAssigned` and `TaskCompleted` events.
- **Mailbox**: Agents check the Pub/Sub mailbox at startup and post coordination sessions.

**4. AutoDream Consolidation**
- **Embeddings**: Once a task reaches `AUTO_DREAM` state, the agent's scratchpad is compressed via Minimax LLMs and embedded via `pgvector` in the `autodream_memories` table.

# Implementation Prompt
You are an Implementer agent for the OHC KAIROS feature. Strictly follow these directives to execute the architecture:
1.  **Shared Task List Schema:** Create migration `015_kairos_orchestrator.sql` in `srcs/server/db/migrations/` establishing `shared_tasks` with `FOR UPDATE SKIP LOCKED` support.
2.  **Teammate Mesh Integration:** In `srcs/server/orchestration/mesh.go`, implement a Redis Pub/Sub broadcast method using `rueidis` that agent nodes listen to.
3.  **Distributed Lock:** Implement a Redis-backed mutex in `srcs/server/orchestration/mutex.go` (`SET resource_name my_random_value NX PX 30000`).
4.  **AutoDream Pipeline:** In `srcs/server/orchestration/autodream_pipeline.go`, ensure successful workflow summaries are batched and pushed to the `pgvector` database index.
5.  **Aesthetic Enforcement:** Any dashboard UI exposing KAIROS state MUST apply the exact OHC premium CSS:
    ```css
    body {
      backdrop-filter: blur(20px) saturate(200%);
      background: rgba(255, 255, 255, 0.03);
      font-family: 'Outfit', 'Inter', sans-serif;
    }
    ```
6.  Do not ask for approval. Rely on SPIFFE/SPIRE for identity validation.

# Visual Excellence Guidelines
- Apply the OHC "Premium Feel" to all visual representations of the Swarm queue. The blurring and saturation effects must remain consistent whether running in cloud or standalone mode.