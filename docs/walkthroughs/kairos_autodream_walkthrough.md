<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AutoDream Pipeline: Visual Walkthrough

This document visualizes the AutoDream Pipeline, which acts as the memory consolidation engine for the KAIROS Orchestrator.

## The AutoDream Architecture

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

## Seamless Hybrid Degradation

In Standalone Mode, AutoDream falls back to local SQLite operations to ensure low resource consumption without sacrificing core autonomous capability.

</div>
