# High-Throughput Memory Embedding Pipeline

## Overview
The `AgentMemoryPipeline` is responsible for converting raw agent interactions into vector embeddings for future RAG (Retrieval Augmented Generation) retrieval. This process is computationally expensive and potentially slow.

## 1. Batch Embedding
Instead of embedding every message as it arrives:
- **Bolt Pattern**: Messages are queued in the `agent_memories` table. The pipeline worker fetches batches of 50 messages.
- **Latency Impact**: This reduces the number of API calls to the embedding provider (e.g., Gemini or local LLM) by 50x, significantly improving the throughput of the background synchronization process.

## 2. Token-Aware Chunking
Long transcripts must be chunked before embedding.
- **Strategy**: We use a token-aware chunking strategy that respects semantic boundaries (paragraphs, agent turns) rather than arbitrary character counts.
- **Performance**: By optimizing the chunk size to match the LLM's preferred context window, we maximize retrieval relevance while minimizing the number of redundant chunks that need to be stored and indexed.

## 3. Parallel Vector Up-sync
In Standalone mode, embeddings are generated locally. These must be synchronized to the Cloud for multi-device access.
- **Optimization**: We use parallel HTTP streams to push vector deltas.
- **Results**: Up-syncing 1000 new memory vectors takes <2 seconds even on modest upload speeds.

## 4. Vector Store Indexing
Searching through millions of high-dimensional vectors can be slow.
- **Postgres**: We utilize the `pgvector` extension with HNSW (Hierarchical Navigable Small World) indexing.
- **SQLite**: We use `sqlite-vss` for local standalone vector search.
- **Bolt Standard**: Vector similarity searches must return in <50ms for a collection of up to 100,000 memories.

## Summary
The Bolt Memory Pipeline ensures that as an organization's "collective intelligence" grows, the speed of retrieving that information remains constant. Intelligence should not come at the cost of performance.
