use async_trait::async_trait;
use std::sync::Arc;
use crate::hub::Hub;
use crate::tasks::SharedTask;

#[async_trait]
pub trait TaskOrchestrator: Send + Sync {
    async fn receive_high_level_request(&self, org_id: &str, title: &str) -> Result<String, String>;
    async fn enqueue_task(&self, task: SharedTask, depends_on: Vec<String>) -> Result<SharedTask, String>;
    async fn acquire_ready_task(&self, agent_id: &str, capabilities: Vec<String>) -> Result<Option<SharedTask>, String>;
    async fn complete_task(&self, task_id: &str, agent_id: &str, result: &str) -> Result<(), String>;
}

pub struct DefaultTaskOrchestrator {
    hub: Arc<Hub>,
}

impl DefaultTaskOrchestrator {
    pub fn new(hub: Arc<Hub>) -> Self {
        DefaultTaskOrchestrator { hub }
    }
}

#[async_trait]
impl TaskOrchestrator for DefaultTaskOrchestrator {
    async fn receive_high_level_request(&self, org_id: &str, title: &str) -> Result<String, String> {
        let task = self.hub.task_manager().create_task(
            org_id.to_string(),
            String::new(),
            title.to_string(),
            "High level request".to_string(),
            "P1".to_string(),
        )?;
        Ok(task.id)
    }

    async fn enqueue_task(&self, task: SharedTask, depends_on: Vec<String>) -> Result<SharedTask, String> {
        let mut task = task;
        task.status = if depends_on.is_empty() { "READY".to_string() } else { "PENDING".to_string() };
        task.dependencies = depends_on;
        
        self.hub.task_manager().insert_task(task.clone());
        Ok(task)
    }

    async fn acquire_ready_task(&self, agent_id: &str, _capabilities: Vec<String>) -> Result<Option<SharedTask>, String> {
        let tasks = self.hub.task_manager().poll_tasks(agent_id, 1);
        Ok(tasks.into_iter().next())
    }

    async fn complete_task(&self, task_id: &str, agent_id: &str, result: &str) -> Result<(), String> {
        self.hub.task_manager().complete_task(task_id, agent_id, result.to_string())?;
        
        // Trigger AutoDream embedding in background
        let task_id = task_id.to_string();
        let result = result.to_string();
        let hub = self.hub.clone();
        
        tokio::spawn(async move {
            
            let context_str = format!("Task ID: {}, Result: {}", task_id, result);
            
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.is_empty() {
                return;
            }
            let client = crate::minimax::MinimaxClient::new(api_key);
            
            match client.generate_embedding(&context_str).await {
                Ok(embedding) => {
                    
                    let org_id = match hub.task_manager().get_task(&task_id) {
                        Ok(task) => task.organization_id.clone(),
                        Err(_) => {
                            "system".to_string() // Fallback
                        }
                    };
                    
                    let sip_db = crate::sip::SipDB::new(hub.pool.clone(), org_id);
                    let mem_id = format!("task-completion-{}", task_id);
                    
                    match sip_db.inject_truth(&mem_id, &context_str, embedding).await {
                    }
                }
                Err(e) => {
                }
            }
        });
        
        Ok(())
    }
}

pub fn start_token_burn_forecaster(
    _hub: Arc<Hub>,
    tick_duration: std::time::Duration,
) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tick_duration);
        let mut history: std::collections::HashMap<String, Vec<i64>> = std::collections::HashMap::new();

        loop {
            interval.tick().await;

            let orgs = vec!["org1".to_string(), "org2".to_string()];
            
            for org_id in orgs {
                let total_tokens = 1000; 
                
                if total_tokens > 0 {
                    let h = history.entry(org_id.clone()).or_insert_with(Vec::new);
                    h.push(total_tokens);
                    
                    if h.len() > 5 {
                        h.remove(0);
                    }
                    
                    if h.len() > 1 {
                        let rate = (h.last().unwrap() - h.first().unwrap()) as f64 / (h.len() - 1) as f64;
                        println!("TokenBurnForecaster: Org {} rate: {}", org_id, rate);
                        
                        let prediction_24h = rate * 60.0 * 24.0;
                        println!("TokenBurnForecaster: Org {} predicted 24h: {}", org_id, prediction_24h);
                        
                        if prediction_24h > 0.0 {
                            println!("TokenBurnForecaster: Predictive cost alert for {}", org_id);
                        }
                    }
                }
            }
        }
    });
}
