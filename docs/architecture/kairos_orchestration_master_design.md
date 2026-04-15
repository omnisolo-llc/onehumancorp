<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS Orchestration: Master Design Document (v2.0)

## 1. Vision
The One Human Corp (OHC) AI OS is powered by the **KAIROS Orchestrator**, a distributed system designed to manage complex agent swarms with zero friction. KAIROS ensures that a single human can orchestrate vast AI teams by providing a unified, aesthetics-first interface for task decomposition, real-time coordination, and long-term memory consolidation.

## 2. Architectural Pillars

### I. Distributed State Machine (Shared Task List)
KAIROS utilizes a database-backed state machine to manage the lifecycle of `shared_tasks`.
- **Hybrid Locking:** PostgreSQL uses `FOR UPDATE SKIP LOCKED` for high-concurrency cloud environments. Standalone mode utilizes SQLite with Go-level Mutexes and explicit transactions to prevent TOCTOU (Time-of-Check to Time-of-Use) vulnerabilities.
- **UltraPlan Integration:** Tasks move through specialized deliberation phases (`PROPOSE`, `CRITIQUE`, `REVISE`, `APPROVED`, `EXECUTE`) before being claimed by worker agents.
- **DAG Support:** Tasks can have multiple dependencies, forming a Directed Acyclic Graph. KAIROS enforces circular dependency checks at the middleware layer.

### II. Teammate Mesh (Real-time Transport)
The Teammate Mesh provides low-latency communication across the swarm.
- **Unified API:** A single gateway (`POST /api/mesh/broadcast`) handles event routing.
- **OHC-SIP Compliance:** All messages MUST include `agent_id`, `action`, and `status` at the JSON root to ensure compatibility across different agent roles and versions.
- **Hybrid Transport:**
    - **Cloud:** Powered by Redis Pub/Sub for horizontal scalability.
    - **Standalone:** Powered by a sharded in-memory Go transport for maximum host-machine efficiency.

### III. autoDream Pipeline (Omni-Context Memory)
The autoDream system consolidates episodic agent memory into a durable vector store.
- **Continuous Sync:** Local SQLite vector embeddings are automatically synced to Cloud pgvector instances.
- **Observability:** Every consolidation event is recorded via OpenTelemetry, providing "Full-Spectrum Observability" into the swarm's intelligence growth.

## 3. Component Mapping

| Component | Responsibility | Hybrid Strategy |
| :--- | :--- | :--- |
| **TaskManager** | CRUD for `shared_tasks`, DAG validation | Postgres (Cloud) / SQLite (Local) |
| **MeshClient** | Pub/Sub for `mesh:tasks`, `mesh:events` | Redis (Cloud) / In-Memory (Local) |
| **SubAgentWorker** | Background execution of isolated jobs | K8s Pods (Cloud) / Goroutines (Local) |
| **MemoryConsolidator** | YAML -> Embedding -> Vector DB | pgvector (Cloud) / SQLite-VSS (Local) |

## 4. Visual Excellence Mandate
All KAIROS dashboards and reports must utilize the OHC "Premium Feel" tokens:
- **Glassmorphism:** 20px blur, 200% saturation.
- **Background:** Translucent white (`rgba(255, 255, 255, 0.03)`).
- **Typography:** 'Outfit' for headings, 'Inter' for body text.

---
*Authored by: Principal Product Architect & KAIROS Orchestrator (L7)*
*Identity: One Human Corp*

</div>
