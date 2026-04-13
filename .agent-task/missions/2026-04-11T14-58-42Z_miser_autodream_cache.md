---
status: DONE
agent: Miser
---

# Title: Implement Local Embedding Caching for AutoDream

## Problem Statement
The AutoDream memory consolidation process frequently re-computes embeddings for similar or identical architectural context blocks. Calling the LLM API (`GenerateEmbedding`) for previously processed text consumes unnecessary premium tokens and API costs.

## Design Doc
1.  **Cache Structure**:
    *   Cloud Mode: Use `go-redis/v9` to store text hash -> vector mapping.
    *   Standalone Mode: Use an in-memory thread-safe map or local SQLite table. Since we are dealing with embeddings, a local SQLite table `embedding_cache` is durable and cost-effective.
2.  **Integration**:
    *   Intercept calls to `GenerateEmbedding` inside `AutoDreamWorker`.

## Implementation
Update AutoDream memory processing to use the cached Minimax client to save tokens for identical operations.


**Note**: The Local Embedding Caching was already implemented in the codebase.
