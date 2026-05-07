use crate::msgbus::{Bus, DistributedLock, Message};
use std::sync::Arc;
use tokio::time::{sleep, timeout, Duration};

/// Simulated generated protobuf types (in a real scenario, this would use prost-build)
/// For now, we mock the encoded structs to satisfy the protobuf requirement.
pub use crate::ohc::interop as proto;

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
    pub async fn handoff(&self, mission_id: &str, tenant_id: &str, state_payload: Vec<u8>) -> Result<(), String> {
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
            tenant_id: tenant_id.to_string(),
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            state_snapshot_json: state_payload,
            source_mode: 0,
            target_mode: 0,
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

    /// Listens for HealthPings and sends HealthAcks
    pub async fn listen_for_pings(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let node_id = self.node_id.clone();
        let bus = self.bus.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:health_ping" {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = proto::HealthPing::decode(&msg.payload[..]) {
                    let ack = proto::HealthAck {
                        target_node_id: decoded.source_node_id.clone(),
                        source_node_id: node_id.clone(),
                        timestamp_ms: chrono::Utc::now().timestamp_millis(),
                    };
                    let mut buf = Vec::new();
                    if ack.encode(&mut buf).is_ok() {
                        let ack_msg = Message {
                            topic: format!("system:health_ack:{}", decoded.source_node_id),
                            payload: buf,
                        };
                        let bus_clone = bus.clone();
                        tokio::spawn(async move {
                            let _ = bus_clone.publish(ack_msg).await;
                        });
                    }
                }
            }
        });

        self.bus.subscribe("system:health_ping".to_string(), handler).await
    }

    /// Health monitor across the swarm using protobuf
    pub async fn check_health(&self, timeout_ms: u64) -> Result<bool, String> {
        use prost::Message as ProstMessage;
        use std::sync::atomic::{AtomicBool, Ordering};

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:health_ack:{}", self.node_id);
        let handler = Box::new(move |msg: Message| {
            if msg.topic == ack_topic {
                rx.store(true, Ordering::SeqCst);
            }
        });

        let cancel = self.bus.subscribe(format!("system:health_ack:{}", self.node_id), handler).await?;

        let ping = proto::HealthPing {
            source_node_id: self.node_id.clone(),
            current_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        let mut buf = Vec::new();
        ping.encode(&mut buf).map_err(|e| e.to_string())?;

        let msg = Message {
            topic: "system:health_ping".to_string(),
            payload: buf,
        };
        self.bus.publish(msg).await?;

        // Wait for up to timeout_ms
        let start = std::time::Instant::now();
        while start.elapsed().as_millis() < timeout_ms as u128 {
            if received.load(Ordering::SeqCst) {
                cancel();
                return Ok(true);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        cancel();
        Ok(false)
    }

    /// Dispatches a background job and waits for acknowledgment
    pub async fn dispatch_job(&self, job_id: &str, tenant_id: &str, action_name: &str, payload: Vec<u8>, timeout_ms: u64) -> Result<bool, String> {
        use prost::Message as ProstMessage;
        use std::sync::atomic::{AtomicBool, Ordering};

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:job_ack:{}", job_id);
        let handler = Box::new(move |msg: Message| {
            if msg.topic == ack_topic {
                rx.store(true, Ordering::SeqCst);
            }
        });

        let cancel = self.bus.subscribe(format!("system:job_ack:{}", job_id), handler).await?;

        let dispatch = proto::JobDispatch {
            job_id: job_id.to_string(),
            tenant_id: tenant_id.to_string(),
            action_name: action_name.to_string(),
            payload_json: payload,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        let mut buf = Vec::new();
        dispatch.encode(&mut buf).map_err(|e| e.to_string())?;

        let msg = Message {
            topic: format!("system:job_dispatch:{}", tenant_id),
            payload: buf,
        };

        // Add internal retry for publishing to ensure dispatch survives partitions
        let mut retries = 0;
        loop {
            match self.bus.publish(msg.clone()).await {
                Ok(_) => break,
                Err(e) => {
                    if retries >= 3 {
                        cancel();
                        return Err(format!("Failed to publish job dispatch after retries: {}", e));
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * retries)).await;
                }
            }
        }

        // Wait for up to timeout_ms
        let start = std::time::Instant::now();
        while start.elapsed().as_millis() < timeout_ms as u128 {
            if received.load(Ordering::SeqCst) {
                cancel();
                return Ok(true);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        cancel();
        Ok(false) // Not acked, implies failure/timeout, dispatch might need to be retried by the caller
    }

    /// Listens for job dispatches and acknowledges them
    pub async fn listen_for_jobs(&self, tenant_id: &str) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let node_id = self.node_id.clone();
        let bus = self.bus.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic.starts_with("system:job_dispatch:") {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = proto::JobDispatch::decode(&msg.payload[..]) {
                    // In a real implementation, we would process the job here or send it to a worker pool
                    // Here, we just acknowledge receipt
                    let ack = proto::JobAck {
                        job_id: decoded.job_id.clone(),
                        node_id: node_id.clone(),
                        timestamp_ms: chrono::Utc::now().timestamp_millis(),
                    };
                    let mut buf = Vec::new();
                    if ack.encode(&mut buf).is_ok() {
                        let ack_msg = Message {
                            topic: format!("system:job_ack:{}", decoded.job_id),
                            payload: buf,
                        };
                        let bus_clone = bus.clone();
                        tokio::spawn(async move {
                            // Retry mechanism to ensure ACK reaches the dispatcher
                            let mut retries = 0;
                            while retries < 3 {
                                if bus_clone.publish(ack_msg.clone()).await.is_ok() {
                                    break;
                                }
                                retries += 1;
                                tokio::time::sleep(tokio::time::Duration::from_millis(50 * retries)).await;
                            }
                        });
                    }
                }
            }
        });

        self.bus.subscribe(format!("system:job_dispatch:{}", tenant_id), handler).await
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

        protocol.handoff("mission_1", "tenant_1", vec![1, 2, 3]).await.unwrap();
        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_health_memory() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol1 = InteropProtocol::new(bus.clone(), lock.clone(), "node1".to_string());
        let protocol2 = InteropProtocol::new(bus.clone(), lock.clone(), "node2".to_string());

        // node2 listens for pings
        let _cancel2 = protocol2.listen_for_pings().await.unwrap();

        // node1 checks health with timeout
        let is_healthy = protocol1.check_health(500).await.unwrap();

        assert!(is_healthy);
    }

    #[tokio::test]
    async fn test_interop_job_dispatch() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_server = InteropProtocol::new(bus.clone(), lock.clone(), "server".to_string());
        let protocol_agent = InteropProtocol::new(bus.clone(), lock.clone(), "agent".to_string());

        // agent listens for jobs on tenant "tenant_a"
        let _cancel_jobs = protocol_agent.listen_for_jobs("tenant_a").await.unwrap();

        // server dispatches job to tenant "tenant_a"
        let is_acked = protocol_server.dispatch_job("job_1", "tenant_a", "do_work", vec![42], 500).await.unwrap();

        assert!(is_acked);
    }
}
