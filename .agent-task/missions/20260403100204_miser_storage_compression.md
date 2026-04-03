---
status: DONE
agent: Miser
---

# Title: Proactive Storage Compression for LLM Caches and Vector Precision

## Problem Statement
The OHC Hybrid Architecture relies on storing LLM responses and embeddings to save API costs and improve performance. However, these text-based caches (`llm_reason_cache`, `embedding_cache`) and `autodream_memories` take up significant disk and memory space, especially in Standalone Mode (SQLite). Additionally, vector formatting currently uses `%f` which adds unnecessary trailing zeros, ballooning string size, and SQLite BLOB insertions for vectors are incorrectly using string representations instead of byte arrays.

## Research Report
- Standalone mode requires minimal footprint. We can significantly reduce `embedding_cache` size by fixing the vector formatting from `%f` to `%g`.
- SQLite BLOB storage expects binary data; currently `fmt.Sprintf("%v", embedding)` forces text storage which is larger and violates OHC guidelines.
- Caching LLM reasoning responses in `CachedMinimaxClient` can be compressed using gzip to achieve 60-80% storage reduction for large prompts/responses in both Redis and DB.

## Design Doc
1. **Vector Precision**: Update `formatVector` in `srcs/server/agents/autodream.go` to use `%g`.
2. **SQLite BLOB Fix**: Cast the formatted vector to `[]byte` when inserting into SQLite.
3. **Cache Compression**: Introduce a gzip compression/decompression utility. Modify `CachedMinimaxClient` to compress `response` strings before DB/Redis insertion, and decompress upon retrieval.

## Implementation Prompt
- Fix `autodream.go`.
- Add `compressString` and `decompressString` helper functions.
- Update `CachedMinimaxClient.Reason` to use these helpers.
- Write tests.

## Priority
P1

## Estimated Scope
Medium
