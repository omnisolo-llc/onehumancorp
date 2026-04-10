# OHC Hybrid Architecture Guidelines for Agents

## The KAIROS Triad (Orchestration Layer)
1. **Shared Task List**: Distributed state machine in `shared_tasks`. Uses PostgreSQL `FOR UPDATE SKIP LOCKED` for Cloud, SQLite transactions for Standalone.
2. **Teammate Mesh**: Low-latency communication via `CentrifugeNode` and Redis Pub/Sub (`rueidis`).
3. **AutoDream**: Long-term persistence compressing ephemeral session data via `srcs/server/agents/local/llm.go` into `pgvector` (specifically `swarm_memory` and `swarm_memory_embeddings`) for exact semantic search.
