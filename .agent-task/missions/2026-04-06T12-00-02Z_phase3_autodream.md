---
status: PENDING
agent: null
Title: "KAIROS Phase 3: AutoDream Data Pipelines"
Priority: P0
Estimated Scope: Large
---

# Problem Statement
The Swarm Intelligence Protocol (OHC-SIP) mandates that all agents share episodic memory by asynchronously converting session data into long-term vector embeddings. Without this "AutoDream" capability, context windows will overflow, and Swarm intelligence will degrade as complex tasks span multiple days and agents.

# Research Report
- AutoDream requires processing `agent_session_data` and `.agent-task/memory/*.yml` files.
- In Cloud-Native mode, it utilizes PostgreSQL with the `pgvector` extension for 1536-dimensional exact Nearest Neighbor search.
- In Standalone mode, it degrades gracefully to SQLite, storing JSON blobs and relying on application-level filtering or native extensions if available.

# Design Doc
**AutoDream Worker Pipeline (`autodream_worker.go`):**
```go
package orchestration

type AutoDreamWorker struct {
    db db.Provider
    llm LLMClient
}

func (a *AutoDreamWorker) Sweep(ctx context.Context) error {
    // 1. Poll completed tasks & session data
    // 2. Chunk & Tokenize
    // 3. Generate Embeddings
    // 4. Upsert into autodream_memories
}
```

**Database Schema:**
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

# Implementation Prompt
You are an Implementer agent. Your mission is to build the long-term memory AutoDream pipeline:
1. Implement the `Sweep` method in `srcs/server/orchestration/autodream_worker.go`.
2. Ensure you process data in batches (e.g., `LIMIT 500`) to prevent unbound queue growth.
3. Use the existing LLM clients (`srcs/server/agents/local/llm.go`) to generate 1536-dim embeddings.
4. Add graceful degradation: `dbWrapper.Provider().IsSQLite()` should disable PostgreSQL `pgvector` queries and store fallback JSON representations.
5. Create an SQL migration `032_autodream_memories_pgvector.sql` for the `vector` extension and table setup in `srcs/server/db/migrations/`, updating `embedsrcs` in `srcs/server/db/BUILD.bazel`.

# Priority
P0

# Estimated Scope
Large
