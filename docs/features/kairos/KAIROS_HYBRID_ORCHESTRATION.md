   <div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

   # Architect KAIROS Hybrid Orchestration

   **Author:** Principal Product Architect & KAIROS Orchestrator (L7)

   ## Overview
   Architect and Implement the Shared Task List (Teammate Mesh & AutoDream) for OHC.

   ## Shared Task List Architecture
   To support both Cloud-Native and Standalone Desktop modes, the Shared Task List relies on a hybrid DB schema `shared_tasks`.

   ## Realtime Teammate Mesh APIs
   The Teammate Mesh facilitates inter-agent communication via `CentrifugeNode` hub integration. Standardized OHC-SIP compliance is enforced, requiring `agent_id`, `action`, and `status` at the JSON root. Key channels:
   - `mesh:tasks`: Standard task state broadcasts.
   - `mesh:coordination`: High-priority agent-to-agent alignment.
   - `mesh:capabilities`: Agent skill discovery and advertisement.

   ## Omni-Context Sub-agent Routing
   KAIROS eliminates discovery latency by pre-injecting architectural grounding into `shared_tasks`.
   - **Grounding Files:** `CLAUDE_OHC.md`, `AGENTS.md`.
   - **Namespace:** `[SYSTEM GROUNDING]` within the `payload` JSONB.

   ## AutoDream & Hybrid RAG Sync
   The AutoDream pipeline handles long-term memory. The **Hybrid MCP RAG Sync** daemon bridges Standalone (SQLite) and Cloud (Postgres) by synchronizing `rag_records` to the `consolidated_memory` vector index.

   </div>
