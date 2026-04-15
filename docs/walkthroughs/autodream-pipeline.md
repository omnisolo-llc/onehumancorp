<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Pipeline: Visual Walkthrough

This guide details the architectural flow of the AutoDream Data Pipelines, the long-term memory consolidation engine that satisfies the Swarm Intelligence Protocol (OHC-SIP).

## 1. Overview of the AutoDream Pipeline

During standard operation, the OHC Swarm generates vast amounts of ephemeral context—logs, scratchpads, and short-term reasoning chunks. To prevent context window overflows and preserve organizational knowledge, the AutoDream Pipeline periodically awakens to read these raw thoughts, compress them into semantic vectors, and persist them into a long-term Vector DB.

### Architectural Flow

```mermaid
graph TD
    Agent[Worker Agent] -->|Produces| SessionData(agent_session_data)
    Agent -->|Produces| MemoryFiles(OHC_MEMORY_DIR/*.yml)

    SessionData -->|Periodic Sweeps| AutoDreamWorker
    MemoryFiles -->|Periodic Sweeps| AutoDreamWorker

    subgraph AutoDream Engine
        AutoDreamWorker[AutoDream Worker Daemon] -->|Chunks Data| Chunker[Chunk & Tokenize]
        Chunker -->|Requests Embeddings| EmbedAPI[Embedding API (Minimax/Cohere)]
        EmbedAPI -->|Returns Vectors| VDB_Insert{Vector Storage}
    end

    VDB_Insert -->|Cloud Native Mode| PGVector[(PostgreSQL pgvector)]
    VDB_Insert -->|Standalone Mode| SQLite[(SQLite + Blob Fallback)]

    PGVector -->|Semantic Search| RAG[Agent Context Retrieval]
    SQLite -->|Semantic Search| RAG

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Agent,SessionData,MemoryFiles,AutoDreamWorker,Chunker,EmbedAPI,VDB_Insert,PGVector,SQLite,RAG premium;
```

## 2. Execution Lifecycle

The pipeline operates as a distributed background worker, ensuring that the primary Teammate Mesh and task orchestration aren't slowed down by heavy vector processing:

1. **Sweep**: The background worker scans completed tasks and recently closed session data (typically anything older than an hour but untouched).
2. **Chunk**: The content is divided into overlapping blocks to maintain semantic meaning.
3. **Embed**: The chunks are sent to the embedding model (e.g., Minimax, Cohere, or local LLMs).
4. **Persist**: The 1536-dimensional embeddings are saved:
   - In **Cloud Mode**, using `pgvector` for scalable nearest-neighbor queries.
   - In **Standalone Mode**, gracefully degrading to SQLite.
5. **Prune**: The original verbose `agent_session_data` is truncated to free up storage, achieving "zero-WIP" cleanliness.

## 3. RAG and Semantic Search

Once data is securely stored, any future agent joining the Swarm can inject relevant historical context into their prompt window. By executing a semantic search query (`ORDER BY embedding <-> $1`), agents can recall architectural decisions, bug fixes, or stylistic mandates created weeks or months prior.

</div>
