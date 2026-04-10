<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Vector Pipeline: Visual Walkthrough

Welcome to the AutoDream Vector Pipeline Walkthrough. This guide explains how the One Human Corp (OHC) Swarm consolidates ephemeral session memory into long-term vector embeddings.

## 1. The AutoDream Lifecycle

```mermaid
graph TD
    A[Worker Agent] -->|Writes to| B(.agent-task/memory/*.yml)
    B -->|Polled by| C(AutoDream Pipeline)
    C -->|Chunk & Embed| D[Embedding Model]
    D -->|Store| E[(pgvector / SQLite)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```

For more detailed API integration, please refer to the [AutoDream Pipeline Feature Docs](../features/kairos/autodream_pipeline.md).

</div>
