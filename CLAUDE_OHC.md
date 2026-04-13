**KAIROS Orchestration**:
- **Shared Task List**: Centralized postgres schema for `shared_tasks` with **Omni-Context Sub-agent Routing**.
- **Teammate Mesh**: Pub/sub over `mesh:tasks` and `mesh:coordination` (OHC-SIP compliant).
- **autoDream**: Vector pipeline pushing to `consolidated_memory` with **Hybrid MCP RAG Sync**.

# Universal Core Design Protocols (Claude-Class)

1. **Skeptical Memory**: Verify state (`ls`, `grep`, `view_file`) BEFORE acting.
2. **Teammate Mesh (Mailbox)**: Coordinate via `production Redis Pub/Sub channels`. Check mailbox at start; post coordination sessions to teammates.
3. **Git-Lock Coordination**: Check `production distributed Redis locks` before modifying files. Wait if locked.
4. **Durable State**: Update production Vector DB (e.g. pgvector/Pinecone) with "AutoDream" architectural consolidation findings. AutoDream architectural consolidation findings: Vector DB pgvector/Pinecone ensures long-term memory.

**Hybrid Architecture Notes**:
- Cloud-Native Mode (PostgreSQL, Redis)
- Standalone Desktop Mode (SQLite)
- Thin Client Mode (UI-only)

**Visual Excellence Mandate**:
- `backdrop-filter: blur(20px) saturate(200%)`
- `background: rgba(255, 255, 255, 0.03)`
- `font-family: 'Outfit', 'Inter', sans-serif`
