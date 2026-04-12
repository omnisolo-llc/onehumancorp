---
status: OPEN
agent: UNASSIGNED
---

# Title
KAIROS Orchestration: Shared Task List, Teammate Mesh, and AutoDream Consolidation

# Problem Statement
OHC requires a distributed, highly scalable, and aesthetically premium Agentic Operating System architecture to orchestrate a vast swarm of AI agents. The current system lacks a robust database design for shared task queues, a real-time communication mesh for agent coordination, and a long-term vector-based memory consolidation pipeline.

# Research Report
Based on OHC-HA architecture, we need to design components that degrade gracefully:
1.  **Cloud-Native Mode**: PostgreSQL for durable state, Redis for realtime pub/sub and distributed locks, pgvector for embeddings.
2.  **Standalone Desktop Mode**: SQLite for durable state and degraded in-memory channels.
3.  **Visual Mandate**: UIs must implement `backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`, and 'Outfit/Inter' typography.

# Design Doc
## Phase 1: Shared Task List (UltraPlan/Decomposition)
**Database Schema (PostgreSQL/SQLite compatible):**
- Table `tasks`: `id` (UUID), `title` (VARCHAR), `status` (VARCHAR), `priority` (VARCHAR), `assignee` (VARCHAR), `created_at` (TIMESTAMP), `updated_at` (TIMESTAMP).
- Table `task_dependencies`: `task_id` (UUID), `depends_on_task_id` (UUID).
**Sequence Diagram (Text):**
1. KAIROS Orchestrator -> Database: INSERT new task
2. Sub-Agent -> Database/Redis: SELECT / POP task queue
3. Sub-Agent -> KAIROS Orchestrator: ACKnowledge via Teammate Mesh

## Phase 2: Teammate Mesh APIs (Orchestration)
**Realtime API Design:**
- Transport: WebSockets / Redis Pub/Sub (fallback to polling in SQLite standalone).
- Channels: `ohc_mesh_global`, `ohc_mesh_task_{task_id}`.
- Payload Contract: JSON containing `event_type`, `sender_id`, `timestamp`, `data`.

## Phase 3: AutoDream Data Pipelines (Memory Consolidation)
**Vector DB Schema (pgvector):**
- Table `memory_embeddings`: `id` (UUID), `content_text` (TEXT), `embedding` (vector(1536)), `metadata` (JSONB).
**Pipeline Flow:**
1. Agent completes task -> Memory written to `memory_embeddings`.
2. Nightly cron / background queue -> Summarizes daily activity and updates Pinecone / pgvector index.

# Implementation Prompt
Dear Implementer Agent:
1. Implement the `tasks` and `task_dependencies` database migrations in `srcs/server/db/migrations`. Ensure compatibility with both PostgreSQL and SQLite. Provide dummy queries `SELECT 1;` for PostgreSQL-specific commands if needed.
2. Create the Teammate Mesh API endpoints in Go, supporting Redis Pub/Sub and WebSocket streaming.
3. Implement the autoDream memory pipeline using `pgvector` in the backend.
4. Ensure any frontend web components strictly apply the OHC Visual Excellence Mandate: `backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter', sans-serif`.
5. Write tests maintaining >90% coverage for these packages.
6. Remember to adhere to the 'No Half-Implementations' rule.

# Priority
P0

# Estimated Scope
Large
