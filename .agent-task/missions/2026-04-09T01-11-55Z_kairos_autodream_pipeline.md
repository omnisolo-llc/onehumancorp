---
status: PENDING
---

# Title: KAIROS Orchestrator: AutoDream Data Pipelines for Memory Consolidation
## Problem Statement
OHC requires an "AutoDream" system to consolidate short-term episodic memory (from active agents and tasks) into long-term vectorized truth. This is critical for the Swarm Intelligence to learn and adapt over time, preventing redundant research and repeated mistakes.

## Research Report
The AutoDream pipeline must periodically or event-driven extract context from completed tasks, summarize the architectural findings using an LLM (like Anthropic/Claude), and embed these summaries into a Vector DB. In Cloud-Native mode, this uses `pgvector`. In Standalone mode, we need a lightweight alternative or a fallback mechanism.

## Design Doc
- **Data Flow**:
  1. Trigger: A complex KAIROS task is marked as COMPLETED.
  2. Extraction: The AutoDream pipeline extracts logs, task outputs, and agent communications.
  3. Summarization: The payload is sent to an LLM to generate a concise "Architectural Insight" summary.
  4. Embedding: The summary is converted into a vector embedding.
  5. Storage: The embedding and metadata are stored in a `vector_memory` table.
- **Database Schema (`vector_memory`)**:
  - `id`: UUID (Primary Key)
  - `source_task_id`: UUID
  - `content`: Text (The insight summary)
  - `embedding`: Vector (e.g., using pgvector extension)
  - `created_at`: Timestamp

## Implementation Prompt
You are an Implementer agent. Your task is to architect and implement the AutoDream data pipelines for OHC's long-term memory consolidation.
1. Create a database migration for the `vector_memory` table. Ensure you handle the `pgvector` extension correctly, remembering that the SQLite migration runner will automatically strip/replace PostgreSQL-specific vector syntax when in Standalone mode.
2. In `srcs/server/orchestration/autodream/`, create the pipeline logic. This should include an interface for the LLM summarizer and the vector embedder.
3. Implement a listener or background worker that processes completed tasks and runs them through the pipeline.
4. Implement the logic to insert the vector embeddings into the database. Remember to marshal the float array to JSON and explicitly cast it to a string for `pgvector` compatibility when using Go's `database/sql`.
5. Write unit tests to verify the pipeline logic, using mock LLM and embedding providers.

## Priority
P1

## Estimated Scope
Large
