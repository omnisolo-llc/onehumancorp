use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CRDTState {
    pub vector_clock: HashMap<String, u64>,
    pub data: Vec<u8>,
}

pub struct StateConflictResolver {
    pub node_id: String,
}

impl StateConflictResolver {
    pub fn new(node_id: String) -> Self {
        Self { node_id }
    }

    pub fn resolve_conflict(&self, local: &CRDTState, remote: &CRDTState) -> CRDTState {
        // Compare vector clocks to find the causal history
        let mut resolved_clock = HashMap::new();

        let local_keys: std::collections::HashSet<_> = local.vector_clock.keys().cloned().collect();
        let remote_keys: std::collections::HashSet<_> = remote.vector_clock.keys().cloned().collect();
        let all_keys: std::collections::HashSet<_> = local_keys.union(&remote_keys).cloned().collect();

        let mut is_local_newer = false;
        let mut is_remote_newer = false;

        for key in all_keys {
            let local_val = local.vector_clock.get(&key).unwrap_or(&0);
            let remote_val = remote.vector_clock.get(&key).unwrap_or(&0);

            resolved_clock.insert(key.clone(), std::cmp::max(*local_val, *remote_val));

            if local_val > remote_val {
                is_local_newer = true;
            } else if remote_val > local_val {
                is_remote_newer = true;
            }
        }

        // Concurrent updates (conflict) or strictly one is newer
        if is_local_newer && !is_remote_newer {
            return local.clone();
        } else if is_remote_newer && !is_local_newer {
            return remote.clone();
        }

        // Concurrent: simple LWW on data size/hash or deterministic merge for this example
        // In reality, this would merge the specific application state CRDTs
        let data = if local.data.len() > remote.data.len() {
            local.data.clone()
        } else {
            remote.data.clone()
        };

        CRDTState {
            vector_clock: resolved_clock,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crdt_resolve_local_newer() {
        let resolver = StateConflictResolver::new("node1".to_string());

        let mut local_vc = HashMap::new();
        local_vc.insert("node1".to_string(), 2);
        let local = CRDTState { vector_clock: local_vc, data: vec![1, 2] };

        let mut remote_vc = HashMap::new();
        remote_vc.insert("node1".to_string(), 1);
        let remote = CRDTState { vector_clock: remote_vc, data: vec![1] };

        let resolved = resolver.resolve_conflict(&local, &remote);
        assert_eq!(resolved, local);
    }

    #[test]
    fn test_crdt_resolve_remote_newer() {
        let resolver = StateConflictResolver::new("node1".to_string());

        let mut local_vc = HashMap::new();
        local_vc.insert("node1".to_string(), 1);
        let local = CRDTState { vector_clock: local_vc, data: vec![1] };

        let mut remote_vc = HashMap::new();
        remote_vc.insert("node1".to_string(), 2);
        let remote = CRDTState { vector_clock: remote_vc, data: vec![1, 2] };

        let resolved = resolver.resolve_conflict(&local, &remote);
        assert_eq!(resolved, remote);
    }

    #[test]
    fn test_crdt_resolve_concurrent() {
        let resolver = StateConflictResolver::new("node1".to_string());

        let mut local_vc = HashMap::new();
        local_vc.insert("node1".to_string(), 2);
        local_vc.insert("node2".to_string(), 1);
        let local = CRDTState { vector_clock: local_vc, data: vec![1, 2, 3] };

        let mut remote_vc = HashMap::new();
        remote_vc.insert("node1".to_string(), 1);
        remote_vc.insert("node2".to_string(), 2);
        let remote = CRDTState { vector_clock: remote_vc, data: vec![1] };

        let resolved = resolver.resolve_conflict(&local, &remote);
        assert_eq!(resolved.data, vec![1, 2, 3]); // Local won because larger data
        assert_eq!(*resolved.vector_clock.get("node1").unwrap(), 2);
        assert_eq!(*resolved.vector_clock.get("node2").unwrap(), 2);
    }
}
