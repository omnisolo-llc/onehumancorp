use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CRDTState {
    pub logical_clock: u64,
    pub inventory: HashMap<String, u32>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PMessage {
    pub device_id: String,
    pub state: CRDTState,
}

pub struct P2PMeshNode {
    pub device_id: String,
    pub state: Arc<RwLock<CRDTState>>,
    pub peers: Arc<RwLock<HashMap<String, CRDTState>>>,
}

impl P2PMeshNode {
    pub fn new() -> Self {
        let device_id = Uuid::new_v4().to_string();
        Self {
            device_id,
            state: Arc::new(RwLock::new(CRDTState {
                logical_clock: 0,
                inventory: HashMap::new(),
                timestamp: 0, // Using 0 as default, should be real timestamp in prod
            })),
            peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn process_inventory_decrement(&self, item_id: &str, amount: u32) {
        let mut state = self.state.write().await;
        state.logical_clock += 1;
        state.timestamp += 1; // Simplified for clock

        let current = state.inventory.get(item_id).copied().unwrap_or(0);
        if current >= amount {
            state.inventory.insert(item_id.to_string(), current - amount);
            info!("Device {} decremented {} by {}. New clock: {}", self.device_id, item_id, amount, state.logical_clock);
        } else {
            warn!("Not enough inventory for {}", item_id);
        }
    }

    pub async fn receive_sync(&self, message: P2PMessage) {
        let mut state = self.state.write().await;
        let mut peers = self.peers.write().await;

        peers.insert(message.device_id.clone(), message.state.clone());

        // LWW Convergence for each item
        let mut changed = false;
        for (item_id, count) in &message.state.inventory {
            // Very simplified LWW. In a real CRDT, we'd compare vector clocks or timestamps
            // and handle decrements correctly. Here, we just take the max clock if there's a conflict
            if message.state.logical_clock > state.logical_clock {
                state.inventory.insert(item_id.clone(), *count);
                changed = true;
            } else if message.state.logical_clock == state.logical_clock {
                 if message.state.timestamp > state.timestamp {
                     state.inventory.insert(item_id.clone(), *count);
                     changed = true;
                 }
            }
        }

        if changed {
             state.logical_clock = std::cmp::max(state.logical_clock, message.state.logical_clock) + 1;
             info!("State converged with peer {}", message.device_id);
        }
    }
}
