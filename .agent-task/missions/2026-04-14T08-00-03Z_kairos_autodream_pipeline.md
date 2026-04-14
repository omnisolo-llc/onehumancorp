# Mission: autoDream Memory Consolidation Pipeline

**Title:** autoDream Memory Consolidation Pipeline
**Problem Statement:** Agent episodic memory is currently scattered across `.agent-task/memory/` files. We need a robust pipeline to consolidate these into long-term vector memory to enable "Full-Spectrum Observability" and "Omni-Context Sub-agent Routing".
**Research Report:**
- `srcs/server/memory/autodream/` contains initial consolidation logic.
- `consolidated_memory` table exists in some migrations with `vector(1536)` support.
- Minimax/Cohere are preferred embedding providers.
**Design Doc:**
- **Watcher:** A Go service that watches `.agent-task/memory/*.yml` for changes.
- **Consolidator:** Upon detection, parse the YAML, chunk the content, and call the embedding API.
- **Durable Storage:** Store in `consolidated_memory` with `task_id` mapping.
- **RAG Sync:** Implement a sync logic that pushes local SQLite embeddings to Cloud pgvector when the user switches modes.
**Implementation Prompt:**
- Implement `MemoryConsolidator` in `srcs/server/memory/autodream/consolidator.go`.
- Use `fsnotify` to watch for new YAML files in `.agent-task/memory/`.
- Map YAML fields (`task_id`, `agent_role`, `content`) to DB columns.
- Implement `PushToCloud(ctx context.Context)` that batches local embeddings and sends them to the Cloud API.
- Add a Grafana metric `ohc_rag_records_synced_total` to track progress.
**Priority:** P1
**Estimated Scope:** Medium
