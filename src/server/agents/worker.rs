use std::sync::Arc;
use tokio::sync::Mutex;
use crate::agents::plane::Client as PlaneClient;
use crate::hub::Hub;
use crate::agents::plane::Issue;
use ohc_mesh::transport::{MeshTransport, Message};

pub struct TaskWorker {
    plane_client: Arc<PlaneClient>,
    hub: Arc<Hub>,
    transport: Arc<dyn MeshTransport>,
    poll_interval: std::time::Duration,
    num_workers: usize,
}

impl TaskWorker {
    pub fn new(plane_client: Arc<PlaneClient>, hub: Arc<Hub>, transport: Arc<dyn MeshTransport>) -> Self {
        TaskWorker {
            plane_client,
            hub,
            transport,
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
            let transport = self.transport.clone();
            
            let handle = tokio::spawn(async move {
                loop {
                    let issue = {
                        let mut rx = task_rx.lock().await;
                        rx.recv().await
                    };
                    
                    if let Some(issue) = issue {
                        println!("Worker {}: processing issue {}", worker_id, issue.id);
                        Self::process_issue_internal(issue, plane_client.clone(), hub.clone(), transport.clone()).await;
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
                                        println!("agent task worker: task channel full, dropping issue dispatch");
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

    async fn process_issue_internal(issue: Issue, plane_client: Arc<PlaneClient>, hub: Arc<Hub>, transport: Arc<dyn MeshTransport>) {
        println!("agent task worker: processing issue: {}, title: {}", issue.id, issue.name);
        
        if let Err(e) = plane_client.update_issue_status(&issue.id, "in_progress").await {
            println!("failed to update plane issue status: {}", e);
            return;
        }
        
        let mut agent_found = false;
        let agents = hub.get_agents();
        
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
                
                println!("agent task worker: issue marked in_progress, delegating to agent: {}", a.id);
                
                if a.provider_type == "builtin" || a.provider_type.is_empty() {
                    println!("agent task worker: dispatching to builtin Rust agent via gRPC: {}", a.id);
                    let payload_str = payload.to_string();
                    let role = a.role.clone();
                    let issue_id = issue.id.clone();
                    let agent_id = a.id.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::dispatch_to_builtin_agent(&payload_str, &format!("plane issue {}", issue_id), &role, transport.clone()).await {
                            println!("builtin agent dispatch error: {}, agent_id: {}", e, agent_id);
                        }
                    });
                }
                
                agent_found = true;
                break;
            }
        }
        
        if !agent_found {
            println!("agent task worker: issue marked in_progress but no available agents to delegate to");
        }
    }

    async fn dispatch_to_builtin_agent(payload: &str, description: &str, role: &str, transport: Arc<dyn MeshTransport>) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let req = crate::ohc::agent::service::RunTaskRequest {
            task_id: format!("task-{}", chrono::Utc::now().timestamp_nanos()),
            task: payload.to_string(),
            department: role.to_string(),
            ..Default::default()
        };
        
        let mut buf = Vec::new();
        req.encode(&mut buf).map_err(|e| e.to_string())?;
        
        transport.publish("agent_jobs", Message {
            topic: "agent_jobs".to_string(),
            payload: buf,
        }).await?;
        
        println!("builtin agent task dispatched via MeshTransport: {}", description);
        Ok(())
    }
}
