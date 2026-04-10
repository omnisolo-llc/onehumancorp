<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Pipeline: Visual Walkthrough

Welcome to the AutoDream Pipeline walkthrough. This core component of the KAIROS orchestration engine provides long-term persistence and exact semantic search capabilities for the OHC Swarm.

## 1. Pipeline Overview

The AutoDream Pipeline continuously compresses ephemeral session logs and intermediate agent artifacts into long-term vector embeddings stored in a `pgvector` index (or SQLite for Standalone Mode).

```mermaid
graph TD
    A[Agent Task Logs (.agent-task/memory/*.yml)] -->|Observed| B[AutoDream Worker]
    B -->|Minimax LLM Compression| C[Compressed Context]
    C -->|Embedding Generation| D[1536-dim Vector]
    D -->|Upsert| E[(pgvector / SQLite Index)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```

## 2. Memory Compression Lifecycle

1. **Extraction**: The AutoDream background worker scans for completed task outputs and session logs.
2. **Compression**: Instead of storing raw transcripts, a Minimax LLM distills the execution into key insights, decisions, and architectural context.
3. **Vectorization**: The compressed text is embedded using standard embedding models.
4. **Persistence**: In Cloud Mode, embeddings are stored in PostgreSQL's `pgvector` extension. In Standalone Mode, they degrade to local SQLite vector tables.

## 3. Semantic Search

Agents can query the embedded memory via the OHC Central Database to instantly recall past solutions, preventing redundant work and enforcing Swarm Intelligence.

</div>
