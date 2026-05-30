use crate::msgbus::{Bus, DistributedLock, Message};
#[cfg(test)]
use crate::msgbus::MemoryBus;
#[cfg(test)]
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::time::{sleep, timeout, Duration};

pub mod proto {
    pub use ::server_ohc::interop::*;
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

        let handoff_msg = ::server_ohc::interop::StateHandoff {
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
    pub async fn listen_for_state_handoff(&self, handler: Box<dyn Fn(::server_ohc::interop::StateHandoff) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = ::server_ohc::interop::StateHandoff::decode(&msg.payload[..]) {
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
                if let Ok(decoded) = ::server_ohc::interop::HealthPing::decode(&msg.payload[..]) {
                    let ack = ::server_ohc::interop::HealthAck {
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

        let ping = ::server_ohc::interop::HealthPing {
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

        let dispatch = ::server_ohc::interop::JobDispatch {
            job_id: job_id.to_string(),
            tenant_id: tenant_id.to_string(),
            action_name: action_name.to_string(),
            payload,
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
                if let Ok(decoded) = ::server_ohc::interop::JobDispatch::decode(&msg.payload[..]) {
                    // In a real implementation, we would process the job here or send it to a worker pool
                    // Here, we just acknowledge receipt
                    let ack = ::server_ohc::interop::JobAck {
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

        let update = ::server_ohc::interop::JobStatusUpdate {
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
    pub async fn listen_for_job_status(&self, job_id: &str, handler: Box<dyn Fn(::server_ohc::interop::JobStatusUpdate) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic.starts_with("system:job_status:") {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = ::server_ohc::interop::JobStatusUpdate::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        self.bus.subscribe(format!("system:job_status:{}", job_id), bus_handler).await
    }

    /// Synchronizes a QueueJob across modes idempotently
    pub async fn sync_queue_job(&self, job: ::server_ohc::interop::QueueJob) -> Result<(), String> {
        use prost::Message as ProstMessage;

        // Idempotency check: ensure we don't duplicate syncing the EXACT same state transition
        // by including updated_at_ms in the lock resource.
        let idempotency_lock_resource = format!("queue_job:processed:{}_{}", job.id, job.updated_at_ms);
        let attempt_owner = format!("{}_{}", self.node_id, chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));

        if !self.lock.acquire_lock(&idempotency_lock_resource, &attempt_owner, 3600).await.unwrap_or(false) {
            return Ok(());
        }

        let mut buf = Vec::new();
        if let Err(e) = job.encode(&mut buf) {
            let _ = self.lock.release_lock(&idempotency_lock_resource, &attempt_owner).await;
            return Err(e.to_string());
        }

        let msg = Message {
            topic: format!("system:queue_job_sync:{}", job.tenant_id),
            payload: buf,
        };

        let mut retries = 0;
        let mut delay_ms = 100;
        let result = loop {
            match self.bus.publish(msg.clone()).await {
                Ok(_) => break Ok(()),
                Err(e) => {
                    if retries >= 5 {
                        break Err(format!("Failed to publish queue job sync after retries: {}", e));
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

        result
    }

    /// Listens for queue job synchronizations
    pub async fn listen_for_queue_jobs(&self, tenant_id: &str, handler: Box<dyn Fn(::server_ohc::interop::QueueJob) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic.starts_with("system:queue_job_sync:") {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = ::server_ohc::interop::QueueJob::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        self.bus.subscribe(format!("system:queue_job_sync:{}", tenant_id), bus_handler).await
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
                let decoded = ::server_ohc::interop::StateHandoff::decode(&msg.payload[..]).unwrap();
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
                let decoded = ::server_ohc::interop::StateHandoff::decode(&msg.payload[..]).unwrap();
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

        let handler = Box::new(move |msg: ::server_ohc::interop::StateHandoff| {
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
        let ping = ::server_ohc::interop::HealthPing {
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
        let dispatch = ::server_ohc::interop::JobDispatch {
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

        let handler = Box::new(move |update: ::server_ohc::interop::JobStatusUpdate| {
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

        let handler = Box::new(move |_msg: ::server_ohc::interop::StateHandoff| {
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

        let handler = Box::new(move |_update: ::server_ohc::interop::JobStatusUpdate| {
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
