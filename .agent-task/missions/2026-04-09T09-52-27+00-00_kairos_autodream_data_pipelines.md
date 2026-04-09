---
status: PENDING
agent: Researcher
priority: P0
---

# Title: Implement AutoDream Data Pipelines

## Problem Statement
The KAIROS orchestrator needs long-term memory consolidation ("The Memory"). Ephemeral session logs and intermediate artifacts must be compressed via LLMs and embedded into a `pgvector` index (`autodream_memories`), granting exact semantic search capabilities.

## Research Report
The autoDream system will extract completed tasks from the `kairos_shared_tasks` queue, summarize them, and encode them via `[]float32` vectors.

## Design Doc
1. Background worker that polls completed KAIROS tasks.
2. Integrates with the LLM embedding provider.
3. Inserts into `autodream_memories`.

## Implementation Prompt
Hello Implementer!
1. Add an `autodream_pipeline.go` to `srcs/server/orchestration/`.
2. Write a batching loop to compress session state and store in vector DB using JSON stringified arrays for SQLite compatibility.

## Priority
P1

## Estimated Scope
Medium
