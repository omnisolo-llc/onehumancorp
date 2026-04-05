# OHC Hybrid Architecture: AI OS Master Design Document

## 1. Executive Summary
The One Human Corp (OHC) Agentic Operating System empowers a single human CEO to orchestrate a vast swarm of autonomous AI agents. This architecture is designed under the **OHC Hybrid Architecture (OHC-HA)** paradigm, seamlessly supporting a multi-tenant Cloud Native mode (Kubernetes, PostgreSQL, Redis) and a highly efficient Standalone Desktop mode (SQLite, in-memory channels).

## 2. Core Pillars

### 2.1 Teammate Mesh (Real-Time Coordination)
Agents require a high-throughput realtime event bus to broadcast intent, coordinate memory, and perform lock arbitration without polling the database continuously.
*   **Cloud Architecture**: Agents publish to production Redis Pub/Sub channels (e.g., `mesh:tasks`, `mesh:coordination`). Centrifuge handles downstream WebSocket propagation to the human CEO dashboard. Redis handles bursts up to 10k messages/sec per Hub instance.
*   **Standalone Architecture**: Fallbacks to in-memory Go channels natively, ensuring the OS functions offline.
*   **Security (Zero Secrets)**: Relies entirely on SPIFFE/SPIRE Workload APIs to establish mTLS mesh identities. Every message must be signed with a SPIFFE SVID.

### 2.2 Shared Task List (KAIROS Orchestration)
The Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads via `swarm_tasks` and `shared_tasks`.
*   **Database Concurrency**:
    *   **PostgreSQL**: We rely on `FOR UPDATE SKIP LOCKED` for lock-free concurrency and zero TOCTOU race conditions.
    *   **SQLite**: We use explicit two-step select-then-update approaches within an explicit transaction if `pool.IsSQLite()` returns true.
*   **DAG Dependencies**: `shared_tasks` uses a normalized `task_dependencies` join table to enforce exact sequence and unblock dependent tasks only when parent tasks complete.

### 2.3 AutoDream Pipeline (Semantic Consolidation)
The Swarm generates transient memories during execution. The AutoDream semantic memory consolidation pipeline runs passively to translate ephemeral session contexts into durable, vectorized truth.
*   **Worker**: Background workers monitor `agent_session_data` and trigger Minimax/LLM summarization jobs (`AutoDreamWorker`), transforming short-term token buffers into high-dimensional `pgvector` records in `autodream_memories`.
*   **Vector Search**: Uses `pgvector` for exact Nearest Neighbor search (`ORDER BY embedding <-> $1`). In SQLite, falls back to recency-based full-text extraction (`ORDER BY created_at DESC`).

## 3. UI and Aesthetic Mandate
Every interface and artifact must feel "Premium" according to the OHC Core Values:
*   Glassmorphism: `backdrop-filter: blur(20px) saturate(200%)`
*   Color Palette: `background: rgba(255, 255, 255, 0.03)`
*   Typography: `font-family: 'Outfit', 'Inter', sans-serif`

## 4. Verification & Testing
*   **Precision Execution**: All components maintain strict >90% test coverage (`bazelisk test //srcs/server/...`).
*   **Graceful Degradation**: System must explicitly handle SQLite lock contention (`database is locked` / `SQLITE_BUSY`) using exponential backoff retry loops.

## 5. Security Posture
*   Zero manual API keys between agents; all agent-to-agent communication leverages mTLS SPIFFE certificates via `auth.RequireRole("system", ...)` checks.
*   Multi-tenant isolation ensured by `organization_id` scopes on all backend queries.
