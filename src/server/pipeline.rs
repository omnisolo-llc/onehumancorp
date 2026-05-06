use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::hub::Hub;
use crate::ohc::orchestration::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(dead_code)]
pub enum PipelineState {
    Implementing,
    Testing,
    StagingReady,
    Deployed,
    Rollback,
}

#[allow(dead_code)]
pub struct Pipeline {
    pub id: String,
    pub branch: String,
    pub state: PipelineState,
    pub agent_id: String,
    pub created_at: DateTime<Utc>,
}

#[allow(dead_code)]
pub struct SpecApprovedEvent {
    pub branch: String,
    pub details: String,
}


#[allow(dead_code)]
pub struct Orchestrator {
    hub: Arc<Hub>,
    pipelines: RwLock<HashMap<String, Pipeline>>,
}

#[allow(dead_code)]
impl Orchestrator {
    pub fn new(hub: Arc<Hub>) -> Self {
        Orchestrator {
            hub,
            pipelines: RwLock::new(HashMap::new()),
        }
    }

    pub fn parse_spec_approved(content: &str) -> Result<SpecApprovedEvent, String> {
        let mut branch = String::new();
        let mut details = String::new();
        
        for part in content.split(',') {
             let kv: Vec<&str> = part.split('=').collect();
             if kv.len() == 2 {
                 match kv[0] {
                     "branch" => branch = kv[1].to_string(),
                     "details" => details = kv[1].to_string(),
                     _ => {}
                 }
             }
        }
        
        if branch.is_empty() {
            return Err("missing branch in spec approved content".to_string());
        }
        
        Ok(SpecApprovedEvent { branch, details })
    }

    pub async fn handle_spec_approved(&self, msg: Message) -> Result<(), String> {
        let event = Self::parse_spec_approved(&msg.content)?;
        
        let swe_agent_id = "swe-1".to_string();
        
        let mut pipelines = self.pipelines.write().unwrap();
        pipelines.insert(event.branch.clone(), Pipeline {
            id: format!("pipeline-{}", event.branch),
            branch: event.branch.clone(),
            state: PipelineState::Implementing,
            agent_id: swe_agent_id.clone(),
            created_at: Utc::now(),
        });
        drop(pipelines);
        
        let task_msg = Message {
            id: format!("msg-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            from_agent: "system-hub".to_string(),
            to_agent: swe_agent_id,
            r#type: "task".to_string(),
            content: format!("Implement {} on branch {}", event.details, event.branch),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        
        self.hub.clone().publish(task_msg).await.map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn get_pipeline_state(&self, branch: &str) -> Result<PipelineState, String> {
        let pipelines = self.pipelines.read().unwrap();
        pipelines.get(branch).map(|p| p.state.clone()).ok_or_else(|| "pipeline not found".to_string())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_spec_approved() {
        let content = "branch=feature-1,details=Implement feature 1";
        let event = Orchestrator::parse_spec_approved(content).unwrap();
        assert_eq!(event.branch, "feature-1");
        assert_eq!(event.details, "Implement feature 1");
    }

    #[tokio::test]
    async fn test_handle_spec_approved() {
        let (tx, _) = tokio::sync::mpsc::channel(100);
        let pool = sqlx::PgPool::connect_lazy("postgres://localhost/test").unwrap();
        let hub = Arc::new(Hub::new(tx, pool));
        let orchestrator = Orchestrator::new(hub.clone());
        
        let msg = Message {
            id: "msg-1".to_string(),
            from_agent: "user".to_string(),
            to_agent: "hub".to_string(),
            r#type: "SpecApproved".to_string(),
            content: "branch=feature-2,details=Implement feature 2".to_string(),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        
        orchestrator.handle_spec_approved(msg).await.unwrap();
        
        let state = orchestrator.get_pipeline_state("feature-2").unwrap();
        assert_eq!(state, PipelineState::Implementing);
    }
}
