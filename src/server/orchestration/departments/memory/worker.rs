use super::layer::MemoryLayer;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct MemoryConsolidationWorker {
    layer: Arc<MemoryLayer>,
    interval: Duration,
}

impl MemoryConsolidationWorker {
    pub fn new(layer: Arc<MemoryLayer>, interval: Duration) -> Self {
        Self { layer, interval }
    }

    pub async fn start(&self) {
        loop {
            self.run_pruning_cycle().await;
            self.run_conflict_resolution_cycle().await;
            sleep(self.interval).await;
        }
    }

    async fn run_pruning_cycle(&self) {
        self.layer.run_pruning().await.unwrap_or_else(|e| {
            println!("Error during pruning cycle: {}", e);
        });
    }

    async fn run_conflict_resolution_cycle(&self) {
        self.layer.run_conflict_resolution().await.unwrap_or_else(|e| {
            println!("Error during conflict resolution cycle: {}", e);
        });
    }
}
