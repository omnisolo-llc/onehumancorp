
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::msgbus::{Bus, Message};
use crate::interop::protocol::proto;
use prost::Message as ProstMessage;

pub trait Middleware: Send + Sync {
    fn before_dispatch(&self, job: &mut proto::JobDispatch) -> Result<(), String>;
    fn on_receive(&self, msg: &Message) -> Result<(), String>;
}

pub struct TopologyRouter {
    routes: Arc<RwLock<HashMap<String, String>>>,
    bus: Arc<dyn Bus>,
}

impl TopologyRouter {
    pub fn new(bus: Arc<dyn Bus>) -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            bus,
        }
    }

    pub async fn update_route(&self, topic: &str, dest: &str) {
        let mut r = self.routes.write().await;
        r.insert(topic.to_string(), dest.to_string());
    }

    pub async fn resolve_destination(&self, topic: &str) -> Option<String> {
        let r = self.routes.read().await;
        r.get(topic).cloned()
    }
}

impl Middleware for TopologyRouter {
    fn before_dispatch(&self, job: &mut proto::JobDispatch) -> Result<(), String> {
        if job.tenant_id.is_empty() {
            return Err("Tenant ID cannot be empty".to_string());
        }
        // In a real scenario we'd do topology lookups to inject routing headers
        Ok(())
    }

    fn on_receive(&self, _msg: &Message) -> Result<(), String> {
        // Validate checksums, topic formats, etc
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgbus::MemoryBus;

    #[tokio::test]
    async fn test_topology_router() {
        let bus = Arc::new(MemoryBus::new());
        let router = TopologyRouter::new(bus);

        router.update_route("tenant_1", "region_eu").await;
        assert_eq!(router.resolve_destination("tenant_1").await.unwrap(), "region_eu");

        let mut job = proto::JobDispatch {
            job_id: "j1".to_string(),
            tenant_id: "".to_string(),
            action_name: "test".to_string(),
            payload: vec![],
            timestamp_ms: 1000,
        };

        assert!(router.before_dispatch(&mut job).is_err());
        job.tenant_id = "tenant_1".to_string();
        assert!(router.before_dispatch(&mut job).is_ok());
    }
}
