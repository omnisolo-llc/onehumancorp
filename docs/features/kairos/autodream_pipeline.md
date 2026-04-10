<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Data Pipelines

The AutoDream Data Pipeline is the long-term memory consolidation engine of the KAIROS Orchestrator. It fulfills the Swarm Intelligence Protocol (OHC-SIP) mandate that all agents share memory by asynchronously processing ephemeral session data into queryable vector embeddings.

## 1. Architecture

During task execution, agents generate significant amounts of context (`agent_session_data` and `.agent-task/memory/*.yml` files). To prevent context window overflow and enable long-term reasoning, an `AutoDreamWorker` daemon sweeps this data, prunes redundancies, and injects the consolidated "truth" into a durable vector database.

- **Cloud-Native Mode:** Utilizes PostgreSQL with the `pgvector` extension for exact Nearest Neighbor search on 1536-dimensional embeddings.
- **Standalone Mode:** Degrades gracefully to SQLite. Embeddings are stored as JSON text blobs, with fallback search mechanisms if vector extensions are unavailable in the local SQLite distribution.

## 2. Database Schema (PostgreSQL)

```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_autodream_org ON autodream_memories(organization_id);
```

## 3. Worker Implementation Details

- **Batch Processing:** The daemon processes data in limited batches (e.g., `LIMIT 500`) to prevent unbound queue growth.
- **LLM Integration:** Utilizes existing LLM clients (`srcs/server/agents/local/llm.go`) to generate embeddings.
- **Graceful Degradation:** The pipeline utilizes conditional logic (`dbWrapper.Provider().IsSQLite()`) to adapt queries for standalone deployments.

</div>
