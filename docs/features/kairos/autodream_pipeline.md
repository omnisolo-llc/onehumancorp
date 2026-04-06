<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Data Pipelines

**Component:** Memory & Context Layer | **Target Audience:** Intelligence & Infrastructure Engineers

## 1. Overview
The **AutoDream Data Pipeline** is responsible for Long-term Memory Consolidation within the Swarm Intelligence Protocol (OHC-SIP). Agents generate vast amounts of episodic memory (`agent_session_data` and `.agent-task/memory/*.yml` files) during execution. Left unchecked, this causes context window overflows and performance degradation.

AutoDream is an asynchronous background stream processor that consolidates these memories, prunes redundancies, resolves conflicting data points, and injects the synthesized "truth" into a vector database for semantic search (Retrieval-Augmented Generation / RAG).

## 2. Pipeline Workflow

The continuous stream processing architecture operates as follows:

```mermaid
graph TD
    Agent[Agent Execution] -->|Writes Episodic Memory| Store[SQLite / Filesystem]
    Store -->|Watches/Polls| Worker[AutoDream Pipeline Worker]
    Worker --> Dedupe[Prune & Deduplicate]
    Dedupe --> Summarize[LLM Summarization & Conflict Resolution]
    Summarize --> Chunk[Chunking & Tokenization]
    Chunk --> Embed[Embedding Model API]
    Embed -->|1536-dim Vector| VectorDB[(pgvector / Local DB)]
    VectorDB -->|Semantic Search| RAG[RAG Sync Engine]
    RAG --> Mesh[Teammate Mesh / Agents]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Agent,Store,Worker,Dedupe,Summarize,Chunk,Embed,VectorDB,RAG,Mesh premium;
```

## 3. Core Stages

1. **Ingestion & Pruning:** The orchestrator reads raw memory dumps and eliminates exact duplicates and immediate transient states.
2. **Conflict Resolution:** If Agent A claims a specific architectural constraint while Agent B claims a conflicting one, a specialized LLM deliberation prompt resolves the dispute to extract the ground truth.
3. **Embedding Generation:** The clean, consolidated text is chunked and sent to an embedding model (e.g., Anthropic, Cohere, or Minimax) to generate high-quality vector embeddings.
4. **Vector Storage:** In Cloud-Native mode, embeddings are stored natively in PostgreSQL using `pgvector`. In Standalone mode, they are stored locally to minimize dependencies.

## 4. API Invocation
The AutoDream pipeline can run continuously as a background daemon or be invoked proactively via the Orchestration API:

**Endpoint:** `POST /api/v1/autodream/`
```json
{
  "target_memory_files": [
    ".agent-task/memory/2026-04-06T12-00-00Z_insight.md"
  ],
  "priority": "high"
}
```

</div>
