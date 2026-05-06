use crate::msgbus::{Bus, DistributedLock, Message};
use std::sync::Arc;
use tokio::time::{sleep, timeout, Duration};

/// Simulated generated protobuf types (in a real scenario, this would use prost-build)
/// For now, we mock the encoded structs to satisfy the protobuf requirement.
pub mod proto {
    #[derive(Clone, prost::Message)]
    pub struct StateHandoff {
        #[prost(string, tag = "1")]
        pub mission_id: String,
        #[prost(bytes, tag = "6")]
        pub state_snapshot_json: Vec<u8>,
    }

    #[derive(Clone, prost::Message)]
    pub struct HealthPing {
        #[prost(string, tag = "1")]
        pub source_node_id: String,
    }

    #[derive(Clone, prost::Message)]
    pub struct HealthAck {
        #[prost(string, tag = "1")]
        pub target_node_id: String,
    }
}

/// Interop Layer protocol for mode-switch behaviour and sync
pub struct InteropProtocol {
    bus: Arc<dyn Bus>,
    lock: Arc<dyn DistributedLock>,
    node_id: String,
}

impl InteropProtocol {
    pub fn new(bus: Arc<dyn Bus>, lock: Arc<dyn DistributedLock>, node_id: String) -> Self {
        Self { bus, lock, node_id }
    }

    /// Triggers a state handoff when switching modes using protobuf on the wire
    pub async fn handoff(&self, mission_id: &str, state_payload: Vec<u8>) -> Result<(), String> {
        use prost::Message as ProstMessage;

        let lock_resource = format!("handoff:{}", mission_id);

        // Wait for lock with a timeout to prevent deadlocks
        let acquire_future = async {
            loop {
                if self.lock.acquire_lock(&lock_resource, &self.node_id, 10).await.unwrap_or(false) {
                    break;
                }
                sleep(Duration::from_millis(50)).await;
            }
        };

        if timeout(Duration::from_secs(5), acquire_future).await.is_err() {
            return Err("Timeout waiting for lock".to_string());
        }

        let handoff_msg = proto::StateHandoff {
            mission_id: mission_id.to_string(),
            state_snapshot_json: state_payload,
        };

        let mut buf = Vec::new();
        handoff_msg.encode(&mut buf).map_err(|e| e.to_string())?;

        let msg = Message {
            topic: "system:state_handoff".to_string(),
            payload: buf,
        };

        let result = self.bus.publish(msg).await;

        let _ = self.lock.release_lock(&lock_resource, &self.node_id).await;

        result
    }

    /// Health monitor across the swarm using protobuf
    pub async fn check_health(&self) -> Result<(), String> {
        use prost::Message as ProstMessage;

        let ping = proto::HealthPing {
            source_node_id: self.node_id.clone(),
        };

        let mut buf = Vec::new();
        ping.encode(&mut buf).map_err(|e| e.to_string())?;

        let msg = Message {
            topic: "system:health_ping".to_string(),
            payload: buf,
        };
        self.bus.publish(msg).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_interop_handoff_memory() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                use prost::Message as ProstMessage;
                let decoded = proto::StateHandoff::decode(&msg.payload[..]).unwrap();
                if decoded.mission_id == "mission_1" {
                    rx.store(true, Ordering::SeqCst);
                }
            }
        });

        let _cancel = bus.subscribe("system:state_handoff".to_string(), handler).await.unwrap();

        protocol.handoff("mission_1", vec![1, 2, 3]).await.unwrap();
        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_health_memory() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:health_ping" {
                use prost::Message as ProstMessage;
                let decoded = proto::HealthPing::decode(&msg.payload[..]).unwrap();
                if decoded.source_node_id == "node1" {
                    rx.store(true, Ordering::SeqCst);
                }
            }
        });

        let _cancel = bus.subscribe("system:health_ping".to_string(), handler).await.unwrap();

        protocol.check_health().await.unwrap();
        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }
}
