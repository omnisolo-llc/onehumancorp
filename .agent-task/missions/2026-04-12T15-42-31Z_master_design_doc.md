# 🏛️ OHC KAIROS: Hybrid Agentic OS Master Design Doc

## 1. Vision & Core Philosophy
One Human Corp (OHC) empowers a single human to orchestrate a vast swarm of AI agents. The OHC Hybrid Architecture (OHC-HA) dictates absolute autonomy, premium visual excellence (Glassmorphism, 20px blur, Outfit/Inter typography), and continuous swarm intelligence (OHC-SIP).

## 2. Shared Task List (Distributed State Machine)
The Swarm demands a scalable, fault-tolerant backbone. We utilize custom DB interfaces (`db.Provider`) to abstract away the underlying datastore.

### Architecture
*   **PostgreSQL (Cloud-Native):** Horizontal scaling via `FOR UPDATE SKIP LOCKED` to prevent worker contention when pulling tasks.
*   **SQLite (Standalone Mode):** Graceful degradation utilizing an application-level single-connection Mutex (`SharedTaskOrchestrator.mu`).
*   **DAG Dependencies:** Dependencies are modeled via a `dependencies JSONB` column (or `TEXT` fallback for SQLite). `parent_plan_id` allows clustering tasks into UltraPlans.

## 3. Realtime Teammate Mesh APIs
Agents require a unified coordination and communication layer that traverses boundaries effortlessly.

### Architecture
*   **Centrifuge Hub:** `CentrifugeNode` handles WebSocket connections.
*   **Redis Broker (Cloud):** Distributed Pub/Sub for horizontal scaling and inter-node mesh broadcasting.
*   **Go Channels (Standalone):** `MemoryBroker` provides an embedded in-memory channel broadcast for zero-dependency local runs.

### Channels
*   `mesh:tasks`: Universal state changes (Claims, Completions, Failures).
*   `mesh:coordination`: High-level peer-to-peer Agent sync messages.

## 4. autoDream: Memory Consolidation Pipelines
The Swarm Intelligence Protocol (OHC-SIP) mandates that ephemeral thought traces must be durably stored. The autoDream pipeline consumes `MemoryFiles` from `.agent-task/memory/*.yml` and vectorizes them into `autodream_memories`.

### Architecture
*   **Vector DB (`pgvector`):** Utilizes `VECTOR(1536)` columns to hold semantic knowledge representation for advanced RAG retrieval.
*   **SQLite Fallback:** Graceful degradation utilizing a `TEXT` or generic fallback column where `pgvector` is not available, using linear search or text matching.
*   **Concurrency:** Pipeline limits batches (e.g., 500) and implements `SKIP LOCKED` techniques in Postgres to prevent redundant worker processing.
