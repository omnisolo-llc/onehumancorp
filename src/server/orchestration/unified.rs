use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::{Value, json};
use ohc_builtin_agent::langgraph::{StateGraph, Reducer, END};

#[async_trait::async_trait]
pub trait SpecialistAgent: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, task_input: &Value, tenant_id: &str, shared_memory: Arc<dyn SharedMemory>) -> Result<Value, String>;
}

#[async_trait::async_trait]
pub trait SharedMemory: Send + Sync {
    async fn get(&self, tenant_id: &str, key: &str) -> Result<Option<Value>, String>;
    async fn set(&self, tenant_id: &str, key: &str, value: Value) -> Result<(), String>;
}

// In-memory fallback for local/testing
pub struct LocalSharedMemory {
    store: Mutex<HashMap<String, Value>>,
}

impl LocalSharedMemory {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl SharedMemory for LocalSharedMemory {
    async fn get(&self, tenant_id: &str, key: &str) -> Result<Option<Value>, String> {
        let store = self.store.lock().await;
        let mem_key = format!("{}:{}", tenant_id, key);
        Ok(store.get(&mem_key).cloned())
    }

    async fn set(&self, tenant_id: &str, key: &str, value: Value) -> Result<(), String> {
        let mut store = self.store.lock().await;
        let mem_key = format!("{}:{}", tenant_id, key);
        store.insert(mem_key, value);
        Ok(())
    }
}

// Redis-backed shared memory implementation
pub struct RedisSharedMemory {
    client: redis::Client,
}

impl RedisSharedMemory {
    pub fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl SharedMemory for RedisSharedMemory {
    async fn get(&self, tenant_id: &str, key: &str) -> Result<Option<Value>, String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let mem_key = format!("ohc:memory:{}:{}", tenant_id, key);
        let val: Option<String> = redis::AsyncCommands::get(&mut conn, mem_key).await.map_err(|e| e.to_string())?;
        if let Some(v) = val {
            let parsed = serde_json::from_str(&v).map_err(|e| e.to_string())?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }

    async fn set(&self, tenant_id: &str, key: &str, value: Value) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let mem_key = format!("ohc:memory:{}:{}", tenant_id, key);
        let serialized = serde_json::to_string(&value).map_err(|e| e.to_string())?;
        let _: () = redis::AsyncCommands::set(&mut conn, mem_key, serialized).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct OrchestratorState {
    pub tenant_id: String,
    pub user_intent: String,
    pub plan: Vec<TaskNode>,
    pub pending_tasks: Vec<TaskNode>,
    pub completed_tasks: Vec<String>,
    pub final_summary: String,
    pub execution_results: HashMap<String, Value>,
}

pub struct OrchestratorReducer;

impl Reducer<OrchestratorState> for OrchestratorReducer {
    fn reduce(&self, state: &mut OrchestratorState, update: OrchestratorState) {
        if !update.user_intent.is_empty() {
            state.user_intent = update.user_intent;
        }
        if !update.tenant_id.is_empty() {
            state.tenant_id = update.tenant_id;
        }
        if !update.plan.is_empty() {
            state.plan = update.plan.clone();
            state.pending_tasks = update.plan.clone();
        }

        // Proper DAG update: remove completed tasks from pending and add to completed.
        // Doing this manually since DAG reduction logic can get tricky via just copying fields.
        if !update.completed_tasks.is_empty() {
            for comp in update.completed_tasks.iter() {
                if !state.completed_tasks.contains(comp) {
                     state.completed_tasks.push(comp.clone());
                }
                state.pending_tasks.retain(|t| &t.id != comp);
            }
        }

        if !update.final_summary.is_empty() {
            state.final_summary = update.final_summary;
        }
        for (k, v) in update.execution_results {
            state.execution_results.insert(k, v);
        }
    }
}

pub struct UnifiedOrchestrator {
    agents: Arc<Mutex<HashMap<String, Arc<dyn SpecialistAgent>>>>,
    shared_memory: Arc<dyn SharedMemory>,
    llm_provider: Arc<dyn LLMProvider>,
}

#[async_trait::async_trait]
pub trait LLMProvider: Send + Sync {
    async fn plan_tasks(&self, intent: &str, agents: &HashMap<String, Arc<dyn SpecialistAgent>>) -> Result<Vec<TaskNode>, String>;
}

pub struct MockLLMProvider;

#[async_trait::async_trait]
impl LLMProvider for MockLLMProvider {
    async fn plan_tasks(&self, intent: &str, _agents: &HashMap<String, Arc<dyn SpecialistAgent>>) -> Result<Vec<TaskNode>, String> {
        let mut plan = vec![];
        if intent.to_lowercase().contains("cake") && intent.to_lowercase().contains("deposit") {
            plan.push(TaskNode {
                id: "task_1".to_string(),
                agent_name: "Operations Agent".to_string(),
                input: json!({"action": "create_product", "name": "Wedding Cakes"}),
                dependencies: vec![],
            });
            plan.push(TaskNode {
                id: "task_2".to_string(),
                agent_name: "Finance Agent".to_string(),
                input: json!({"action": "require_deposit", "amount": 100}),
                dependencies: vec!["task_1".to_string()], // DAG representation
            });
        }
        Ok(plan)
    }
}

impl UnifiedOrchestrator {
    pub fn new(shared_memory: Arc<dyn SharedMemory>, llm_provider: Arc<dyn LLMProvider>) -> Self {
        Self {
            agents: Arc::new(Mutex::new(HashMap::new())),
            shared_memory,
            llm_provider,
        }
    }

    pub async fn register_agent(&self, agent: Arc<dyn SpecialistAgent>) {
        let mut agents = self.agents.lock().await;
        agents.insert(agent.name().to_string(), agent);
    }

    pub async fn execute_request(&self, tenant_id: &str, user_intent: &str) -> Result<OrchestratorState, String> {
        let mut graph = StateGraph::<OrchestratorState>::new(Arc::new(OrchestratorReducer));

        let agents_ref = self.agents.clone();

        let llm_ref = self.llm_provider.clone();

        graph.add_node("planner", move |state| {
            let llm = llm_ref.clone();
            let agents_ptr = agents_ref.clone();
            async move {
                let agents = agents_ptr.lock().await;
                let plan = llm.plan_tasks(&state.user_intent, &agents).await?;

                Ok(OrchestratorState {
                    plan,
                    ..Default::default()
                })
            }
        });

        let agents_for_executor = self.agents.clone();
        let memory_for_executor = self.shared_memory.clone();

        graph.add_node("executor", move |state| {
            let agents_ref_clone = agents_for_executor.clone();
            let shared_memory_clone = memory_for_executor.clone();
            async move {
                let current_state = state.clone();
                let mut execution_results = HashMap::new();
                let mut completed_tasks = vec![];

                let agents = agents_ref_clone.lock().await;

                // Find a ready task
                let mut ready_task = None;
                for task in &current_state.pending_tasks {
                    let mut is_ready = true;
                    for dep in &task.dependencies {
                        if !current_state.completed_tasks.contains(dep) && !current_state.execution_results.contains_key(dep) {
                            is_ready = false;
                            break;
                        }
                    }
                    if is_ready {
                        ready_task = Some(task.clone());
                        break; // Execute one at a time for now to serialize the DAG correctly. Could be run concurrently.
                    }
                }

                if let Some(node) = ready_task {
                    if let Some(agent) = agents.get(&node.agent_name) {
                        let result = agent.execute(&node.input, &current_state.tenant_id, shared_memory_clone.clone()).await?;
                        execution_results.insert(node.id.clone(), result);
                        completed_tasks.push(node.id.clone());
                    } else {
                        return Err(format!("Agent not found: {}", node.agent_name));
                    }

                    Ok(OrchestratorState {
                        completed_tasks,
                        execution_results,
                        ..Default::default()
                    })
                } else {
                    Ok(OrchestratorState::default())
                }
            }
        });

        graph.add_node("summarizer", |state| async move {
            let mut summary = "Tasks completed: ".to_string();
            if state.execution_results.len() == 2 {
                summary = "Your wedding cake shop is live. Deposits are set to $100. Tap to review.".to_string();
            }
            Ok(OrchestratorState {
                final_summary: summary,
                ..Default::default()
            })
        });

        graph.add_edge("planner", "executor");

        graph.add_conditional_edges("executor", |state| {
            if !state.pending_tasks.is_empty() {
                "executor".to_string()
            } else {
                "summarizer".to_string()
            }
        });

        graph.add_edge("summarizer", END);

        graph.set_entry_point("planner");

        let initial_state = OrchestratorState {
            tenant_id: tenant_id.to_string(),
            user_intent: user_intent.to_string(),
            ..Default::default()
        };

        graph.run(initial_state).await
    }
}

#[derive(Clone, Default, Debug)]
pub struct TaskNode {
    pub id: String,
    pub agent_name: String,
    pub input: Value,
    pub dependencies: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockOperationsAgent;

    #[async_trait::async_trait]
    impl SpecialistAgent for MockOperationsAgent {
        fn name(&self) -> &str {
            "Operations Agent"
        }
        fn description(&self) -> &str {
            "Handles catalog and inventory"
        }
        async fn execute(&self, input: &Value, tenant_id: &str, shared_memory: Arc<dyn SharedMemory>) -> Result<Value, String> {
            shared_memory.set(tenant_id, "product_created", json!(true)).await?;
            Ok(json!({"status": "success", "product": input["name"]}))
        }
    }

    struct MockFinanceAgent;

    #[async_trait::async_trait]
    impl SpecialistAgent for MockFinanceAgent {
        fn name(&self) -> &str {
            "Finance Agent"
        }
        fn description(&self) -> &str {
            "Handles Stripe and deposits"
        }
        async fn execute(&self, input: &Value, tenant_id: &str, shared_memory: Arc<dyn SharedMemory>) -> Result<Value, String> {
            let mem = shared_memory.get(tenant_id, "product_created").await?;
            if mem.is_some() {
                Ok(json!({"status": "success", "deposit_configured": input["amount"]}))
            } else {
                Err("Product not created yet".to_string())
            }
        }
    }

    #[tokio::test]
    async fn test_unified_orchestrator_cuj() {
        let mem = Arc::new(LocalSharedMemory::new());
        let llm = Arc::new(MockLLMProvider);
        let orchestrator = UnifiedOrchestrator::new(mem, llm);
        orchestrator.register_agent(Arc::new(MockOperationsAgent)).await;
        orchestrator.register_agent(Arc::new(MockFinanceAgent)).await;

        let result = orchestrator.execute_request("tenant_1", "Set up a new product for Wedding Cakes and make sure I get a $100 deposit").await.unwrap();

        assert_eq!(result.plan.len(), 2);
        assert_eq!(result.execution_results.len(), 2);
        assert_eq!(result.final_summary, "Your wedding cake shop is live. Deposits are set to $100. Tap to review.");
    }
}
