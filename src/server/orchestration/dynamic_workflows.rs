use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use crate::queue::{Job, TaskQueue};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
    #[serde(rename = "xhigh", alias = "x_high")]
    XHigh,
    Ultracode,
}

impl Default for ReasoningEffort {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowTrigger {
    ExplicitRequest,
    UltracodeAuto,
    NotTriggered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowStatus {
    AwaitingConfirmation,
    Queued,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicWorkflowRequest {
    pub tenant_id: String,
    pub parent_task_id: Option<String>,
    pub prompt: String,
    #[serde(default)]
    pub effort: ReasoningEffort,
    #[serde(default)]
    pub auto_mode: bool,
    #[serde(default)]
    pub confirm: bool,
    pub max_parallel_agents: Option<usize>,
    pub verifier_agents_per_task: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerDecision {
    pub should_create_workflow: bool,
    pub trigger: WorkflowTrigger,
    pub requires_confirmation: bool,
    pub reason: String,
    pub complexity_score: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub role: String,
    pub phase: String,
    pub dependencies: Vec<String>,
    pub verification_of: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicWorkflowPlan {
    pub id: String,
    pub tenant_id: String,
    pub parent_task_id: String,
    pub prompt: String,
    pub status: WorkflowStatus,
    pub trigger: WorkflowTrigger,
    pub requires_confirmation: bool,
    pub estimated_subagents: usize,
    pub estimated_token_multiplier: f32,
    pub tasks: Vec<WorkflowTask>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicWorkflowStart {
    pub plan: DynamicWorkflowPlan,
    pub enqueued_jobs: usize,
}

pub struct DynamicWorkflowManager {
    queue: Arc<dyn TaskQueue>,
    plans: RwLock<HashMap<String, DynamicWorkflowPlan>>,
    state_dir: Option<PathBuf>,
}

impl DynamicWorkflowManager {
    pub fn new(queue: Arc<dyn TaskQueue>) -> Self {
        Self {
            queue,
            plans: RwLock::new(HashMap::new()),
            state_dir: None,
        }
    }

    pub fn with_state_dir(queue: Arc<dyn TaskQueue>, state_dir: PathBuf) -> Self {
        Self {
            queue,
            plans: RwLock::new(HashMap::new()),
            state_dir: Some(state_dir),
        }
    }

    pub fn decide(request: &DynamicWorkflowRequest) -> TriggerDecision {
        let prompt = request.prompt.to_lowercase();
        let explicit = prompt.contains("create a workflow")
            || prompt.contains("dynamic workflow")
            || prompt.contains("run a workflow")
            || prompt.contains("start a workflow");

        let complexity_score = score_complexity(&prompt);
        let ultracode = request.effort == ReasoningEffort::Ultracode
            || (request.effort == ReasoningEffort::XHigh && request.auto_mode);

        if explicit {
            return TriggerDecision {
                should_create_workflow: true,
                trigger: WorkflowTrigger::ExplicitRequest,
                requires_confirmation: !request.confirm,
                reason: "prompt explicitly requested a workflow".to_string(),
                complexity_score,
            };
        }

        if ultracode && complexity_score >= 3 {
            return TriggerDecision {
                should_create_workflow: true,
                trigger: WorkflowTrigger::UltracodeAuto,
                requires_confirmation: !request.confirm,
                reason: "ultracode selected a complex parallelizable task".to_string(),
                complexity_score,
            };
        }

        TriggerDecision {
            should_create_workflow: false,
            trigger: WorkflowTrigger::NotTriggered,
            requires_confirmation: false,
            reason: "task does not require a dynamic workflow".to_string(),
            complexity_score,
        }
    }

    pub async fn start_workflow(
        &self,
        request: DynamicWorkflowRequest,
    ) -> Result<DynamicWorkflowStart, String> {
        let decision = Self::decide(&request);
        if !decision.should_create_workflow {
            return Err(decision.reason);
        }

        let mut plan = build_plan(request, decision)?;
        let enqueued_jobs = if plan.requires_confirmation {
            0
        } else {
            let jobs = self.build_jobs(&plan);
            let count = jobs.len();
            self.queue.enqueue_batch(jobs).await?;
            plan.status = WorkflowStatus::Queued;
            plan.updated_at = Utc::now();
            count
        };

        self.store_plan(plan.clone())?;
        Ok(DynamicWorkflowStart {
            plan,
            enqueued_jobs,
        })
    }

    pub async fn confirm_workflow(
        &self,
        workflow_id: &str,
    ) -> Result<DynamicWorkflowStart, String> {
        let mut plan = self
            .get_workflow(workflow_id)?
            .ok_or_else(|| "workflow not found".to_string())?;

        if plan.status != WorkflowStatus::AwaitingConfirmation {
            return Ok(DynamicWorkflowStart {
                plan,
                enqueued_jobs: 0,
            });
        }

        let jobs = self.build_jobs(&plan);
        let enqueued_jobs = jobs.len();
        self.queue.enqueue_batch(jobs).await?;

        plan.status = WorkflowStatus::Queued;
        plan.requires_confirmation = false;
        plan.updated_at = Utc::now();
        self.store_plan(plan.clone())?;

        Ok(DynamicWorkflowStart {
            plan,
            enqueued_jobs,
        })
    }

    pub fn get_workflow(&self, workflow_id: &str) -> Result<Option<DynamicWorkflowPlan>, String> {
        if let Some(plan) = self.plans.read().unwrap().get(workflow_id).cloned() {
            return Ok(Some(plan));
        }

        if let Some(plan) = self.load_plan(workflow_id)? {
            self.plans
                .write()
                .unwrap()
                .insert(workflow_id.to_string(), plan.clone());
            return Ok(Some(plan));
        }

        Ok(None)
    }

    fn build_jobs(&self, plan: &DynamicWorkflowPlan) -> Vec<Job> {
        let now = Utc::now();
        plan.tasks
            .iter()
            .map(|task| {
                let payload = serde_json::json!({
                    "dynamic_workflow": true,
                    "workflow_id": plan.id,
                    "task_id": task.id,
                    "title": task.title,
                    "description": task.description,
                    "phase": task.phase,
                    "dependencies": task.dependencies,
                    "verification_of": task.verification_of,
                    "agent_role": task.role,
                    "prompt": plan.prompt,
                });

                Job {
                    id: task.id.clone(),
                    tenant_id: plan.tenant_id.clone(),
                    parent_task_id: plan.parent_task_id.clone(),
                    job_type: task.role.clone(),
                    payload: payload.to_string(),
                    status: "PENDING".to_string(),
                    retry_count: 0,
                    max_retries: 2,
                    next_retry_at: now,
                    locked_until: None,
                    created_at: now,
                    updated_at: now,
                }
            })
            .collect()
    }

    fn store_plan(&self, plan: DynamicWorkflowPlan) -> Result<(), String> {
        self.plans
            .write()
            .unwrap()
            .insert(plan.id.clone(), plan.clone());

        if let Some(state_dir) = &self.state_dir {
            std::fs::create_dir_all(state_dir).map_err(|e| e.to_string())?;
            let path = state_path(state_dir, &plan.id);
            let content = serde_json::to_string_pretty(&plan).map_err(|e| e.to_string())?;
            std::fs::write(path, content).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn load_plan(&self, workflow_id: &str) -> Result<Option<DynamicWorkflowPlan>, String> {
        let Some(state_dir) = &self.state_dir else {
            return Ok(None);
        };
        let path = state_path(state_dir, workflow_id);
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let plan = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        Ok(Some(plan))
    }
}

fn score_complexity(prompt: &str) -> usize {
    let signals = [
        "codebase-wide",
        "entire",
        "all files",
        "hundreds",
        "thousands",
        "migration",
        "migrate",
        "rewrite",
        "modernization",
        "framework swap",
        "audit",
        "security",
        "profiler",
        "optimization",
        "verify",
        "adversarial",
        "large",
        "legacy",
    ];

    signals
        .iter()
        .filter(|signal| prompt.contains(**signal))
        .count()
}

fn build_plan(
    request: DynamicWorkflowRequest,
    decision: TriggerDecision,
) -> Result<DynamicWorkflowPlan, String> {
    let tenant_id = if request.tenant_id.trim().is_empty() {
        "default".to_string()
    } else {
        request.tenant_id
    };
    let parent_task_id = request
        .parent_task_id
        .filter(|id| !id.trim().is_empty())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let workflow_id = format!("dwf-{}", uuid::Uuid::new_v4());
    let max_parallel_agents = request.max_parallel_agents.unwrap_or(8).clamp(2, 128);
    let verifiers_per_task = request.verifier_agents_per_task.unwrap_or(1).clamp(1, 3);
    let now = Utc::now();

    let work_items = infer_work_items(&request.prompt, max_parallel_agents);
    let mut tasks = Vec::new();

    let planner_id = format!("{}-plan", workflow_id);
    tasks.push(WorkflowTask {
        id: planner_id.clone(),
        title: "Plan workflow shards".to_string(),
        description: format!(
            "Map the requested work into independently executable shards: {}",
            request.prompt
        ),
        role: "workflow-planner".to_string(),
        phase: "planning".to_string(),
        dependencies: vec![],
        verification_of: None,
    });

    let mut execution_ids = Vec::new();
    for (idx, item) in work_items.iter().enumerate() {
        let task_id = format!("{}-exec-{}", workflow_id, idx + 1);
        execution_ids.push(task_id.clone());
        tasks.push(WorkflowTask {
            id: task_id,
            title: item.title.clone(),
            description: item.description.clone(),
            role: item.role.clone(),
            phase: "execution".to_string(),
            dependencies: vec![planner_id.clone()],
            verification_of: None,
        });
    }

    let mut verification_ids = Vec::new();
    for execution_id in &execution_ids {
        for verifier_idx in 0..verifiers_per_task {
            let task_id = format!(
                "{}-verify-{}-{}",
                workflow_id,
                verification_ids.len() + 1,
                verifier_idx + 1
            );
            verification_ids.push(task_id.clone());
            tasks.push(WorkflowTask {
                id: task_id,
                title: "Verify workflow shard".to_string(),
                description: "Independently review the assigned shard result, try to refute it, and report only confirmed issues or fixes.".to_string(),
                role: if verifier_idx == 0 {
                    "adversarial-reviewer".to_string()
                } else {
                    "independent-verifier".to_string()
                },
                phase: "verification".to_string(),
                dependencies: vec![execution_id.clone()],
                verification_of: Some(execution_id.clone()),
            });
        }
    }

    let mut synthesis_deps = execution_ids;
    synthesis_deps.extend(verification_ids);
    tasks.push(WorkflowTask {
        id: format!("{}-synthesis", workflow_id),
        title: "Synthesize checked workflow result".to_string(),
        description: "Fold only verified results into the final coordinated answer and call out residual risks.".to_string(),
        role: "workflow-synthesizer".to_string(),
        phase: "synthesis".to_string(),
        dependencies: synthesis_deps,
        verification_of: None,
    });

    Ok(DynamicWorkflowPlan {
        id: workflow_id,
        tenant_id,
        parent_task_id,
        prompt: request.prompt,
        status: if decision.requires_confirmation {
            WorkflowStatus::AwaitingConfirmation
        } else {
            WorkflowStatus::Queued
        },
        trigger: decision.trigger,
        requires_confirmation: decision.requires_confirmation,
        estimated_subagents: tasks.len(),
        estimated_token_multiplier: estimate_token_multiplier(tasks.len(), verifiers_per_task),
        tasks,
        created_at: now,
        updated_at: now,
    })
}

#[derive(Debug)]
struct WorkItem {
    title: String,
    description: String,
    role: String,
}

fn infer_work_items(prompt: &str, max_parallel_agents: usize) -> Vec<WorkItem> {
    let prompt_lc = prompt.to_lowercase();
    let mut titles = if prompt_lc.contains("security") || prompt_lc.contains("audit") {
        vec![
            ("Authentication and authorization audit", "security-auditor"),
            ("Input validation and injection audit", "security-auditor"),
            (
                "Secret handling and data exposure audit",
                "security-auditor",
            ),
            ("Dependency and configuration audit", "security-auditor"),
        ]
    } else if prompt_lc.contains("migration")
        || prompt_lc.contains("migrate")
        || prompt_lc.contains("rewrite")
        || prompt_lc.contains("modernization")
    {
        vec![
            ("Inventory migration surface", "migration-mapper"),
            ("Port shared interfaces", "migration-worker"),
            ("Port data and persistence layer", "migration-worker"),
            ("Port user-facing workflows", "migration-worker"),
            ("Fix build and test failures", "migration-worker"),
        ]
    } else if prompt_lc.contains("optimization") || prompt_lc.contains("profiler") {
        vec![
            ("Profile hot paths", "performance-analyst"),
            (
                "Inspect allocation and copying behavior",
                "performance-analyst",
            ),
            ("Review IO and database latency", "performance-analyst"),
            ("Prepare low-risk optimizations", "performance-worker"),
        ]
    } else {
        vec![
            ("Map affected code and docs", "codebase-mapper"),
            (
                "Implement independent work shard A",
                "implementation-worker",
            ),
            (
                "Implement independent work shard B",
                "implementation-worker",
            ),
            ("Run focused validation", "validation-worker"),
        ]
    };

    titles.truncate(max_parallel_agents);
    titles
        .into_iter()
        .map(|(title, role)| WorkItem {
            title: title.to_string(),
            description: format!("{} for prompt: {}", title, prompt),
            role: role.to_string(),
        })
        .collect()
}

fn estimate_token_multiplier(task_count: usize, verifiers_per_task: usize) -> f32 {
    let fanout = task_count as f32;
    let verifier_weight = verifiers_per_task as f32 * 0.35;
    (1.0 + fanout * (0.45 + verifier_weight)).min(100.0)
}

fn state_path(state_dir: &Path, workflow_id: &str) -> PathBuf {
    state_dir.join(format!("{}.json", workflow_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    #[derive(Default)]
    struct RecordingQueue {
        jobs: Mutex<Vec<Job>>,
    }

    #[async_trait]
    impl TaskQueue for RecordingQueue {
        async fn enqueue(&self, job: Job) -> Result<(), String> {
            self.jobs.lock().unwrap().push(job);
            Ok(())
        }

        async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
            self.jobs.lock().unwrap().extend(jobs);
            Ok(())
        }

        async fn dequeue(&self, _roles: Vec<String>) -> Result<Option<Job>, String> {
            Ok(None)
        }

        async fn complete(&self, _job_id: &str, _tenant_id: &str) -> Result<(), String> {
            Ok(())
        }

        async fn fail(&self, _job_id: &str, _tenant_id: &str, _reason: &str) -> Result<(), String> {
            Ok(())
        }

        async fn requeue(&self, job: Job) -> Result<(), String> {
            self.jobs.lock().unwrap().push(job);
            Ok(())
        }
    }

    fn request(prompt: &str) -> DynamicWorkflowRequest {
        DynamicWorkflowRequest {
            tenant_id: "tenant-1".to_string(),
            parent_task_id: Some("parent-1".to_string()),
            prompt: prompt.to_string(),
            effort: ReasoningEffort::Medium,
            auto_mode: false,
            confirm: false,
            max_parallel_agents: Some(4),
            verifier_agents_per_task: Some(1),
        }
    }

    #[test]
    fn explicit_request_requires_confirmation() {
        let req = request("Create a workflow to audit the entire service");
        let decision = DynamicWorkflowManager::decide(&req);

        assert!(decision.should_create_workflow);
        assert_eq!(decision.trigger, WorkflowTrigger::ExplicitRequest);
        assert!(decision.requires_confirmation);
    }

    #[test]
    fn ultracode_auto_only_triggers_complex_tasks() {
        let mut simple = request("Fix typo");
        simple.effort = ReasoningEffort::Ultracode;
        simple.auto_mode = true;
        assert!(!DynamicWorkflowManager::decide(&simple).should_create_workflow);

        let mut complex =
            request("Perform a codebase-wide security audit and verify every finding");
        complex.effort = ReasoningEffort::Ultracode;
        complex.auto_mode = true;
        let decision = DynamicWorkflowManager::decide(&complex);
        assert!(decision.should_create_workflow);
        assert_eq!(decision.trigger, WorkflowTrigger::UltracodeAuto);
    }

    #[tokio::test]
    async fn confirmed_workflow_enqueues_fanout_and_verification_jobs() {
        let queue = Arc::new(RecordingQueue::default());
        let manager = DynamicWorkflowManager::new(queue.clone());
        let mut req = request("Create a workflow to migrate the legacy API");
        req.confirm = true;

        let start = manager.start_workflow(req).await.unwrap();

        assert_eq!(start.plan.status, WorkflowStatus::Queued);
        assert!(start.enqueued_jobs >= 4);
        let jobs = queue.jobs.lock().unwrap();
        assert_eq!(jobs.len(), start.enqueued_jobs);
        assert!(jobs.iter().any(|job| {
            let payload: serde_json::Value = serde_json::from_str(&job.payload).unwrap();
            payload["phase"] == "verification" && payload["agent_role"] == "adversarial-reviewer"
        }));
    }

    #[tokio::test]
    async fn confirmation_queues_prepared_plan() {
        let queue = Arc::new(RecordingQueue::default());
        let manager = DynamicWorkflowManager::new(queue.clone());

        let start = manager
            .start_workflow(request("Create a workflow to optimize the entire service"))
            .await
            .unwrap();
        assert_eq!(start.enqueued_jobs, 0);
        assert_eq!(start.plan.status, WorkflowStatus::AwaitingConfirmation);

        let confirmed = manager.confirm_workflow(&start.plan.id).await.unwrap();
        assert_eq!(confirmed.plan.status, WorkflowStatus::Queued);
        assert_eq!(queue.jobs.lock().unwrap().len(), confirmed.enqueued_jobs);
    }
}
