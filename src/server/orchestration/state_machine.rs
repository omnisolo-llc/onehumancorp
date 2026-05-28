use std::sync::Arc;
use crate::orchestration::mesh::TeammateMesh;

pub struct MeshStateMachine {
    mesh: Arc<dyn TeammateMesh>,
}

impl MeshStateMachine {
    pub fn new(mesh: Arc<dyn TeammateMesh>) -> Self {
        Self { mesh }
    }

    pub async fn transition(
        &self,
        task_id: &str,
        agent_id: &str,
        from_state: &str,
        to_state: &str,
    ) -> Result<(), String> {
        let resource = format!("mesh:lock:{}", task_id);

        let acquired = self.mesh.acquire_lock(&resource, agent_id, 30).await?;
        if !acquired {
            return Err("Failed to acquire lock".to_string());
        }

        let payload = format!("{{\"task_id\": \"{}\", \"from\": \"{}\", \"to\": \"{}\"}}", task_id, from_state, to_state).into_bytes();
        let topic = format!("mesh:state_transition:{}", task_id);

        if let Err(e) = self.mesh.publish(&topic, payload).await {
            let _ = self.mesh.release_lock(&resource, agent_id).await;
            return Err(e);
        }

        self.mesh.release_lock(&resource, agent_id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use ohc_builtin_agent::mesh::transport::Message;
    use std::sync::Mutex;
    use std::collections::HashMap;

    struct MockMesh {
        locks: Mutex<HashMap<String, String>>,
        published: Mutex<Vec<(String, Vec<u8>)>>,
        acquire_fail: bool,
    }

    impl MockMesh {
        fn new(acquire_fail: bool) -> Self {
            Self {
                locks: Mutex::new(HashMap::new()),
                published: Mutex::new(Vec::new()),
                acquire_fail,
            }
        }
    }

    #[async_trait]
    impl TeammateMesh for MockMesh {
        async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
            self.published.lock().unwrap().push((topic.to_string(), payload));
            Ok(())
        }
        async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
            Ok(())
        }
        async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }

        async fn acquire_lock(&self, resource: &str, owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
            if self.acquire_fail {
                return Ok(false);
            }
            let mut locks = self.locks.lock().unwrap();
            if locks.contains_key(resource) {
                Ok(false)
            } else {
                locks.insert(resource.to_string(), owner.to_string());
                Ok(true)
            }
        }
        async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
            let mut locks = self.locks.lock().unwrap();
            if let Some(current_owner) = locks.get(resource) {
                if current_owner == owner {
                    locks.remove(resource);
                }
            }
            Ok(())
        }

        async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> {
            Ok(())
        }
        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
            Ok(vec![])
        }

        async fn ping(&self) -> Result<(), String> {
            Ok(())
        }
        async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }

        async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> {
            Ok(())
        }
        async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }
    }

    #[tokio::test]
    async fn test_transition_success() {
        let mesh = Arc::new(MockMesh::new(false));
        let sm = MeshStateMachine::new(mesh.clone());

        let res = sm.transition("task1", "agent1", "pending", "in_progress").await;
        assert!(res.is_ok());

        let published = mesh.published.lock().unwrap();
        assert_eq!(published.len(), 1);
        assert_eq!(published[0].0, "mesh:state_transition:task1");

        let locks = mesh.locks.lock().unwrap();
        assert!(locks.is_empty(), "Lock should be released");
    }

    #[tokio::test]
    async fn test_transition_lock_fail() {
        let mesh = Arc::new(MockMesh::new(true));
        let sm = MeshStateMachine::new(mesh.clone());

        let res = sm.transition("task1", "agent1", "pending", "in_progress").await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Failed to acquire lock");

        let published = mesh.published.lock().unwrap();
        assert_eq!(published.len(), 0);
    }
}

#[cfg(test)]
mod tests2 {
    use super::*;
    use async_trait::async_trait;
    use ohc_builtin_agent::mesh::transport::Message;

    struct PublishFailMesh;

    #[async_trait]
    impl TeammateMesh for PublishFailMesh {
        async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
            Err("Publish failed".to_string())
        }
        async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
        async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
        async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
        async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
        async fn ping(&self) -> Result<(), String> { Ok(()) }
        async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
        async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
        async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    }

    #[tokio::test]
    async fn test_transition_publish_fail() {
        let mesh = Arc::new(PublishFailMesh);
        let sm = MeshStateMachine::new(mesh);

        let res = sm.transition("task1", "agent1", "pending", "in_progress").await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Publish failed");
    }
}
