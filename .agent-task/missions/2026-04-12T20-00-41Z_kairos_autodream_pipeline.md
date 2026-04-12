---
status: PENDING
priority: P1
scope: Large
title: "KAIROS: Architect AutoDream Data Pipelines"
---

# Title: Architect AutoDream Data Pipelines

## Problem Statement
Phase 3 of the KAIROS playbook focuses on "autoDream": architecting the data pipelines for OHC's long-term memory consolidation system. As agents execute tasks, they generate vast amounts of unstructured context. This context must be consolidated, embedded, and stored durably to improve future swarm intelligence. This must rely on a Vector DB (pgvector/Pinecone) as defined in the Cloud-Native mode, and degrade appropriately for Standalone mode.

## Research Report
- Raw task logs and `.agent-task/memory` YAMLs are too noisy for direct context retrieval.
- "AutoDream" is the process of synthesizing this raw data into semantic concepts, generating LLM embeddings, and storing them.
- Vector storage: We will use `pgvector` for Cloud-Native (PostgreSQL) and a local SQLite embedding cache or in-memory vector search for Standalone Desktop Mode.

## Design Doc
1.  **AutoDream Pipeline Architecture:**
    - **Source:** Polling new completed `shared_tasks` or raw memory files.
    - **Synthesis:** Call Anthropic/Minimax LLM with a "Synthesis Prompt" to summarize the learning.
    - **Embedding:** Generate an embedding vector for the synthesized summary.
    - **Storage:** Upsert the embedding and metadata into the Vector Database.
2.  **Database Schema (Vector):**
    - Table: `autodream_memories`
      - `id`: UUID
      - `task_id`: UUID (Nullable reference to the source task)
      - `summary`: TEXT
      - `embedding`: VECTOR(1536) (Assuming standard 1536 dim embeddings)
      - `created_at`: TIMESTAMP
3.  **Hybrid Mode Degradation:**
    - In Standalone mode (SQLite), true `pgvector` is unavailable unless using a specific SQLite extension. We should define an interface `VectorStore` with a Postgres implementation (`pgvector`) and a naive SQLite implementation (storing JSON arrays and doing exact text match or in-memory cosine similarity on load).

## Implementation Prompt
- Define the `VectorStore` interface in `srcs/server/memory/vector_store.go`.
- Create a Goose migration script for `autodream_memories`. Use IF NOT EXISTS and database dialect checks if possible to handle SQLite vs Postgres syntax for the `VECTOR` type, or provide separate migrations based on the driver.
- Implement the `AutoDreamWorker` in `srcs/server/memory/autodream.go` that defines the pipeline logic (Extract -> Synthesize -> Embed -> Store).
- Provide mocked tests to ensure the pipeline executes the correct sequence of operations.
