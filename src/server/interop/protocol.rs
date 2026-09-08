use crate::msgbus::{Bus, DistributedLock, Message};
use server_harness::middleware::capsule::{PortableRecord, SessionCapsule};

use tokio::time::{Duration, sleep, timeout};

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
pub mod proto {
    pub use ::server_ohc::interop::*;
}

pub const HARNESS_SESSION_OPERATION_TOPIC: &str = "system:harness_session_operation";
const HARNESS_SESSION_OPERATION_SCHEMA: &str = "omnisolo.session_capsule.v1";

#[derive(Clone, Debug)]
pub struct HarnessCapsuleOperation {
    pub envelope: ::server_ohc::harness_middleware::SessionOperationEnvelope,
    pub capsule: SessionCapsule,
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
    pub async fn handoff(
        &self,
        mission_id: &str,
        tenant_id: &str,
        state_payload: Vec<u8>,
    ) -> Result<(), String> {
        use prost::Message as ProstMessage;

        tracing::info!(mission_id = %mission_id, tenant_id = %tenant_id, "Initiating interop state handoff"); // pii-safe

        let lock_resource = format!("handoff:{}", mission_id);

        // Wait for lock with a timeout to prevent deadlocks and apply backoff.
        let acquire_future = async {
            let mut retries = 0;
            loop {
                if self
                    .lock
                    .acquire_lock(&lock_resource, &self.node_id, 10)
                    .await
                    .unwrap_or(false)
                {
                    break Ok::<(), ()>(());
                }
                tracing::debug!(mission_id = %mission_id, "Waiting to acquire handoff lock");
                retries += 1;
                let sleep_ms = 50 * retries;
                sleep(Duration::from_millis(sleep_ms)).await;
            }
        };

        if timeout(Duration::from_secs(5), acquire_future)
            .await
            .is_err()
        {
            tracing::error!(mission_id = %mission_id, "Timeout waiting for handoff lock");
            return Err("Timeout waiting for lock".to_string());
        }

        // Idempotency check: once we hold the execution lock, check if it was processed.
        let idempotency_lock_resource = format!("handoff:processed:{}", mission_id);
        // Generate a unique owner ID for this specific handoff attempt to prevent lock extension.
        let attempt_owner = format!(
            "{}_{}",
            self.node_id,
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        if !self
            .lock
            .acquire_lock(&idempotency_lock_resource, &attempt_owner, 3600)
            .await
            .unwrap_or(false)
        {
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
            let _ = self
                .lock
                .release_lock(&idempotency_lock_resource, &attempt_owner)
                .await;
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
                        break Err(format!(
                            "Failed to publish state handoff after retries: {}",
                            e
                        ));
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2; // Exponential backoff
                }
            }
        };

        if result.is_err() {
            // Failed to publish, release idempotency lock so it can be retried
            let _ = self
                .lock
                .release_lock(&idempotency_lock_resource, &attempt_owner)
                .await;
        }

        let _ = self.lock.release_lock(&lock_resource, &self.node_id).await;

