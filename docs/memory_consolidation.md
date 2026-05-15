# OHC Long-term Memory & Consolidation System

## 1. Introduction
The OHC AI Agent ecosystem is designed for long-running, multi-departmental business operations. A critical requirement for such a system is the ability to retain context and knowledge across sessions, threads, and different agents. The Memory Consolidation System provides this persistent storage layer, enabling a "Global Brain" for each tenant.

Whether Maya's bakery is interacting with a customer today or planning a growth strategy next month, the agents share a unified, persistent memory that respects tenant isolation and handles information lifecycle automatically.

## 2. Architecture Overview

### 2.1. Vector Storage Layer (`memory_store.rs`)
The system utilizes a dual-backend approach to support both Cloud and Standalone environments:
- **Cloud Mode**: Leverages PostgreSQL with the `pgvector` extension.
- **Standalone Mode**: Utilizes SQLite with the `sqlite-vec` extension (or an in-memory fallback).

Each memory is stored as an `EmbeddingRecord`, which contains:
- **Content**: The textual data.
- **Embedding**: A 1536-dimensional vector for semantic similarity.
- **Scoping**: `tenant_id` and `agent_id`.
- **Lifecycle Metadata**: `created_at`, `last_referenced_at`, `reference_count`, `reliability_score`, `owner_override`, and `archived`.

### 2.2. Intelligent Consolidation (`consolidation_agent.rs`)
While rule-based deduplication is fast, it often loses nuance. The `ConsolidationAgent` is a specialized AI agent that:
1. Receives groups of semantically similar or conflicting records.
2. Uses a Large Language Model (LLM) to perform a "Semantic Merge".
3. Resolves factual conflicts by weighing source reliability and explicit overrides.
4. Generates a "Golden Record" (`GOLDEN_RECORD`) that represents the consolidated truth.

### 2.3. Background Lifecycle Worker (`consolidation_worker.rs`)
Context quality degrades if memory grows without bound. The `ConsolidationWorker` manages this through a periodic background loop:
1. **Deduplication**: Identifies nearly identical memories and merges them.
2. **Conflict Resolution**: Triggers the `ConsolidationAgent` for nuanced conflicts.
3. **Archival**: Records not referenced for a long period (e.g., 90 days) are marked as `archived`.
4. **Pruning**: Archived records that are deemed low-value (low reference count, low reliability) are permanently removed.

## 3. Usage for Agents (`memory_tool.rs`)
Agents interact with the memory system via a set of high-level tools:

### `store_memory`
Used when an agent learns a persistent fact.
- **Input**: `content`, `tags`.
- **Example**: "Customer Maya prefers sourdough over rye."

### `search_memory`
Used to retrieve context relevant to the current task.
- **Input**: `query`, `limit`.
- **Example**: "What are the customer's bread preferences?"

## 4. Tenant Isolation and Security
The system is built on a "Tenant-First" security model:
- Every query includes a mandatory `tenant_id` filter.
- In Cloud mode, PostgreSQL Row-Level Security (RLS) provides an additional layer of hardware-enforced isolation.
- Memory from one business is cryptographically and logically inaccessible to another.

## 5. Performance Considerations
- **Async I/O**: All database and LLM operations are non-blocking.
- **Batching**: The worker processes records in batches to minimize database overhead.
- **Indexing**: Vector indices are maintained to ensure sub-100ms retrieval even as memory grows.

## 6. Future Roadmap
- **Cross-Tenant Knowledge (Opt-in)**: Allowing businesses to share non-sensitive industry benchmarks.
- **Temporal Weighting**: Giving higher search priority to recent business events.
- **Visual Memory**: Support for embedding and retrieving image-based context (e.g., product photos).
