<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Data Pipelines

The AutoDream Data Pipeline is the long-term memory consolidation engine of the KAIROS Orchestration. It fulfills the Swarm Intelligence Protocol (OHC-SIP) mandate that all agents share memory by asynchronously processing ephemeral session data into queryable vector embeddings.

## 1. The Need for AutoDream

During task execution, agents generate significant amounts of context (`agent_session_data` and `.agent-task/memory/*.yml` files). To prevent context window overflow and enable long-term reasoning, AutoDream sweeps this data, prunes redundancies, and injects the consolidated "truth" into a durable vector database.

## 2. Architecture and Storage

AutoDream adapts its storage mechanism based on the OHC operating mode:

- **Cloud-Native Mode:** Utilizes PostgreSQL with the `pgvector` extension for exact Nearest Neighbor search on embeddings (using `swarm_memory` and `swarm_memory_embeddings`).
- **Standalone Mode:** Degrades gracefully to SQLite. Embeddings are stored as JSON text blobs, with fallback search mechanisms if vector extensions are unavailable in the local SQLite distribution.

### Pipeline Workflow

```mermaid
graph TD
    A[Agent Session Data / Memory Files] -->|Periodic Sweep| B(AutoDream Worker)
    B -->|Chunking & Tokenization| C[existing LLM clients srcs/server/agents/local/llm.go]
    C -->|Generate Vector| D{Storage Engine}
    D -->|Cloud| E[(pgvector: swarm_memory_embeddings)]
    D -->|Standalone| F[(SQLite: JSON Blobs)]

    E -->|Semantic Search| G[Agent Context Window]
    F -->|Semantic Search| G

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F,G premium;
```

## 3. Database Schema

The persistent vector database schema (Cloud mode) is structured around `swarm_memory` and `swarm_memory_embeddings` as verified in Phase 4 design docs.

## 4. Implementation Details

- **Batch Processing:** The `AutoDreamWorker` daemon processes data in batches (e.g., `LIMIT 500`) to prevent unbound queue growth and ensure stable memory utilization.
- **LLM Integration:** Utilizes existing LLM clients (`srcs/server/agents/local/llm.go`) to generate embeddings for the consolidated memory chunks.
- **Graceful Degradation:** Conditional logic (`dbWrapper.Provider().IsSQLite()`) disables PostgreSQL-specific locks or exact-neighbor queries when operating in Standalone mode.

</div>
