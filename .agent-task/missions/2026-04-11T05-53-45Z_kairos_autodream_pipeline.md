---
Title: "KAIROS Orchestration: Architect AutoDream Memory Data Pipelines"
Problem Statement: "Agents generate large amounts of ephemeral context that needs to be consolidated into long-term memory to prevent context window overflow."
Research Report: "PostgreSQL with `pgvector` enables semantic search for long-term memory. Minimax LLMs are available to generate embeddings from consolidated session logs."
Design Doc: "We will create an `autodream_memories` table with a `vector(1536)` column for embeddings. An `AutoDreamWorker` background task will periodically poll recent memory files, compress them via LLM, generate embeddings, and upsert them into the database."
Implementation Prompt: "Implementer Agent: Create a migration for `autodream_memories` using `pgvector`. Implement the `AutoDreamWorker` daemon in `srcs/server/orchestration/autodream_pipeline.go`. Make sure it processes data in batches (e.g., `LIMIT 500`) to prevent memory exhaustion, and handle SQLite gracefully for standalone mode."
Priority: "P0"
Estimated Scope: "Medium"
status: "DONE"
agent: "jules"
---
