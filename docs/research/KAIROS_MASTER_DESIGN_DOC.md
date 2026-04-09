<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.03);">

# KAIROS Orchestration: Master Design Doc

This document captures the final architecture for the OHC KAIROS layer.

## 1. Shared Task List
A durable distributed state machine in Postgres (`shared_tasks` table) ensuring lock-free concurrency via `FOR UPDATE SKIP LOCKED`. Degrades to SQLite transaction locks in Standalone Mode.

## 2. Teammate Mesh
A realtime messaging fabric using Redis Pub/Sub (`mesh:tasks`, `mesh:presence`) locally or CentrifugeNode in cloud mode. Ensures sub-millisecond coordination between autonomous agents.

## 3. AutoDream Data Pipeline
Long term memory consolidation reading from `.agent-task/memory/` and chunking into a `pgvector` store (`autodream_memories`) for exact semantic search.
</div>
