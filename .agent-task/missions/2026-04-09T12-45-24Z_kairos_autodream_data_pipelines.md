---
Title: "Implement KAIROS AutoDream Data Pipelines"
Problem Statement: "During task execution, agents generate significant amounts of context. To prevent context window overflow and enable long-term reasoning, AutoDream sweeps this data, prunes redundancies, and injects the consolidated truth into a durable vector database."
Research Report: "Based on docs/features/kairos/autodream_pipeline.md, AutoDream utilizes PostgreSQL with pgvector for Cloud-Native mode, and degrades to SQLite with JSON text blobs for Standalone mode. The pipeline sweeps agent session data, chunks/tokenizes it, generates embeddings using Minimax/Cohere, and stores it in `autodream_memories`."
Design Doc: "1. Batch Processing: Implement an `AutoDreamWorker` daemon to process data in batches. 2. LLM Integration: Use `srcs/server/agents/local/llm.go` for embeddings. 3. Database: Create or verify the `autodream_memories` table schema. Ensure `pgvector` is used in Postgres. 4. Fallback: Implement a JSON blob fallback for SQLite. 5. Observability: Add OpenTelemetry metrics for processed batches, tokens used, and pipeline latency."
Implementation Prompt: "1. Read docs/features/kairos/autodream_pipeline.md. 2. Create `srcs/server/orchestration/autodream.go`. 3. Implement the worker loop with batch processing. 4. Integrate embedding generation. 5. Handle DB inserts correctly based on the dialect (pgvector vs JSON). 6. Instrument with OpenTelemetry. 7. Write tests."
Priority: "P0"
Estimated Scope: "Large"
---
