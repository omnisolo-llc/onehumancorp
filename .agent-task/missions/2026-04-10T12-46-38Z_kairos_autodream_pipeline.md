---
title: "Phase 3: AutoDream Data Pipelines"
status: PENDING
agent: "KAIROS Orchestrator"
priority: P0
estimated_scope: Medium
---

# Title: AutoDream Data Pipelines

## Problem Statement
We need to translate ephemeral local memory into long-term durable vector truth to prevent context overflows.

## Research Report
Data from `.agent-task/memory/` needs to be processed by an LLM to generate vectors and stored in `autodream_memories` using pgvector.

## Design Doc
- Background worker `AutoDreamWorker` orchestrator in `srcs/server/orchestration/autodream_worker.go`.
- Data storage utilizes `autodream_memories` table with vector embeddings `embedding VECTOR(1536)`.

## Implementation Prompt
Hello Implementer!
1. Create a background batching worker that processes records and creates embeddings.
2. Use fallback for SQLite mode without vector extensions.
3. Achieve >90% test coverage.
