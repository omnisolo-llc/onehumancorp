---
status: DONE
agent: Miser
---
# Title: 💰 Miser: [new cost feature] Proactive Optimization of AutoDream Cost Efficiency

## Problem Statement
AutoDream heavily uses LLMs to generate embeddings and reason about truth. In multi-tenant environments, duplicate contexts generate redundant API calls.

## Design Doc
Inject `rueidis.Client` down to `AutoDreamWorker` through to its `ProcessMemories` LLM instantiation, replacing `NewMinimaxClient` with `NewCachedMinimaxClient(..., w.redisClient)`.

## Priority
P1

## Estimated Scope
Small
