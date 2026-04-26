use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::hub::Hub;
use crate::ohc::orchestration::Message;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PipelineState {
    Implementing,
    Testing,
    StagingReady,
    Deployed,
    Rollback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub branch: String,
    pub state: PipelineState,
    pub agent_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecApprovedEvent {
    pub branch: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CIJob {
    pub command: String,
    pub branch: String,
}

pub struct Orchestrator {
    hub: Arc<Hub>,
    pipelines: RwLock<HashMap<String, Pipeline>>,
    ci_jobs: RwLock<Vec<CIJob>>,
}

impl Orchestrator {
    pub fn new(hub: Arc<Hub>) -> Self {
        Orchestrator {
            hub,
            pipelines: RwLock::new(HashMap::new()),
            ci_jobs: RwLock::new(Vec::new()),
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
        
        self.hub.clone().publish(task_msg).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn handle_pr_created(&self, msg: Message) -> Result<(), String> {
        let branch = msg.content.clone();
        
        let mut pipelines = self.pipelines.write().unwrap();
        if let Some(pipeline) = pipelines.get_mut(&branch) {
            pipeline.state = PipelineState::Testing;
            
            let job = CIJob {
                command: format!("bazel test //... --branch={}", branch),
                branch: branch.clone(),
            };
            
            let mut ci_jobs = self.ci_jobs.write().unwrap();
            ci_jobs.push(job);
            
            return Ok(());
        }
        
        Err("pipeline not found for branch".to_string())
    }

    pub async fn handle_test_results(&self, msg: Message) -> Result<(), String> {
        let mut branch = String::new();
        let mut logs = String::new();
        
        for part in msg.content.split(',') {
             let kv: Vec<&str> = part.split('=').collect();
             if kv.len() == 2 {
                 match kv[0] {
                     "branch" => branch = kv[1].to_string(),
                     "logs" => logs = kv[1].to_string(),
                     _ => {}
                 }
             }
        }
        
        if branch.is_empty() {
            branch = msg.content.clone();
        }
        
        let mut pipelines = self.pipelines.write().unwrap();
        let pipeline = pipelines.get_mut(&branch).ok_or_else(|| "pipeline not found for branch".to_string())?;
        
        if msg.r#type == "TestsPassed" {
            pipeline.state = PipelineState::StagingReady;
            
            let approval_msg = Message {
                id: format!("msg-{}", Utc::now().timestamp_nanos_opt().unwrap()),
                from_agent: "system-hub".to_string(),
                to_agent: "ceo-1".to_string(),
                r#type: "ApprovalNeeded".to_string(),
                content: format!("branch={},url=https://staging.onehumancorp.com/{}", branch, branch),
                occurred_at_unix: Utc::now().timestamp(),
                meeting_id: String::new(),
            };
            
            self.hub.clone().publish(approval_msg).map_err(|e| e.to_string())?;
        } else if msg.r#type == "TestsFailed" {
            pipeline.state = PipelineState::Implementing;
            let swe_id = pipeline.agent_id.clone();
            
            let fail_msg = Message {
                id: format!("msg-{}", Utc::now().timestamp_nanos_opt().unwrap()),
                from_agent: "system-hub".to_string(),
                to_agent: swe_id,
                r#type: "TestsFailed".to_string(),
                content: format!("branch={},logs={}", branch, logs),
                occurred_at_unix: Utc::now().timestamp(),
                meeting_id: String::new(),
            };
            
            self.hub.clone().publish(fail_msg).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }

    pub async fn reject_staging(&self, branch: &str, reason: &str) -> Result<(), String> {
        let mut pipelines = self.pipelines.write().unwrap();
        let pipeline = pipelines.get_mut(branch).ok_or_else(|| "pipeline not found for branch".to_string())?;
        
        pipeline.state = PipelineState::Rollback;
        let swe_id = pipeline.agent_id.clone();
        
        let reject_msg = Message {
            id: format!("msg-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            from_agent: "ceo-1".to_string(),
            to_agent: swe_id,
            r#type: "task".to_string(),
            content: format!("Rejection on branch {}: {}", branch, reason),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        
        self.hub.clone().publish(reject_msg).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub async fn approve_for_production(&self, branch: &str) -> Result<(), String> {
        let mut pipelines = self.pipelines.write().unwrap();
        let pipeline = pipelines.get_mut(branch).ok_or_else(|| "pipeline not found for branch".to_string())?;
        
        if pipeline.state != PipelineState::StagingReady {
            return Err("pipeline is not in STAGING_READY state".to_string());
        }
        
        pipeline.state = PipelineState::Deployed;
        
        let merge_msg = Message {
            id: format!("msg-{}", Utc::now().timestamp_nanos_opt().unwrap()),
            from_agent: "system-hub".to_string(),
            to_agent: "system-hub".to_string(),
            r#type: "PRMerged".to_string(),
            content: format!("branch={}", branch),
            occurred_at_unix: Utc::now().timestamp(),
            meeting_id: String::new(),
        };
        
        self.hub.clone().publish(merge_msg).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn get_pipeline_state(&self, branch: &str) -> Result<PipelineState, String> {
        let pipelines = self.pipelines.read().unwrap();
        pipelines.get(branch).map(|p| p.state.clone()).ok_or_else(|| "pipeline not found".to_string())
    }

    pub fn get_ci_jobs(&self) -> Vec<CIJob> {
        let ci_jobs = self.ci_jobs.read().unwrap();
        ci_jobs.clone()
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
        let hub = Arc::new(Hub::new(tx));
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
