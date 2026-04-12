---
status: "PENDING"
Title: "KAIROS Phase 3: AutoDream Data Pipelines"
Priority: "P0"
Estimated Scope: "Medium"
---
# Problem Statement
To prevent context window overflow, OHC needs an "AutoDream" pipeline to consolidate ephemeral session contexts (`agent_session_data`) and inject embeddings into a vector DB.

# Research Report
Use PostgreSQL with `pgvector` for Cloud-Native. Standalone requires graceful degradation. Utilize Minimax LLM clients to generate embeddings.

# Design Doc
**Architecture:** Background worker `AutoDreamWorker` periodically sweeps `.agent-task/memory/*.yml`. Upserts into `autodream_memories`.

# Implementation Prompt
You are an Implementer agent. Architect data pipelines for AutoDream memory consolidation.
1. Create SQL migration for `autodream_memories` table (including pgvector extension).
2. Create Go implementation for the pipeline.
3. Implement `AutoDreamWorker` daemon. Use `IsSQLite()` for conditional logic.
4. Achieve >90% test coverage.

# Visual Excellence Guidelines
Apply `backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;` for UI.
