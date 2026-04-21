Based on `docs/research/50_features_mandate.json`, rank 29 is "Semantic Cache": "Caching LLM responses based on semantic similarity of the prompt to save costs and reduce latency on repetitive queries."

We already have:
1. `SemanticDeduplicator` in `srcs/server/lib/pricing/deduplicator.go` which uses Jaccard Similarity to deduplicate prompts.
2. We have `LocalEmbeddingCache`, `CompressedEmbeddingCache`, and `BoundedEmbeddingCache` in `srcs/server/lib/pricing/cache.go` which caches by exactly matching a SHA-256 hash.

We need to add a `SemanticCache` that leverages `jaccardSimilarity` or another similarity method.
Let's see what we can do in `srcs/server/lib/pricing/cache.go`. We can add a `SemanticCache` struct that combines caching logic with semantic similarity!
