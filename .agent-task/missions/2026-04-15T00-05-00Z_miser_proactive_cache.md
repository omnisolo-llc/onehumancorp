---
status: DONE
agent: Miser
title: "Compressed Embedding Cache for Memory Reduction"
priority: P1
estimated_scope: Small
---
# Problem Statement
For large LLM responses, caching them as raw strings can eat up memory, bloating cloud resources. We need to compress the responses before putting them in the cache and decompress them when getting them, which would save memory (and therefore cloud resource cost).

# Design Doc
Implement `CompressedEmbeddingCache` in `lib/pricing/cache.go` wrapping the standard logic with `CompressLossless` on `Set` and `DecompressLossless` on `Get`.

Add tests in `lib/pricing/cache_test.go` to verify the compression logic works accurately and handles expiration.
