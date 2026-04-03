---
status: DONE
agent: Miser
---
# Title: Implement Storage Compression for Local Embedding and Reason Cache

## Problem Statement
The caching layer currently stores raw `response` text and raw JSON-marshaled `embedding` float arrays in both Redis and SQLite (`llm_reason_cache` and `embedding_cache`). These float arrays and extensive text reasoning responses consume large amounts of memory in Redis (Cloud mode) and disk space in SQLite (Standalone mode). This unnecessarily spikes operational storage costs.

## Research Report
- Current `CachedMinimaxClient` saves string data as-is.
- Compressing this data using `gzip` before caching and decompressing after retrieving will drastically reduce the byte footprint of both JSON embeddings and text reasoning responses, adhering to the "Miser" cost engineer mandate.

## Design Doc
1. **Cache Compression**:
   - Create helper methods `compress(data []byte) ([]byte, error)` and `decompress(data []byte) ([]byte, error)` using `compress/gzip`.
   - Update `CachedMinimaxClient.Reason` and `GenerateEmbedding` to compress data before sending to Redis/DB.
   - For backward compatibility in cache, we can detect gzip magic headers (`0x1F`, `0x8B`) to gracefully fallback if existing data is uncompressed.

## Implementation Prompt
- Modify `srcs/server/orchestration/cached_minimax_client.go`.
- Ensure tests still pass.

## Priority
P2

## Estimated Scope
Small
