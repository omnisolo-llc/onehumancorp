<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Pipeline: Visual Walkthrough

This guide details the architectural flow of the AutoDream Pipeline, the long-term memory consolidation engine of the KAIROS Orchestrator.

## 1. Overview of the AutoDream Pipeline

During task execution, agents generate significant amounts of context. To prevent context window overflow and enable long-term reasoning, AutoDream sweeps this data, prunes redundancies, and injects the consolidated "truth" into a durable vector database.

### Architecture Flow

```mermaid
graph TD
    A[Agent Session Data / Memory Files] -->|Periodic Sweep| B(AutoDream Worker)
    B -->|Generate Vector| D{Storage Engine}
    D -->|Cloud| E[(pgvector Database)]
    D -->|Standalone| F[(SQLite Database)]

    E -->|Semantic Search| G[Agent Context Window]
    F -->|Semantic Search| G

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,D,E,F,G premium;
```

## 2. Implementation Details

- **Cloud-Native Mode:** Utilizes PostgreSQL with the `pgvector` extension for exact Nearest Neighbor search on vector embeddings.
- **Standalone Mode:** Degrades gracefully to SQLite. Embeddings are stored securely, with fallback search mechanisms.

</div>
