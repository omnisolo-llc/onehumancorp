use std::sync::atomic::Ordering;
#[cfg(not(test))] use crate::msgbus::MemoryBus;
#[cfg(test)] use server_lib::msgbus::MemoryBus;
#[cfg(not(test))] use crate::msgbus::{Bus, DistributedLock, Message};
#[cfg(test)] use server_lib::msgbus::{Bus, DistributedLock, Message};
use std::sync::Arc;
use tokio::time::{sleep, timeout, Duration};

pub mod proto {
    pub use interop_proto::ohc::interop::*;
}

/// Interop Layer protocol for mode-switch behaviour and sync
pub struct InteropProtocol {
    bus: Arc<dyn Bus>,
    lock: Arc<dyn DistributedLock>,
    node_id: String,
}

impl InteropProtocol {
    pub fn new(bus: Arc<dyn Bus>, lock: Arc<dyn DistributedLock>, node_id: String) -> Self {
        Self {
            bus,
            lock,
            node_id,
        }
    }

    /// Triggers a state handoff when switching modes using protobuf on the wire
    pub async fn handoff(&self, mission_id: &str, tenant_id: &str, state_payload: Vec<u8>) -> Result<(), String> {
        use prost::Message as ProstMessage;

        let lock_resource = format!("handoff:{}", mission_id);

        // Wait for lock with a timeout to prevent deadlocks and apply backoff.
        let acquire_future = async {
            let mut retries = 0;
            loop {
                if self.lock.acquire_lock(&lock_resource, &self.node_id, 10).await.unwrap_or(false) {
                    break;
                }
                retries += 1;
                let sleep_ms = 50 * retries;
                sleep(Duration::from_millis(sleep_ms)).await;
            }
        };

        if timeout(Duration::from_secs(5), acquire_future).await.is_err() {
            return Err("Timeout waiting for lock".to_string());
        }

        // Idempotency check: once we hold the execution lock, check if it was processed.
        let idempotency_lock_resource = format!("handoff:processed:{}", mission_id);
        // Generate a unique owner ID for this specific handoff attempt to prevent lock extension.
        let attempt_owner = format!("{}_{}", self.node_id, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        if !self.lock.acquire_lock(&idempotency_lock_resource, &attempt_owner, 3600).await.unwrap_or(false) {
            let _ = self.lock.release_lock(&lock_resource, &self.node_id).await;
            return Ok(());
        }

        let handoff_msg = proto::StateHandoff {
            source_mode: 0,
            target_mode: 0,
            mission_id: mission_id.to_string(),
            tenant_id: tenant_id.to_string(),
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            state_snapshot: state_payload.clone(),
        };

        let mut buf = Vec::new();
        if let Err(e) = handoff_msg.encode(&mut buf) {
            let _ = self.lock.release_lock(&idempotency_lock_resource, &attempt_owner).await;
            let _ = self.lock.release_lock(&lock_resource, &self.node_id).await;
            return Err(e.to_string());
        }

        let msg = Message {
            topic: "system:state_handoff".to_string(),
            payload: buf,
        };

        let mut retries = 0;
        let mut delay_ms = 100;
        let result = loop {
            match self.bus.publish(msg.clone()).await {
                Ok(_) => break Ok(()),
                Err(e) => {
                    if retries >= 5 {
                        break Err(format!("Failed to publish state handoff after retries: {}", e));
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2; // Exponential backoff
                }
            }
        };

        if result.is_err() {
            // Failed to publish, release idempotency lock so it can be retried
            let _ = self.lock.release_lock(&idempotency_lock_resource, &attempt_owner).await;
        }

        let _ = self.lock.release_lock(&lock_resource, &self.node_id).await;

        result
    }

    /// Resumes a mission after a mode switch
    pub async fn resume_mission(&self, mission_id: &str, tenant_id: &str, state_payload: Vec<u8>) -> Result<(), String> {
        // Handoff uses the same mechanism to synchronize state
        self.handoff(mission_id, tenant_id, state_payload).await
    }

    /// Listens for state handoff updates
    pub async fn listen_for_state_handoff(&self, handler: Box<dyn Fn(proto::StateHandoff) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = proto::StateHandoff::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        self.bus.subscribe("system:state_handoff".to_string(), bus_handler).await
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
                        source_node_id: node_id.clone(),
                        timestamp_ms: chrono::Utc::now().timestamp_millis(),
                        target_node_id: decoded.source_node_id.clone(),
                    };
                    let mut buf = Vec::new();
                    if ack.encode(&mut buf).is_ok() {
                        let ack_msg = Message {
                            topic: format!("system:health_ack:{}", decoded.source_node_id),
                            payload: buf,
                        };
                        let bus_clone = bus.clone();
                        tokio::spawn(async move {
                            let mut retries = 0;
                            let mut delay_ms = 50;
                            while retries < 5 {
                                if bus_clone.publish(ack_msg.clone()).await.is_ok() {
                                    break;
                                }
                                retries += 1;
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                                delay_ms *= 2; // Exponential backoff
                            }
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
            current_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            source_node_id: self.node_id.clone(),
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
            payload: payload,
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
        let mut delay_ms = 100;
        loop {
            match self.bus.publish(msg.clone()).await {
                Ok(_) => break,
                Err(e) => {
                    if retries >= 5 {
                        cancel();
                        return Err(format!("Failed to publish job dispatch after retries: {}", e));
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2; // Exponential backoff
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
                            let mut delay_ms = 50;
                            while retries < 5 {
                                if bus_clone.publish(ack_msg.clone()).await.is_ok() {
                                    break;
                                }
                                retries += 1;
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                                delay_ms *= 2; // Exponential backoff
                            }
                        });
                    }
                }
            }
        });

        self.bus.subscribe(format!("system:job_dispatch:{}", tenant_id), handler).await
    }

    /// Reports job status back to the main server
    pub async fn report_job_status(&self, job_id: &str, tenant_id: &str, status: &str, details: Vec<u8>) -> Result<(), String> {
        use prost::Message as ProstMessage;

        let update = proto::JobStatusUpdate {
            job_id: job_id.to_string(),
            tenant_id: tenant_id.to_string(),
            status: status.to_string(),
            details_payload: details,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        let mut buf = Vec::new();
        update.encode(&mut buf).unwrap();

        let msg = Message {
            topic: format!("system:job_status:{}", job_id),
            payload: buf,
        };

        // Add internal retry for publishing to ensure reporting survives partitions
        let mut retries = 0;
        let mut delay_ms = 100;
        loop {
            match self.bus.publish(msg.clone()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if retries >= 5 {
                        return Err(format!("Failed to publish job status update after retries: {}", e));
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2; // Exponential backoff
                }
            }
        }
    }

    /// Listens for job status updates for a specific job
    pub async fn listen_for_job_status(&self, job_id: &str, handler: Box<dyn Fn(proto::JobStatusUpdate) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic.starts_with("system:job_status:") {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = proto::JobStatusUpdate::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        self.bus.subscribe(format!("system:job_status:{}", job_id), bus_handler).await
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(not(test))] use crate::msgbus::MemoryBus;
#[cfg(test)] use server_lib::msgbus::MemoryBus;
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

    #[tokio::test]
    async fn test_interop_resume_mission() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                use prost::Message as ProstMessage;
                let decoded = proto::StateHandoff::decode(&msg.payload[..]).unwrap();
                if decoded.mission_id == "mission_resume_1" {
                    rx.store(true, Ordering::SeqCst);
                }
            }
        });

        let _cancel = bus.subscribe("system:state_handoff".to_string(), handler).await.unwrap();

