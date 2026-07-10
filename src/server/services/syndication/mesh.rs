use std::sync::Arc;
use crate::msgbus::{Bus, Message};
use super::adapter::SalesChannelAdapter;
use tokio::sync::Mutex;
use serde_json::Value;

pub struct SyndicationMeshService {
    bus: Arc<dyn Bus>,
    adapters: Arc<Mutex<Vec<Box<dyn SalesChannelAdapter>>>>,
}

impl SyndicationMeshService {
    pub fn new(bus: Arc<dyn Bus>) -> Self {
        Self {
            bus,
            adapters: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn add_adapter(&self, adapter: Box<dyn SalesChannelAdapter>) {
        let mut adapters = self.adapters.lock().await;
        adapters.push(adapter);
    }

    pub async fn start(&self) -> Result<(), String> {
        let adapters_clone = self.adapters.clone();

        let handler = Box::new(move |msg: Message| {
            if let Ok(payload_str) = String::from_utf8(msg.payload.clone()) {
                if let Ok(payload_json) = serde_json::from_str::<Value>(&payload_str) {
                    if let Some(action) = payload_json.get("action").and_then(|a| a.as_str()) {
                        if action == "InventoryUpdated" || action == "ProductCreated" || action == "OrderCreated" {
                            let adapters = adapters_clone.clone();
                            let payload = payload_json.clone();

                            tokio::spawn(async move {
                                let adapters = adapters.lock().await;
                                let futures = adapters.iter().map(|adapter| {
                                    let payload_clone = payload.clone();
                                    async move {
                                        let _ = adapter.push_product_update(&payload_clone).await;
                                    }
                                });
                                futures::future::join_all(futures).await;
                            });
                        }
                    }
                }
            }
        });

        let _ = self.bus.subscribe("system:catalog_events".to_string(), handler).await?;
        let _ = self.bus.subscribe("system:order_events".to_string(), Box::new(|_| {})).await?;

        Ok(())
    }
}
