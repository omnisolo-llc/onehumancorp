
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::{HashMap, HashSet};
use crate::msgbus::{Bus, DistributedLock};
use crate::interop::protocol::{InteropProtocol, proto};
use tokio::time::{sleep, Duration};

/// The NetworkPartitionHealer detects split-brain scenarios and initiates reconciliation.
pub struct NetworkPartitionHealer {
    node_id: String,
    bus: Arc<dyn Bus>,
    lock: Arc<dyn DistributedLock>,

    // Track nodes we consider partitioned
    partitioned_nodes: Arc<RwLock<HashSet<String>>>,
    // Track topology generation to detect shifts
    topology_epoch: Arc<RwLock<u64>>,
}

impl NetworkPartitionHealer {
    pub fn new(node_id: String, bus: Arc<dyn Bus>, lock: Arc<dyn DistributedLock>) -> Self {
        Self {
            node_id,
            bus,
            lock,
            partitioned_nodes: Arc::new(RwLock::new(HashSet::new())),
            topology_epoch: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn mark_node_partitioned(&self, target_node: &str) {
        let mut nodes = self.partitioned_nodes.write().await;
        nodes.insert(target_node.to_string());

        let mut epoch = self.topology_epoch.write().await;
        *epoch += 1;
    }

    pub async fn is_partitioned(&self, target_node: &str) -> bool {
        let nodes = self.partitioned_nodes.read().await;
        nodes.contains(target_node)
    }

    pub async fn initiate_healing_sequence(&self, target_node: &str) -> Result<bool, String> {
        let lock_key = format!("healing_sequence:{}", target_node);
        if !self.lock.acquire_lock(&lock_key, &self.node_id, 60).await.unwrap_or(false) {
            return Ok(false); // Someone else is healing
        }

        // Complex healing logic goes here. For example, syncing missing state snapshots.
        // We will simulate a healing process.
        sleep(Duration::from_millis(10)).await;

        let mut nodes = self.partitioned_nodes.write().await;
        nodes.remove(target_node);

        let mut epoch = self.topology_epoch.write().await;
        *epoch += 1;

        let _ = self.lock.release_lock(&lock_key, &self.node_id).await;
        Ok(true)
    }
}

/// Fallback RPC client when message bus pub/sub is down.
pub struct FallbackRpcClient {
    endpoints: HashMap<String, String>,
}

impl FallbackRpcClient {
    pub fn new() -> Self {
        Self { endpoints: HashMap::new() }
    }

    pub fn register_endpoint(&mut self, node_id: &str, uri: &str) {
        self.endpoints.insert(node_id.to_string(), uri.to_string());
    }

    pub async fn invoke(&self, node_id: &str, _payload: &[u8]) -> Result<Vec<u8>, String> {
        let uri = self.endpoints.get(node_id).ok_or("Node endpoint unknown")?;
        // Simulating HTTP/gRPC invocation
        if uri.starts_with("http") {
            Ok(vec![200]) // Success
        } else {
            Err("Invalid URI scheme".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;

    #[tokio::test]
    async fn test_partition_healer() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let healer = NetworkPartitionHealer::new("node1".to_string(), bus, lock);

        assert!(!healer.is_partitioned("node2").await);

        healer.mark_node_partitioned("node2").await;
        assert!(healer.is_partitioned("node2").await);

        let result = healer.initiate_healing_sequence("node2").await.unwrap();
        assert!(result);

        assert!(!healer.is_partitioned("node2").await);
    }

    #[tokio::test]
    async fn test_fallback_rpc() {
        let mut rpc = FallbackRpcClient::new();
        rpc.register_endpoint("node_a", "http://10.0.0.1:8080");
        rpc.register_endpoint("node_b", "grpc://10.0.0.2:9090");

        let res1 = rpc.invoke("node_a", &[1,2,3]).await;
        assert!(res1.is_ok());

        let res2 = rpc.invoke("node_b", &[1,2,3]).await;
        assert!(res2.is_err());
    }
}
