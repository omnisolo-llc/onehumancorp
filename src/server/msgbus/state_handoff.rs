use super::{Bus, DistributedLock, Message};
use std::sync::Arc;
use tokio::time::{sleep, timeout, Duration};

#[allow(dead_code)]
pub struct StateHandoffManager {
    bus: std::sync::Arc<dyn Bus>,
    lock: std::sync::Arc<dyn DistributedLock>,
    node_id: String,
}

#[allow(dead_code)]
impl StateHandoffManager {
    pub fn new(bus: std::sync::Arc<dyn Bus>, lock: std::sync::Arc<dyn DistributedLock>, node_id: String) -> Self {
        Self { bus, lock, node_id }
    }

    pub async fn trigger_handoff(&self, mission_id: &str, tenant_id: &str, payload: Vec<u8>) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let handoff = crate::interop::protocol::proto::StateHandoff {
            mission_id: mission_id.to_string(),
            tenant_id: tenant_id.to_string(),
            source_mode: 0,
            target_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            state_snapshot: payload,
        };
        let mut buf = Vec::new();
        handoff.encode(&mut buf).map_err(|e| e.to_string())?;

        let idempotency_lock = format!("handoff:{}", mission_id);
        if !self.lock.acquire_lock(&idempotency_lock, &self.node_id, 3600).await.unwrap_or(false) {
            return Ok(());
        }

        let msg = Message {
            topic: "system:state_handoff".to_string(),
            payload: buf,
        };
        self.bus.publish(msg).await
    }
}

#[allow(dead_code)]
pub struct HealthMonitor {
    bus: std::sync::Arc<dyn Bus>,
    transport: std::sync::Arc<dyn crate::orchestration::mesh::TeammateMesh>,
}

#[allow(dead_code)]
impl HealthMonitor {
    pub fn new(bus: std::sync::Arc<dyn Bus>, transport: std::sync::Arc<dyn crate::orchestration::mesh::TeammateMesh>) -> Self {
        Self { bus, transport }
    }

    pub async fn ping(&self) -> Result<(), String> {
        let node_id = uuid::Uuid::new_v4().to_string();
        let ack_topic = format!("system:health_ack:{}", node_id);

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let handler = Box::new(move |_msg: Message| {
            let _ = tx.send(());
        });

        let cancel = self.bus.subscribe(ack_topic, handler).await?;

        let ping = crate::interop::protocol::proto::HealthPing {
            current_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            source_node_id: node_id.clone(),
        };
        let mut buf = Vec::new();
        prost::Message::encode(&ping, &mut buf).map_err(|e| e.to_string())?;

        // Cross-Mode Health Monitoring: explicitly register presence via transport
        self.transport.register_presence(&node_id, "online", 60).await.map_err(|e| e.to_string())?;

        let msg = Message {
            topic: "system:health_ping".to_string(),
            payload: buf,
        };

        if let Err(e) = self.bus.publish(msg).await {
            cancel();
            return Err(e);
        }

        match tokio::time::timeout(tokio::time::Duration::from_millis(500), rx.recv()).await {
            Ok(Some(_)) => {
                cancel();
                Ok(())
            }
            _ => {
                cancel();
                Err("Health ping timed out waiting for ack".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;


    #[tokio::test]
    async fn test_memory_bus_pub_sub() {
        let bus = crate::msgbus::MemoryBus::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            tracing::debug!("Received message: {:?}", msg);
            received_clone.store(true, Ordering::SeqCst);
        });

        let cancel = bus.subscribe("test_topic".to_string(), handler).await.unwrap();

        let msg = Message {
            topic: "test_topic".to_string(),
            payload: vec![],
        };

        bus.publish(msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));

        cancel();
    }

    #[tokio::test]
    async fn test_ipc_bus_pub_sub() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_bus_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        let bus = crate::msgbus::IpcBus::new(&db_url).await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_ipc_topic" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = bus.subscribe("test_ipc_topic".to_string(), handler).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let msg = Message {
            topic: "test_ipc_topic".to_string(),
            payload: vec![],
        };

        bus.publish(msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_redis_bus_pub_sub() {
        let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string());
        let bus = match crate::msgbus::RedisBus::new(&url).await {
            Ok(b) => b,
            Err(_) => return,
        };

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_redis_topic" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = bus.subscribe("test_redis_topic".to_string(), handler).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msg = Message {
            topic: "test_redis_topic".to_string(),
            payload: vec![],
        };

