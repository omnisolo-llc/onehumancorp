<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Document: LangGraph Checkpointing

## 1. Executive Summary
**Objective:** Enable long-term agent memory persistence using LangGraph's checkpointer interface integrated with a highly available PostgreSQL backend.
**Scope:** Develop the Go wrapper `pg-checkpointer` implementing the LangGraph Checkpointer interface.

## 2. Architecture & Components
- **LangGraph Node:** Interacts with the interface.
- **PG Checkpointer Service:** The Go service handling Postgres serialization.
- **Database Layer:** Optimized JSONB columns for state storage.

## 3. Data Flow
1. Agent executes a node and triggers `SaveCheckpoint`.
2. Service serializes the current context and tool responses.
3. Data is persisted to Postgres.
4. When resuming, `LoadCheckpoint` retrieves the data.

## 4. API & Data Models
```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Checkpoint {
    thread_id: String,
    state: serde_json::Value,
}
```

## 5. Implementation Details
- Use typed serde deserialization when reloading checkpoints to ensure schema integrity.
- Optimize PostgreSQL indexes on `thread_id`.
- Maintain Zero-Lock stack compatibility.

</div>