        result
    }

    /// Resumes a mission after a mode switch
    pub async fn resume_mission(
        &self,
        mission_id: &str,
        tenant_id: &str,
        state_payload: Vec<u8>,
    ) -> Result<(), String> {
        // Handoff uses the same mechanism to synchronize state
        self.handoff(mission_id, tenant_id, state_payload).await
    }

    /// Publishes a verified portable capsule as a fenced, idempotent session operation.
    /// This is the typed handoff path for independently scaled harness workers; the
    /// legacy `StateHandoff` path above remains available for older deployments.
    pub async fn handoff_capsule(
        &self,
        capsule: &SessionCapsule,
        operation_generation: i64,
        fencing_token: &str,
    ) -> Result<(), String> {
        self.publish_capsule_operation(capsule, "handoff", operation_generation, fencing_token)
            .await
    }

    /// Publishes a verified portable capsule as a resume operation.
    pub async fn resume_capsule(
        &self,
        capsule: &SessionCapsule,
        operation_generation: i64,
        fencing_token: &str,
    ) -> Result<(), String> {
        self.publish_capsule_operation(capsule, "resume", operation_generation, fencing_token)
            .await
    }

    /// Listens for the protocol-neutral session operation envelope without
    /// interpreting its payload. This is useful for non-Rust harness workers.
    pub async fn listen_for_session_operations(
        &self,
        handler: Box<
            dyn Fn(::server_ohc::harness_middleware::SessionOperationEnvelope) + Send + Sync,
        >,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic != HARNESS_SESSION_OPERATION_TOPIC {
                return;
            }
            use prost::Message as ProstMessage;
            if let Ok(decoded) =
                ::server_ohc::harness_middleware::SessionOperationEnvelope::decode(&msg.payload[..])
            {
                handler(decoded);
            }
        });

        self.bus
            .subscribe(HARNESS_SESSION_OPERATION_TOPIC.to_owned(), bus_handler)
            .await
    }

    /// Listens for session operations and only forwards capsules that pass
    /// integrity, tenant, identity, version, and fencing validation.
    pub async fn listen_for_capsule_operations(
        &self,
        handler: Box<dyn Fn(HarnessCapsuleOperation) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.listen_for_session_operations(Box::new(
            move |envelope| match decode_capsule_operation(envelope) {
                Ok(operation) => handler(operation),
                Err(error) => {
                    tracing::warn!(error = %error, "Ignoring invalid harness capsule operation")
                }
            },
        ))
        .await
    }

    fn capsule_operation(
        &self,
        capsule: &SessionCapsule,
        kind: &str,
        operation_generation: i64,
        fencing_token: &str,
    ) -> Result<::server_ohc::harness_middleware::SessionOperationEnvelope, String> {
        capsule
            .verify_integrity()
            .map_err(|error| format!("capsule integrity verification failed: {error:?}"))?;
        if capsule.manifest.tenant_id.trim().is_empty() {
            return Err("capsule tenant id is empty".to_owned());
        }
        if capsule.manifest.session_id.is_nil() {
            return Err("capsule session id is empty".to_owned());
        }
        if capsule.manifest.handoff_id.is_nil() {
            return Err("capsule handoff id is empty".to_owned());
        }
        if capsule.manifest.target_harness_id.trim().is_empty() {
            return Err("capsule target harness id is empty".to_owned());
        }
        if operation_generation <= 0 {
            return Err("operation generation must be positive".to_owned());
        }
        if fencing_token.trim().is_empty() {
            return Err("fencing token must not be empty".to_owned());
        }
        if !matches!(kind, "handoff" | "resume") {
            return Err(format!("unsupported capsule operation kind: {kind}"));
        }

        let payload = serde_json::to_vec(capsule)
            .map_err(|error| format!("failed to encode session capsule: {error}"))?;
        let task_id = capsule.records.iter().find_map(|record| match record {
            PortableRecord::Task(task) => Some(task.task_id.to_string()),
            _ => None,
        });
        let idempotency_key = format!("capsule:{kind}:{}", capsule.manifest_digest);
        let extensions = [
            (
                "target_harness_id".to_owned(),
                capsule.manifest.target_harness_id.clone(),
            ),
            (
                "manifest_digest".to_owned(),
                capsule.manifest_digest.clone(),
            ),
            (
                "loss_report_digest".to_owned(),
                capsule.loss_report_digest.clone(),
            ),
        ]
        .into_iter()
        .collect();

        Ok(::server_ohc::harness_middleware::SessionOperationEnvelope {
            protocol_version: 1,
            tenant_id: capsule.manifest.tenant_id.clone(),
            session_id: capsule.manifest.session_id.to_string(),
            operation_id: capsule.manifest.handoff_id.to_string(),
            operation_generation,
            fencing_token: fencing_token.to_owned(),
            kind: kind.to_owned(),
            task_id: task_id.unwrap_or_default(),
            correlation_id: capsule
                .head_event_id
                .unwrap_or(capsule.manifest.session_id)
                .to_string(),
            idempotency_key,
            payload_schema: HARNESS_SESSION_OPERATION_SCHEMA.to_owned(),
            payload_version: capsule.manifest.schema_version,
            payload,
            extensions,
            worker_id: String::new(),
            pool_id: String::new(),
            harness_id: capsule.manifest.target_harness_id.clone(),
            capability_version: 1,
            binding_id: String::new(),
            binding_generation: 0,
            workspace_mutation_scope_id: String::new(),
        })
    }

    async fn publish_capsule_operation(
        &self,
        capsule: &SessionCapsule,
        kind: &str,
        operation_generation: i64,
        fencing_token: &str,
    ) -> Result<(), String> {
        use prost::Message as ProstMessage;

        let operation =
            self.capsule_operation(capsule, kind, operation_generation, fencing_token)?;
        let lock_resource = format!(
            "harness:session:{}:operation:{}:{}",
            operation.session_id, operation.operation_id, operation.kind
        );

        let acquire_future = async {
            let mut retries = 0;
            loop {
                if self
                    .lock
                    .acquire_lock(&lock_resource, &self.node_id, 10)
                    .await
                    .unwrap_or(false)
                {
                    break Ok::<(), ()>(());
                }
                retries += 1;
                sleep(Duration::from_millis(50 * retries)).await;
            }
        };
        if timeout(Duration::from_secs(5), acquire_future)
            .await
            .is_err()
        {
            return Err("Timeout waiting for harness session operation lock".to_owned());
        }

        let idempotency_key = operation.idempotency_key.clone();
        let idempotency_lock_resource =
            format!("harness:session-operation:processed:{}", idempotency_key);
        let attempt_owner = format!(
            "{}_{}",
            self.node_id,
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );
        if !self
            .lock
            .acquire_lock(&idempotency_lock_resource, &attempt_owner, 3600)
            .await
            .unwrap_or(false)
        {
            let _ = self.lock.release_lock(&lock_resource, &self.node_id).await;
            return Ok(());
        }

        let mut payload = Vec::new();
        operation
            .encode(&mut payload)
            .map_err(|error| format!("failed to encode harness session operation: {error}"))?;
        let message = Message {
            topic: HARNESS_SESSION_OPERATION_TOPIC.to_owned(),
            payload,
        };

        let mut retries = 0;
        let mut delay_ms = 100;
        let result = loop {
            match self.bus.publish(message.clone()).await {
                Ok(()) => break Ok(()),
                Err(error) if retries < 5 => {
                    retries += 1;
                    sleep(Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2;
                    tracing::debug!(error = %error, retries, "Retrying harness session operation publish");
                }
                Err(error) => {
                    break Err(format!(
                        "Failed to publish harness session operation after retries: {error}"
                    ));
                }
            }
        };

        if result.is_err() {
            let _ = self
                .lock
                .release_lock(&idempotency_lock_resource, &attempt_owner)
                .await;
        }
        let _ = self.lock.release_lock(&lock_resource, &self.node_id).await;
        result
    }

    /// Listens for state handoff updates
    pub async fn listen_for_state_handoff(
        &self,
        handler: Box<dyn Fn(::server_ohc::interop::StateHandoff) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = ::server_ohc::interop::StateHandoff::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        self.bus
            .subscribe("system:state_handoff".to_string(), bus_handler)
            .await
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
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                    .await;
                                delay_ms *= 2; // Exponential backoff
                            }
                        });
                    }
                }
            }
        });

        self.bus
            .subscribe("system:health_ping".to_string(), handler)
            .await
    }

    /// Health monitor across the swarm using protobuf
    pub async fn check_health(&self, timeout_ms: u64) -> Result<bool, String> {
        use prost::Message as ProstMessage;
        use std::sync::atomic::Ordering;

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:health_ack:{}", self.node_id);
        let handler = Box::new(move |msg: Message| {
            if msg.topic == ack_topic {
                rx.store(true, Ordering::SeqCst);
            }
        });

        let cancel = self
            .bus
            .subscribe(format!("system:health_ack:{}", self.node_id), handler)
            .await?;

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
    pub async fn dispatch_job(
        &self,
        job_id: &str,
        tenant_id: &str,
        action_name: &str,
        payload: Vec<u8>,
        timeout_ms: u64,
    ) -> Result<bool, String> {
        use prost::Message as ProstMessage;
        use std::sync::atomic::Ordering;

        tracing::info!(job_id = %job_id, tenant_id = %tenant_id, action_name = %action_name, "Dispatching background job"); // pii-safe

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:job_ack:{}", job_id);
        let handler = Box::new(move |msg: Message| {
            if msg.topic == ack_topic {
                rx.store(true, Ordering::SeqCst);
            }
        });

        let cancel = self
            .bus
            .subscribe(format!("system:job_ack:{}", job_id), handler)
            .await?;

        let dispatch = ::server_ohc::interop::JobDispatch {
            job_id: job_id.to_string(),
            tenant_id: tenant_id.to_string(),
            action_name: action_name.to_string(),
            payload,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        let mut buf = Vec::new();
        if let Err(e) = dispatch.encode(&mut buf) {
            tracing::error!(job_id = %job_id, error = %e, "Failed to encode job dispatch message");
            return Err(e.to_string());
        }

        let msg = Message {
            topic: format!("system:job_dispatch:{}", tenant_id),
            payload: buf,
        };

        // Add internal retry for publishing to ensure dispatch survives partitions
        let mut retries = 0;
        let mut delay_ms = 100;
        loop {
            match self.bus.publish(msg.clone()).await {
                Ok(_) => {
                    tracing::debug!(job_id = %job_id, "Successfully published job dispatch message");
                    break;
                }
                Err(e) => {
                    if retries >= 5 {
                        cancel();
                        tracing::error!(job_id = %job_id, error = %e, "Failed to publish job dispatch after max retries");
                        return Err(format!(
                            "Failed to publish job dispatch after retries: {}",
                            e
                        ));
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
                tracing::info!(job_id = %job_id, "Job dispatch acknowledged");
                return Ok(true);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        cancel();
        tracing::warn!(job_id = %job_id, timeout_ms = %timeout_ms, "Job dispatch timeout waiting for acknowledgment");
        Ok(false) // Not acked, implies failure/timeout, dispatch might need to be retried by the caller
    }

    /// Listens for job dispatches and acknowledges them
    pub async fn listen_for_jobs(
        &self,
        tenant_id: &str,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
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
                                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                    .await;
                                delay_ms *= 2; // Exponential backoff
                            }
                        });
                    }
                }
            }
        });

        self.bus
            .subscribe(format!("system:job_dispatch:{}", tenant_id), handler)
            .await
    }

    /// Reports job status back to the main server
    pub async fn report_job_status(
        &self,
        job_id: &str,
        tenant_id: &str,
        status: &str,
        details: Vec<u8>,
    ) -> Result<(), String> {
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
                        return Err(format!(
                            "Failed to publish job status update after retries: {}",
                            e
                        ));
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2; // Exponential backoff
                }
            }
        }
    }

    /// Listens for job status updates for a specific job
    pub async fn listen_for_job_status(
        &self,
        job_id: &str,
        handler: Box<dyn Fn(::server_ohc::interop::JobStatusUpdate) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic.starts_with("system:job_status:") {
                use prost::Message as ProstMessage;
                if let Ok(decoded) =
                    ::server_ohc::interop::JobStatusUpdate::decode(&msg.payload[..])
                {
                    handler(decoded);
                }
            }
        });

        self.bus
            .subscribe(format!("system:job_status:{}", job_id), bus_handler)
            .await
    }

    /// Synchronizes a QueueJob across modes idempotently
    pub async fn sync_queue_job(&self, job: ::server_ohc::interop::QueueJob) -> Result<(), String> {
        use prost::Message as ProstMessage;

        // Idempotency check: ensure we don't duplicate syncing the EXACT same state transition
        // by including updated_at_ms in the lock resource.
        let idempotency_lock_resource =
            format!("queue_job:processed:{}_{}", job.id, job.updated_at_ms);
        let attempt_owner = format!(
            "{}_{}",
            self.node_id,
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        );

        if !self
            .lock
            .acquire_lock(&idempotency_lock_resource, &attempt_owner, 3600)
            .await
            .unwrap_or(false)
        {
            return Ok(());
        }

        let mut buf = Vec::new();
        if let Err(e) = job.encode(&mut buf) {
            let _ = self
                .lock
                .release_lock(&idempotency_lock_resource, &attempt_owner)
                .await;
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
                        break Err(format!(
                            "Failed to publish queue job sync after retries: {}",
                            e
                        ));
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                    delay_ms *= 2; // Exponential backoff
                }
            }
        };

        if result.is_err() {
            // Failed to publish, release idempotency lock so it can be retried
            let _ = self
                .lock
                .release_lock(&idempotency_lock_resource, &attempt_owner)
                .await;
        }

        result
    }

    /// Listens for queue job synchronizations
    pub async fn listen_for_queue_jobs(
        &self,
        tenant_id: &str,
        handler: Box<dyn Fn(::server_ohc::interop::QueueJob) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let bus_handler = Box::new(move |msg: Message| {
            if msg.topic.starts_with("system:queue_job_sync:") {
                use prost::Message as ProstMessage;
                if let Ok(decoded) = ::server_ohc::interop::QueueJob::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        self.bus
            .subscribe(format!("system:queue_job_sync:{}", tenant_id), bus_handler)
            .await
    }
}

fn decode_capsule_operation(
    envelope: ::server_ohc::harness_middleware::SessionOperationEnvelope,
) -> Result<HarnessCapsuleOperation, String> {
    if envelope.protocol_version == 0 {
        return Err("unsupported session operation protocol version".to_owned());
    }
    if envelope.operation_generation <= 0 {
        return Err("operation generation must be positive".to_owned());
    }
    if envelope.fencing_token.trim().is_empty() {
        return Err("fencing token must not be empty".to_owned());
    }
    if !matches!(envelope.kind.as_str(), "handoff" | "resume") {
        return Err(format!(
            "unsupported capsule operation kind: {}",
            envelope.kind
        ));
    }
    if envelope.payload_schema != HARNESS_SESSION_OPERATION_SCHEMA {
        return Err("unsupported session capsule payload schema".to_owned());
    }

    let capsule: SessionCapsule = serde_json::from_slice(&envelope.payload)
        .map_err(|error| format!("invalid session capsule payload: {error}"))?;
    capsule
        .verify_integrity()
        .map_err(|error| format!("capsule integrity verification failed: {error:?}"))?;

    if envelope.tenant_id != capsule.manifest.tenant_id {
        return Err("session operation tenant does not match capsule".to_owned());
    }
    if envelope.session_id != capsule.manifest.session_id.to_string() {
        return Err("session operation session does not match capsule".to_owned());
    }
    if envelope.operation_id != capsule.manifest.handoff_id.to_string() {
        return Err("session operation id does not match capsule handoff".to_owned());
    }
    if envelope.payload_version != capsule.manifest.schema_version {
        return Err("session capsule schema version does not match envelope".to_owned());
    }

    let expected_idempotency = format!("capsule:{}:{}", envelope.kind, capsule.manifest_digest);
    if envelope.idempotency_key != expected_idempotency {
        return Err("session capsule idempotency key does not match manifest".to_owned());
    }
    if envelope.extensions.get("manifest_digest") != Some(&capsule.manifest_digest) {
        return Err("session capsule manifest digest extension does not match".to_owned());
    }
    if envelope.extensions.get("loss_report_digest") != Some(&capsule.loss_report_digest) {
        return Err("session capsule loss report digest extension does not match".to_owned());
    }
    if envelope.extensions.get("target_harness_id") != Some(&capsule.manifest.target_harness_id) {
        return Err("session capsule target harness extension does not match".to_owned());
    }

    let task_id = capsule.records.iter().find_map(|record| match record {
        PortableRecord::Task(task) => Some(task.task_id.to_string()),
        _ => None,
    });
    if envelope.task_id != task_id.unwrap_or_default() {
        return Err("session operation task does not match capsule".to_owned());
    }

    Ok(HarnessCapsuleOperation { envelope, capsule })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;
    use server_harness::middleware::adapter::{
        OmniSoloEvent, OmniSoloHarnessAdapter, OmniSoloRunConfig,
    };
    use server_harness::middleware::capsule::{PortableRecord, SessionCapsule};
    use std::sync::atomic::Ordering;

    fn test_capsule() -> SessionCapsule {
        let mut adapter = OmniSoloHarnessAdapter::start(
            OmniSoloRunConfig::new("tenant_capsule", "transfer this task").with_turn(),
        )
        .unwrap();
        adapter
            .record(OmniSoloEvent::ToolCall {
                name: "read_file".to_owned(),
                args_json: r#"{"path":"README.md"}"#.to_owned(),
                result: "portable result".to_owned(),
                iteration: 1,
            })
            .unwrap();
        adapter
            .export_capsule("opencode", uuid::Uuid::new_v4())
            .unwrap()
    }

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
                let decoded =
                    ::server_ohc::interop::StateHandoff::decode(&msg.payload[..]).unwrap();
                if decoded.mission_id == "mission_1" {
                    rx.store(true, Ordering::SeqCst);
                }
            }
        });

        let _cancel = bus
            .subscribe("system:state_handoff".to_string(), handler)
            .await
            .unwrap();

        protocol
            .handoff("mission_1", "tenant_1", vec![1, 2, 3])
            .await
            .unwrap();
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
        let is_acked = protocol_server
            .dispatch_job("job_1", "tenant_a", "do_work", vec![42], 500)
            .await
            .unwrap();

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
                let decoded =
                    ::server_ohc::interop::StateHandoff::decode(&msg.payload[..]).unwrap();
                if decoded.mission_id == "mission_resume_1" {
                    rx.store(true, Ordering::SeqCst);
                }
            }
        });

        let _cancel = bus
            .subscribe("system:state_handoff".to_string(), handler)
            .await
            .unwrap();

        protocol
            .resume_mission("mission_resume_1", "tenant_1", vec![1, 2, 3])
            .await
            .unwrap();
        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_capsule_handoff_round_trip_preserves_portable_identity() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());
        let capsule = test_capsule();
        let received = Arc::new(tokio::sync::Mutex::new(Vec::new()));
        let received_clone = received.clone();

        let _cancel = protocol
            .listen_for_capsule_operations(Box::new(move |operation| {
                let received = received_clone.clone();
                tokio::spawn(async move {
                    received.lock().await.push(operation);
                });
            }))
            .await
            .unwrap();

        protocol
            .handoff_capsule(&capsule, 7, "session-fence-7")
            .await
            .unwrap();
        sleep(Duration::from_millis(100)).await;

        let operations = received.lock().await;
        assert_eq!(operations.len(), 1);
        let operation = &operations[0];
        assert_eq!(operation.envelope.tenant_id, "tenant_capsule");
        assert_eq!(
            operation.envelope.session_id,
            capsule.manifest.session_id.to_string()
        );
        assert_eq!(
            operation.envelope.operation_id,
            capsule.manifest.handoff_id.to_string()
        );
        assert_eq!(operation.envelope.operation_generation, 7);
        assert_eq!(operation.envelope.fencing_token, "session-fence-7");
        assert_eq!(operation.envelope.kind, "handoff");
        assert_eq!(
            operation.envelope.task_id,
            operation_task_id(&capsule).unwrap()
        );
        assert_eq!(operation.capsule, capsule);
        assert!(
            operation
                .capsule
                .records
                .iter()
                .any(|record| matches!(record, PortableRecord::ToolResult(_)))
        );
    }

    #[tokio::test]
    async fn test_interop_capsule_resume_is_idempotent_and_rejects_invalid_payloads() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock.clone(), "node1".to_string());
        let duplicate_protocol = InteropProtocol::new(bus.clone(), lock, "node2".to_string());
        let capsule = test_capsule();
        let received_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let received_count_clone = received_count.clone();

        let _cancel = protocol
            .listen_for_capsule_operations(Box::new(move |operation| {
                if operation.envelope.kind == "resume" {
                    received_count_clone.fetch_add(1, Ordering::SeqCst);
                }
            }))
            .await
            .unwrap();

        protocol
            .resume_capsule(&capsule, 8, "session-fence-8")
            .await
            .unwrap();
        duplicate_protocol
            .resume_capsule(&capsule, 8, "session-fence-8")
            .await
            .unwrap();
        sleep(Duration::from_millis(100)).await;
        assert_eq!(received_count.load(Ordering::SeqCst), 1);

        assert!(
            protocol
                .handoff_capsule(&capsule, 0, "session-fence-0")
                .await
                .unwrap_err()
                .contains("operation generation")
        );
        assert!(
            protocol
                .handoff_capsule(&capsule, 9, "")
                .await
                .unwrap_err()
                .contains("fencing token")
        );

        let mut tampered = capsule.clone();
        tampered.manifest.target_harness_id = "tampered".to_owned();
        assert!(
            protocol
                .handoff_capsule(&tampered, 9, "session-fence-9")
                .await
                .unwrap_err()
                .contains("integrity")
        );
    }

    #[tokio::test]
    async fn test_interop_capsule_listener_ignores_malformed_envelopes() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();
        let protocol = InteropProtocol::new(bus.clone(), lock, "node1".to_string());
        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let received_clone = received.clone();

        let _cancel = protocol
            .listen_for_capsule_operations(Box::new(move |_operation| {
                received_clone.store(true, Ordering::SeqCst);
            }))
            .await
            .unwrap();

        bus.publish(Message {
            topic: "system:harness_session_operation".to_owned(),
            payload: vec![255, 255, 255],
        })
        .await
        .unwrap();
        sleep(Duration::from_millis(50)).await;
        assert!(!received.load(Ordering::SeqCst));
    }

    fn operation_task_id(capsule: &SessionCapsule) -> Option<String> {
        capsule.records.iter().find_map(|record| match record {
            PortableRecord::Task(task) => Some(task.task_id.to_string()),
            _ => None,
        })
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

        let _cancel = bus
            .subscribe("system:state_handoff".to_string(), handler)
            .await
            .unwrap();

        // Simulate identical payload handoffs to ensure we process gracefully
        protocol
            .handoff("mission_1", "tenant_1", vec![1, 2, 3])
            .await
            .unwrap();

        // Wait briefly for the lock to be fully acquired in the mock environment
        sleep(Duration::from_millis(50)).await;

        // Try the same handoff again, it should immediately return Ok() due to lock idempotency check.
        let protocol2 = InteropProtocol::new(bus.clone(), lock.clone(), "node2".to_string());
        protocol2
            .handoff("mission_1", "tenant_1", vec![1, 2, 3])
            .await
            .unwrap();

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
        protocol
            .handoff("mission_2", "tenant_2", vec![1, 2, 3])
            .await
            .unwrap();
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
        let is_acked = protocol_server
            .dispatch_job("job_timeout", "tenant_a", "do_work", vec![42], 100)
            .await
            .unwrap();

        assert!(!is_acked);
    }

    #[tokio::test]
    async fn test_interop_listen_for_pings() {
        let bus = Arc::new(MemoryBus::new());
        let lock = bus.clone();

        let protocol_listener =
            InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());

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

        let protocol_listener =
            InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());

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
        assert!(
            lock.acquire_lock("handoff:mission_locked", "node_other", 10)
                .await
                .unwrap()
        );

        // This should timeout instead of deadlocking, because of our new timeout semantics
        let result = protocol1
            .handoff("mission_locked", "tenant_1", vec![1, 2, 3])
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Timeout waiting for lock");

        // Release
        let _ = lock
            .release_lock("handoff:mission_locked", "node_other")
            .await;
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
        let _cancel = protocol_server
            .listen_for_job_status("job_status_123", handler)
            .await
            .unwrap();

        // Agent reports status
        protocol_agent
            .report_job_status("job_status_123", "tenant_a", "COMPLETED", vec![1, 2, 3])
            .await
            .unwrap();

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

        let result = protocol
            .dispatch_job("job_retry_1", "tenant_a", "do_work", vec![], 10)
            .await;
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

        let result = protocol
            .dispatch_job("job_retry_2", "tenant_a", "do_work", vec![], 10)
            .await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Failed to publish job dispatch after retries")
        );
    }

    #[tokio::test]
    async fn test_interop_handoff_retry_success() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(3),
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "node1".to_string());

        let result = protocol
            .handoff("mission_retry_1", "tenant_1", vec![1, 2, 3])
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_interop_handoff_retry_failure() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(10),
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "node1".to_string());

        let result = protocol
            .handoff("mission_retry_2", "tenant_1", vec![1, 2, 3])
            .await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Failed to publish state handoff after retries")
        );
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
        async fn subscribe(
            &self,
            _topic: String,
            _handler: Box<dyn Fn(crate::msgbus::Message) + Send + Sync>,
        ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
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

        let result = protocol
            .report_job_status("job_retry_1", "tenant_a", "FAILED", vec![])
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_interop_job_status_reporting_retry_failure() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(10), // More than max retries
        });
        let lock = Arc::new(MemoryBus::new());
        let protocol = InteropProtocol::new(bus, lock, "agent".to_string());

        let result = protocol
            .report_job_status("job_retry_2", "tenant_a", "FAILED", vec![])
            .await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Failed to publish job status update after retries")
        );
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

        let protocol_listener =
            InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());
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

        let protocol_listener =
            InteropProtocol::new(bus.clone(), lock.clone(), "listener_node".to_string());
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

        let _cancel = protocol_server
            .listen_for_job_status("job_status_123", handler)
            .await
            .unwrap();

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
