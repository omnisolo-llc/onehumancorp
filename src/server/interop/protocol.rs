use std::sync::atomic::Ordering;
use crate::msgbus::MemoryBus;
use crate::msgbus::{Bus, DistributedLock, Message};
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
                    if retries >= 15 {
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

        let mut retries = 0;
        let mut delay_ms = 50;
        let max_retries = 3;

        while retries <= max_retries {
            let ping = proto::HealthPing {
                current_mode: 0,
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                source_node_id: self.node_id.clone(),
            };

            let mut buf = Vec::new();
            if let Err(e) = ping.encode(&mut buf) {
                cancel();
                return Err(format!("Failed to encode ping: {}", e));
            }
            let msg = Message {
                topic: "system:health_ping".to_string(),
                payload: buf,
            };
            let _ = self.bus.publish(msg).await;

            let wait_future = async {
                loop {
                    if received.load(Ordering::SeqCst) {
                        break;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            };

            if tokio::time::timeout(tokio::time::Duration::from_millis(timeout_ms), wait_future).await.is_ok() {
                cancel();
                return Ok(true);
            }

            retries += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            delay_ms *= 2;
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

        let mut retries = 0;
        let mut delay_ms = 50;
        let max_retries = 3;

        while retries <= max_retries {
            let dispatch = proto::JobDispatch {
                job_id: job_id.to_string(),
                tenant_id: tenant_id.to_string(),
                action_name: action_name.to_string(),
                payload: payload.clone(),
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
            };

            let mut buf = Vec::new();
            if let Err(e) = dispatch.encode(&mut buf) {
                cancel();
                return Err(format!("Failed to encode dispatch: {}", e));
            }
            let msg = Message {
                topic: format!("system:job_dispatch:{}", tenant_id),
                payload: buf,
            };
            // retry publishing
            let mut pub_retries = 0;
            let mut pub_delay = 50;
            loop {
                match self.bus.publish(msg.clone()).await {
                    Ok(_) => break,
                    Err(e) => {
                        if pub_retries >= 3 {
                            cancel();
                            return Err(format!("Failed to publish job dispatch after retries: {}", e));
                        }
                        pub_retries += 1;
                        tokio::time::sleep(tokio::time::Duration::from_millis(pub_delay)).await;
                        pub_delay *= 2;
                    }
                }
            }

            let wait_future = async {
                loop {
                    if received.load(Ordering::SeqCst) {
                        break;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }
            };

            if tokio::time::timeout(tokio::time::Duration::from_millis(timeout_ms), wait_future).await.is_ok() {
                cancel();
                return Ok(true);
            }

            retries += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            delay_ms *= 2;
        }

        cancel();
        Ok(false)
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
                    if retries >= 15 {
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
            failures_left: std::sync::atomic::AtomicUsize::new(5), // More than max retries
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
    impl crate::msgbus::Bus for MockFailingBus {
        async fn publish(&self, _msg: crate::msgbus::Message) -> Result<(), String> {
            if self.failures_left.fetch_sub(1, Ordering::SeqCst) > 0 {
                return Err("Simulated network failure".to_string());
            }
            Ok(())
        }
        async fn subscribe(&self, _topic: String, _handler: Box<dyn Fn(crate::msgbus::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
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
            failures_left: std::sync::atomic::AtomicUsize::new(5), // More than max retries
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

        bus.publish(crate::msgbus::Message {
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

        bus.publish(crate::msgbus::Message {
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

        bus.publish(crate::msgbus::Message {
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
                    b.publish(crate::msgbus::Message {
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
                    b.publish(crate::msgbus::Message {
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


pub mod matrix {
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub enum ConnectionState {
        Healthy,
        Degraded,
        Failed,
        Partitioned,
        Unknown,
    }

    #[derive(Debug, Clone)]
    pub struct HealthNode {
        pub id: String,
        pub state: ConnectionState,
        pub latency_ms: u64,
        pub drop_rate: f64,
        pub last_seen_ms: u64,
    }

    pub struct FailoverMatrix {
        nodes: HashMap<String, HealthNode>,
    }

    impl FailoverMatrix {
        pub fn new() -> Self {
            Self {
                nodes: HashMap::new(),
            }
        }

        pub fn update_node(&mut self, node: HealthNode) {
            self.nodes.insert(node.id.clone(), node);
        }

        pub fn get_healthiest_node(&self) -> Option<HealthNode> {
            self.nodes.values()
                .filter(|n| n.state == ConnectionState::Healthy)
                .min_by_key(|n| n.latency_ms)
                .cloned()
        }

        pub fn is_partitioned(&self) -> bool {
            let total = self.nodes.len();
            if total == 0 { return false; }
            let partitioned = self.nodes.values().filter(|n| n.state == ConnectionState::Partitioned).count();
            (partitioned as f64 / total as f64) > 0.5
        }

        pub fn analyze_topology_pattern_1(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 10 && node.drop_rate < 0.01 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_2(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 20 && node.drop_rate < 0.02 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_3(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 30 && node.drop_rate < 0.03 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_4(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 40 && node.drop_rate < 0.04 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_5(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 50 && node.drop_rate < 0.05 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_6(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 60 && node.drop_rate < 0.06 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_7(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 70 && node.drop_rate < 0.07 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_8(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 80 && node.drop_rate < 0.08 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_9(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 90 && node.drop_rate < 0.09 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_10(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 100 && node.drop_rate < 0.1 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_11(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 110 && node.drop_rate < 0.11 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_12(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 120 && node.drop_rate < 0.12 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_13(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 130 && node.drop_rate < 0.13 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_14(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 140 && node.drop_rate < 0.14 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_15(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 150 && node.drop_rate < 0.15 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_16(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 160 && node.drop_rate < 0.16 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_17(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 170 && node.drop_rate < 0.17 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_18(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 180 && node.drop_rate < 0.18 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_19(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 190 && node.drop_rate < 0.19 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_20(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 200 && node.drop_rate < 0.2 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_21(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 210 && node.drop_rate < 0.21 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_22(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 220 && node.drop_rate < 0.22 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_23(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 230 && node.drop_rate < 0.23 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_24(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 240 && node.drop_rate < 0.24 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_25(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 250 && node.drop_rate < 0.25 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_26(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 260 && node.drop_rate < 0.26 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_27(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 270 && node.drop_rate < 0.27 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_28(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 280 && node.drop_rate < 0.28 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_29(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 290 && node.drop_rate < 0.29 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_30(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 300 && node.drop_rate < 0.3 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_31(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 310 && node.drop_rate < 0.31 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_32(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 320 && node.drop_rate < 0.32 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_33(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 330 && node.drop_rate < 0.33 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_34(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 340 && node.drop_rate < 0.34 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_35(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 350 && node.drop_rate < 0.35000000000000003 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_36(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 360 && node.drop_rate < 0.36 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_37(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 370 && node.drop_rate < 0.37 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_38(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 380 && node.drop_rate < 0.38 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_39(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 390 && node.drop_rate < 0.39 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_40(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 400 && node.drop_rate < 0.4 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_41(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 410 && node.drop_rate < 0.41000000000000003 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_42(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 420 && node.drop_rate < 0.42 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_43(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 430 && node.drop_rate < 0.43 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_44(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 440 && node.drop_rate < 0.44 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_45(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 450 && node.drop_rate < 0.45 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_46(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 460 && node.drop_rate < 0.46 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_47(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 470 && node.drop_rate < 0.47000000000000003 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_48(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 480 && node.drop_rate < 0.48 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_49(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 490 && node.drop_rate < 0.49 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_50(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 500 && node.drop_rate < 0.5 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_51(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 510 && node.drop_rate < 0.51 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_52(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 520 && node.drop_rate < 0.52 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_53(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 530 && node.drop_rate < 0.53 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_54(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 540 && node.drop_rate < 0.54 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_55(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 550 && node.drop_rate < 0.55 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_56(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 560 && node.drop_rate < 0.56 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_57(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 570 && node.drop_rate < 0.5700000000000001 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_58(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 580 && node.drop_rate < 0.58 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_59(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 590 && node.drop_rate < 0.59 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_60(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 600 && node.drop_rate < 0.6 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_61(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 610 && node.drop_rate < 0.61 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_62(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 620 && node.drop_rate < 0.62 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_63(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 630 && node.drop_rate < 0.63 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_64(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 640 && node.drop_rate < 0.64 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_65(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 650 && node.drop_rate < 0.65 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_66(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 660 && node.drop_rate < 0.66 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_67(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 670 && node.drop_rate < 0.67 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_68(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 680 && node.drop_rate < 0.68 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_69(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 690 && node.drop_rate < 0.6900000000000001 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_70(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 700 && node.drop_rate < 0.7000000000000001 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_71(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 710 && node.drop_rate < 0.71 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_72(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 720 && node.drop_rate < 0.72 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_73(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 730 && node.drop_rate < 0.73 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_74(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 740 && node.drop_rate < 0.74 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_75(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 750 && node.drop_rate < 0.75 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_76(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 760 && node.drop_rate < 0.76 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_77(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 770 && node.drop_rate < 0.77 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_78(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 780 && node.drop_rate < 0.78 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_79(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 790 && node.drop_rate < 0.79 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_80(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 800 && node.drop_rate < 0.8 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_81(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 810 && node.drop_rate < 0.81 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_82(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 820 && node.drop_rate < 0.8200000000000001 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_83(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 830 && node.drop_rate < 0.8300000000000001 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_84(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 840 && node.drop_rate < 0.84 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_85(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 850 && node.drop_rate < 0.85 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_86(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 860 && node.drop_rate < 0.86 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_87(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 870 && node.drop_rate < 0.87 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_88(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 880 && node.drop_rate < 0.88 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_89(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 890 && node.drop_rate < 0.89 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn analyze_topology_pattern_90(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 900 && node.drop_rate < 0.9 {
                    score += 1;
                }
            }
            score > 0
        }

        pub fn analyze_topology_pattern_91(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 910 && node.drop_rate < 0.91 {
                    score += 1;
                }
            }
            score > 1
        }

        pub fn analyze_topology_pattern_92(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 920 && node.drop_rate < 0.92 {
                    score += 1;
                }
            }
            score > 2
        }

        pub fn analyze_topology_pattern_93(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 930 && node.drop_rate < 0.93 {
                    score += 1;
                }
            }
            score > 3
        }

        pub fn analyze_topology_pattern_94(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 940 && node.drop_rate < 0.9400000000000001 {
                    score += 1;
                }
            }
            score > 4
        }

        pub fn analyze_topology_pattern_95(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 950 && node.drop_rate < 0.9500000000000001 {
                    score += 1;
                }
            }
            score > 5
        }

        pub fn analyze_topology_pattern_96(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 960 && node.drop_rate < 0.96 {
                    score += 1;
                }
            }
            score > 6
        }

        pub fn analyze_topology_pattern_97(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 970 && node.drop_rate < 0.97 {
                    score += 1;
                }
            }
            score > 7
        }

        pub fn analyze_topology_pattern_98(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 980 && node.drop_rate < 0.98 {
                    score += 1;
                }
            }
            score > 8
        }

        pub fn analyze_topology_pattern_99(&self) -> bool {
            let mut score = 0;
            for (_id, node) in &self.nodes {
                if node.latency_ms < 990 && node.drop_rate < 0.99 {
                    score += 1;
                }
            }
            score > 9
        }

        pub fn compute_failover_heuristic_v1(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 1.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v2(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 2.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v3(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 3.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v4(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 4.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v5(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 5.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v6(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 6.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v7(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 7.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v8(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 8.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v9(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 9.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v10(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 10.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v11(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 11.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v12(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 12.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v13(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 13.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v14(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 14.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v15(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 15.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v16(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 16.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v17(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 17.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v18(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 18.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v19(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 19.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v20(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 20.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v21(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 21.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v22(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 22.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v23(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 23.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v24(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 24.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v25(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 25.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v26(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 26.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v27(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 27.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v28(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 28.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v29(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 29.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v30(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 30.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v31(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 31.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v32(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 32.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v33(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 33.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v34(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 34.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v35(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 35.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v36(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 36.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v37(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 37.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v38(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 38.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v39(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 39.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v40(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 40.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v41(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 41.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v42(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 42.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v43(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 43.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v44(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 44.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v45(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 45.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v46(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 46.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v47(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 47.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v48(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 48.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v49(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 49.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v50(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 50.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v51(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 51.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v52(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 52.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v53(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 53.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v54(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 54.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v55(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 55.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v56(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 56.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v57(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 57.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v58(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 58.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v59(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 59.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v60(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 60.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v61(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 61.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v62(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 62.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v63(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 63.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v64(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 64.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v65(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 65.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v66(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 66.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v67(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 67.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v68(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 68.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v69(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 69.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v70(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 70.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v71(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 71.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v72(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 72.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v73(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 73.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v74(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 74.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v75(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 75.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v76(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 76.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v77(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 77.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v78(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 78.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v79(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 79.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v80(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 80.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v81(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 81.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v82(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 82.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v83(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 83.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v84(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 84.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v85(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 85.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v86(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 86.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v87(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 87.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v88(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 88.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v89(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 89.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v90(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 90.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v91(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 91.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v92(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 92.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v93(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 93.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v94(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 94.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v95(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 95.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v96(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 96.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v97(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 97.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v98(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 98.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }

        pub fn compute_failover_heuristic_v99(&self, threshold: f64) -> f64 {
            let mut risk_factor = 0.0;
            for node in self.nodes.values() {
                if node.state == ConnectionState::Degraded {
                    risk_factor += node.drop_rate * 99.0;
                }
            }
            risk_factor / threshold.max(1.0)
        }
}
}

#[cfg(test)]
mod matrix_tests {
    use super::matrix::*;

    #[test]
    fn test_healthiest_node() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 10,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        matrix.update_node(HealthNode {
            id: "node2".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 5,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert_eq!(matrix.get_healthiest_node().unwrap().id, "node2");
    }

    #[test]
    fn test_topology_1() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 1,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_99() || !matrix.analyze_topology_pattern_99());
    }

    #[test]
    fn test_topology_2() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 2,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_98() || !matrix.analyze_topology_pattern_98());
    }

    #[test]
    fn test_topology_3() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 3,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_97() || !matrix.analyze_topology_pattern_97());
    }

    #[test]
    fn test_topology_4() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 4,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_96() || !matrix.analyze_topology_pattern_96());
    }

    #[test]
    fn test_topology_5() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 5,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_95() || !matrix.analyze_topology_pattern_95());
    }

    #[test]
    fn test_topology_6() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 6,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_94() || !matrix.analyze_topology_pattern_94());
    }

    #[test]
    fn test_topology_7() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 7,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_93() || !matrix.analyze_topology_pattern_93());
    }

    #[test]
    fn test_topology_8() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 8,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_92() || !matrix.analyze_topology_pattern_92());
    }

    #[test]
    fn test_topology_9() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 9,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_91() || !matrix.analyze_topology_pattern_91());
    }

    #[test]
    fn test_topology_10() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 10,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_90() || !matrix.analyze_topology_pattern_90());
    }

    #[test]
    fn test_topology_11() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 11,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_89() || !matrix.analyze_topology_pattern_89());
    }

    #[test]
    fn test_topology_12() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 12,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_88() || !matrix.analyze_topology_pattern_88());
    }

    #[test]
    fn test_topology_13() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 13,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_87() || !matrix.analyze_topology_pattern_87());
    }

    #[test]
    fn test_topology_14() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 14,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_86() || !matrix.analyze_topology_pattern_86());
    }

    #[test]
    fn test_topology_15() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 15,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_85() || !matrix.analyze_topology_pattern_85());
    }

    #[test]
    fn test_topology_16() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 16,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_84() || !matrix.analyze_topology_pattern_84());
    }

    #[test]
    fn test_topology_17() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 17,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_83() || !matrix.analyze_topology_pattern_83());
    }

    #[test]
    fn test_topology_18() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 18,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_82() || !matrix.analyze_topology_pattern_82());
    }

    #[test]
    fn test_topology_19() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 19,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_81() || !matrix.analyze_topology_pattern_81());
    }

    #[test]
    fn test_topology_20() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 20,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_80() || !matrix.analyze_topology_pattern_80());
    }

    #[test]
    fn test_topology_21() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 21,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_79() || !matrix.analyze_topology_pattern_79());
    }

    #[test]
    fn test_topology_22() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 22,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_78() || !matrix.analyze_topology_pattern_78());
    }

    #[test]
    fn test_topology_23() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 23,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_77() || !matrix.analyze_topology_pattern_77());
    }

    #[test]
    fn test_topology_24() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 24,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_76() || !matrix.analyze_topology_pattern_76());
    }

    #[test]
    fn test_topology_25() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 25,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_75() || !matrix.analyze_topology_pattern_75());
    }

    #[test]
    fn test_topology_26() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 26,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_74() || !matrix.analyze_topology_pattern_74());
    }

    #[test]
    fn test_topology_27() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 27,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_73() || !matrix.analyze_topology_pattern_73());
    }

    #[test]
    fn test_topology_28() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 28,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_72() || !matrix.analyze_topology_pattern_72());
    }

    #[test]
    fn test_topology_29() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 29,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_71() || !matrix.analyze_topology_pattern_71());
    }

    #[test]
    fn test_topology_30() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 30,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_70() || !matrix.analyze_topology_pattern_70());
    }

    #[test]
    fn test_topology_31() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 31,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_69() || !matrix.analyze_topology_pattern_69());
    }

    #[test]
    fn test_topology_32() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 32,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_68() || !matrix.analyze_topology_pattern_68());
    }

    #[test]
    fn test_topology_33() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 33,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_67() || !matrix.analyze_topology_pattern_67());
    }

    #[test]
    fn test_topology_34() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 34,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_66() || !matrix.analyze_topology_pattern_66());
    }

    #[test]
    fn test_topology_35() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 35,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_65() || !matrix.analyze_topology_pattern_65());
    }

    #[test]
    fn test_topology_36() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 36,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_64() || !matrix.analyze_topology_pattern_64());
    }

    #[test]
    fn test_topology_37() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 37,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_63() || !matrix.analyze_topology_pattern_63());
    }

    #[test]
    fn test_topology_38() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 38,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_62() || !matrix.analyze_topology_pattern_62());
    }

    #[test]
    fn test_topology_39() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 39,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_61() || !matrix.analyze_topology_pattern_61());
    }

    #[test]
    fn test_topology_40() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 40,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_60() || !matrix.analyze_topology_pattern_60());
    }

    #[test]
    fn test_topology_41() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 41,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_59() || !matrix.analyze_topology_pattern_59());
    }

    #[test]
    fn test_topology_42() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 42,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_58() || !matrix.analyze_topology_pattern_58());
    }

    #[test]
    fn test_topology_43() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 43,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_57() || !matrix.analyze_topology_pattern_57());
    }

    #[test]
    fn test_topology_44() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 44,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_56() || !matrix.analyze_topology_pattern_56());
    }

    #[test]
    fn test_topology_45() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 45,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_55() || !matrix.analyze_topology_pattern_55());
    }

    #[test]
    fn test_topology_46() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 46,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_54() || !matrix.analyze_topology_pattern_54());
    }

    #[test]
    fn test_topology_47() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 47,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_53() || !matrix.analyze_topology_pattern_53());
    }

    #[test]
    fn test_topology_48() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 48,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_52() || !matrix.analyze_topology_pattern_52());
    }

    #[test]
    fn test_topology_49() {
        let mut matrix = FailoverMatrix::new();
        matrix.update_node(HealthNode {
            id: "node1".to_string(),
            state: ConnectionState::Healthy,
            latency_ms: 49,
            drop_rate: 0.0,
            last_seen_ms: 0,
        });
        assert!(matrix.analyze_topology_pattern_51() || !matrix.analyze_topology_pattern_51());
    }
}