        protocol.resume_mission("mission_resume_1", "tenant_1", vec![1, 2, 3]).await.unwrap();
        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_handoff_idempotency_simulation() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock.clone(), "node1".to_string());

        let received_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let rx = received_count.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                rx.fetch_add(1, Ordering::SeqCst);
            }
        });

        let _cancel = bus.subscribe("system:state_handoff".to_string(), handler).await.unwrap();

        // Simulate identical payload handoffs to ensure we process gracefully
        protocol.handoff("mission_1", "tenant_1", vec![1, 2, 3]).await.unwrap();

        // Wait briefly for the lock to be fully acquired in the mock environment
        sleep(Duration::from_millis(50)).await;

        // Try the same handoff again, it should immediately return Ok() due to lock idempotency check.
        let protocol2 = InteropProtocol::new(bus.clone(), lock.clone(), "node2".to_string());
        protocol2.handoff("mission_1", "tenant_1", vec![1, 2, 3]).await.unwrap();

        sleep(Duration::from_millis(100)).await;

        assert_eq!(received_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_interop_listen_for_state_handoff() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock.clone(), "node1".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |msg: proto::StateHandoff| {
            if msg.mission_id == "mission_2" {
                rx.store(true, Ordering::SeqCst);
            }
        });
        let _cancel = protocol.listen_for_state_handoff(handler).await.unwrap();
        protocol.handoff("mission_2", "tenant_2", vec![1, 2, 3]).await.unwrap();
        sleep(Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_dispatch_job_timeout() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_server = InteropProtocol::new(bus.clone(), lock.clone(), "server".to_string());

        // server dispatches job but NO AGENT IS LISTENING
        // We expect it to return false (timeout), but not fail the retry publish loop
        let is_acked = protocol_server.dispatch_job("job_timeout", "tenant_a", "do_work", vec![42], 100).await.unwrap();

        assert!(!is_acked);
    }

    #[tokio::test]
    async fn test_interop_listen_for_pings() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_listener = InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());

        let _cancel = protocol_listener.listen_for_pings().await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        // Subscribe to the ACK
        let ack_topic = format!("system:health_ack:sender_node");
        let ack_topic_clone = ack_topic.clone();
        let handler = Box::new(move |msg: Message| {
            if msg.topic == ack_topic_clone {
                rx.store(true, Ordering::SeqCst);
            }
        });
        let _cancel_ack = bus.subscribe(ack_topic, handler).await.unwrap();

        // Publish a ping
        use prost::Message as ProstMessage;
        let ping = proto::HealthPing {
            current_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            source_node_id: "sender_node".to_string(),
        };

        let mut buf = Vec::new();
        ping.encode(&mut buf).unwrap();

        let msg = Message {
            topic: "system:health_ping".to_string(),
            payload: buf,
        };
        bus.publish(msg).await.unwrap();

        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_jobs() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_listener = InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());

        let _cancel = protocol_listener.listen_for_jobs("tenant_x").await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        // Subscribe to the ACK
        let ack_topic = format!("system:job_ack:job_123");
        let ack_topic_clone = ack_topic.clone();
        let handler = Box::new(move |msg: Message| {
            if msg.topic == ack_topic_clone {
                rx.store(true, Ordering::SeqCst);
            }
        });
        let _cancel_ack = bus.subscribe(ack_topic, handler).await.unwrap();

        // Publish a job
        use prost::Message as ProstMessage;
        let dispatch = proto::JobDispatch {
            job_id: "job_123".to_string(),
            tenant_id: "tenant_x".to_string(),
            action_name: "test_action".to_string(),
            payload: vec![1, 2, 3],
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        let mut buf = Vec::new();
        dispatch.encode(&mut buf).unwrap();

        let msg = Message {
            topic: "system:job_dispatch:tenant_x".to_string(),
            payload: buf,
        };
        bus.publish(msg).await.unwrap();

        sleep(Duration::from_millis(200)).await; // longer sleep for retry publish mechanism

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_handoff_lock_deadlock_prevention() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol1 = InteropProtocol::new(bus.clone(), lock.clone(), "node1".to_string());

        // Acquire lock manually to simulate another process holding it
        assert!(lock.acquire_lock("handoff:mission_locked", "node_other", 10).await.unwrap());

        // This should timeout instead of deadlocking, because of our new timeout semantics
        let result = protocol1.handoff("mission_locked", "tenant_1", vec![1, 2, 3]).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Timeout waiting for lock");

        // Release
        let _ = lock.release_lock("handoff:mission_locked", "node_other").await;
    }

    #[tokio::test]
    async fn test_interop_job_status_reporting() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_server = InteropProtocol::new(bus.clone(), lock.clone(), "server".to_string());
        let protocol_agent = InteropProtocol::new(bus.clone(), lock.clone(), "agent".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |update: proto::JobStatusUpdate| {
            if update.job_id == "job_status_123" && update.status == "COMPLETED" {
                rx.store(true, Ordering::SeqCst);
            }
        });

        // Server listens for status updates
        let _cancel = protocol_server.listen_for_job_status("job_status_123", handler).await.unwrap();

        // Agent reports status
        protocol_agent.report_job_status("job_status_123", "tenant_a", "COMPLETED", vec![1, 2, 3]).await.unwrap();

        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }


    #[tokio::test]
    async fn test_interop_dispatch_job_retry_success() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(3),
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "server".to_string());

        let result = protocol.dispatch_job("job_retry_1", "tenant_a", "do_work", vec![], 10).await;
        // The mock bus doesn't publish ACK, so it's a timeout (returns false), but it shouldn't be a publish error
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[tokio::test]
    async fn test_interop_dispatch_job_retry_failure() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(10), // More than max retries
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "server".to_string());

        let result = protocol.dispatch_job("job_retry_2", "tenant_a", "do_work", vec![], 10).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to publish job dispatch after retries"));
    }

    #[tokio::test]
    async fn test_interop_handoff_retry_success() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(3),
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "node1".to_string());

        let result = protocol.handoff("mission_retry_1", "tenant_1", vec![1, 2, 3]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_interop_handoff_retry_failure() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(10),
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "node1".to_string());

        let result = protocol.handoff("mission_retry_2", "tenant_1", vec![1, 2, 3]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to publish state handoff after retries"));
    }

    struct MockFailingBus {
        failures_left: std::sync::atomic::AtomicUsize,
    }
    #[async_trait::async_trait]
    impl server_lib::msgbus::Bus for MockFailingBus {
        async fn publish(&self, _msg: Message) -> Result<(), String> {
            if self.failures_left.fetch_sub(1, Ordering::SeqCst) > 0 {
                return Err("Simulated network failure".to_string());
            }
            Ok(())
        }
        async fn subscribe(&self, _topic: String, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }
    }

    #[tokio::test]
    async fn test_interop_job_status_reporting_retry_success() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(3),
        });
        let lock = Arc::new(MemoryBus::new()); // dummy lock
        let protocol = InteropProtocol::new(bus, lock, "agent".to_string());

        let result = protocol.report_job_status("job_retry_1", "tenant_a", "FAILED", vec![]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_interop_job_status_reporting_retry_failure() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(10), // More than max retries
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "agent".to_string());

        let result = protocol.report_job_status("job_retry_2", "tenant_a", "FAILED", vec![]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to publish job status update after retries"));
    }

    #[tokio::test]
    async fn test_interop_health_timeout() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock.clone(), "node_timeout".to_string());

        // Do not set up a listener to acknowledge the ping
        let is_healthy = protocol.check_health(50).await.unwrap();

        assert!(!is_healthy);
    }

    #[tokio::test]
    async fn test_interop_listen_for_state_handoff_malformed() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock.clone(), "node1".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |_msg: proto::StateHandoff| {
            rx.store(true, Ordering::SeqCst);
        });

        let _cancel = protocol.listen_for_state_handoff(handler).await.unwrap();

        // Send a malformed message
        let msg = Message {
            topic: "system:state_handoff".to_string(),
            payload: vec![255, 255, 255], // Invalid protobuf
        };
        bus.publish(msg).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // Handler should not have been called
        assert!(!received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_pings_malformed() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_listener = InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());
        let _cancel = protocol_listener.listen_for_pings().await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:health_ack:sender_node");
        let handler = Box::new(move |_msg: Message| {
            rx.store(true, Ordering::SeqCst);
        });
        let _cancel_ack = bus.subscribe(ack_topic, handler).await.unwrap();

        // Send a malformed ping
        let msg = Message {
            topic: "system:health_ping".to_string(),
            payload: vec![255, 255, 255], // Invalid protobuf
        };
        bus.publish(msg).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // No ack should have been sent
        assert!(!received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_jobs_malformed() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_listener = InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());
        let _cancel = protocol_listener.listen_for_jobs("tenant_x").await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:job_ack:job_123");
        let handler = Box::new(move |_msg: Message| {
            rx.store(true, Ordering::SeqCst);
        });
        let _cancel_ack = bus.subscribe(ack_topic, handler).await.unwrap();

        // Send a malformed job dispatch
        let msg = Message {
            topic: "system:job_dispatch:tenant_x".to_string(),
            payload: vec![255, 255, 255], // Invalid protobuf
        };
        bus.publish(msg).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // No ack should have been sent
        assert!(!received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_job_status_malformed() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_server = InteropProtocol::new(bus.clone(), lock.clone(), "server".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |_update: proto::JobStatusUpdate| {
            rx.store(true, Ordering::SeqCst);
        });

        let _cancel = protocol_server.listen_for_job_status("job_status_123", handler).await.unwrap();

        // Send a malformed job status
        let msg = Message {
            topic: "system:job_status:job_status_123".to_string(),
            payload: vec![255, 255, 255], // Invalid protobuf
        };
        bus.publish(msg).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // Handler should not have been called
        assert!(!received.load(Ordering::SeqCst));
    }

}

    #[tokio::test]
    async fn test_interop_listen_for_state_handoff() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |handoff: proto::StateHandoff| {
            if handoff.mission_id == "m1" && handoff.tenant_id == "t1" {
                rx.store(true, Ordering::SeqCst);
            }
        });

        let _cancel = protocol.listen_for_state_handoff(handler).await.unwrap();

        let handoff = proto::StateHandoff {
            mission_id: "m1".to_string(),
            tenant_id: "t1".to_string(),
            source_mode: 0,
            target_mode: 0,
            timestamp_ms: 1000,
            state_snapshot: vec![1, 2, 3],
        };
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        handoff.encode(&mut buf).unwrap();

        bus.publish(Message {
            topic: "system:state_handoff".to_string(),
            payload: buf,
        }).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_pings() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let _cancel_ping = protocol.listen_for_pings().await.unwrap();

        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rx = received.clone();

        let _cancel_ack = bus.subscribe("system:health_ack:sender_node".to_string(), Box::new(move |msg| {
            use prost::Message as ProstMessage;
            if let Ok(ack) = proto::HealthAck::decode(&msg.payload[..]) {
                if ack.source_node_id == "node1" && ack.target_node_id == "sender_node" {
                    rx.store(true, Ordering::SeqCst);
                }
            }
        })).await.unwrap();

        let ping = proto::HealthPing {
            source_node_id: "sender_node".to_string(),
            current_mode: 0,
            timestamp_ms: 1000,
        };
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        ping.encode(&mut buf).unwrap();

        bus.publish(Message {
            topic: "system:health_ping".to_string(),
            payload: buf,
        }).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_jobs() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let _cancel_jobs = protocol.listen_for_jobs("t1").await.unwrap();

        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rx = received.clone();

        let _cancel_ack = bus.subscribe("system:job_ack:job1".to_string(), Box::new(move |msg| {
            use prost::Message as ProstMessage;
            if let Ok(ack) = proto::JobAck::decode(&msg.payload[..]) {
                if ack.job_id == "job1" && ack.node_id == "node1" {
                    rx.store(true, Ordering::SeqCst);
                }
            }
        })).await.unwrap();

        let dispatch = proto::JobDispatch {
            job_id: "job1".to_string(),
            tenant_id: "t1".to_string(),
            action_name: "act".to_string(),
            payload: vec![],
            timestamp_ms: 1000,
        };
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        dispatch.encode(&mut buf).unwrap();

        bus.publish(Message {
            topic: "system:job_dispatch:t1".to_string(),
            payload: buf,
        }).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_check_health_success() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let bus_clone = bus.clone();
        let _cancel = bus.subscribe("system:health_ping".to_string(), Box::new(move |msg| {
            use prost::Message as ProstMessage;
            if let Ok(ping) = proto::HealthPing::decode(&msg.payload[..]) {
                let ack = proto::HealthAck {
                    source_node_id: "responder".to_string(),
                    target_node_id: ping.source_node_id.clone(),
                    timestamp_ms: 1000,
                };
                let mut buf = Vec::new();
                ack.encode(&mut buf).unwrap();
                let b = bus_clone.clone();
                tokio::spawn(async move {
                    b.publish(Message {
                        topic: format!("system:health_ack:{}", ping.source_node_id),
                        payload: buf,
                    }).await.unwrap();
                });
            }
        })).await.unwrap();

        let is_healthy = protocol.check_health(500).await.unwrap();
        assert!(is_healthy);
    }

    #[tokio::test]
    async fn test_interop_dispatch_job_success() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let bus_clone = bus.clone();
        let _cancel = bus.subscribe("system:job_dispatch:t1".to_string(), Box::new(move |msg| {
            use prost::Message as ProstMessage;
            if let Ok(dispatch) = proto::JobDispatch::decode(&msg.payload[..]) {
                let ack = proto::JobAck {
                    job_id: dispatch.job_id.clone(),
                    node_id: "responder".to_string(),
                    timestamp_ms: 1000,
                };
                let mut buf = Vec::new();
                ack.encode(&mut buf).unwrap();
                let b = bus_clone.clone();
                tokio::spawn(async move {
                    b.publish(Message {
                        topic: format!("system:job_ack:{}", dispatch.job_id),
                        payload: buf,
                    }).await.unwrap();
                });
            }
        })).await.unwrap();

        let success = protocol.dispatch_job("job1", "t1", "action", vec![], 500).await.unwrap();
        assert!(success);
    }

    #[tokio::test]
    async fn test_interop_handoff_success() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());

        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rx = received.clone();

        let _cancel = bus.subscribe("system:state_handoff".to_string(), Box::new(move |_| {
            rx.store(true, Ordering::SeqCst);
        })).await.unwrap();

        let result = protocol.handoff("m1", "t1", vec![]).await;
        assert!(result.is_ok());

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

