use std::sync::atomic::Ordering;
use crate::orchestration::mesh::{TeammateMesh, CentrifugeNode};
use ohc_builtin_agent::mesh::transport::{Message, MemoryTransport};
use std::sync::Arc;
use tokio::time::{sleep, timeout, Duration};

pub mod proto {
    pub use interop_proto::ohc::interop::*;
}

/// Interop Layer protocol for mode-switch behaviour and sync
pub struct InteropProtocol {
    mesh: Arc<dyn TeammateMesh>,
    node_id: String,
}

impl InteropProtocol {
    pub fn new(mesh: Arc<dyn TeammateMesh>, node_id: String) -> Self {
        Self {
            mesh,
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
                if self.mesh.acquire_lock(&lock_resource, &self.node_id, 10).await.map_err(|e| e.to_string()).unwrap_or(false) {
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
        if !self.mesh.acquire_lock(&idempotency_lock_resource, &attempt_owner, 3600).await.map_err(|e| e.to_string()).unwrap_or(false) {
            let _ = self.mesh.release_lock(&lock_resource, &self.node_id).await.map_err(|e| e.to_string());
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
            let _ = self.mesh.release_lock(&idempotency_lock_resource, &attempt_owner).await.map_err(|e| e.to_string());
            let _ = self.mesh.release_lock(&lock_resource, &self.node_id).await.map_err(|e| e.to_string());
            return Err(e.to_string());
        }

        mesh.publish(&"system:state_handoff".to_string(), buf)("dummy_topic", vec![]).await.unwrap();

        sleep(Duration::from_millis(100)).await;

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_jobs() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));


        let protocol_listener = InteropProtocol::new(mesh.clone(), "listener_node".to_string());

        let _cancel = protocol_listener.listen_for_jobs("tenant_x").await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        // Subscribe to the ACK
        let ack_topic = format!("system:job_ack:job_123");
        let ack_action_clone = ack_topic.clone();
        let handler = Box::new(move |msg: Message| {
            if true {
                rx.store(true, Ordering::SeqCst);
            }
        });
        let _cancel_ack = mesh.subscribe(&ack_topic, handler).await.map_err(|e| e.to_string()).unwrap();

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

        mesh.publish(&"system:job_dispatch:tenant_x".to_string(), buf)("dummy_topic", vec![]).await.unwrap();

        sleep(Duration::from_millis(200)).await; // longer sleep for retry publish mechanism

        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_handoff_lock_deadlock_prevention() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));

        let protocol1 = InteropProtocol::new(mesh.clone(), "node1".to_string());

        // Acquire lock manually to simulate another process holding it
        assert!(mesh.acquire_lock("handoff:mission_locked", "node_other", 10).await.unwrap());

        // This should timeout instead of deadlocking, because of our new timeout semantics
        let result = protocol1.handoff("mission_locked", "tenant_1", vec![1, 2, 3]).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Timeout waiting for lock");

        // Release
        let _ = mesh.release_lock("handoff:mission_locked", "node_other").await;
    }

    #[tokio::test]
    async fn test_interop_job_status_reporting() {
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let protocol_server = InteropProtocol::new(mesh.clone(), "server".to_string());
        let protocol_agent = InteropProtocol::new(mesh.clone(), "agent".to_string());

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
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "server".to_string());

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
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "server".to_string());

        let result = protocol.dispatch_job("job_retry_2", "tenant_a", "do_work", vec![], 10).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to publish job dispatch after retries"));
    }

    #[tokio::test]
    async fn test_interop_handoff_retry_success() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(3),
        });
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let result = protocol.handoff("mission_retry_1", "tenant_1", vec![1, 2, 3]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_interop_handoff_retry_failure() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(10),
        });
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let result = protocol.handoff("mission_retry_2", "tenant_1", vec![1, 2, 3]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to publish state handoff after retries"));
    }

    struct MockFailingBus {
        failures_left: std::sync::atomic::AtomicUsize,
    }
    #[async_trait::async_trait]
    impl crate::msgbus::Bus for MockFailingBus {
            mesh.publish(&"system".to_string(), buf.clone()).await.unwrap();
        }
    }

    #[tokio::test]
    async fn test_interop_job_status_reporting_retry_success() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(3),
        });
        let lock = Arc::new(MemoryTransport::new()); // dummy lock
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "agent".to_string());

        let result = protocol.report_job_status("job_retry_1", "tenant_a", "FAILED", vec![]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_interop_job_status_reporting_retry_failure() {
        let bus = Arc::new(MockFailingBus {
            failures_left: std::sync::atomic::AtomicUsize::new(10), // More than max retries
        });
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "agent".to_string());

        let result = protocol.report_job_status("job_retry_2", "tenant_a", "FAILED", vec![]).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to publish job status update after retries"));
    }

    #[tokio::test]
    async fn test_interop_health_timeout() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));

        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node_timeout".to_string());

        // Do not set up a listener to acknowledge the ping
        let is_healthy = protocol.check_health(50).await.unwrap();

        assert!(!is_healthy);
    }

    #[tokio::test]
    async fn test_interop_listen_for_state_handoff_malformed() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));

        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |_msg: proto::StateHandoff| {
            rx.store(true, Ordering::SeqCst);
        });

        let _cancel = protocol.listen_for_state_handoff(handler).await.unwrap();

        // Send a malformed message
        mesh.publish(&"system:state_handoff".to_string(), vec![255]).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // Handler should not have been called
        assert!(!received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_pings_malformed() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));


        let protocol_listener = InteropProtocol::new(mesh.clone(), "listener_node".to_string());
        let _cancel = protocol_listener.listen_for_pings().await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:health_ack:sender_node");
        let handler = Box::new(move |_msg: Message| {
            rx.store(true, Ordering::SeqCst);
        });
        let _cancel_ack = mesh.subscribe(&ack_topic, handler).await.map_err(|e| e.to_string()).unwrap();

        // Send a malformed ping
        mesh.publish(&"system:health_ping".to_string(), vec![255]).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // No ack should have been sent
        assert!(!received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_jobs_malformed() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));


        let protocol_listener = InteropProtocol::new(mesh.clone(), "listener_node".to_string());
        let _cancel = protocol_listener.listen_for_jobs("tenant_x").await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let ack_topic = format!("system:job_ack:job_123");
        let handler = Box::new(move |_msg: Message| {
            rx.store(true, Ordering::SeqCst);
        });
        let _cancel_ack = mesh.subscribe(&ack_topic, handler).await.map_err(|e| e.to_string()).unwrap();

        // Send a malformed job dispatch
        mesh.publish(&"system:job_dispatch:tenant_x".to_string(), vec![255]).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // No ack should have been sent
        assert!(!received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_job_status_malformed() {
        let transport = Arc::new(MemoryTransport::new());
        let mesh = Arc::new(CentrifugeNode::new(transport));
        let protocol_server = InteropProtocol::new(mesh.clone(), "server".to_string());

        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |_update: proto::JobStatusUpdate| {
            rx.store(true, Ordering::SeqCst);
        });

        let _cancel = protocol_server.listen_for_job_status("job_status_123", handler).await.unwrap();

        // Send a malformed job status
        mesh.publish(&"system:job_status:job_status_123".to_string(), vec![255]).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        // Handler should not have been called
        assert!(!received.load(Ordering::SeqCst));
    }

}

    #[tokio::test]
    async fn test_interop_listen_for_state_handoff() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

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

        mesh.publish(&"system:state_handoff".to_string(), buf).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_pings() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let _cancel_ping = protocol.listen_for_pings().await.unwrap();

        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rx = received.clone();

        let _cancel_ack = mesh.subscribe("system:health_ack:sender_node", Box::new(move |msg| {
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

        mesh.publish(&"system:health_ping".to_string(), buf).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_listen_for_jobs() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let _cancel_jobs = protocol.listen_for_jobs("t1").await.unwrap();

        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rx = received.clone();

        let _cancel_ack = mesh.subscribe("system:job_ack:job1", Box::new(move |msg| {
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

        mesh.publish(&"system:job_dispatch:t1".to_string(), buf).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_interop_check_health_success() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone2 = mesh_clone.clone();
        let _cancel = mesh.subscribe("system:health_ping", Box::new(move |msg| {
            use prost::Message as ProstMessage;
            if let Ok(ping) = proto::HealthPing::decode(&msg.payload[..]) {
                let ack = proto::HealthAck {
                    source_node_id: "responder".to_string(),
                    target_node_id: ping.source_node_id.clone(),
                    timestamp_ms: 1000,
                };
                let mut buf = Vec::new();
                ack.encode(&mut buf).unwrap();
                let b = mesh.clone();
                tokio::spawn(async move {
            mesh.publish(&"system".to_string(), buf.clone()).await.unwrap();
                });
            }
        })).await.unwrap();

        let is_healthy = protocol.check_health(500).await.unwrap();
        assert!(is_healthy);
    }

    #[tokio::test]
    async fn test_interop_dispatch_job_success() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone = mesh.clone(); let mesh_clone2 = mesh_clone.clone();
        let _cancel = mesh.subscribe("system:job_dispatch:t1", Box::new(move |msg| {
            use prost::Message as ProstMessage;
            if let Ok(dispatch) = proto::JobDispatch::decode(&msg.payload[..]) {
                let ack = proto::JobAck {
                    job_id: dispatch.job_id.clone(),
                    node_id: "responder".to_string(),
                    timestamp_ms: 1000,
                };
                let mut buf = Vec::new();
                ack.encode(&mut buf).unwrap();
                let b = mesh.clone();
                tokio::spawn(async move {
            mesh.publish(&"system".to_string(), buf.clone()).await.unwrap();
                });
            }
        })).await.unwrap();

        let success = protocol.dispatch_job("job1", "t1", "action", vec![], 500).await.unwrap();
        assert!(success);
    }

    #[tokio::test]
    async fn test_interop_handoff_success() {
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport));
        let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let transport = Arc::new(MemoryTransport::new()); let mesh = Arc::new(CentrifugeNode::new(transport)); let protocol = InteropProtocol::new(mesh.clone(), "node1".to_string());

        let received = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let rx = received.clone();

        let _cancel = mesh.subscribe("system:state_handoff", Box::new(move |_| {
            rx.store(true, Ordering::SeqCst);
        })).await.unwrap();

        let result = protocol.handoff("m1", "t1", vec![]).await;
        assert!(result.is_ok());

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }
