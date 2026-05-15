#![cfg(test)]
use super::*;
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;


#[tokio::test]
async fn test_memory_bus_pub_sub() {
    let bus = MemoryBus::new();
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

    let bus = IpcBus::new(&db_url).await.unwrap();

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
    let bus = match RedisBus::new(&url).await {
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
    let bus = std::sync::Arc::new(MemoryBus::new());
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
    let bus = std::sync::Arc::new(MemoryBus::new());
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
    let bus = std::sync::Arc::new(MemoryBus::new());
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
    let bus = std::sync::Arc::new(MemoryBus::new());
    let transport = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(std::sync::Arc::new(ohc_builtin_agent::mesh::transport::MemoryTransport::new())));
    let monitor = HealthMonitor::new(bus.clone(), transport);

    // Without any handler to respond with an ack, ping should timeout.
    let result = monitor.ping().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Health ping timed out waiting for ack");
}

#[tokio::test]
async fn test_memory_bus_distributed_lock() {
    let bus = MemoryBus::new();
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

    let bus = IpcBus::new(&db_url).await.unwrap();
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
    let bus = match RedisBus::new(&url).await {
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

#[cfg(test)]
mod tests_ipc {
use super::*;

#[tokio::test]
async fn test_ipc_lock() {
    let db_url = "sqlite::memory:";
    let bus = IpcBus::new(db_url).await.unwrap();

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
    let bus = MemoryBus::new();
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
    let bus = MemoryBus::new();

    let acquired = bus.acquire_lock("resource1", "owner1", 10).await.unwrap();
    assert!(acquired);

    let acquired_again = bus.acquire_lock("resource1", "owner2", 10).await.unwrap();
    assert!(!acquired_again);

    bus.release_lock("resource1", "owner1").await.unwrap();

    let acquired_after_release = bus.acquire_lock("resource1", "owner2", 10).await.unwrap();
    assert!(acquired_after_release);
}
}
