<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# OHC AI OS Orchestration: Shared Task List, Teammate Mesh & AutoDream

## 1. Introduction
The One Human Corp (OHC) Swarm requires the **KAIROS Orchestrator** to define the structural and aesthetic vision for the OHC "Hybrid Agentic OS". KAIROS orchestrates the agent team by decomposing high-level feature requests into actionable tasks within a distributed **Shared Task List**.

## 2. Architecture: Shared Task List
The Shared Task List relies on database-backed state machines to prevent race conditions during task claiming.

*   **Cloud-Native Mode:** Uses PostgreSQL `FOR UPDATE SKIP LOCKED` to allow safe concurrent polling and distributed lock management.
*   **Standalone Mode:** Degrades gracefully to local SQLite transactions, using single-process Mutex locks for state coordination.

### Schema Blueprint
- **shared_tasks**: Stores `id`, `organization_id`, `title`, `status`, `agent_id`.
- **task_dependencies**: Stores `task_id` and `depends_on_task_id`.

## 3. Realtime Teammate Mesh APIs
The Teammate Mesh ensures agents coordinate without delays.

*   **Cloud-Native Mode:** Redis Pub/Sub drives the Centrifuge WebSocket hubs (`mesh:tasks`, `mesh:coordination`).
*   **Standalone Mode:** In-Memory channel broadcast ensures low-latency IPC.

**API Contracts:**
Agents use `POST /api/mesh/broadcast` to announce task claims and updates. All updates sent to the Centrifuge channel must enforce the OHC-SIP JSON structure, guaranteeing that `agent_id`, `action`, and `status` reside at the root level.

## 4. AutoDream Vector Pipeline (Memory Consolidation)
The Swarm Intelligence Protocol (OHC-SIP) dictates that temporary agent scratchpads be consolidated into long-term durable state.

*   **Worker:** A Go daemon polls `.agent-task/memory/*.yml`.
*   **Vector DB:** The content is passed through an LLM to generate an embedding (e.g., Ada 1536), which is upserted into PostgreSQL (via `pgvector`) in the `agent_memories` table.

## 5. Visual Excellence Mandate
All associated UI components must represent the OHC "Premium Feel".
- Backdrop Filter: `blur(20px) saturate(200%)`
- Background: `rgba(255, 255, 255, 0.03)`
- Typography: `'Outfit', 'Inter', sans-serif`


</div>
