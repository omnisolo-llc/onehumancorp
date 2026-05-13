
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::msgbus::DistributedLock;
use crate::interop::protocol::{InteropProtocol, proto};

/// Implements a Vector Clock for causality tracking
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VectorClock {
    clocks: HashMap<String, u64>,
}

impl VectorClock {
    pub fn new() -> Self {
        Self { clocks: HashMap::new() }
    }

    pub fn increment(&mut self, node_id: &str) {
        let count = self.clocks.entry(node_id.to_string()).or_insert(0);
        *count += 1;
    }

    pub fn merge(&mut self, other: &VectorClock) {
        for (node, count) in &other.clocks {
            let local_count = self.clocks.entry(node.to_string()).or_insert(0);
            *local_count = (*local_count).max(*count);
        }
    }

    pub fn dominates(&self, other: &VectorClock) -> bool {
        let mut strictly_greater = false;
        for (node, count) in &other.clocks {
            let local_count = self.clocks.get(node).unwrap_or(&0);
            if local_count < count {
                return false;
            }
            if local_count > count {
                strictly_greater = true;
            }
        }
        for (node, local_count) in &self.clocks {
            if !other.clocks.contains_key(node) && *local_count > 0 {
                strictly_greater = true;
            }
        }
        strictly_greater
    }
}

/// State resolving using vector clocks
pub struct StateResolver {
    node_id: String,
    state_store: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    clock_store: Arc<RwLock<HashMap<String, VectorClock>>>,
    lock: Arc<dyn DistributedLock>,
}

impl StateResolver {
    pub fn new(node_id: String, lock: Arc<dyn DistributedLock>) -> Self {
        Self {
            node_id,
            state_store: Arc::new(RwLock::new(HashMap::new())),
            clock_store: Arc::new(RwLock::new(HashMap::new())),
            lock,
        }
    }

    pub async fn apply_handoff(&self, handoff: proto::StateHandoff, mut remote_clock: VectorClock) -> Result<bool, String> {
        let lock_key = format!("state_handoff:{}", handoff.mission_id);
        if !self.lock.acquire_lock(&lock_key, &self.node_id, 30).await.unwrap_or(false) {
            return Err("Lock acquisition failed".to_string());
        }

        let mut clocks = self.clock_store.write().await;
        let mut states = self.state_store.write().await;

        let local_clock = clocks.entry(handoff.mission_id.clone()).or_insert_with(VectorClock::new);

        if local_clock.dominates(&remote_clock) {
            // Local state is newer, reject
            let _ = self.lock.release_lock(&lock_key, &self.node_id).await;
            return Ok(false);
        }

        // Merge clocks and accept state
        local_clock.merge(&remote_clock);
        local_clock.increment(&self.node_id);

        states.insert(handoff.mission_id.clone(), handoff.state_snapshot.clone());

        let _ = self.lock.release_lock(&lock_key, &self.node_id).await;
        Ok(true)
    }

    pub async fn get_state(&self, mission_id: &str) -> Option<Vec<u8>> {
        let states = self.state_store.read().await;
        states.get(mission_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;

    #[test]
    fn test_vector_clock_dominates() {
        let mut vc1 = VectorClock::new();
        vc1.increment("a");
        vc1.increment("a");
        vc1.increment("b");

        let mut vc2 = VectorClock::new();
        vc2.increment("a");
        vc2.increment("b");

        assert!(vc1.dominates(&vc2));
        assert!(!vc2.dominates(&vc1));
    }

    #[tokio::test]
    async fn test_state_resolver_apply() {
        let lock = Arc::new(MemoryBus::new());
        let resolver = StateResolver::new("node1".to_string(), lock);

        let mut remote_clock = VectorClock::new();
        remote_clock.increment("node2");

        let handoff = proto::StateHandoff {
            mission_id: "m1".to_string(),
            tenant_id: "t1".to_string(),
            source_mode: 1,
            target_mode: 2,
            timestamp_ms: 1000,
            state_snapshot: vec![1, 2, 3],
        };

        assert!(resolver.apply_handoff(handoff, remote_clock).await.unwrap());
        assert_eq!(resolver.get_state("m1").await.unwrap(), vec![1, 2, 3]);
    }
}
