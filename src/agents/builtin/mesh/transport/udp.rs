use super::MeshTransport;
use crate::proto::hub::TeammateMeshEvent as Message;
use async_trait::async_trait;
use dashmap::DashMap;
use prost::Message as ProstMessage;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::broadcast;

pub struct UdpTransport {
    subs: DashMap<String, broadcast::Sender<Message>>,
    socket: Arc<UdpSocket>,
    peers: DashMap<String, std::net::SocketAddr>, // Node ID -> Addr
}

impl UdpTransport {
    pub async fn new(bind_addr: &str) -> Result<Self, String> {
        let socket = UdpSocket::bind(bind_addr)
            .await
            .map_err(|e| e.to_string())?;
        socket.set_broadcast(true).map_err(|e| e.to_string())?;

        let t = UdpTransport {
            subs: DashMap::new(),
            socket: Arc::new(socket),
            peers: DashMap::new(),
        };

        Ok(t)
    }

    pub fn start_worker(&self) {
        let subs = self.subs.clone();
        let socket = self.socket.clone();

        tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            loop {
                match socket.recv_from(&mut buf).await {
                    Ok((size, _peer)) => {
                        if let Ok(msg) = Message::decode(&buf[..size]) {
                            let topic = msg.action.clone(); // In OHC, topic is often the action or channel
                            if let Some(tx) = subs.get(&topic) {
                                let _ = tx.send(msg);
                            } else if let Some(tx) = subs.get("broadcast") {
                                let _ = tx.send(msg);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("UdpTransport recv error: {}", e);
                    }
                }
            }
        });
    }
}

#[async_trait]
impl MeshTransport for UdpTransport {
    async fn publish(&self, topic: &str, mut message: Message) -> Result<(), String> {
        // If topic is not empty, ensure action has it.
        if message.action.is_empty() {
            message.action = topic.to_string();
        }

        let mut buf = Vec::new();
        message.encode(&mut buf).map_err(|e| e.to_string())?;

        // Broadcast to a known multicast or broadcast address, or fallback to known peers
        // For simplicity in the overlay, we'll try to broadcast to 255.255.255.255 on a specific port
        // Or if UDP_PEERS is set, send to them
        let port = self.socket.local_addr().map(|a| a.port()).unwrap_or(9999);
        let broadcast_addr = format!("255.255.255.255:{}", port);

        // Send
        let _ = self.socket.send_to(&buf, broadcast_addr).await;

        // Also send locally
        if let Some(tx) = self.subs.get(topic) {
            let _ = tx.send(message);
        }

        Ok(())
    }

    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let tx = self
            .subs
            .entry(topic.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(100);
                tx
            })
            .clone();

        let mut rx = tx.subscribe();

        let worker = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                handler(msg);
            }
        });

        Ok(Box::new(move || {
            worker.abort();
        }))
    }

    async fn acquire_lock(
        &self,
        _resource: &str,
        _owner: &str,
        _ttl_seconds: u64,
    ) -> Result<bool, String> {
        // UDP overlay doesn't do distributed locking out of the box without Raft/Paxos
        // Return true to avoid blocking, but warn
        tracing::warn!("acquire_lock called on UdpTransport. Not strictly supported.");
        Ok(true)
    }

    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
        Ok(())
    }

    async fn register_presence(
        &self,
        _agent_id: &str,
        _status: &str,
        _ttl_seconds: u64,
    ) -> Result<(), String> {
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        Ok(Vec::new())
    }
}
