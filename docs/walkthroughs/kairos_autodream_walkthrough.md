<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AutoDream Pipeline: Visual Walkthrough

This document visualizes the **AutoDream Pipeline**, the memory consolidation engine for the KAIROS Orchestrator.

## Architectural Overview

AutoDream continuously embeds ephemeral agent session contexts into durable long-term memory via `pgvector` (in Cloud mode) and a simulated local vector store (in Standalone Desktop mode).

```mermaid
sequenceDiagram
    participant Worker as Agent
    participant FS as Local Session (Ephemeral)
    participant AutoDream as AutoDream API
    participant LLM as Embedding Service (Ada/Minimax)
    participant DB as VectorDB (pgvector)

    Worker->>FS: Write Context to .ohc/runtime/memory/
    loop Periodic Consolidation
        AutoDream->>FS: Scan for new session context
        AutoDream->>LLM: Generate 1536-dim Embedding
        LLM-->>AutoDream: Return Embedding Vector
        AutoDream->>DB: Upsert to `autodream_memories`
        AutoDream->>Worker: Broadcast Consolidation Success (via Teammate Mesh)
    end
```

## Hybrid Resilience

OHC is designed for "Hybrid Parity". Whether running in a massive Kubernetes cluster or on a single developer laptop, the AutoDream pipeline ensures semantic continuity.

### Cloud Mode
- **Persistence:** PostgreSQL with `pgvector` extension.
- **Embeddings:** High-throughput cloud APIs (OpenAI Ada / Minimax).
- **Scale:** Multi-tenant isolation at the database row level.

### Standalone Mode
- **Persistence:** Local SQLite with vector simulation.
- **Embeddings:** Localized or efficient edge API calls.
- **Privacy:** All session data remains on the local machine until explicitly synced.

---

## Next Steps
- [Interactive CLI Guide](./autodream_cli_interactive_guide.md)
- [Teammate Mesh Walkthrough](../technical/walkthroughs/teammate_mesh.md)

</div>
