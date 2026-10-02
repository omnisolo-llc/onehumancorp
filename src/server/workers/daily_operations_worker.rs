//! Compatibility name for the previously unmounted daily-operations module.
//!
//! Keep a single factual, deduplicated implementation. This module is not
//! registered in `workers/mod.rs`; reusing the old name must not resurrect the
//! former unconditional checklist generator or its copied test implementation.
pub use super::proactive_operations_worker::ProactiveOperationsWorker as DailyOperationsWorker;
