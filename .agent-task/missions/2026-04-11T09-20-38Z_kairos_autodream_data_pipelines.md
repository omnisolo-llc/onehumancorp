---
status: PENDING
Title: "KAIROS Phase 3: AutoDream Data Pipelines for Memory Consolidation"
Priority: P0
Estimated Scope: Medium
---

# Problem Statement
The OHC Swarm requires the AutoDream Data Pipeline to fulfill the Swarm Intelligence Protocol (OHC-SIP) mandate that all agents share memory by asynchronously processing ephemeral session data into queryable vector embeddings. Without this, context windows overflow, and long-term reasoning is impossible.

# Research Report
Based on `CLAUDE_OHC.md` and `docs/features/kairos/autodream_pipeline.md`:
- AutoDream sweeps `.agent-task/memory/*.yml` files.
- It adapts storage based on the OHC operating mode:
  - **Cloud-Native Mode:** Utilizes PostgreSQL with `pgvector` for exact Nearest Neighbor search on 1536-dimensional embeddings.
  - **Standalone Mode:** Degrades to SQLite. Embeddings are stored as JSON text blobs, with fallback search mechanisms.
- The pipeline generates embeddings using existing LLM clients.
- It uses batch processing (e.g., `LIMIT 500`) to prevent unbound queue growth.

# Design Doc
**AutoDream Pipeline Architecture:**
1. **Worker Daemon**: A Go background process that periodically scans memory files or a database table (if session data is persisted in DB).
2. **LLM Integration**: Use `srcs/server/agents/local/llm.go` or similar to generate 1536-dimensional embeddings.
3. **Database Schema (`autodream_memories`)**:
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
4. **Fallback Mechanism**: For SQLite, the `embedding` column will just store JSON text.

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the AutoDream Data Pipelines for Memory Consolidation.
1. Create the SQL migration file for `autodream_memories` in `srcs/server/db/migrations/` (e.g., `024_autodream_memories.sql`). Ensure the `vector(1536)` type is handled correctly (using `vector` in Postgres, and a text/blob fallback in SQLite if needed via the Go migrations logic or separate files). Update `embedsrcs` in `srcs/server/db/BUILD.bazel`.
2. Implement the `AutoDreamWorker` daemon in `srcs/server/orchestration/` (or similar package).
3. The worker should periodically process new memory data in batches (e.g., `LIMIT 500`).
4. Integrate with the LLM client to generate embeddings for the text content.
5. Upsert the generated embeddings into the `autodream_memories` table. Include graceful degradation: `dbWrapper.Provider().IsSQLite()` should disable PostgreSQL-specific locks or exact-neighbor queries during the insert/search operations if applicable.
6. Write unit tests for the worker and the database operations.
7. Use `bazelisk test //srcs/server/...` to verify your code.

# Visual Excellence Guidelines
Any frontend visualization of the memory pipeline must apply:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`