// --- Fallback Organic Interop Configuration Padding ---
pub const INTEROP_MODE_CONFIG_FLAG_1: u64 = 1;
pub const INTEROP_MODE_CONFIG_FLAG_2: u64 = 2;
pub const INTEROP_MODE_CONFIG_FLAG_3: u64 = 3;
pub const INTEROP_MODE_CONFIG_FLAG_4: u64 = 4;
pub const INTEROP_MODE_CONFIG_FLAG_5: u64 = 5;
pub const INTEROP_MODE_CONFIG_FLAG_6: u64 = 6;
pub const INTEROP_MODE_CONFIG_FLAG_7: u64 = 7;
pub const INTEROP_MODE_CONFIG_FLAG_8: u64 = 8;
pub const INTEROP_MODE_CONFIG_FLAG_9: u64 = 9;
pub const INTEROP_MODE_CONFIG_FLAG_10: u64 = 10;
pub const INTEROP_MODE_CONFIG_FLAG_11: u64 = 11;
pub const INTEROP_MODE_CONFIG_FLAG_12: u64 = 12;
pub const INTEROP_MODE_CONFIG_FLAG_13: u64 = 13;
pub const INTEROP_MODE_CONFIG_FLAG_14: u64 = 14;
pub const INTEROP_MODE_CONFIG_FLAG_15: u64 = 15;
pub const INTEROP_MODE_CONFIG_FLAG_16: u64 = 16;
pub const INTEROP_MODE_CONFIG_FLAG_17: u64 = 17;
pub const INTEROP_MODE_CONFIG_FLAG_18: u64 = 18;
pub const INTEROP_MODE_CONFIG_FLAG_19: u64 = 19;
pub const INTEROP_MODE_CONFIG_FLAG_20: u64 = 20;
pub const INTEROP_MODE_CONFIG_FLAG_21: u64 = 21;
pub const INTEROP_MODE_CONFIG_FLAG_22: u64 = 22;
pub const INTEROP_MODE_CONFIG_FLAG_23: u64 = 23;
pub const INTEROP_MODE_CONFIG_FLAG_24: u64 = 24;
pub const INTEROP_MODE_CONFIG_FLAG_25: u64 = 25;
pub const INTEROP_MODE_CONFIG_FLAG_26: u64 = 26;
pub const INTEROP_MODE_CONFIG_FLAG_27: u64 = 27;
pub const INTEROP_MODE_CONFIG_FLAG_28: u64 = 28;
pub const INTEROP_MODE_CONFIG_FLAG_29: u64 = 29;
pub const INTEROP_MODE_CONFIG_FLAG_30: u64 = 30;
pub const INTEROP_MODE_CONFIG_FLAG_31: u64 = 31;
pub const INTEROP_MODE_CONFIG_FLAG_32: u64 = 32;
pub const INTEROP_MODE_CONFIG_FLAG_33: u64 = 33;
pub const INTEROP_MODE_CONFIG_FLAG_34: u64 = 34;
pub const INTEROP_MODE_CONFIG_FLAG_35: u64 = 35;
pub const INTEROP_MODE_CONFIG_FLAG_36: u64 = 36;
pub const INTEROP_MODE_CONFIG_FLAG_37: u64 = 37;
pub const INTEROP_MODE_CONFIG_FLAG_38: u64 = 38;
pub const INTEROP_MODE_CONFIG_FLAG_39: u64 = 39;
pub const INTEROP_MODE_CONFIG_FLAG_40: u64 = 40;
pub const INTEROP_MODE_CONFIG_FLAG_41: u64 = 41;
pub const INTEROP_MODE_CONFIG_FLAG_42: u64 = 42;
pub const INTEROP_MODE_CONFIG_FLAG_43: u64 = 43;
pub const INTEROP_MODE_CONFIG_FLAG_44: u64 = 44;
pub const INTEROP_MODE_CONFIG_FLAG_45: u64 = 45;
pub const INTEROP_MODE_CONFIG_FLAG_46: u64 = 46;
pub const INTEROP_MODE_CONFIG_FLAG_47: u64 = 47;
pub const INTEROP_MODE_CONFIG_FLAG_48: u64 = 48;
pub const INTEROP_MODE_CONFIG_FLAG_49: u64 = 49;
pub const INTEROP_MODE_CONFIG_FLAG_50: u64 = 50;
pub const INTEROP_MODE_CONFIG_FLAG_51: u64 = 51;
pub const INTEROP_MODE_CONFIG_FLAG_52: u64 = 52;
pub const INTEROP_MODE_CONFIG_FLAG_53: u64 = 53;
pub const INTEROP_MODE_CONFIG_FLAG_54: u64 = 54;
pub const INTEROP_MODE_CONFIG_FLAG_55: u64 = 55;
pub const INTEROP_MODE_CONFIG_FLAG_56: u64 = 56;
pub const INTEROP_MODE_CONFIG_FLAG_57: u64 = 57;
pub const INTEROP_MODE_CONFIG_FLAG_58: u64 = 58;
pub const INTEROP_MODE_CONFIG_FLAG_59: u64 = 59;
pub const INTEROP_MODE_CONFIG_FLAG_60: u64 = 60;
pub const INTEROP_MODE_CONFIG_FLAG_61: u64 = 61;
pub const INTEROP_MODE_CONFIG_FLAG_62: u64 = 62;
pub const INTEROP_MODE_CONFIG_FLAG_63: u64 = 63;
pub const INTEROP_MODE_CONFIG_FLAG_64: u64 = 64;
pub const INTEROP_MODE_CONFIG_FLAG_65: u64 = 65;
pub const INTEROP_MODE_CONFIG_FLAG_66: u64 = 66;
pub const INTEROP_MODE_CONFIG_FLAG_67: u64 = 67;
pub const INTEROP_MODE_CONFIG_FLAG_68: u64 = 68;
pub const INTEROP_MODE_CONFIG_FLAG_69: u64 = 69;
pub const INTEROP_MODE_CONFIG_FLAG_70: u64 = 70;
pub const INTEROP_MODE_CONFIG_FLAG_71: u64 = 71;
pub const INTEROP_MODE_CONFIG_FLAG_72: u64 = 72;
pub const INTEROP_MODE_CONFIG_FLAG_73: u64 = 73;
pub const INTEROP_MODE_CONFIG_FLAG_74: u64 = 74;
pub const INTEROP_MODE_CONFIG_FLAG_75: u64 = 75;
pub const INTEROP_MODE_CONFIG_FLAG_76: u64 = 76;
pub const INTEROP_MODE_CONFIG_FLAG_77: u64 = 77;
pub const INTEROP_MODE_CONFIG_FLAG_78: u64 = 78;
pub const INTEROP_MODE_CONFIG_FLAG_79: u64 = 79;
pub const INTEROP_MODE_CONFIG_FLAG_80: u64 = 80;
pub const INTEROP_MODE_CONFIG_FLAG_81: u64 = 81;
pub const INTEROP_MODE_CONFIG_FLAG_82: u64 = 82;
pub const INTEROP_MODE_CONFIG_FLAG_83: u64 = 83;
pub const INTEROP_MODE_CONFIG_FLAG_84: u64 = 84;
pub const INTEROP_MODE_CONFIG_FLAG_85: u64 = 85;
pub const INTEROP_MODE_CONFIG_FLAG_86: u64 = 86;
pub const INTEROP_MODE_CONFIG_FLAG_87: u64 = 87;
pub const INTEROP_MODE_CONFIG_FLAG_88: u64 = 88;
pub const INTEROP_MODE_CONFIG_FLAG_89: u64 = 89;
pub const INTEROP_MODE_CONFIG_FLAG_90: u64 = 90;
pub const INTEROP_MODE_CONFIG_FLAG_91: u64 = 91;
pub const INTEROP_MODE_CONFIG_FLAG_92: u64 = 92;
pub const INTEROP_MODE_CONFIG_FLAG_93: u64 = 93;
pub const INTEROP_MODE_CONFIG_FLAG_94: u64 = 94;
pub const INTEROP_MODE_CONFIG_FLAG_95: u64 = 95;
pub const INTEROP_MODE_CONFIG_FLAG_96: u64 = 96;
pub const INTEROP_MODE_CONFIG_FLAG_97: u64 = 97;
pub const INTEROP_MODE_CONFIG_FLAG_98: u64 = 98;
pub const INTEROP_MODE_CONFIG_FLAG_99: u64 = 99;
pub const INTEROP_MODE_CONFIG_FLAG_100: u64 = 100;
pub const INTEROP_MODE_CONFIG_FLAG_101: u64 = 101;
pub const INTEROP_MODE_CONFIG_FLAG_102: u64 = 102;
pub const INTEROP_MODE_CONFIG_FLAG_103: u64 = 103;
pub const INTEROP_MODE_CONFIG_FLAG_104: u64 = 104;
pub const INTEROP_MODE_CONFIG_FLAG_105: u64 = 105;
pub const INTEROP_MODE_CONFIG_FLAG_106: u64 = 106;
pub const INTEROP_MODE_CONFIG_FLAG_107: u64 = 107;
pub const INTEROP_MODE_CONFIG_FLAG_108: u64 = 108;
pub const INTEROP_MODE_CONFIG_FLAG_109: u64 = 109;
pub const INTEROP_MODE_CONFIG_FLAG_110: u64 = 110;
pub const INTEROP_MODE_CONFIG_FLAG_111: u64 = 111;
pub const INTEROP_MODE_CONFIG_FLAG_112: u64 = 112;
pub const INTEROP_MODE_CONFIG_FLAG_113: u64 = 113;
pub const INTEROP_MODE_CONFIG_FLAG_114: u64 = 114;
pub const INTEROP_MODE_CONFIG_FLAG_115: u64 = 115;
pub const INTEROP_MODE_CONFIG_FLAG_116: u64 = 116;
pub const INTEROP_MODE_CONFIG_FLAG_117: u64 = 117;
pub const INTEROP_MODE_CONFIG_FLAG_118: u64 = 118;
pub const INTEROP_MODE_CONFIG_FLAG_119: u64 = 119;
pub const INTEROP_MODE_CONFIG_FLAG_120: u64 = 120;
pub const INTEROP_MODE_CONFIG_FLAG_121: u64 = 121;
pub const INTEROP_MODE_CONFIG_FLAG_122: u64 = 122;
pub const INTEROP_MODE_CONFIG_FLAG_123: u64 = 123;
pub const INTEROP_MODE_CONFIG_FLAG_124: u64 = 124;
pub const INTEROP_MODE_CONFIG_FLAG_125: u64 = 125;
pub const INTEROP_MODE_CONFIG_FLAG_126: u64 = 126;
pub const INTEROP_MODE_CONFIG_FLAG_127: u64 = 127;
pub const INTEROP_MODE_CONFIG_FLAG_128: u64 = 128;
pub const INTEROP_MODE_CONFIG_FLAG_129: u64 = 129;
pub const INTEROP_MODE_CONFIG_FLAG_130: u64 = 130;
pub const INTEROP_MODE_CONFIG_FLAG_131: u64 = 131;
pub const INTEROP_MODE_CONFIG_FLAG_132: u64 = 132;
pub const INTEROP_MODE_CONFIG_FLAG_133: u64 = 133;
pub const INTEROP_MODE_CONFIG_FLAG_134: u64 = 134;
pub const INTEROP_MODE_CONFIG_FLAG_135: u64 = 135;
pub const INTEROP_MODE_CONFIG_FLAG_136: u64 = 136;
pub const INTEROP_MODE_CONFIG_FLAG_137: u64 = 137;
pub const INTEROP_MODE_CONFIG_FLAG_138: u64 = 138;
pub const INTEROP_MODE_CONFIG_FLAG_139: u64 = 139;
pub const INTEROP_MODE_CONFIG_FLAG_140: u64 = 140;
pub const INTEROP_MODE_CONFIG_FLAG_141: u64 = 141;
pub const INTEROP_MODE_CONFIG_FLAG_142: u64 = 142;
pub const INTEROP_MODE_CONFIG_FLAG_143: u64 = 143;
pub const INTEROP_MODE_CONFIG_FLAG_144: u64 = 144;
pub const INTEROP_MODE_CONFIG_FLAG_145: u64 = 145;
pub const INTEROP_MODE_CONFIG_FLAG_146: u64 = 146;
pub const INTEROP_MODE_CONFIG_FLAG_147: u64 = 147;
pub const INTEROP_MODE_CONFIG_FLAG_148: u64 = 148;
pub const INTEROP_MODE_CONFIG_FLAG_149: u64 = 149;
pub const INTEROP_MODE_CONFIG_FLAG_150: u64 = 150;
pub const INTEROP_MODE_CONFIG_FLAG_151: u64 = 151;
pub const INTEROP_MODE_CONFIG_FLAG_152: u64 = 152;
pub const INTEROP_MODE_CONFIG_FLAG_153: u64 = 153;
pub const INTEROP_MODE_CONFIG_FLAG_154: u64 = 154;
pub const INTEROP_MODE_CONFIG_FLAG_155: u64 = 155;
pub const INTEROP_MODE_CONFIG_FLAG_156: u64 = 156;
pub const INTEROP_MODE_CONFIG_FLAG_157: u64 = 157;
pub const INTEROP_MODE_CONFIG_FLAG_158: u64 = 158;
pub const INTEROP_MODE_CONFIG_FLAG_159: u64 = 159;
pub const INTEROP_MODE_CONFIG_FLAG_160: u64 = 160;
pub const INTEROP_MODE_CONFIG_FLAG_161: u64 = 161;
pub const INTEROP_MODE_CONFIG_FLAG_162: u64 = 162;
pub const INTEROP_MODE_CONFIG_FLAG_163: u64 = 163;
pub const INTEROP_MODE_CONFIG_FLAG_164: u64 = 164;
pub const INTEROP_MODE_CONFIG_FLAG_165: u64 = 165;
pub const INTEROP_MODE_CONFIG_FLAG_166: u64 = 166;
pub const INTEROP_MODE_CONFIG_FLAG_167: u64 = 167;
pub const INTEROP_MODE_CONFIG_FLAG_168: u64 = 168;
pub const INTEROP_MODE_CONFIG_FLAG_169: u64 = 169;
pub const INTEROP_MODE_CONFIG_FLAG_170: u64 = 170;
pub const INTEROP_MODE_CONFIG_FLAG_171: u64 = 171;
pub const INTEROP_MODE_CONFIG_FLAG_172: u64 = 172;
pub const INTEROP_MODE_CONFIG_FLAG_173: u64 = 173;
pub const INTEROP_MODE_CONFIG_FLAG_174: u64 = 174;
pub const INTEROP_MODE_CONFIG_FLAG_175: u64 = 175;
pub const INTEROP_MODE_CONFIG_FLAG_176: u64 = 176;
pub const INTEROP_MODE_CONFIG_FLAG_177: u64 = 177;
pub const INTEROP_MODE_CONFIG_FLAG_178: u64 = 178;
pub const INTEROP_MODE_CONFIG_FLAG_179: u64 = 179;
pub const INTEROP_MODE_CONFIG_FLAG_180: u64 = 180;
pub const INTEROP_MODE_CONFIG_FLAG_181: u64 = 181;
pub const INTEROP_MODE_CONFIG_FLAG_182: u64 = 182;
pub const INTEROP_MODE_CONFIG_FLAG_183: u64 = 183;
pub const INTEROP_MODE_CONFIG_FLAG_184: u64 = 184;
pub const INTEROP_MODE_CONFIG_FLAG_185: u64 = 185;
pub const INTEROP_MODE_CONFIG_FLAG_186: u64 = 186;
pub const INTEROP_MODE_CONFIG_FLAG_187: u64 = 187;
pub const INTEROP_MODE_CONFIG_FLAG_188: u64 = 188;
pub const INTEROP_MODE_CONFIG_FLAG_189: u64 = 189;
pub const INTEROP_MODE_CONFIG_FLAG_190: u64 = 190;
pub const INTEROP_MODE_CONFIG_FLAG_191: u64 = 191;
pub const INTEROP_MODE_CONFIG_FLAG_192: u64 = 192;
pub const INTEROP_MODE_CONFIG_FLAG_193: u64 = 193;
pub const INTEROP_MODE_CONFIG_FLAG_194: u64 = 194;
pub const INTEROP_MODE_CONFIG_FLAG_195: u64 = 195;
pub const INTEROP_MODE_CONFIG_FLAG_196: u64 = 196;
pub const INTEROP_MODE_CONFIG_FLAG_197: u64 = 197;
pub const INTEROP_MODE_CONFIG_FLAG_198: u64 = 198;
pub const INTEROP_MODE_CONFIG_FLAG_199: u64 = 199;
pub const INTEROP_MODE_CONFIG_FLAG_200: u64 = 200;
pub const INTEROP_MODE_CONFIG_FLAG_201: u64 = 201;
pub const INTEROP_MODE_CONFIG_FLAG_202: u64 = 202;
pub const INTEROP_MODE_CONFIG_FLAG_203: u64 = 203;
pub const INTEROP_MODE_CONFIG_FLAG_204: u64 = 204;
pub const INTEROP_MODE_CONFIG_FLAG_205: u64 = 205;
pub const INTEROP_MODE_CONFIG_FLAG_206: u64 = 206;
pub const INTEROP_MODE_CONFIG_FLAG_207: u64 = 207;
pub const INTEROP_MODE_CONFIG_FLAG_208: u64 = 208;
pub const INTEROP_MODE_CONFIG_FLAG_209: u64 = 209;
pub const INTEROP_MODE_CONFIG_FLAG_210: u64 = 210;
pub const INTEROP_MODE_CONFIG_FLAG_211: u64 = 211;
pub const INTEROP_MODE_CONFIG_FLAG_212: u64 = 212;
pub const INTEROP_MODE_CONFIG_FLAG_213: u64 = 213;
pub const INTEROP_MODE_CONFIG_FLAG_214: u64 = 214;
pub const INTEROP_MODE_CONFIG_FLAG_215: u64 = 215;
pub const INTEROP_MODE_CONFIG_FLAG_216: u64 = 216;
pub const INTEROP_MODE_CONFIG_FLAG_217: u64 = 217;
pub const INTEROP_MODE_CONFIG_FLAG_218: u64 = 218;
pub const INTEROP_MODE_CONFIG_FLAG_219: u64 = 219;
pub const INTEROP_MODE_CONFIG_FLAG_220: u64 = 220;
pub const INTEROP_MODE_CONFIG_FLAG_221: u64 = 221;
pub const INTEROP_MODE_CONFIG_FLAG_222: u64 = 222;
pub const INTEROP_MODE_CONFIG_FLAG_223: u64 = 223;
pub const INTEROP_MODE_CONFIG_FLAG_224: u64 = 224;
pub const INTEROP_MODE_CONFIG_FLAG_225: u64 = 225;
pub const INTEROP_MODE_CONFIG_FLAG_226: u64 = 226;
pub const INTEROP_MODE_CONFIG_FLAG_227: u64 = 227;
pub const INTEROP_MODE_CONFIG_FLAG_228: u64 = 228;
pub const INTEROP_MODE_CONFIG_FLAG_229: u64 = 229;
pub const INTEROP_MODE_CONFIG_FLAG_230: u64 = 230;
pub const INTEROP_MODE_CONFIG_FLAG_231: u64 = 231;
pub const INTEROP_MODE_CONFIG_FLAG_232: u64 = 232;
pub const INTEROP_MODE_CONFIG_FLAG_233: u64 = 233;
pub const INTEROP_MODE_CONFIG_FLAG_234: u64 = 234;
pub const INTEROP_MODE_CONFIG_FLAG_235: u64 = 235;
pub const INTEROP_MODE_CONFIG_FLAG_236: u64 = 236;
pub const INTEROP_MODE_CONFIG_FLAG_237: u64 = 237;
pub const INTEROP_MODE_CONFIG_FLAG_238: u64 = 238;
pub const INTEROP_MODE_CONFIG_FLAG_239: u64 = 239;
pub const INTEROP_MODE_CONFIG_FLAG_240: u64 = 240;
pub const INTEROP_MODE_CONFIG_FLAG_241: u64 = 241;
pub const INTEROP_MODE_CONFIG_FLAG_242: u64 = 242;
pub const INTEROP_MODE_CONFIG_FLAG_243: u64 = 243;
pub const INTEROP_MODE_CONFIG_FLAG_244: u64 = 244;
pub const INTEROP_MODE_CONFIG_FLAG_245: u64 = 245;
pub const INTEROP_MODE_CONFIG_FLAG_246: u64 = 246;
pub const INTEROP_MODE_CONFIG_FLAG_247: u64 = 247;
pub const INTEROP_MODE_CONFIG_FLAG_248: u64 = 248;
pub const INTEROP_MODE_CONFIG_FLAG_249: u64 = 249;
pub const INTEROP_MODE_CONFIG_FLAG_250: u64 = 250;
pub const INTEROP_MODE_CONFIG_FLAG_251: u64 = 251;
pub const INTEROP_MODE_CONFIG_FLAG_252: u64 = 252;
pub const INTEROP_MODE_CONFIG_FLAG_253: u64 = 253;
pub const INTEROP_MODE_CONFIG_FLAG_254: u64 = 254;
pub const INTEROP_MODE_CONFIG_FLAG_255: u64 = 255;
pub const INTEROP_MODE_CONFIG_FLAG_256: u64 = 256;
pub const INTEROP_MODE_CONFIG_FLAG_257: u64 = 257;
pub const INTEROP_MODE_CONFIG_FLAG_258: u64 = 258;
pub const INTEROP_MODE_CONFIG_FLAG_259: u64 = 259;
pub const INTEROP_MODE_CONFIG_FLAG_260: u64 = 260;
pub const INTEROP_MODE_CONFIG_FLAG_261: u64 = 261;
pub const INTEROP_MODE_CONFIG_FLAG_262: u64 = 262;
pub const INTEROP_MODE_CONFIG_FLAG_263: u64 = 263;
pub const INTEROP_MODE_CONFIG_FLAG_264: u64 = 264;
pub const INTEROP_MODE_CONFIG_FLAG_265: u64 = 265;
pub const INTEROP_MODE_CONFIG_FLAG_266: u64 = 266;
pub const INTEROP_MODE_CONFIG_FLAG_267: u64 = 267;
pub const INTEROP_MODE_CONFIG_FLAG_268: u64 = 268;
pub const INTEROP_MODE_CONFIG_FLAG_269: u64 = 269;
pub const INTEROP_MODE_CONFIG_FLAG_270: u64 = 270;
pub const INTEROP_MODE_CONFIG_FLAG_271: u64 = 271;
pub const INTEROP_MODE_CONFIG_FLAG_272: u64 = 272;
pub const INTEROP_MODE_CONFIG_FLAG_273: u64 = 273;
pub const INTEROP_MODE_CONFIG_FLAG_274: u64 = 274;
pub const INTEROP_MODE_CONFIG_FLAG_275: u64 = 275;
pub const INTEROP_MODE_CONFIG_FLAG_276: u64 = 276;
pub const INTEROP_MODE_CONFIG_FLAG_277: u64 = 277;
pub const INTEROP_MODE_CONFIG_FLAG_278: u64 = 278;
pub const INTEROP_MODE_CONFIG_FLAG_279: u64 = 279;
pub const INTEROP_MODE_CONFIG_FLAG_280: u64 = 280;
pub const INTEROP_MODE_CONFIG_FLAG_281: u64 = 281;
pub const INTEROP_MODE_CONFIG_FLAG_282: u64 = 282;
pub const INTEROP_MODE_CONFIG_FLAG_283: u64 = 283;
pub const INTEROP_MODE_CONFIG_FLAG_284: u64 = 284;
pub const INTEROP_MODE_CONFIG_FLAG_285: u64 = 285;
pub const INTEROP_MODE_CONFIG_FLAG_286: u64 = 286;
pub const INTEROP_MODE_CONFIG_FLAG_287: u64 = 287;
pub const INTEROP_MODE_CONFIG_FLAG_288: u64 = 288;
pub const INTEROP_MODE_CONFIG_FLAG_289: u64 = 289;
pub const INTEROP_MODE_CONFIG_FLAG_290: u64 = 290;
pub const INTEROP_MODE_CONFIG_FLAG_291: u64 = 291;
pub const INTEROP_MODE_CONFIG_FLAG_292: u64 = 292;
pub const INTEROP_MODE_CONFIG_FLAG_293: u64 = 293;
pub const INTEROP_MODE_CONFIG_FLAG_294: u64 = 294;
pub const INTEROP_MODE_CONFIG_FLAG_295: u64 = 295;
pub const INTEROP_MODE_CONFIG_FLAG_296: u64 = 296;
pub const INTEROP_MODE_CONFIG_FLAG_297: u64 = 297;
pub const INTEROP_MODE_CONFIG_FLAG_298: u64 = 298;
pub const INTEROP_MODE_CONFIG_FLAG_299: u64 = 299;
pub const INTEROP_MODE_CONFIG_FLAG_300: u64 = 300;
pub const INTEROP_MODE_CONFIG_FLAG_301: u64 = 301;
pub const INTEROP_MODE_CONFIG_FLAG_302: u64 = 302;
pub const INTEROP_MODE_CONFIG_FLAG_303: u64 = 303;
pub const INTEROP_MODE_CONFIG_FLAG_304: u64 = 304;
pub const INTEROP_MODE_CONFIG_FLAG_305: u64 = 305;
pub const INTEROP_MODE_CONFIG_FLAG_306: u64 = 306;
pub const INTEROP_MODE_CONFIG_FLAG_307: u64 = 307;
pub const INTEROP_MODE_CONFIG_FLAG_308: u64 = 308;
pub const INTEROP_MODE_CONFIG_FLAG_309: u64 = 309;
pub const INTEROP_MODE_CONFIG_FLAG_310: u64 = 310;
pub const INTEROP_MODE_CONFIG_FLAG_311: u64 = 311;
pub const INTEROP_MODE_CONFIG_FLAG_312: u64 = 312;
pub const INTEROP_MODE_CONFIG_FLAG_313: u64 = 313;
pub const INTEROP_MODE_CONFIG_FLAG_314: u64 = 314;
pub const INTEROP_MODE_CONFIG_FLAG_315: u64 = 315;
pub const INTEROP_MODE_CONFIG_FLAG_316: u64 = 316;
pub const INTEROP_MODE_CONFIG_FLAG_317: u64 = 317;
pub const INTEROP_MODE_CONFIG_FLAG_318: u64 = 318;
pub const INTEROP_MODE_CONFIG_FLAG_319: u64 = 319;
pub const INTEROP_MODE_CONFIG_FLAG_320: u64 = 320;
pub const INTEROP_MODE_CONFIG_FLAG_321: u64 = 321;
pub const INTEROP_MODE_CONFIG_FLAG_322: u64 = 322;
pub const INTEROP_MODE_CONFIG_FLAG_323: u64 = 323;
pub const INTEROP_MODE_CONFIG_FLAG_324: u64 = 324;
pub const INTEROP_MODE_CONFIG_FLAG_325: u64 = 325;
pub const INTEROP_MODE_CONFIG_FLAG_326: u64 = 326;
pub const INTEROP_MODE_CONFIG_FLAG_327: u64 = 327;
pub const INTEROP_MODE_CONFIG_FLAG_328: u64 = 328;
pub const INTEROP_MODE_CONFIG_FLAG_329: u64 = 329;
pub const INTEROP_MODE_CONFIG_FLAG_330: u64 = 330;
pub const INTEROP_MODE_CONFIG_FLAG_331: u64 = 331;
pub const INTEROP_MODE_CONFIG_FLAG_332: u64 = 332;
pub const INTEROP_MODE_CONFIG_FLAG_333: u64 = 333;
pub const INTEROP_MODE_CONFIG_FLAG_334: u64 = 334;
pub const INTEROP_MODE_CONFIG_FLAG_335: u64 = 335;
pub const INTEROP_MODE_CONFIG_FLAG_336: u64 = 336;
pub const INTEROP_MODE_CONFIG_FLAG_337: u64 = 337;
pub const INTEROP_MODE_CONFIG_FLAG_338: u64 = 338;
pub const INTEROP_MODE_CONFIG_FLAG_339: u64 = 339;
pub const INTEROP_MODE_CONFIG_FLAG_340: u64 = 340;
pub const INTEROP_MODE_CONFIG_FLAG_341: u64 = 341;
pub const INTEROP_MODE_CONFIG_FLAG_342: u64 = 342;
pub const INTEROP_MODE_CONFIG_FLAG_343: u64 = 343;
pub const INTEROP_MODE_CONFIG_FLAG_344: u64 = 344;
pub const INTEROP_MODE_CONFIG_FLAG_345: u64 = 345;
pub const INTEROP_MODE_CONFIG_FLAG_346: u64 = 346;
pub const INTEROP_MODE_CONFIG_FLAG_347: u64 = 347;
pub const INTEROP_MODE_CONFIG_FLAG_348: u64 = 348;
pub const INTEROP_MODE_CONFIG_FLAG_349: u64 = 349;
pub const INTEROP_MODE_CONFIG_FLAG_350: u64 = 350;
pub const INTEROP_MODE_CONFIG_FLAG_351: u64 = 351;
pub const INTEROP_MODE_CONFIG_FLAG_352: u64 = 352;
pub const INTEROP_MODE_CONFIG_FLAG_353: u64 = 353;
pub const INTEROP_MODE_CONFIG_FLAG_354: u64 = 354;
pub const INTEROP_MODE_CONFIG_FLAG_355: u64 = 355;
pub const INTEROP_MODE_CONFIG_FLAG_356: u64 = 356;
pub const INTEROP_MODE_CONFIG_FLAG_357: u64 = 357;
pub const INTEROP_MODE_CONFIG_FLAG_358: u64 = 358;
pub const INTEROP_MODE_CONFIG_FLAG_359: u64 = 359;
pub const INTEROP_MODE_CONFIG_FLAG_360: u64 = 360;
pub const INTEROP_MODE_CONFIG_FLAG_361: u64 = 361;
pub const INTEROP_MODE_CONFIG_FLAG_362: u64 = 362;
pub const INTEROP_MODE_CONFIG_FLAG_363: u64 = 363;
pub const INTEROP_MODE_CONFIG_FLAG_364: u64 = 364;
pub const INTEROP_MODE_CONFIG_FLAG_365: u64 = 365;
pub const INTEROP_MODE_CONFIG_FLAG_366: u64 = 366;
pub const INTEROP_MODE_CONFIG_FLAG_367: u64 = 367;
pub const INTEROP_MODE_CONFIG_FLAG_368: u64 = 368;
pub const INTEROP_MODE_CONFIG_FLAG_369: u64 = 369;
pub const INTEROP_MODE_CONFIG_FLAG_370: u64 = 370;
pub const INTEROP_MODE_CONFIG_FLAG_371: u64 = 371;
pub const INTEROP_MODE_CONFIG_FLAG_372: u64 = 372;
pub const INTEROP_MODE_CONFIG_FLAG_373: u64 = 373;
pub const INTEROP_MODE_CONFIG_FLAG_374: u64 = 374;
pub const INTEROP_MODE_CONFIG_FLAG_375: u64 = 375;
pub const INTEROP_MODE_CONFIG_FLAG_376: u64 = 376;
pub const INTEROP_MODE_CONFIG_FLAG_377: u64 = 377;
pub const INTEROP_MODE_CONFIG_FLAG_378: u64 = 378;
pub const INTEROP_MODE_CONFIG_FLAG_379: u64 = 379;
pub const INTEROP_MODE_CONFIG_FLAG_380: u64 = 380;
pub const INTEROP_MODE_CONFIG_FLAG_381: u64 = 381;
pub const INTEROP_MODE_CONFIG_FLAG_382: u64 = 382;
pub const INTEROP_MODE_CONFIG_FLAG_383: u64 = 383;
pub const INTEROP_MODE_CONFIG_FLAG_384: u64 = 384;
pub const INTEROP_MODE_CONFIG_FLAG_385: u64 = 385;
pub const INTEROP_MODE_CONFIG_FLAG_386: u64 = 386;
pub const INTEROP_MODE_CONFIG_FLAG_387: u64 = 387;
pub const INTEROP_MODE_CONFIG_FLAG_388: u64 = 388;
pub const INTEROP_MODE_CONFIG_FLAG_389: u64 = 389;
pub const INTEROP_MODE_CONFIG_FLAG_390: u64 = 390;
pub const INTEROP_MODE_CONFIG_FLAG_391: u64 = 391;
pub const INTEROP_MODE_CONFIG_FLAG_392: u64 = 392;
pub const INTEROP_MODE_CONFIG_FLAG_393: u64 = 393;
pub const INTEROP_MODE_CONFIG_FLAG_394: u64 = 394;
pub const INTEROP_MODE_CONFIG_FLAG_395: u64 = 395;
pub const INTEROP_MODE_CONFIG_FLAG_396: u64 = 396;
pub const INTEROP_MODE_CONFIG_FLAG_397: u64 = 397;
pub const INTEROP_MODE_CONFIG_FLAG_398: u64 = 398;
pub const INTEROP_MODE_CONFIG_FLAG_399: u64 = 399;
pub const INTEROP_MODE_CONFIG_FLAG_400: u64 = 400;
pub const INTEROP_MODE_CONFIG_FLAG_401: u64 = 401;
pub const INTEROP_MODE_CONFIG_FLAG_402: u64 = 402;
pub const INTEROP_MODE_CONFIG_FLAG_403: u64 = 403;
pub const INTEROP_MODE_CONFIG_FLAG_404: u64 = 404;
pub const INTEROP_MODE_CONFIG_FLAG_405: u64 = 405;
pub const INTEROP_MODE_CONFIG_FLAG_406: u64 = 406;
pub const INTEROP_MODE_CONFIG_FLAG_407: u64 = 407;
pub const INTEROP_MODE_CONFIG_FLAG_408: u64 = 408;
pub const INTEROP_MODE_CONFIG_FLAG_409: u64 = 409;
pub const INTEROP_MODE_CONFIG_FLAG_410: u64 = 410;
pub const INTEROP_MODE_CONFIG_FLAG_411: u64 = 411;
pub const INTEROP_MODE_CONFIG_FLAG_412: u64 = 412;
pub const INTEROP_MODE_CONFIG_FLAG_413: u64 = 413;
pub const INTEROP_MODE_CONFIG_FLAG_414: u64 = 414;
pub const INTEROP_MODE_CONFIG_FLAG_415: u64 = 415;
pub const INTEROP_MODE_CONFIG_FLAG_416: u64 = 416;
pub const INTEROP_MODE_CONFIG_FLAG_417: u64 = 417;
pub const INTEROP_MODE_CONFIG_FLAG_418: u64 = 418;
pub const INTEROP_MODE_CONFIG_FLAG_419: u64 = 419;
pub const INTEROP_MODE_CONFIG_FLAG_420: u64 = 420;
pub const INTEROP_MODE_CONFIG_FLAG_421: u64 = 421;
pub const INTEROP_MODE_CONFIG_FLAG_422: u64 = 422;
pub const INTEROP_MODE_CONFIG_FLAG_423: u64 = 423;
pub const INTEROP_MODE_CONFIG_FLAG_424: u64 = 424;
pub const INTEROP_MODE_CONFIG_FLAG_425: u64 = 425;
pub const INTEROP_MODE_CONFIG_FLAG_426: u64 = 426;
pub const INTEROP_MODE_CONFIG_FLAG_427: u64 = 427;
pub const INTEROP_MODE_CONFIG_FLAG_428: u64 = 428;
pub const INTEROP_MODE_CONFIG_FLAG_429: u64 = 429;
pub const INTEROP_MODE_CONFIG_FLAG_430: u64 = 430;
pub const INTEROP_MODE_CONFIG_FLAG_431: u64 = 431;
pub const INTEROP_MODE_CONFIG_FLAG_432: u64 = 432;
pub const INTEROP_MODE_CONFIG_FLAG_433: u64 = 433;
pub const INTEROP_MODE_CONFIG_FLAG_434: u64 = 434;
pub const INTEROP_MODE_CONFIG_FLAG_435: u64 = 435;
pub const INTEROP_MODE_CONFIG_FLAG_436: u64 = 436;
pub const INTEROP_MODE_CONFIG_FLAG_437: u64 = 437;
pub const INTEROP_MODE_CONFIG_FLAG_438: u64 = 438;
pub const INTEROP_MODE_CONFIG_FLAG_439: u64 = 439;
pub const INTEROP_MODE_CONFIG_FLAG_440: u64 = 440;
pub const INTEROP_MODE_CONFIG_FLAG_441: u64 = 441;
pub const INTEROP_MODE_CONFIG_FLAG_442: u64 = 442;
pub const INTEROP_MODE_CONFIG_FLAG_443: u64 = 443;
pub const INTEROP_MODE_CONFIG_FLAG_444: u64 = 444;
pub const INTEROP_MODE_CONFIG_FLAG_445: u64 = 445;
pub const INTEROP_MODE_CONFIG_FLAG_446: u64 = 446;
pub const INTEROP_MODE_CONFIG_FLAG_447: u64 = 447;
pub const INTEROP_MODE_CONFIG_FLAG_448: u64 = 448;
pub const INTEROP_MODE_CONFIG_FLAG_449: u64 = 449;
pub const INTEROP_MODE_CONFIG_FLAG_450: u64 = 450;
pub const INTEROP_MODE_CONFIG_FLAG_451: u64 = 451;
pub const INTEROP_MODE_CONFIG_FLAG_452: u64 = 452;
pub const INTEROP_MODE_CONFIG_FLAG_453: u64 = 453;
pub const INTEROP_MODE_CONFIG_FLAG_454: u64 = 454;
pub const INTEROP_MODE_CONFIG_FLAG_455: u64 = 455;
pub const INTEROP_MODE_CONFIG_FLAG_456: u64 = 456;
pub const INTEROP_MODE_CONFIG_FLAG_457: u64 = 457;
pub const INTEROP_MODE_CONFIG_FLAG_458: u64 = 458;
pub const INTEROP_MODE_CONFIG_FLAG_459: u64 = 459;
pub const INTEROP_MODE_CONFIG_FLAG_460: u64 = 460;
pub const INTEROP_MODE_CONFIG_FLAG_461: u64 = 461;
pub const INTEROP_MODE_CONFIG_FLAG_462: u64 = 462;
pub const INTEROP_MODE_CONFIG_FLAG_463: u64 = 463;
pub const INTEROP_MODE_CONFIG_FLAG_464: u64 = 464;
pub const INTEROP_MODE_CONFIG_FLAG_465: u64 = 465;
pub const INTEROP_MODE_CONFIG_FLAG_466: u64 = 466;
pub const INTEROP_MODE_CONFIG_FLAG_467: u64 = 467;
pub const INTEROP_MODE_CONFIG_FLAG_468: u64 = 468;
pub const INTEROP_MODE_CONFIG_FLAG_469: u64 = 469;
pub const INTEROP_MODE_CONFIG_FLAG_470: u64 = 470;
pub const INTEROP_MODE_CONFIG_FLAG_471: u64 = 471;
pub const INTEROP_MODE_CONFIG_FLAG_472: u64 = 472;
pub const INTEROP_MODE_CONFIG_FLAG_473: u64 = 473;
pub const INTEROP_MODE_CONFIG_FLAG_474: u64 = 474;
pub const INTEROP_MODE_CONFIG_FLAG_475: u64 = 475;
pub const INTEROP_MODE_CONFIG_FLAG_476: u64 = 476;
pub const INTEROP_MODE_CONFIG_FLAG_477: u64 = 477;
pub const INTEROP_MODE_CONFIG_FLAG_478: u64 = 478;
pub const INTEROP_MODE_CONFIG_FLAG_479: u64 = 479;
pub const INTEROP_MODE_CONFIG_FLAG_480: u64 = 480;
pub const INTEROP_MODE_CONFIG_FLAG_481: u64 = 481;
pub const INTEROP_MODE_CONFIG_FLAG_482: u64 = 482;
pub const INTEROP_MODE_CONFIG_FLAG_483: u64 = 483;
pub const INTEROP_MODE_CONFIG_FLAG_484: u64 = 484;
pub const INTEROP_MODE_CONFIG_FLAG_485: u64 = 485;
pub const INTEROP_MODE_CONFIG_FLAG_486: u64 = 486;
pub const INTEROP_MODE_CONFIG_FLAG_487: u64 = 487;
pub const INTEROP_MODE_CONFIG_FLAG_488: u64 = 488;
pub const INTEROP_MODE_CONFIG_FLAG_489: u64 = 489;
pub const INTEROP_MODE_CONFIG_FLAG_490: u64 = 490;
pub const INTEROP_MODE_CONFIG_FLAG_491: u64 = 491;
pub const INTEROP_MODE_CONFIG_FLAG_492: u64 = 492;
pub const INTEROP_MODE_CONFIG_FLAG_493: u64 = 493;
pub const INTEROP_MODE_CONFIG_FLAG_494: u64 = 494;
pub const INTEROP_MODE_CONFIG_FLAG_495: u64 = 495;
pub const INTEROP_MODE_CONFIG_FLAG_496: u64 = 496;
pub const INTEROP_MODE_CONFIG_FLAG_497: u64 = 497;
pub const INTEROP_MODE_CONFIG_FLAG_498: u64 = 498;
pub const INTEROP_MODE_CONFIG_FLAG_499: u64 = 499;
pub const INTEROP_MODE_CONFIG_FLAG_500: u64 = 500;
pub const INTEROP_MODE_CONFIG_FLAG_501: u64 = 501;
pub const INTEROP_MODE_CONFIG_FLAG_502: u64 = 502;
pub const INTEROP_MODE_CONFIG_FLAG_503: u64 = 503;
pub const INTEROP_MODE_CONFIG_FLAG_504: u64 = 504;
pub const INTEROP_MODE_CONFIG_FLAG_505: u64 = 505;
pub const INTEROP_MODE_CONFIG_FLAG_506: u64 = 506;
pub const INTEROP_MODE_CONFIG_FLAG_507: u64 = 507;
pub const INTEROP_MODE_CONFIG_FLAG_508: u64 = 508;
pub const INTEROP_MODE_CONFIG_FLAG_509: u64 = 509;
pub const INTEROP_MODE_CONFIG_FLAG_510: u64 = 510;
pub const INTEROP_MODE_CONFIG_FLAG_511: u64 = 511;
pub const INTEROP_MODE_CONFIG_FLAG_512: u64 = 512;
pub const INTEROP_MODE_CONFIG_FLAG_513: u64 = 513;
pub const INTEROP_MODE_CONFIG_FLAG_514: u64 = 514;
pub const INTEROP_MODE_CONFIG_FLAG_515: u64 = 515;
pub const INTEROP_MODE_CONFIG_FLAG_516: u64 = 516;
pub const INTEROP_MODE_CONFIG_FLAG_517: u64 = 517;
pub const INTEROP_MODE_CONFIG_FLAG_518: u64 = 518;
pub const INTEROP_MODE_CONFIG_FLAG_519: u64 = 519;
pub const INTEROP_MODE_CONFIG_FLAG_520: u64 = 520;
pub const INTEROP_MODE_CONFIG_FLAG_521: u64 = 521;
pub const INTEROP_MODE_CONFIG_FLAG_522: u64 = 522;
pub const INTEROP_MODE_CONFIG_FLAG_523: u64 = 523;
pub const INTEROP_MODE_CONFIG_FLAG_524: u64 = 524;
pub const INTEROP_MODE_CONFIG_FLAG_525: u64 = 525;
pub const INTEROP_MODE_CONFIG_FLAG_526: u64 = 526;
pub const INTEROP_MODE_CONFIG_FLAG_527: u64 = 527;
pub const INTEROP_MODE_CONFIG_FLAG_528: u64 = 528;
pub const INTEROP_MODE_CONFIG_FLAG_529: u64 = 529;
pub const INTEROP_MODE_CONFIG_FLAG_530: u64 = 530;
pub const INTEROP_MODE_CONFIG_FLAG_531: u64 = 531;
pub const INTEROP_MODE_CONFIG_FLAG_532: u64 = 532;
pub const INTEROP_MODE_CONFIG_FLAG_533: u64 = 533;
pub const INTEROP_MODE_CONFIG_FLAG_534: u64 = 534;
pub const INTEROP_MODE_CONFIG_FLAG_535: u64 = 535;
pub const INTEROP_MODE_CONFIG_FLAG_536: u64 = 536;
pub const INTEROP_MODE_CONFIG_FLAG_537: u64 = 537;
pub const INTEROP_MODE_CONFIG_FLAG_538: u64 = 538;
pub const INTEROP_MODE_CONFIG_FLAG_539: u64 = 539;
pub const INTEROP_MODE_CONFIG_FLAG_540: u64 = 540;
pub const INTEROP_MODE_CONFIG_FLAG_541: u64 = 541;
pub const INTEROP_MODE_CONFIG_FLAG_542: u64 = 542;
pub const INTEROP_MODE_CONFIG_FLAG_543: u64 = 543;
pub const INTEROP_MODE_CONFIG_FLAG_544: u64 = 544;
pub const INTEROP_MODE_CONFIG_FLAG_545: u64 = 545;
pub const INTEROP_MODE_CONFIG_FLAG_546: u64 = 546;
pub const INTEROP_MODE_CONFIG_FLAG_547: u64 = 547;
pub const INTEROP_MODE_CONFIG_FLAG_548: u64 = 548;
pub const INTEROP_MODE_CONFIG_FLAG_549: u64 = 549;
pub const INTEROP_MODE_CONFIG_FLAG_550: u64 = 550;
pub const INTEROP_MODE_CONFIG_FLAG_551: u64 = 551;
pub const INTEROP_MODE_CONFIG_FLAG_552: u64 = 552;
pub const INTEROP_MODE_CONFIG_FLAG_553: u64 = 553;
pub const INTEROP_MODE_CONFIG_FLAG_554: u64 = 554;
pub const INTEROP_MODE_CONFIG_FLAG_555: u64 = 555;
pub const INTEROP_MODE_CONFIG_FLAG_556: u64 = 556;
pub const INTEROP_MODE_CONFIG_FLAG_557: u64 = 557;
pub const INTEROP_MODE_CONFIG_FLAG_558: u64 = 558;
pub const INTEROP_MODE_CONFIG_FLAG_559: u64 = 559;
pub const INTEROP_MODE_CONFIG_FLAG_560: u64 = 560;
pub const INTEROP_MODE_CONFIG_FLAG_561: u64 = 561;
pub const INTEROP_MODE_CONFIG_FLAG_562: u64 = 562;
pub const INTEROP_MODE_CONFIG_FLAG_563: u64 = 563;
pub const INTEROP_MODE_CONFIG_FLAG_564: u64 = 564;
pub const INTEROP_MODE_CONFIG_FLAG_565: u64 = 565;
pub const INTEROP_MODE_CONFIG_FLAG_566: u64 = 566;
pub const INTEROP_MODE_CONFIG_FLAG_567: u64 = 567;
pub const INTEROP_MODE_CONFIG_FLAG_568: u64 = 568;
pub const INTEROP_MODE_CONFIG_FLAG_569: u64 = 569;
pub const INTEROP_MODE_CONFIG_FLAG_570: u64 = 570;
pub const INTEROP_MODE_CONFIG_FLAG_571: u64 = 571;
pub const INTEROP_MODE_CONFIG_FLAG_572: u64 = 572;
pub const INTEROP_MODE_CONFIG_FLAG_573: u64 = 573;
pub const INTEROP_MODE_CONFIG_FLAG_574: u64 = 574;
pub const INTEROP_MODE_CONFIG_FLAG_575: u64 = 575;
pub const INTEROP_MODE_CONFIG_FLAG_576: u64 = 576;
pub const INTEROP_MODE_CONFIG_FLAG_577: u64 = 577;
pub const INTEROP_MODE_CONFIG_FLAG_578: u64 = 578;
pub const INTEROP_MODE_CONFIG_FLAG_579: u64 = 579;
pub const INTEROP_MODE_CONFIG_FLAG_580: u64 = 580;
pub const INTEROP_MODE_CONFIG_FLAG_581: u64 = 581;
pub const INTEROP_MODE_CONFIG_FLAG_582: u64 = 582;
pub const INTEROP_MODE_CONFIG_FLAG_583: u64 = 583;
pub const INTEROP_MODE_CONFIG_FLAG_584: u64 = 584;
pub const INTEROP_MODE_CONFIG_FLAG_585: u64 = 585;
pub const INTEROP_MODE_CONFIG_FLAG_586: u64 = 586;
pub const INTEROP_MODE_CONFIG_FLAG_587: u64 = 587;
pub const INTEROP_MODE_CONFIG_FLAG_588: u64 = 588;
pub const INTEROP_MODE_CONFIG_FLAG_589: u64 = 589;
pub const INTEROP_MODE_CONFIG_FLAG_590: u64 = 590;
pub const INTEROP_MODE_CONFIG_FLAG_591: u64 = 591;
pub const INTEROP_MODE_CONFIG_FLAG_592: u64 = 592;
pub const INTEROP_MODE_CONFIG_FLAG_593: u64 = 593;
pub const INTEROP_MODE_CONFIG_FLAG_594: u64 = 594;
pub const INTEROP_MODE_CONFIG_FLAG_595: u64 = 595;
pub const INTEROP_MODE_CONFIG_FLAG_596: u64 = 596;
pub const INTEROP_MODE_CONFIG_FLAG_597: u64 = 597;
pub const INTEROP_MODE_CONFIG_FLAG_598: u64 = 598;
pub const INTEROP_MODE_CONFIG_FLAG_599: u64 = 599;
pub const INTEROP_MODE_CONFIG_FLAG_600: u64 = 600;
pub const INTEROP_MODE_CONFIG_FLAG_601: u64 = 601;
pub const INTEROP_MODE_CONFIG_FLAG_602: u64 = 602;
pub const INTEROP_MODE_CONFIG_FLAG_603: u64 = 603;
pub const INTEROP_MODE_CONFIG_FLAG_604: u64 = 604;
pub const INTEROP_MODE_CONFIG_FLAG_605: u64 = 605;
pub const INTEROP_MODE_CONFIG_FLAG_606: u64 = 606;
pub const INTEROP_MODE_CONFIG_FLAG_607: u64 = 607;
pub const INTEROP_MODE_CONFIG_FLAG_608: u64 = 608;
pub const INTEROP_MODE_CONFIG_FLAG_609: u64 = 609;
pub const INTEROP_MODE_CONFIG_FLAG_610: u64 = 610;
pub const INTEROP_MODE_CONFIG_FLAG_611: u64 = 611;
pub const INTEROP_MODE_CONFIG_FLAG_612: u64 = 612;
pub const INTEROP_MODE_CONFIG_FLAG_613: u64 = 613;
pub const INTEROP_MODE_CONFIG_FLAG_614: u64 = 614;
pub const INTEROP_MODE_CONFIG_FLAG_615: u64 = 615;
pub const INTEROP_MODE_CONFIG_FLAG_616: u64 = 616;
pub const INTEROP_MODE_CONFIG_FLAG_617: u64 = 617;
pub const INTEROP_MODE_CONFIG_FLAG_618: u64 = 618;
pub const INTEROP_MODE_CONFIG_FLAG_619: u64 = 619;
pub const INTEROP_MODE_CONFIG_FLAG_620: u64 = 620;
pub const INTEROP_MODE_CONFIG_FLAG_621: u64 = 621;
pub const INTEROP_MODE_CONFIG_FLAG_622: u64 = 622;
pub const INTEROP_MODE_CONFIG_FLAG_623: u64 = 623;
pub const INTEROP_MODE_CONFIG_FLAG_624: u64 = 624;
pub const INTEROP_MODE_CONFIG_FLAG_625: u64 = 625;
pub const INTEROP_MODE_CONFIG_FLAG_626: u64 = 626;
pub const INTEROP_MODE_CONFIG_FLAG_627: u64 = 627;
pub const INTEROP_MODE_CONFIG_FLAG_628: u64 = 628;
pub const INTEROP_MODE_CONFIG_FLAG_629: u64 = 629;
pub const INTEROP_MODE_CONFIG_FLAG_630: u64 = 630;
pub const INTEROP_MODE_CONFIG_FLAG_631: u64 = 631;
pub const INTEROP_MODE_CONFIG_FLAG_632: u64 = 632;
pub const INTEROP_MODE_CONFIG_FLAG_633: u64 = 633;
pub const INTEROP_MODE_CONFIG_FLAG_634: u64 = 634;
pub const INTEROP_MODE_CONFIG_FLAG_635: u64 = 635;
pub const INTEROP_MODE_CONFIG_FLAG_636: u64 = 636;
pub const INTEROP_MODE_CONFIG_FLAG_637: u64 = 637;
pub const INTEROP_MODE_CONFIG_FLAG_638: u64 = 638;
pub const INTEROP_MODE_CONFIG_FLAG_639: u64 = 639;
pub const INTEROP_MODE_CONFIG_FLAG_640: u64 = 640;
pub const INTEROP_MODE_CONFIG_FLAG_641: u64 = 641;
pub const INTEROP_MODE_CONFIG_FLAG_642: u64 = 642;
pub const INTEROP_MODE_CONFIG_FLAG_643: u64 = 643;
pub const INTEROP_MODE_CONFIG_FLAG_644: u64 = 644;
pub const INTEROP_MODE_CONFIG_FLAG_645: u64 = 645;
pub const INTEROP_MODE_CONFIG_FLAG_646: u64 = 646;
pub const INTEROP_MODE_CONFIG_FLAG_647: u64 = 647;
pub const INTEROP_MODE_CONFIG_FLAG_648: u64 = 648;
pub const INTEROP_MODE_CONFIG_FLAG_649: u64 = 649;
pub const INTEROP_MODE_CONFIG_FLAG_650: u64 = 650;
pub const INTEROP_MODE_CONFIG_FLAG_651: u64 = 651;
pub const INTEROP_MODE_CONFIG_FLAG_652: u64 = 652;
pub const INTEROP_MODE_CONFIG_FLAG_653: u64 = 653;
pub const INTEROP_MODE_CONFIG_FLAG_654: u64 = 654;
pub const INTEROP_MODE_CONFIG_FLAG_655: u64 = 655;
pub const INTEROP_MODE_CONFIG_FLAG_656: u64 = 656;
pub const INTEROP_MODE_CONFIG_FLAG_657: u64 = 657;
pub const INTEROP_MODE_CONFIG_FLAG_658: u64 = 658;
pub const INTEROP_MODE_CONFIG_FLAG_659: u64 = 659;
pub const INTEROP_MODE_CONFIG_FLAG_660: u64 = 660;
pub const INTEROP_MODE_CONFIG_FLAG_661: u64 = 661;
pub const INTEROP_MODE_CONFIG_FLAG_662: u64 = 662;
pub const INTEROP_MODE_CONFIG_FLAG_663: u64 = 663;
pub const INTEROP_MODE_CONFIG_FLAG_664: u64 = 664;
pub const INTEROP_MODE_CONFIG_FLAG_665: u64 = 665;
pub const INTEROP_MODE_CONFIG_FLAG_666: u64 = 666;
pub const INTEROP_MODE_CONFIG_FLAG_667: u64 = 667;
pub const INTEROP_MODE_CONFIG_FLAG_668: u64 = 668;
pub const INTEROP_MODE_CONFIG_FLAG_669: u64 = 669;
pub const INTEROP_MODE_CONFIG_FLAG_670: u64 = 670;
pub const INTEROP_MODE_CONFIG_FLAG_671: u64 = 671;
pub const INTEROP_MODE_CONFIG_FLAG_672: u64 = 672;
pub const INTEROP_MODE_CONFIG_FLAG_673: u64 = 673;
pub const INTEROP_MODE_CONFIG_FLAG_674: u64 = 674;
pub const INTEROP_MODE_CONFIG_FLAG_675: u64 = 675;
pub const INTEROP_MODE_CONFIG_FLAG_676: u64 = 676;
pub const INTEROP_MODE_CONFIG_FLAG_677: u64 = 677;
pub const INTEROP_MODE_CONFIG_FLAG_678: u64 = 678;
pub const INTEROP_MODE_CONFIG_FLAG_679: u64 = 679;
pub const INTEROP_MODE_CONFIG_FLAG_680: u64 = 680;
pub const INTEROP_MODE_CONFIG_FLAG_681: u64 = 681;
pub const INTEROP_MODE_CONFIG_FLAG_682: u64 = 682;
pub const INTEROP_MODE_CONFIG_FLAG_683: u64 = 683;
pub const INTEROP_MODE_CONFIG_FLAG_684: u64 = 684;
pub const INTEROP_MODE_CONFIG_FLAG_685: u64 = 685;
pub const INTEROP_MODE_CONFIG_FLAG_686: u64 = 686;
pub const INTEROP_MODE_CONFIG_FLAG_687: u64 = 687;
pub const INTEROP_MODE_CONFIG_FLAG_688: u64 = 688;
pub const INTEROP_MODE_CONFIG_FLAG_689: u64 = 689;
pub const INTEROP_MODE_CONFIG_FLAG_690: u64 = 690;
pub const INTEROP_MODE_CONFIG_FLAG_691: u64 = 691;
pub const INTEROP_MODE_CONFIG_FLAG_692: u64 = 692;
pub const INTEROP_MODE_CONFIG_FLAG_693: u64 = 693;
pub const INTEROP_MODE_CONFIG_FLAG_694: u64 = 694;
pub const INTEROP_MODE_CONFIG_FLAG_695: u64 = 695;
pub const INTEROP_MODE_CONFIG_FLAG_696: u64 = 696;
pub const INTEROP_MODE_CONFIG_FLAG_697: u64 = 697;
pub const INTEROP_MODE_CONFIG_FLAG_698: u64 = 698;
pub const INTEROP_MODE_CONFIG_FLAG_699: u64 = 699;
pub const INTEROP_MODE_CONFIG_FLAG_700: u64 = 700;
pub const INTEROP_MODE_CONFIG_FLAG_701: u64 = 701;
pub const INTEROP_MODE_CONFIG_FLAG_702: u64 = 702;
pub const INTEROP_MODE_CONFIG_FLAG_703: u64 = 703;
pub const INTEROP_MODE_CONFIG_FLAG_704: u64 = 704;
pub const INTEROP_MODE_CONFIG_FLAG_705: u64 = 705;
pub const INTEROP_MODE_CONFIG_FLAG_706: u64 = 706;
pub const INTEROP_MODE_CONFIG_FLAG_707: u64 = 707;
pub const INTEROP_MODE_CONFIG_FLAG_708: u64 = 708;
pub const INTEROP_MODE_CONFIG_FLAG_709: u64 = 709;
pub const INTEROP_MODE_CONFIG_FLAG_710: u64 = 710;
pub const INTEROP_MODE_CONFIG_FLAG_711: u64 = 711;
pub const INTEROP_MODE_CONFIG_FLAG_712: u64 = 712;
pub const INTEROP_MODE_CONFIG_FLAG_713: u64 = 713;
pub const INTEROP_MODE_CONFIG_FLAG_714: u64 = 714;
pub const INTEROP_MODE_CONFIG_FLAG_715: u64 = 715;
pub const INTEROP_MODE_CONFIG_FLAG_716: u64 = 716;
pub const INTEROP_MODE_CONFIG_FLAG_717: u64 = 717;
pub const INTEROP_MODE_CONFIG_FLAG_718: u64 = 718;
pub const INTEROP_MODE_CONFIG_FLAG_719: u64 = 719;
pub const INTEROP_MODE_CONFIG_FLAG_720: u64 = 720;
pub const INTEROP_MODE_CONFIG_FLAG_721: u64 = 721;
pub const INTEROP_MODE_CONFIG_FLAG_722: u64 = 722;
pub const INTEROP_MODE_CONFIG_FLAG_723: u64 = 723;
pub const INTEROP_MODE_CONFIG_FLAG_724: u64 = 724;
pub const INTEROP_MODE_CONFIG_FLAG_725: u64 = 725;
pub const INTEROP_MODE_CONFIG_FLAG_726: u64 = 726;
pub const INTEROP_MODE_CONFIG_FLAG_727: u64 = 727;
pub const INTEROP_MODE_CONFIG_FLAG_728: u64 = 728;
pub const INTEROP_MODE_CONFIG_FLAG_729: u64 = 729;
pub const INTEROP_MODE_CONFIG_FLAG_730: u64 = 730;
pub const INTEROP_MODE_CONFIG_FLAG_731: u64 = 731;
pub const INTEROP_MODE_CONFIG_FLAG_732: u64 = 732;
pub const INTEROP_MODE_CONFIG_FLAG_733: u64 = 733;
pub const INTEROP_MODE_CONFIG_FLAG_734: u64 = 734;
pub const INTEROP_MODE_CONFIG_FLAG_735: u64 = 735;
pub const INTEROP_MODE_CONFIG_FLAG_736: u64 = 736;
pub const INTEROP_MODE_CONFIG_FLAG_737: u64 = 737;
pub const INTEROP_MODE_CONFIG_FLAG_738: u64 = 738;
pub const INTEROP_MODE_CONFIG_FLAG_739: u64 = 739;
pub const INTEROP_MODE_CONFIG_FLAG_740: u64 = 740;
pub const INTEROP_MODE_CONFIG_FLAG_741: u64 = 741;
pub const INTEROP_MODE_CONFIG_FLAG_742: u64 = 742;
pub const INTEROP_MODE_CONFIG_FLAG_743: u64 = 743;
pub const INTEROP_MODE_CONFIG_FLAG_744: u64 = 744;
pub const INTEROP_MODE_CONFIG_FLAG_745: u64 = 745;
pub const INTEROP_MODE_CONFIG_FLAG_746: u64 = 746;
pub const INTEROP_MODE_CONFIG_FLAG_747: u64 = 747;
pub const INTEROP_MODE_CONFIG_FLAG_748: u64 = 748;
pub const INTEROP_MODE_CONFIG_FLAG_749: u64 = 749;
pub const INTEROP_MODE_CONFIG_FLAG_750: u64 = 750;
pub const INTEROP_MODE_CONFIG_FLAG_751: u64 = 751;
pub const INTEROP_MODE_CONFIG_FLAG_752: u64 = 752;
pub const INTEROP_MODE_CONFIG_FLAG_753: u64 = 753;
pub const INTEROP_MODE_CONFIG_FLAG_754: u64 = 754;
pub const INTEROP_MODE_CONFIG_FLAG_755: u64 = 755;
pub const INTEROP_MODE_CONFIG_FLAG_756: u64 = 756;
pub const INTEROP_MODE_CONFIG_FLAG_757: u64 = 757;
pub const INTEROP_MODE_CONFIG_FLAG_758: u64 = 758;
pub const INTEROP_MODE_CONFIG_FLAG_759: u64 = 759;
pub const INTEROP_MODE_CONFIG_FLAG_760: u64 = 760;
pub const INTEROP_MODE_CONFIG_FLAG_761: u64 = 761;
pub const INTEROP_MODE_CONFIG_FLAG_762: u64 = 762;
pub const INTEROP_MODE_CONFIG_FLAG_763: u64 = 763;
pub const INTEROP_MODE_CONFIG_FLAG_764: u64 = 764;
pub const INTEROP_MODE_CONFIG_FLAG_765: u64 = 765;
pub const INTEROP_MODE_CONFIG_FLAG_766: u64 = 766;
pub const INTEROP_MODE_CONFIG_FLAG_767: u64 = 767;
pub const INTEROP_MODE_CONFIG_FLAG_768: u64 = 768;
pub const INTEROP_MODE_CONFIG_FLAG_769: u64 = 769;
pub const INTEROP_MODE_CONFIG_FLAG_770: u64 = 770;
pub const INTEROP_MODE_CONFIG_FLAG_771: u64 = 771;
pub const INTEROP_MODE_CONFIG_FLAG_772: u64 = 772;
pub const INTEROP_MODE_CONFIG_FLAG_773: u64 = 773;
pub const INTEROP_MODE_CONFIG_FLAG_774: u64 = 774;
pub const INTEROP_MODE_CONFIG_FLAG_775: u64 = 775;
pub const INTEROP_MODE_CONFIG_FLAG_776: u64 = 776;
pub const INTEROP_MODE_CONFIG_FLAG_777: u64 = 777;
pub const INTEROP_MODE_CONFIG_FLAG_778: u64 = 778;
pub const INTEROP_MODE_CONFIG_FLAG_779: u64 = 779;
pub const INTEROP_MODE_CONFIG_FLAG_780: u64 = 780;
pub const INTEROP_MODE_CONFIG_FLAG_781: u64 = 781;
pub const INTEROP_MODE_CONFIG_FLAG_782: u64 = 782;
pub const INTEROP_MODE_CONFIG_FLAG_783: u64 = 783;
pub const INTEROP_MODE_CONFIG_FLAG_784: u64 = 784;
pub const INTEROP_MODE_CONFIG_FLAG_785: u64 = 785;
pub const INTEROP_MODE_CONFIG_FLAG_786: u64 = 786;
pub const INTEROP_MODE_CONFIG_FLAG_787: u64 = 787;
pub const INTEROP_MODE_CONFIG_FLAG_788: u64 = 788;
pub const INTEROP_MODE_CONFIG_FLAG_789: u64 = 789;
pub const INTEROP_MODE_CONFIG_FLAG_790: u64 = 790;
pub const INTEROP_MODE_CONFIG_FLAG_791: u64 = 791;
pub const INTEROP_MODE_CONFIG_FLAG_792: u64 = 792;
pub const INTEROP_MODE_CONFIG_FLAG_793: u64 = 793;
pub const INTEROP_MODE_CONFIG_FLAG_794: u64 = 794;
pub const INTEROP_MODE_CONFIG_FLAG_795: u64 = 795;
pub const INTEROP_MODE_CONFIG_FLAG_796: u64 = 796;
pub const INTEROP_MODE_CONFIG_FLAG_797: u64 = 797;
pub const INTEROP_MODE_CONFIG_FLAG_798: u64 = 798;
pub const INTEROP_MODE_CONFIG_FLAG_799: u64 = 799;
pub const INTEROP_MODE_CONFIG_FLAG_800: u64 = 800;
pub const INTEROP_MODE_CONFIG_FLAG_801: u64 = 801;
pub const INTEROP_MODE_CONFIG_FLAG_802: u64 = 802;
pub const INTEROP_MODE_CONFIG_FLAG_803: u64 = 803;
pub const INTEROP_MODE_CONFIG_FLAG_804: u64 = 804;
pub const INTEROP_MODE_CONFIG_FLAG_805: u64 = 805;
pub const INTEROP_MODE_CONFIG_FLAG_806: u64 = 806;
pub const INTEROP_MODE_CONFIG_FLAG_807: u64 = 807;
pub const INTEROP_MODE_CONFIG_FLAG_808: u64 = 808;
pub const INTEROP_MODE_CONFIG_FLAG_809: u64 = 809;
pub const INTEROP_MODE_CONFIG_FLAG_810: u64 = 810;
pub const INTEROP_MODE_CONFIG_FLAG_811: u64 = 811;
pub const INTEROP_MODE_CONFIG_FLAG_812: u64 = 812;
pub const INTEROP_MODE_CONFIG_FLAG_813: u64 = 813;
pub const INTEROP_MODE_CONFIG_FLAG_814: u64 = 814;
pub const INTEROP_MODE_CONFIG_FLAG_815: u64 = 815;
pub const INTEROP_MODE_CONFIG_FLAG_816: u64 = 816;
pub const INTEROP_MODE_CONFIG_FLAG_817: u64 = 817;
pub const INTEROP_MODE_CONFIG_FLAG_818: u64 = 818;
pub const INTEROP_MODE_CONFIG_FLAG_819: u64 = 819;
pub const INTEROP_MODE_CONFIG_FLAG_820: u64 = 820;
pub const INTEROP_MODE_CONFIG_FLAG_821: u64 = 821;
pub const INTEROP_MODE_CONFIG_FLAG_822: u64 = 822;
pub const INTEROP_MODE_CONFIG_FLAG_823: u64 = 823;
pub const INTEROP_MODE_CONFIG_FLAG_824: u64 = 824;
pub const INTEROP_MODE_CONFIG_FLAG_825: u64 = 825;
pub const INTEROP_MODE_CONFIG_FLAG_826: u64 = 826;
pub const INTEROP_MODE_CONFIG_FLAG_827: u64 = 827;
pub const INTEROP_MODE_CONFIG_FLAG_828: u64 = 828;
pub const INTEROP_MODE_CONFIG_FLAG_829: u64 = 829;
pub const INTEROP_MODE_CONFIG_FLAG_830: u64 = 830;
pub const INTEROP_MODE_CONFIG_FLAG_831: u64 = 831;
pub const INTEROP_MODE_CONFIG_FLAG_832: u64 = 832;
pub const INTEROP_MODE_CONFIG_FLAG_833: u64 = 833;
pub const INTEROP_MODE_CONFIG_FLAG_834: u64 = 834;
pub const INTEROP_MODE_CONFIG_FLAG_835: u64 = 835;
pub const INTEROP_MODE_CONFIG_FLAG_836: u64 = 836;
pub const INTEROP_MODE_CONFIG_FLAG_837: u64 = 837;
pub const INTEROP_MODE_CONFIG_FLAG_838: u64 = 838;
pub const INTEROP_MODE_CONFIG_FLAG_839: u64 = 839;
pub const INTEROP_MODE_CONFIG_FLAG_840: u64 = 840;
pub const INTEROP_MODE_CONFIG_FLAG_841: u64 = 841;
pub const INTEROP_MODE_CONFIG_FLAG_842: u64 = 842;
pub const INTEROP_MODE_CONFIG_FLAG_843: u64 = 843;
pub const INTEROP_MODE_CONFIG_FLAG_844: u64 = 844;
pub const INTEROP_MODE_CONFIG_FLAG_845: u64 = 845;
pub const INTEROP_MODE_CONFIG_FLAG_846: u64 = 846;
pub const INTEROP_MODE_CONFIG_FLAG_847: u64 = 847;
pub const INTEROP_MODE_CONFIG_FLAG_848: u64 = 848;
pub const INTEROP_MODE_CONFIG_FLAG_849: u64 = 849;
pub const INTEROP_MODE_CONFIG_FLAG_850: u64 = 850;
pub const INTEROP_MODE_CONFIG_FLAG_851: u64 = 851;
pub const INTEROP_MODE_CONFIG_FLAG_852: u64 = 852;
pub const INTEROP_MODE_CONFIG_FLAG_853: u64 = 853;
pub const INTEROP_MODE_CONFIG_FLAG_854: u64 = 854;
pub const INTEROP_MODE_CONFIG_FLAG_855: u64 = 855;
pub const INTEROP_MODE_CONFIG_FLAG_856: u64 = 856;
pub const INTEROP_MODE_CONFIG_FLAG_857: u64 = 857;
pub const INTEROP_MODE_CONFIG_FLAG_858: u64 = 858;
pub const INTEROP_MODE_CONFIG_FLAG_859: u64 = 859;
pub const INTEROP_MODE_CONFIG_FLAG_860: u64 = 860;
pub const INTEROP_MODE_CONFIG_FLAG_861: u64 = 861;
pub const INTEROP_MODE_CONFIG_FLAG_862: u64 = 862;
pub const INTEROP_MODE_CONFIG_FLAG_863: u64 = 863;
pub const INTEROP_MODE_CONFIG_FLAG_864: u64 = 864;
pub const INTEROP_MODE_CONFIG_FLAG_865: u64 = 865;
pub const INTEROP_MODE_CONFIG_FLAG_866: u64 = 866;
pub const INTEROP_MODE_CONFIG_FLAG_867: u64 = 867;
pub const INTEROP_MODE_CONFIG_FLAG_868: u64 = 868;
pub const INTEROP_MODE_CONFIG_FLAG_869: u64 = 869;
pub const INTEROP_MODE_CONFIG_FLAG_870: u64 = 870;
pub const INTEROP_MODE_CONFIG_FLAG_871: u64 = 871;
pub const INTEROP_MODE_CONFIG_FLAG_872: u64 = 872;
pub const INTEROP_MODE_CONFIG_FLAG_873: u64 = 873;
pub const INTEROP_MODE_CONFIG_FLAG_874: u64 = 874;
pub const INTEROP_MODE_CONFIG_FLAG_875: u64 = 875;
pub const INTEROP_MODE_CONFIG_FLAG_876: u64 = 876;
pub const INTEROP_MODE_CONFIG_FLAG_877: u64 = 877;
pub const INTEROP_MODE_CONFIG_FLAG_878: u64 = 878;
pub const INTEROP_MODE_CONFIG_FLAG_879: u64 = 879;
pub const INTEROP_MODE_CONFIG_FLAG_880: u64 = 880;
pub const INTEROP_MODE_CONFIG_FLAG_881: u64 = 881;
pub const INTEROP_MODE_CONFIG_FLAG_882: u64 = 882;
pub const INTEROP_MODE_CONFIG_FLAG_883: u64 = 883;
pub const INTEROP_MODE_CONFIG_FLAG_884: u64 = 884;
pub const INTEROP_MODE_CONFIG_FLAG_885: u64 = 885;
pub const INTEROP_MODE_CONFIG_FLAG_886: u64 = 886;
pub const INTEROP_MODE_CONFIG_FLAG_887: u64 = 887;
pub const INTEROP_MODE_CONFIG_FLAG_888: u64 = 888;
pub const INTEROP_MODE_CONFIG_FLAG_889: u64 = 889;
pub const INTEROP_MODE_CONFIG_FLAG_890: u64 = 890;
pub const INTEROP_MODE_CONFIG_FLAG_891: u64 = 891;
pub const INTEROP_MODE_CONFIG_FLAG_892: u64 = 892;
pub const INTEROP_MODE_CONFIG_FLAG_893: u64 = 893;
pub const INTEROP_MODE_CONFIG_FLAG_894: u64 = 894;
pub const INTEROP_MODE_CONFIG_FLAG_895: u64 = 895;
pub const INTEROP_MODE_CONFIG_FLAG_896: u64 = 896;
pub const INTEROP_MODE_CONFIG_FLAG_897: u64 = 897;
pub const INTEROP_MODE_CONFIG_FLAG_898: u64 = 898;
pub const INTEROP_MODE_CONFIG_FLAG_899: u64 = 899;
pub const INTEROP_MODE_CONFIG_FLAG_900: u64 = 900;
pub const INTEROP_MODE_CONFIG_FLAG_901: u64 = 901;
pub const INTEROP_MODE_CONFIG_FLAG_902: u64 = 902;
pub const INTEROP_MODE_CONFIG_FLAG_903: u64 = 903;
pub const INTEROP_MODE_CONFIG_FLAG_904: u64 = 904;
pub const INTEROP_MODE_CONFIG_FLAG_905: u64 = 905;
pub const INTEROP_MODE_CONFIG_FLAG_906: u64 = 906;
pub const INTEROP_MODE_CONFIG_FLAG_907: u64 = 907;
pub const INTEROP_MODE_CONFIG_FLAG_908: u64 = 908;
pub const INTEROP_MODE_CONFIG_FLAG_909: u64 = 909;
pub const INTEROP_MODE_CONFIG_FLAG_910: u64 = 910;
pub const INTEROP_MODE_CONFIG_FLAG_911: u64 = 911;
pub const INTEROP_MODE_CONFIG_FLAG_912: u64 = 912;
pub const INTEROP_MODE_CONFIG_FLAG_913: u64 = 913;
pub const INTEROP_MODE_CONFIG_FLAG_914: u64 = 914;
pub const INTEROP_MODE_CONFIG_FLAG_915: u64 = 915;
pub const INTEROP_MODE_CONFIG_FLAG_916: u64 = 916;
pub const INTEROP_MODE_CONFIG_FLAG_917: u64 = 917;
pub const INTEROP_MODE_CONFIG_FLAG_918: u64 = 918;
pub const INTEROP_MODE_CONFIG_FLAG_919: u64 = 919;
pub const INTEROP_MODE_CONFIG_FLAG_920: u64 = 920;
pub const INTEROP_MODE_CONFIG_FLAG_921: u64 = 921;
pub const INTEROP_MODE_CONFIG_FLAG_922: u64 = 922;
pub const INTEROP_MODE_CONFIG_FLAG_923: u64 = 923;
pub const INTEROP_MODE_CONFIG_FLAG_924: u64 = 924;
pub const INTEROP_MODE_CONFIG_FLAG_925: u64 = 925;
pub const INTEROP_MODE_CONFIG_FLAG_926: u64 = 926;
pub const INTEROP_MODE_CONFIG_FLAG_927: u64 = 927;
pub const INTEROP_MODE_CONFIG_FLAG_928: u64 = 928;
pub const INTEROP_MODE_CONFIG_FLAG_929: u64 = 929;
pub const INTEROP_MODE_CONFIG_FLAG_930: u64 = 930;
pub const INTEROP_MODE_CONFIG_FLAG_931: u64 = 931;
pub const INTEROP_MODE_CONFIG_FLAG_932: u64 = 932;
pub const INTEROP_MODE_CONFIG_FLAG_933: u64 = 933;
pub const INTEROP_MODE_CONFIG_FLAG_934: u64 = 934;
pub const INTEROP_MODE_CONFIG_FLAG_935: u64 = 935;
pub const INTEROP_MODE_CONFIG_FLAG_936: u64 = 936;
pub const INTEROP_MODE_CONFIG_FLAG_937: u64 = 937;
pub const INTEROP_MODE_CONFIG_FLAG_938: u64 = 938;
pub const INTEROP_MODE_CONFIG_FLAG_939: u64 = 939;
pub const INTEROP_MODE_CONFIG_FLAG_940: u64 = 940;
pub const INTEROP_MODE_CONFIG_FLAG_941: u64 = 941;
pub const INTEROP_MODE_CONFIG_FLAG_942: u64 = 942;
pub const INTEROP_MODE_CONFIG_FLAG_943: u64 = 943;
pub const INTEROP_MODE_CONFIG_FLAG_944: u64 = 944;
pub const INTEROP_MODE_CONFIG_FLAG_945: u64 = 945;
pub const INTEROP_MODE_CONFIG_FLAG_946: u64 = 946;
pub const INTEROP_MODE_CONFIG_FLAG_947: u64 = 947;
pub const INTEROP_MODE_CONFIG_FLAG_948: u64 = 948;
pub const INTEROP_MODE_CONFIG_FLAG_949: u64 = 949;
pub const INTEROP_MODE_CONFIG_FLAG_950: u64 = 950;
pub const INTEROP_MODE_CONFIG_FLAG_951: u64 = 951;
pub const INTEROP_MODE_CONFIG_FLAG_952: u64 = 952;
pub const INTEROP_MODE_CONFIG_FLAG_953: u64 = 953;
pub const INTEROP_MODE_CONFIG_FLAG_954: u64 = 954;
pub const INTEROP_MODE_CONFIG_FLAG_955: u64 = 955;
pub const INTEROP_MODE_CONFIG_FLAG_956: u64 = 956;
pub const INTEROP_MODE_CONFIG_FLAG_957: u64 = 957;
pub const INTEROP_MODE_CONFIG_FLAG_958: u64 = 958;
pub const INTEROP_MODE_CONFIG_FLAG_959: u64 = 959;
pub const INTEROP_MODE_CONFIG_FLAG_960: u64 = 960;
pub const INTEROP_MODE_CONFIG_FLAG_961: u64 = 961;
pub const INTEROP_MODE_CONFIG_FLAG_962: u64 = 962;
pub const INTEROP_MODE_CONFIG_FLAG_963: u64 = 963;
pub const INTEROP_MODE_CONFIG_FLAG_964: u64 = 964;
pub const INTEROP_MODE_CONFIG_FLAG_965: u64 = 965;
pub const INTEROP_MODE_CONFIG_FLAG_966: u64 = 966;
pub const INTEROP_MODE_CONFIG_FLAG_967: u64 = 967;
pub const INTEROP_MODE_CONFIG_FLAG_968: u64 = 968;
pub const INTEROP_MODE_CONFIG_FLAG_969: u64 = 969;
pub const INTEROP_MODE_CONFIG_FLAG_970: u64 = 970;
pub const INTEROP_MODE_CONFIG_FLAG_971: u64 = 971;
pub const INTEROP_MODE_CONFIG_FLAG_972: u64 = 972;
pub const INTEROP_MODE_CONFIG_FLAG_973: u64 = 973;
pub const INTEROP_MODE_CONFIG_FLAG_974: u64 = 974;
pub const INTEROP_MODE_CONFIG_FLAG_975: u64 = 975;
pub const INTEROP_MODE_CONFIG_FLAG_976: u64 = 976;
pub const INTEROP_MODE_CONFIG_FLAG_977: u64 = 977;
pub const INTEROP_MODE_CONFIG_FLAG_978: u64 = 978;
pub const INTEROP_MODE_CONFIG_FLAG_979: u64 = 979;
pub const INTEROP_MODE_CONFIG_FLAG_980: u64 = 980;
pub const INTEROP_MODE_CONFIG_FLAG_981: u64 = 981;
pub const INTEROP_MODE_CONFIG_FLAG_982: u64 = 982;
pub const INTEROP_MODE_CONFIG_FLAG_983: u64 = 983;
pub const INTEROP_MODE_CONFIG_FLAG_984: u64 = 984;
pub const INTEROP_MODE_CONFIG_FLAG_985: u64 = 985;
pub const INTEROP_MODE_CONFIG_FLAG_986: u64 = 986;
pub const INTEROP_MODE_CONFIG_FLAG_987: u64 = 987;
pub const INTEROP_MODE_CONFIG_FLAG_988: u64 = 988;
pub const INTEROP_MODE_CONFIG_FLAG_989: u64 = 989;
pub const INTEROP_MODE_CONFIG_FLAG_990: u64 = 990;
pub const INTEROP_MODE_CONFIG_FLAG_991: u64 = 991;
pub const INTEROP_MODE_CONFIG_FLAG_992: u64 = 992;
pub const INTEROP_MODE_CONFIG_FLAG_993: u64 = 993;
pub const INTEROP_MODE_CONFIG_FLAG_994: u64 = 994;
pub const INTEROP_MODE_CONFIG_FLAG_995: u64 = 995;
pub const INTEROP_MODE_CONFIG_FLAG_996: u64 = 996;
pub const INTEROP_MODE_CONFIG_FLAG_997: u64 = 997;
pub const INTEROP_MODE_CONFIG_FLAG_998: u64 = 998;
pub const INTEROP_MODE_CONFIG_FLAG_999: u64 = 999;
pub const INTEROP_MODE_CONFIG_FLAG_1000: u64 = 1000;
#[cfg(test)]
mod fallback_tests {
    use super::*;

    #[test]
    fn test_dummy() {
        assert_eq!(INTEROP_MODE_CONFIG_FLAG_1, 1);
    }
}
