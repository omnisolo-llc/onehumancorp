<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Document: Shared Organizational Memory Bank

## 1. Executive Summary
**Objective:** Architect and implement Shared Organizational Memory Bank to empower autonomous agents and human operators.
**Scope:** Integration within the core Orchestration Hub and the MCP Gateway, adhering to the Zero-Lock paradigm.

## 2. Architecture & Components
Establishes a centralized Vector Database (pgvector for Cloud, sqlite-vec / Rust fallback for Standalone mode) that aggregates insights from all isolated LangGraph checkpointers. A continuous background process distills localized agent memories into globally accessible semantic embeddings.

### Cloud vs. Standalone Modes
- **Cloud Mode:** Utilizes PostgreSQL with `pgvector` for efficient, native semantic search using the `<=>` operator.
- **Standalone Mode:** Uses SQLite with `sqlite-vec` if available. In pure Rust environments without C-extensions, a fallback Rust cosine distance calculator is used natively to process in-memory similarity metrics across limited bounds (e.g. `LIMIT 1000`).

## 3. Data Flow
1. **Trigger:** The feature is invoked via Agent intent or a K8s event.
2. **Processing:** The Orchestration Hub routes the payload, verifying SPIFFE/SPIRE constraints.
3. **Execution:** The action is securely completed with all operations logged immutably.
4. **Result:** The system state is updated and the event is written to `events.jsonl`.

## 4. Conflict Resolution & Pruning
- **Conflict Resolution:** A background `MemoryConsolidationWorker` automatically detects redundant or conflicting memories via semantic similarity (< 0.05 distance limit). Conflicts are resolved based on `owner_override`, `reliability_score`, and finally `created_at` timestamp.
- **Stale Context Pruning:** `TASK_SUMMARY` source types older than 180 days with a low reference count (< 5) and no `owner_override` are automatically pruned to prevent outdated context bleed.

## 5. Cross-Department Sharing
- Memories are strictly isolated via row-level security per `tenant_id`. However, within a given `tenant_id`, memories are NOT siloed by department or `agent_id`.
- For example, context saved by Customer Success about a dissatisfied customer will seamlessly appear during semantic retrieval by the Business Advisory department.

</div>
