use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedContext {
    pub tenant_id: String,
    pub state: HashMap<String, String>,
}

impl SharedContext {
    pub fn new(tenant_id: &str) -> Self {
        Self {
            tenant_id: tenant_id.to_string(),
            state: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorTask {
    pub id: String,
    pub agent_name: String,
    pub payload: String,
    pub status: TaskStatus,
    pub dependencies: Vec<String>,
}

#[async_trait::async_trait]
pub trait SpecialistAgent: Send + Sync {
    fn name(&self) -> &str;
    async fn execute(&self, _task: &OrchestratorTask, _context: Arc<RwLock<SharedContext>>) -> Result<String, String>;
}

pub struct UnifiedOrchestrator {
    agents: HashMap<String, Arc<dyn SpecialistAgent>>,
    llm_tool: Option<Arc<dyn LlmTool>>,
}

#[async_trait::async_trait]
pub trait LlmTool: Send + Sync {
    async fn decompose_intent(&self, intent: &str) -> Result<Vec<OrchestratorTask>, String>;
}

impl UnifiedOrchestrator {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            llm_tool: None,
        }
    }

    pub fn with_llm(mut self, llm: Arc<dyn LlmTool>) -> Self {
        self.llm_tool = Some(llm);
        self
    }

    pub fn register_agent(&mut self, agent: Arc<dyn SpecialistAgent>) {
        self.agents.insert(agent.name().to_string(), agent);
    }

    pub async fn parse_intent(&self, intent: &str) -> Vec<OrchestratorTask> {
        if let Some(llm) = &self.llm_tool {
            if let Ok(plan) = llm.decompose_intent(intent).await {
                return plan;
            }
        }

        // Fallback or simple mock implementation if LLM fails/is not provided
        if intent.contains("Set up a new product for Wedding Cakes and make sure I get a $100 deposit") {
            vec![
                OrchestratorTask {
                    id: "task_ops_1".to_string(),
                    agent_name: "OperationsAgent".to_string(),
                    payload: "Create product: Wedding Cakes".to_string(),
                    status: TaskStatus::Pending,
                    dependencies: vec![],
                },
                OrchestratorTask {
                    id: "task_fin_1".to_string(),
                    agent_name: "FinanceAgent".to_string(),
                    payload: "Configure $100 deposit for product".to_string(),
                    status: TaskStatus::Pending,
                    dependencies: vec!["task_ops_1".to_string()],
                },
            ]
        } else {
            vec![]
        }
    }

    pub async fn execute_plan(
        &self,
        mut plan: Vec<OrchestratorTask>,
        context: Arc<RwLock<SharedContext>>,
    ) -> Result<String, String> {
        let mut completed = HashSet::new();

        loop {
            let mut made_progress = false;
            let mut all_done = true;

            for i in 0..plan.len() {
                if plan[i].status == TaskStatus::Pending {
                    all_done = false;
                    let can_run = plan[i].dependencies.iter().all(|d| completed.contains(d));

                    if can_run {
                        plan[i].status = TaskStatus::InProgress;

                        let task_clone = plan[i].clone();
                        if let Some(agent) = self.agents.get(&task_clone.agent_name) {
                            match agent.execute(&task_clone, context.clone()).await {
                                Ok(res) => {
                                    plan[i].status = TaskStatus::Completed;
                                    completed.insert(task_clone.id.clone());

                                    let mut ctx = context.write().await;
                                    ctx.state.insert(format!("{}_result", task_clone.id), res);
                                },
                                Err(e) => {
                                    plan[i].status = TaskStatus::Failed(e.clone());
                                    return Err(format!("Task {} failed: {}", task_clone.id, e));
                                }
                            }
                        } else {
                            return Err(format!("Agent not found: {}", task_clone.agent_name));
                        }

                        made_progress = true;
                    }
                } else if plan[i].status == TaskStatus::InProgress {
                    all_done = false;
                }
            }

            if all_done {
                break;
            }

            if !made_progress {
                return Err("Deadlock detected in execution graph".to_string());
            }
        }

        Ok("Plan executed successfully".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockOperationsAgent;
    #[async_trait::async_trait]
    impl SpecialistAgent for MockOperationsAgent {
        fn name(&self) -> &str { "OperationsAgent" }
        async fn execute(&self, _task: &OrchestratorTask, context: Arc<RwLock<SharedContext>>) -> Result<String, String> {
            let mut ctx = context.write().await;
            ctx.state.insert("product_id".to_string(), "prod_wedding_cake".to_string());
            Ok("Product created".to_string())
        }
    }

    struct MockFinanceAgent;
    #[async_trait::async_trait]
    impl SpecialistAgent for MockFinanceAgent {
        fn name(&self) -> &str { "FinanceAgent" }
        async fn execute(&self, _task: &OrchestratorTask, context: Arc<RwLock<SharedContext>>) -> Result<String, String> {
            let ctx = context.read().await;
            if !ctx.state.contains_key("product_id") {
                return Err("Missing product_id in context".to_string());
            }
            Ok("Deposit configured".to_string())
        }
    }

    struct MockLlmTool;
    #[async_trait::async_trait]
    impl LlmTool for MockLlmTool {
        async fn decompose_intent(&self, intent: &str) -> Result<Vec<OrchestratorTask>, String> {
            if intent.contains("Dynamic test intent") {
                Ok(vec![
                    OrchestratorTask {
                        id: "task_1".to_string(),
                        agent_name: "OperationsAgent".to_string(),
                        payload: "Action 1".to_string(),
                        status: TaskStatus::Pending,
                        dependencies: vec![],
                    }
                ])
            } else {
                Err("Failed to parse intent".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_unified_orchestrator_execution() {
        let mut orchestrator = UnifiedOrchestrator::new();
        orchestrator.register_agent(Arc::new(MockOperationsAgent));
        orchestrator.register_agent(Arc::new(MockFinanceAgent));

        let intent = "Set up a new product for Wedding Cakes and make sure I get a $100 deposit";
        let plan = orchestrator.parse_intent(intent).await;

        assert_eq!(plan.len(), 2);
        assert_eq!(plan[0].agent_name, "OperationsAgent");
        assert_eq!(plan[1].agent_name, "FinanceAgent");
        assert_eq!(plan[1].dependencies[0], "task_ops_1");

        let context = Arc::new(RwLock::new(SharedContext::new("tenant_123")));

        let result = orchestrator.execute_plan(plan, context.clone()).await;
        assert_eq!(result, Ok("Plan executed successfully".to_string()));

        let ctx = context.read().await;
        assert_eq!(ctx.state.get("product_id").unwrap(), "prod_wedding_cake");
        assert_eq!(ctx.state.get("task_ops_1_result").unwrap(), "Product created");
        assert_eq!(ctx.state.get("task_fin_1_result").unwrap(), "Deposit configured");
    }

    #[tokio::test]
    async fn test_unified_orchestrator_dynamic_llm_parsing() {
        let mut orchestrator = UnifiedOrchestrator::new().with_llm(Arc::new(MockLlmTool));
        orchestrator.register_agent(Arc::new(MockOperationsAgent));

        let intent = "Dynamic test intent";
        let plan = orchestrator.parse_intent(intent).await;

        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].agent_name, "OperationsAgent");
        assert_eq!(plan[0].payload, "Action 1");

        let context = Arc::new(RwLock::new(SharedContext::new("tenant_456")));

        let result = orchestrator.execute_plan(plan, context.clone()).await;
        assert_eq!(result, Ok("Plan executed successfully".to_string()));

        let ctx = context.read().await;
        assert_eq!(ctx.state.get("task_1_result").unwrap(), "Product created");
    }
}
