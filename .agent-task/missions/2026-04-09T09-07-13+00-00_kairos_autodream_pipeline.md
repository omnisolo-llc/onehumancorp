---
agent: "KAIROS Orchestrator"
status: "PENDING"
Title: "Implement AutoDream Long-Term Memory Pipeline"
Priority: "P0"
Estimated Scope: "Medium"
---

# Problem Statement
The swarm requires a reliable long-term persistence layer (The Memory) to avoid repeating past mistakes and context loss. The existing AutoDream implementation handles basic embedding generation, but needs an architecturally sound pipeline for Postgres vector searches and SQLite fallbacks.

# Research Report
Minimax LLMs are used to generate embeddings (1536 dims). These should be stored in `pgvector` (`VECTOR(1536)`) for optimal semantic search in Cloud Mode, and fallback to serialized JSON/base64 strings in SQLite for Standalone Mode.

# Design Doc
**Architecture:**
- **Table:** `autodream_memories`
- **Fields:** `id` (UUID), `content` (TEXT), `embedding` (VECTOR/TEXT), `source_mission_id` (TEXT), `organization_id` (TEXT), `agent_id` (TEXT).
- **Service:** `srcs/server/agents/autodream.go`

**Sequence:**
1. Background tick -> Query completed shared tasks.
2. Formulate context -> LLM compression -> generate embedding vector.
3. UPSERT into `autodream_memories`.
4. Feature Agents query `SearchMemory(ctx, query_embedding)` -> Uses L2 distance `<->` operator in pgvector, or brute-force cosine similarity in Go for SQLite.

# Implementation Prompt
Enhance the `autodream_memories` schema via a new migration in `srcs/server/db/migrations/` to explicitly enforce the Hybrid Data Model (Postgres `VECTOR(1536)` vs SQLite `TEXT`). Update `srcs/server/agents/autodream.go` to implement `SearchMemory(ctx context.Context, query []float32, limit int) ([]Memory, error)`. For Postgres, use `ORDER BY embedding <-> $1 LIMIT $2`. For SQLite, read all candidate vectors, unmarshal from JSON, and calculate cosine similarity in memory using Go. Ensure robust unit testing covering both scenarios. Add telemetry tracking for embedding generation latency.
