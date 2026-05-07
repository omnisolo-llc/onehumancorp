//! Memory Consolidation Layer
//!
//! This module serves as the central facade for the OHC AI agents' long-term memory system.
//! The memory layer allows AI departments to retain cross-session knowledge,
//! resolve conflicting insights automatically, and prune stale context to maintain efficiency.
//!
//! # Architecture
//! The architecture is driven by the `PersistentMemoryStore` and `VectorRepository` (located in `ohc_builtin_agent::memory_store`),
//! and augmented by automated consolidation logic:
//!
//! - **Persistent Memory**: Uses pgvector in Cloud (PostgreSQL) and simple vector similarity in Standalone (SQLite). All queries enforce strict tenant scoping (`tenant_id`).
//! - **Conflict Resolution**: Detected conflicting pairs are resolved using heuristics like owner override, reliability scores, and recency.
//! - **Stale Context Pruning**: Background workers regularly groom the database, deleting outdated records depending on business relevance and reference counts.
//! - **Cross-Department Sharing**: Notes taken by one department are embedded and tagged, making them semantically searchable for any authorized department in future sessions.

pub use ohc_builtin_agent::memory_store::{
    PersistentMemoryStore,
    VectorRepository,
    EmbeddingRecord,
    LongTermMemory,
    VectorMemoryStore
};

pub use crate::workers::memory::MemoryConsolidationWorker;
pub use super::pruning::prune_stale;
pub use super::conflict::{auto_resolve_conflicts, determine_conflict_winner};
