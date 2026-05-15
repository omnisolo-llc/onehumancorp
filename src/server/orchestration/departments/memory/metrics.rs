use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub contexts_stored: Arc<AtomicU64>,
    pub contexts_pruned: Arc<AtomicU64>,
    pub conflicts_resolved: Arc<AtomicU64>,
    pub cross_department_queries: Arc<AtomicU64>,
}

impl MemoryMetrics {
    pub fn new() -> Self {
        Self {
            contexts_stored: Arc::new(AtomicU64::new(0)),
            contexts_pruned: Arc::new(AtomicU64::new(0)),
            conflicts_resolved: Arc::new(AtomicU64::new(0)),
            cross_department_queries: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_store(&self) {
        self.contexts_stored.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_prune(&self, count: u64) {
        self.contexts_pruned.fetch_add(count, Ordering::Relaxed);
    }

    pub fn record_conflict_resolution(&self) {
        self.conflicts_resolved.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cross_dept_query(&self) {
        self.cross_department_queries.fetch_add(1, Ordering::Relaxed);
    }
}
