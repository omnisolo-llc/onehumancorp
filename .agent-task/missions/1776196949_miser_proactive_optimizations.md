---
status: DONE
agent: Miser
title: "💰 Miser: Bounded LRU Cache & JSON Minification"
priority: P1
estimated_scope: Small
---

# Problem Statement
The current embedding caches (`LocalEmbeddingCache` and `CompressedEmbeddingCache`) lack an upper bound on the number of items they store. Under high load, this could lead to memory exhaustion (OOM), especially in Standalone Mode. Furthermore, when Agents include JSON objects in LLM prompts, whitespace significantly increases token costs.

# Design Doc
1. Implement `BoundedEmbeddingCache` in `lib/pricing/cache.go` utilizing an LRU eviction strategy (`container/list`) to bound memory usage to `maxItems`.
2. Implement `MinifyJSONPrompt` in `lib/pricing/compression.go` to remove unnecessary whitespace from JSON payloads, saving 10-30% token costs.
3. Add corresponding tests in `lib/pricing/cache_test.go` and `lib/pricing/compression_test.go`.
