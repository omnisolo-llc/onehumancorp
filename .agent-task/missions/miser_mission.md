---
status: DONE
agent: Miser
---
# Title: Implement Local Embedding Caching for AutoDream

## Problem Statement
The AutoDream memory consolidation process frequently re-computes embeddings for similar or identical architectural context blocks. Calling the LLM API (`GenerateEmbedding`) for previously processed text consumes unnecessary premium tokens and API costs.

## Research Report
- Current AutoDream uses the LLM client directly for all chunks.
- We can implement a caching layer (in-memory or Redis-based for Cloud mode) to store a hash of the text chunk and its resulting embedding.
- This fulfills the Miser Cost Engineer mandate of token efficiency.

## Design Doc
1. **Cache Structure**:
   - Cloud Mode: Use `go-redis/v9` to store text hash -> vector mapping.
   - Standalone Mode: Use an in-memory thread-safe map or local SQLite table. Since we are dealing with embeddings, a local SQLite table `embedding_cache` is durable and cost-effective.
2. **Integration**:
   - Intercept calls to `GenerateEmbedding` inside `AutoDreamWorker`.

## Implementation Prompt
- Create a caching wrapper around the LLM embedding client.
- Test that duplicate contents don't hit the real client twice.

## Priority
P1

## Estimated Scope
Medium
