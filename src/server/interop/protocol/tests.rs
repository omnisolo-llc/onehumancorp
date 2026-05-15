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