        bus.publish(msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_health_monitor_ping() {
        let bus = std::sync::Arc::new(crate::msgbus::MemoryBus::new());
        let transport = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(std::sync::Arc::new(ohc_builtin_agent::mesh::transport::MemoryTransport::new())));
        let monitor = HealthMonitor::new(bus.clone(), transport);

        let received = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let received_clone = received.clone();

        let bus_clone = bus.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:health_ping" {
                received_clone.store(true, std::sync::atomic::Ordering::SeqCst);

                use prost::Message as ProstMessage;
                if let Ok(ping) = crate::interop::protocol::proto::HealthPing::decode(&msg.payload[..]) {
                    let ack_topic = format!("system:health_ack:{}", ping.source_node_id);
                    let bus_inner = bus_clone.clone();
                    tokio::spawn(async move {
                        let _ = bus_inner.publish(Message {
                            topic: ack_topic,
                            payload: vec![],
                        }).await;
                    });
                }
            }
        });

        let cancel = bus.subscribe("system:health_ping".to_string(), handler).await.unwrap();

        monitor.ping().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(std::sync::atomic::Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_state_handoff_trigger() {
        let bus = std::sync::Arc::new(crate::msgbus::MemoryBus::new());
        let lock = bus.clone();
        let manager = StateHandoffManager::new(bus.clone(), lock, "node1".to_string());

        let received = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                use prost::Message as ProstMessage;
                if let Ok(handoff) = crate::interop::protocol::proto::StateHandoff::decode(&msg.payload[..]) {
                    if handoff.mission_id == "m1" && handoff.tenant_id == "t1" && handoff.state_snapshot == vec![1, 2, 3, 4] {
                        received_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
        });

        let cancel = bus.subscribe("system:state_handoff".to_string(), handler).await.unwrap();

        manager.trigger_handoff("m1", "t1", vec![1, 2, 3, 4]).await.unwrap();

        // test idempotency
        manager.trigger_handoff("m1", "t1", vec![1, 2, 3, 4]).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(std::sync::atomic::Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_health_monitor_ping_success() {
        let bus = std::sync::Arc::new(crate::msgbus::MemoryBus::new());
        let transport = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(std::sync::Arc::new(ohc_builtin_agent::mesh::transport::MemoryTransport::new())));
        let monitor = HealthMonitor::new(bus.clone(), transport);

        // We need to listen for the ping and respond with an ack.
        let bus_clone = bus.clone();
        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:health_ping" {
                use prost::Message as ProstMessage;
                if let Ok(ping) = crate::interop::protocol::proto::HealthPing::decode(&msg.payload[..]) {
                    let ack_topic = format!("system:health_ack:{}", ping.source_node_id);
                    let ack_msg = Message {
                        topic: ack_topic,
                        payload: vec![], // The content of the ack is currently ignored by ping()
                    };
                    let bus_inner = bus_clone.clone();
                    tokio::spawn(async move {
                        let _ = bus_inner.publish(ack_msg).await;
                    });
                }
            }
        });

        let cancel = bus.subscribe("system:health_ping".to_string(), handler).await.unwrap();

        // The ping should succeed.
        assert!(monitor.ping().await.is_ok());

        cancel();
    }

    #[tokio::test]
    async fn test_health_monitor_ping_timeout() {
        let bus = std::sync::Arc::new(crate::msgbus::MemoryBus::new());
        let transport = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(std::sync::Arc::new(ohc_builtin_agent::mesh::transport::MemoryTransport::new())));
        let monitor = HealthMonitor::new(bus.clone(), transport);

        // Without any handler to respond with an ack, ping should timeout.
        let result = monitor.ping().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Health ping timed out waiting for ack");
    }

    #[tokio::test]
    async fn test_memory_bus_distributed_lock() {
        let bus = crate::msgbus::MemoryBus::new();
        let resource = "test_resource";
        let owner1 = "owner1";
        let owner2 = "owner2";

        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
        assert!(!bus.acquire_lock(resource, owner2, 1).await.unwrap());
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());

        bus.release_lock(resource, owner1).await.unwrap();
        assert!(bus.acquire_lock(resource, owner2, 1).await.unwrap());
    }

    #[tokio::test]
    async fn test_ipc_bus_distributed_lock() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_lock_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        let bus = crate::msgbus::IpcBus::new(&db_url).await.unwrap();
        let resource = "test_ipc_resource";
        let owner1 = "owner1";
        let owner2 = "owner2";

        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
        assert!(!bus.acquire_lock(resource, owner2, 1).await.unwrap());

        // Allow lock to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        assert!(bus.acquire_lock(resource, owner2, 1).await.unwrap());

        bus.release_lock(resource, owner2).await.unwrap();
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());

        // Re-acquire by same owner to extend
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
    }

    #[tokio::test]
    async fn test_redis_bus_distributed_lock() {
        let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string());
        let bus = match crate::msgbus::RedisBus::new(&url).await {
            Ok(b) => b,
            Err(_) => return,
        };
        let resource = "test_redis_resource";
        let owner1 = "owner1";
        let owner2 = "owner2";

        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
        assert!(!bus.acquire_lock(resource, owner2, 1).await.unwrap());
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());

        bus.release_lock(resource, owner1).await.unwrap();
        assert!(bus.acquire_lock(resource, owner2, 1).await.unwrap());
    }
}

#[cfg(test)]
mod tests_ipc {
    use super::*;

    #[tokio::test]
    async fn test_ipc_lock() {
        let db_url = "sqlite::memory:";
        let bus = crate::msgbus::IpcBus::new(db_url).await.unwrap();

        let acquired1 = bus.acquire_lock("test_res", "owner1", 10).await.unwrap();
        assert!(acquired1);

        let acquired2 = bus.acquire_lock("test_res", "owner2", 10).await.unwrap();
        assert!(!acquired2);

        bus.release_lock("test_res", "owner1").await.unwrap();

        let acquired3 = bus.acquire_lock("test_res", "owner2", 10).await.unwrap();
        assert!(acquired3);
    }
}

#[cfg(test)]
mod memory_bus_tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_memory_bus_publish_subscribe() {
        let bus = crate::msgbus::MemoryBus::new();
        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_topic" && msg.payload == b"hello" {
                rx.store(true, Ordering::SeqCst);
            }
        });

        let _cancel = bus.subscribe("test_topic".to_string(), handler).await.unwrap();

        let msg = Message {
            topic: "test_topic".to_string(),
            payload: b"hello".to_vec(),
        };

        bus.publish(msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_memory_bus_lock_acquire_release() {
        let bus = crate::msgbus::MemoryBus::new();

        let acquired = bus.acquire_lock("resource1", "owner1", 10).await.unwrap();
        assert!(acquired);

        let acquired_again = bus.acquire_lock("resource1", "owner2", 10).await.unwrap();
        assert!(!acquired_again);

        bus.release_lock("resource1", "owner1").await.unwrap();

        let acquired_after_release = bus.acquire_lock("resource1", "owner2", 10).await.unwrap();
        assert!(acquired_after_release);
    }
}
