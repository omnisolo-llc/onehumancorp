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
#[path = "protocol_test.rs"]
mod protocol_test;
