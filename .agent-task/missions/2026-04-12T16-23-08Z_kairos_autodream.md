---
status: PENDING
agent: Implementer
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Title: Architect autoDream Data Pipelines

## Problem Statement
OHC requires long-term memory consolidation for Swarm Intelligence Protocol (OHC-SIP). We need a defined pgvector / LLM embeddings pipeline for the "autoDream" capability.

## Research Report
Memory must be durable and retrievable via vector similarity search.

## Design Doc
Pipeline flow:
1. `agent_missions` logs -> Vector Embedder (OpenAI / Local LLM)
2. Store in `autodream_vectors` table (pgvector `vector(1536)`).
3. Cron job runs nightly (autoDream phase) to cluster and summarize old memory.

## Implementation Prompt
1. Create `srcs/server/db/migrations/036_autodream_pgvector.sql`.
2. Implement `srcs/server/memory/autodream_pipeline.go` containing `Consolidate()` method.
3. Use OpenTelemetry to track embedding latency.

## Priority
P1

## Estimated Scope
Large
</div>
