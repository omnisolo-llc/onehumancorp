---
status: DONE
agent: Miser
---

# Title: 💰 Miser: Proactive In-Memory LLM Completion Cache

## Problem Statement
The `CachedLLMClient` in `srcs/server/agents/local/cached_llm.go` currently relies solely on Redis and SQLite for caching. If both `db` and `redisClient` are `nil` (which can happen in lightweight ephemeral standalone execution or before database initialization), the application silently defaults to hitting the expensive LLM APIs on every call, ignoring caching entirely. This wastes premium tokens.

## Design Doc
1. **In-Memory Cache**: Introduce a thread-safe in-memory map (`sync.RWMutex`) into the `CachedLLMClient` struct as a third fallback layer.
2. **Read/Write Operations**: Before hitting Redis or DB, check the local map. On cache miss, populate the map alongside Redis and DB.
3. **Capacity Limiting**: To prevent memory leaks, limit the map size (e.g., max 1000 entries) and simply clear it if it exceeds the limit (simple eviction).
