#![allow(dead_code)]

use std::sync::Arc;
use tracing::{info, debug, error};
use tokio::sync::Mutex;
use ohc_builtin_agent::plane::Client as PlaneClient;
use crate::hub::Hub;
use ohc_builtin_agent::plane::Issue;

pub struct TaskWorker {
    plane_client: Arc<PlaneClient>,
    hub: Arc<Hub>,
    poll_interval: std::time::Duration,
    num_workers: usize,
}

impl TaskWorker {
    pub fn new(plane_client: Arc<PlaneClient>, hub: Arc<Hub>) -> Self {
        TaskWorker {
            plane_client,
            hub,
            poll_interval: std::time::Duration::from_secs(30),
            num_workers: 3,
        }
    }

    pub async fn start(&self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        self.start_with_workers(self.num_workers, shutdown_rx).await;
    }

    pub async fn start_with_workers(&self, workers: usize, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        let (task_tx, task_rx) = tokio::sync::mpsc::channel::<Issue>(100);
        let task_rx = Arc::new(Mutex::new(task_rx));
        
        let mut handles = Vec::new();
        for i in 0..workers {
            let task_rx = task_rx.clone();
            let worker_id = i;
            let plane_client = self.plane_client.clone();
            let hub = self.hub.clone();
            
            let handle = tokio::spawn(async move {
                loop {
                    let issue = {
                        let mut rx = task_rx.lock().await;
                        rx.recv().await
                    };
                    
                    if let Some(issue) = issue {
                        debug!("Worker {}: processing issue {}", worker_id, issue.id);
                        Self::process_issue_internal(issue, plane_client.clone(), hub.clone()).await;
                    } else {
                        break; // Channel closed
                    }
                }
            });
            handles.push(handle);
        }
        
        let poll_interval = self.poll_interval;
        let plane_client = self.plane_client.clone();
        let task_tx = task_tx.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(poll_interval);
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if let Ok(issues) = plane_client.list_open_issues().await {
                            if !issues.is_empty() {
                                let dispatch_count = workers.min(issues.len());
                                for issue in issues.into_iter().take(dispatch_count) {
                                    if let Err(_) = task_tx.send(issue).await {
                                        debug!("agent task worker: task channel full, dropping issue dispatch");
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    _ = shutdown_rx.recv() => {
                        break;
                    }
                }
            }
        });
    }

    async fn process_issue_internal(issue: Issue, plane_client: Arc<PlaneClient>, hub: Arc<Hub>) {
        debug!("agent task worker: processing issue: {}, title: {}", issue.id, issue.name);
        
        if let Err(e) = plane_client.update_issue_status(&issue.id, "in_progress").await {
            error!("failed to update plane issue status: {}", e);
            return;
        }
        
        let mut agent_found = false;
        let agents = tokio::task::spawn_blocking({ let hub_clone = hub.clone(); move || hub_clone.get_agents() }).await.unwrap_or_else(|e| { tracing::error!("Failed to get agents: {}", e); Vec::new() });
        
        for a in agents {
            if a.status == "ACTIVE" || a.status == "WAITING_FOR_TOOLS" {
                let payload = serde_json::json!({
                    "issue_id":   issue.id,
                    "issue_name": issue.name,
                    "directive":  "Please resolve the attached issue descriptor.",
                });
                
                let msg = crate::ohc::orchestration::Message {
                    id: format!("task-{}", issue.id),
                    from_agent: "SYSTEM".to_string(),
                    to_agent: a.id.clone(),
                    r#type: "TaskAssignment".to_string(),
                    content: payload.to_string(),
                    meeting_id: "".to_string(),
                    occurred_at_unix: chrono::Utc::now().timestamp(),
                };
                
                let _ = hub.publish(msg);
                
                debug!("agent task worker: issue marked in_progress, delegating to agent: {}", a.id);
                
                if a.provider_type == "builtin" || a.provider_type.is_empty() {
                    debug!("agent task worker: dispatching to builtin Rust agent via gRPC: {}", a.id);
                    let payload_str = payload.to_string();
                    let role = a.role.clone();
                    let issue_id = issue.id.clone();
                    let agent_id = a.id.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::dispatch_to_builtin_agent(&payload_str, &format!("plane issue {}", issue_id), &role).await {
                            debug!("builtin agent dispatch error: {}, agent_id: {}", e, agent_id);
                        }
                    });
                }
                
                agent_found = true;
                break;
            }
        }
        
        if !agent_found {
            debug!("agent task worker: issue marked in_progress but no available agents to delegate to");
        }
    }

    async fn dispatch_to_builtin_agent(payload: &str, description: &str, role: &str) -> Result<(), String> {
        let address = std::env::var("OHC_AGENT_ADDRESS").unwrap_or_else(|_| "127.0.0.1:50051".to_string());
        
        let mut client = crate::ohc::agent::service::agent_service_client::AgentServiceClient::connect(format!("http://{}", address))
            .await
            .map_err(|e| format!("connect to builtin agent at {}: {}", address, e))?;
            
        let req = crate::ohc::agent::service::RunTaskRequest {
            task: payload.to_string(),
            department: role.to_string(),
            ..Default::default()
        };
        
        let response = client.run_task(req).await.map_err(|e| e.to_string())?;
        let mut stream = response.into_inner();
        
        let mut last_content = String::new();
        while let Some(event) = stream.message().await.map_err(|e| e.to_string())? {
            if !event.content.is_empty() {
                last_content = event.content;
            }
        }
        
        debug!("builtin agent task completed: {}, result_len: {}", description, last_content.len());
        Ok(())
    }
}
