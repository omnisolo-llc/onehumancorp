# Hybrid Agentic OS Features Design Document

## 1. Introduction
One Human Corp (OHC) is building a Hybrid Agentic OS to empower a single human to orchestrate a vast swarm of AI agents. This design document outlines the architectural blueprints for four core pillars of the Agentic OS: the Shared Task List, Realtime Teammate Mesh, Sub-Agent Orchestration, and the autoDream Data Pipeline.

## 2. Core Pillars

### 2.1 Shared Task List & Distributed State Machine
**Goal:** Provide a resilient, distributed tracking system for complex multi-agent workflows.

**Design:**
- **State Machine:** A strict state machine enforces deterministic transitions (`PENDING` -> `IN_PROGRESS` -> `REVIEW` -> `COMPLETED`).
- **Distributed Locking:** Utilizes `redis` distributed locks (Cloud mode) or DB row-level locks (Standalone mode) to prevent race conditions during state transitions.
- **Dependency Resolution:** Agents evaluate a DAG (Directed Acyclic Graph) of dependencies. A task is only `PENDING` if all its dependencies are `COMPLETED`.

### 2.2 Realtime Teammate Mesh APIs
**Goal:** Enable low-latency, resilient communication across the agent swarm.

**Design:**
- **Protocols:** gRPC for backend agent-to-agent communication, WebSockets for frontend clients.
- **Transport:** A `MeshTransport` interface abstracts the underlying message bus.
  - `RedisMeshTransport`: Uses production Redis Pub/Sub channels for multi-tenant, distributed deployments.
  - `MemoryMeshTransport`: Uses in-memory channels for standalone deployments.
- **APIs:** Expand `src/proto/hub.proto` to support capabilities advertising, agent discovery, and real-time event streaming.

### 2.3 Sub-Agent/Worker Orchestration Queue
**Goal:** Provide scalable background queuing for spawning and managing isolated sub-agents.

**Design:**
- **Queue Interface:** A `TaskQueue` interface handles Enqueue, Dequeue, Complete, and Fail operations.
- **Implementations:**
  - `RedisTaskQueue`: Built on `redis`, utilizing Redis Lists or Sorted Sets for delayed execution and robust retry mechanisms.
  - `SQLiteTaskQueue`: Built on `database/sql`, mapping to a local `sub_agent_jobs` table, utilizing concurrent write locking for dequeuing.
- **Features:** Granular timeouts, retry policies, and OpenTelemetry instrumentation for queue length and processing time.

### 2.4 autoDream Data Pipelines
**Goal:** Continuously consolidate episodic memory to prevent context window overflow and enable long-term reasoning.

**Design:**
- **Orchestration:** A background worker (`AutoDreamPipeline`) extracts raw memory, chunks it, and feeds it into the existing `AutoDream` logic.
- **Embeddings:** Consolidated memory is embedded using existing LLM clients (`src/server/agents/local/llm.go`).
- **Storage:** Embeddings are stored in a `pgvector`-enabled PostgreSQL table (`consolidated_memory`) for efficient similarity search.

## 3. Hybrid Consistency
All features are designed to gracefully degrade. When deployed in **Standalone Desktop Mode**, external dependencies like Redis are replaced with local SQLite alternatives, ensuring the OS remains fully functional without cloud connectivity.

## 4. Aesthetic Excellence
All frontend representations of these features (e.g., Task List UI, Mesh Visualization) must adhere to the OHC Premium Feel: Glassmorphism (`backdrop-filter: blur(20px)`), translucent backgrounds, and the Outfit/Inter typography stack.

## 5. Security & Identity
All agent operations and mesh communications rely entirely on SPIFFE/SPIRE for identity authentication, adhering to the "Zero Secrets" constraint.