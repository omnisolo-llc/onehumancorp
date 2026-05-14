
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::{HashMap, VecDeque};
use crate::msgbus::{Bus, Message};
use crate::interop::protocol::{InteropProtocol, proto};
use tokio::time::{sleep, Duration};

const MAX_WINDOW_SIZE: usize = 10;

pub struct NodeStats {
    pub latencies: VecDeque<u64>,
    pub last_seen: i64,
}

impl NodeStats {
    pub fn new() -> Self {
        Self {
            latencies: VecDeque::with_capacity(MAX_WINDOW_SIZE),
            last_seen: chrono::Utc::now().timestamp_millis(),
        }
    }

    pub fn add_latency(&mut self, latency: u64) {
        if self.latencies.len() == MAX_WINDOW_SIZE {
            self.latencies.pop_front();
        }
        self.latencies.push_back(latency);
    }

    pub fn average_latency(&self) -> u64 {
        if self.latencies.is_empty() { return 0; }
        let sum: u64 = self.latencies.iter().sum();
        sum / (self.latencies.len() as u64)
    }

    pub fn jitter(&self) -> u64 {
        if self.latencies.len() < 2 { return 0; }
        let avg = self.average_latency();
        let variance_sum: u64 = self.latencies.iter()
            .map(|l| if *l > avg { *l - avg } else { avg - *l })
            .sum();
        variance_sum / (self.latencies.len() as u64)
    }
}

pub struct ActiveHealthMonitor {
    node_id: String,
    bus: Arc<dyn Bus>,
    protocol: Arc<InteropProtocol>,
    peers: Arc<RwLock<HashMap<String, NodeStats>>>,
}

impl ActiveHealthMonitor {
    pub fn new(node_id: String, bus: Arc<dyn Bus>, protocol: Arc<InteropProtocol>) -> Self {
        Self {
            node_id,
            bus,
            protocol,
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_active_probing(&self) {
        let node_id = self.node_id.clone();
        let bus = self.bus.clone();

        tokio::spawn(async move {
            loop {
                use prost::Message as ProstMessage;
                let ping = proto::HealthPing {
                    source_node_id: node_id.clone(),
                    current_mode: 1,
                    timestamp_ms: chrono::Utc::now().timestamp_millis(),
                };
                let mut buf = Vec::new();
                if ping.encode(&mut buf).is_ok() {
                    let _ = bus.publish(Message {
                        topic: "system:health_ping".to_string(),
                        payload: buf,
                    }).await;
                }
                sleep(Duration::from_secs(5)).await;
            }
        });
    }

    pub async fn process_ack(&self, ack: proto::HealthAck) {
        let now = chrono::Utc::now().timestamp_millis();
        let rtt = (now - ack.timestamp_ms).max(0) as u64;

        let mut p = self.peers.write().await;
        let mut stats = p.entry(ack.source_node_id).or_insert_with(NodeStats::new);
        stats.last_seen = ack.timestamp_ms;
        stats.add_latency(rtt);
    }

    pub async fn evict_dead_nodes(&self, timeout_ms: i64) -> Vec<String> {
        let now = chrono::Utc::now().timestamp_millis();
        let mut p = self.peers.write().await;

        let mut dead = Vec::new();
        p.retain(|node_id, stats| {
            if now - stats.last_seen > timeout_ms {
                dead.push(node_id.clone());
                false
            } else {
                true
            }
        });
        dead
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;

    #[test]
    fn test_node_stats_jitter_latency() {
        let mut stats = NodeStats::new();
        stats.add_latency(10);
        stats.add_latency(20);
        stats.add_latency(30);

        assert_eq!(stats.average_latency(), 20);
        assert_eq!(stats.jitter(), 6); // |10-20|=10, |20-20|=0, |30-20|=10. sum=20/3=6
    }

    #[tokio::test]
    async fn test_active_monitor_eviction() {
        let bus = Arc::new(MemoryBus::new());
        let lock = Arc::new(MemoryBus::new());
        let protocol = Arc::new(InteropProtocol::new(bus.clone(), lock, "test".to_string()));
        let monitor = ActiveHealthMonitor::new("test".to_string(), bus, protocol);

        let ack = proto::HealthAck {
            source_node_id: "node_dead".to_string(),
            target_node_id: "test".to_string(),
            timestamp_ms: chrono::Utc::now().timestamp_millis() - 10000,
        };
        monitor.process_ack(ack).await;

        let dead = monitor.evict_dead_nodes(5000).await;
        assert_eq!(dead.len(), 1);
        assert_eq!(dead[0], "node_dead");
    }
}
