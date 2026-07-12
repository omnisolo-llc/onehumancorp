use crate::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
struct WorkflowShard {
    label: &'static str,
    title: &'static str,
    scope: &'static str,
}

#[derive(Clone)]
struct BusinessShard {
    label: &'static str,
    title: &'static str,
    focus: &'static str,
}

#[derive(serde::Deserialize)]
pub struct WorkflowArgs {
    workflow: Option<String>,
    task: Option<String>,
}

pub struct WorkflowExecutor {
    pub runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<WorkflowArgs> for WorkflowExecutor {
    async fn execute_typed(&self, args: WorkflowArgs) -> Result<String, ToolError> {
        let workflow = args.workflow.unwrap_or("ohc_review_branch".to_string());
        let task = args.task.unwrap_or("Review the current branch for correctness, security, deployment, and test coverage issues.".to_string());

        match workflow.as_str() {
            "ohc_review_branch" | "review_branch" => self.run_review_branch(&task).await,
            "ohc_business_swarm" | "business_swarm" => self.run_business_swarm(&task).await,
            other => Err(ToolError::LlmRecoverable(format!(
                "Unknown workflow '{}'. Supported workflows: ohc_review_branch, ohc_business_swarm",
                other
            ))),
        }
    }
}

impl WorkflowExecutor {
    async fn run_review_branch(&self, task: &str) -> Result<String, ToolError> {
        let shards = vec![
            WorkflowShard {
                label: "rust-bazel-reviewer",
                title: "Rust, Go, proto, and Bazel code",
                scope: "Cargo.toml, src/**, go.mod, MODULE.bazel, BUILD.bazel files, bazel/**, scripts/**",
            },
            WorkflowShard {
                label: "frontend-node-reviewer",
                title: "Node, TypeScript, frontend, and Playwright code",
                scope: "package.json, pnpm-workspace.yaml, playwright.config.ts, vitest.config.ts, src/cli/**",
            },
            WorkflowShard {
                label: "deploy-observability-reviewer",
                title: "Deployment, Helm, Docker, and observability assets",
                scope: "deploy/**, .github/workflows/**, Grafana dashboards, Prometheus config, release scripts",
            },
            WorkflowShard {
                label: "docs-contract-reviewer",
                title: "Documentation, API contracts, and developer workflow guidance",
                scope: "docs/**, README.md, CHANGELOG.md, RELEASE_NOTES.md, mkdocs.yml",
            },
            WorkflowShard {
                label: "test-coverage-reviewer",
                title: "Test coverage and validation strategy",
                scope: "all BUILD.bazel targets, deploy/tests/**, scripts/*test*, package test scripts",
            },
        ];

        let mut set = tokio::task::JoinSet::new();
        for shard in shards {
            let runner = self.runner.clone();
            let prompt = review_prompt(task, &shard);
            let label = shard.label.to_string();
            set.spawn(async move {
                let result = run_builtin_agent(runner, &prompt, false).await;
                (label, result)
            });
        }

        let mut shard_reports = Vec::new();
        while let Some(joined) = set.join_next().await {
            let (label, result) = joined.map_err(|e| {
                ToolError::LlmRecoverable(format!("Workflow shard task join failed: {}", e))
            })?;
            match result {
                Ok(report) => shard_reports.push(format!("## {}\n{}", label, report)),
                Err(err) => shard_reports.push(format!("## {}\nERROR: {}", label, err)),
            }
        }

        let shard_bundle = shard_reports.join("\n\n");
        let verification_prompt = format!(
            "You are the adversarial verifier for an OHC built-in agent workflow.\n\
             Workflow task: {}\n\n\
             Cross-check these shard reports. Reject duplicates, vague claims, unsupported claims, and findings that do not identify a concrete file and line. \
             Keep only actionable findings. Return a concise verified finding list plus rejected-findings notes.\n\n{}",
            task, shard_bundle
        );
        let verification = run_builtin_agent(self.runner.clone(), &verification_prompt, false).await?;

        let synthesis_prompt = format!(
            "You are the synthesizer for an OHC built-in agent workflow.\n\
             Produce the final branch review report from the verified findings.\n\n\
             Rules:\n\
             - Put blocker and high severity findings first.\n\
             - Include file and line references for every finding.\n\
             - Include test gaps and residual risk.\n\
             - Do not invent findings beyond the verified input.\n\n\
             Original task:\n{}\n\n\
             Shard reports:\n{}\n\n\
             Verified findings:\n{}",
            task, shard_bundle, verification
        );
        let final_report = run_builtin_agent(self.runner.clone(), &synthesis_prompt, false).await?;

        Ok(format!(
            "[Workflow: ohc_review_branch]\n\
             Phase 1 - shard review: completed with {} shard reports\n\
             Phase 2 - adversarial verification: completed\n\
             Phase 3 - synthesis: completed\n\n{}",
            shard_reports.len(),
            final_report
        ))
    }

    async fn run_business_swarm(&self, task: &str) -> Result<String, ToolError> {
        let shards = vec![
            BusinessShard {
                label: "revenue-strategist",
                title: "Revenue strategist",
                focus: "pricing, conversion, channel mix, promotions, and upsell opportunities",
            },
            BusinessShard {
                label: "operations-analyst",
                title: "Operations analyst",
                focus: "fulfillment, staffing, bottlenecks, support load, and process automation",
            },
            BusinessShard {
                label: "finance-controller",
                title: "Finance controller",
                focus: "cash flow, margin, unit economics, spend control, and risk-adjusted ROI",
            },
            BusinessShard {
                label: "customer-success-lead",
                title: "Customer success lead",
                focus: "retention, reviews, response quality, churn signals, and loyalty loops",
            },
            BusinessShard {
                label: "risk-compliance-reviewer",
                title: "Risk and compliance reviewer",
                focus: "operational, legal, privacy, payment, and brand safety risks",
            },
        ];

        let mut set = tokio::task::JoinSet::new();
        for shard in shards {
            let runner = self.runner.clone();
            let prompt = business_prompt(task, &shard);
            let label = shard.label.to_string();
            set.spawn(async move {
                let result = run_builtin_agent(runner, &prompt, true).await;
                (label, result)
            });
        }

        let mut shard_reports = Vec::new();
        while let Some(joined) = set.join_next().await {
            let (label, result) = joined.map_err(|e| {
                ToolError::LlmRecoverable(format!("Business swarm shard task join failed: {}", e))
            })?;
            match result {
                Ok(report) => shard_reports.push(format!("## {}\n{}", label, report)),
                Err(err) => shard_reports.push(format!("## {}\nERROR: {}", label, err)),
            }
        }

        let shard_bundle = shard_reports.join("\n\n");
        let verification_prompt = format!(
            "You are the verifier for an OHC business swarm.\n\
             Business objective: {}\n\n\
             Cross-check these specialist reports. Remove unsupported claims, duplicated recommendations, and actions that are not specific enough to execute. \
             Keep recommendations that are concrete, measurable, and useful to a small business operator. Return verified findings, rejected notes, and missing data.\n\n{}",
            task, shard_bundle
        );
        let verification = run_builtin_agent(self.runner.clone(), &verification_prompt, true).await?;

        let synthesis_prompt = format!(
            "You are the operating chief of staff for a small business using OHC agents.\n\
             Produce a concise operating plan from the verified specialist findings.\n\n\
             Rules:\n\
             - Start with the highest leverage actions.\n\
             - Assign each action to an agent role.\n\
             - Include expected business impact, required inputs, and first next step.\n\
             - Do not invent facts beyond the specialist reports and verified findings.\n\n\
             Original business objective:\n{}\n\n\
             Specialist reports:\n{}\n\n\
             Verified findings:\n{}",
            task, shard_bundle, verification
        );
        let final_report = run_builtin_agent(self.runner.clone(), &synthesis_prompt, true).await?;

        Ok(format!(
            "[Workflow: ohc_business_swarm]\n\
             Phase 1 - specialist agents: completed with {} shard reports\n\
             Phase 2 - adversarial verification: completed\n\
             Phase 3 - operating-plan synthesis: completed\n\n{}",
            shard_reports.len(),
            final_report
        ))
    }
}

fn review_prompt(task: &str, shard: &WorkflowShard) -> String {
    format!(
        "You are running as an OHC built-in workflow shard reviewer.\n\n\
         Workflow task: {}\n\
         Shard: {}\n\
         Scope: {}\n\n\
         Do a read-only review. Do not edit files. Use tools only to inspect repository state.\n\
         Prioritize concrete bugs, behavioral regressions, security risks, deployment failures, broken tests, and missing validation.\n\
         Every finding must cite an existing file and line. If you cannot prove a finding from repository evidence, omit it.\n\
         Return a concise report with findings, evidence, impact, recommendation, and test gaps.",
        task, shard.title, shard.scope
    )
}

fn business_prompt(task: &str, shard: &BusinessShard) -> String {
    format!(
        "You are running as an OHC business specialist agent.\n\n\
         Business objective: {}\n\
         Specialist: {}\n\
         Focus: {}\n\n\
         Analyze the business from your specialty. Use available context and tools when they are useful, but do not fabricate missing facts. \
         Return a concise report with: observations, recommended actions, expected impact, data needed, and risks. \
         Keep recommendations specific enough that another agent can execute the first step.",
        task, shard.title, shard.focus
    )
}

async fn run_builtin_agent(
    runner: Arc<dyn crate::runner::CommandRunner>,
    task: &str,
    disable_tools: bool,
) -> Result<String, ToolError> {
    let program = std::env::var("OHC_BUILTIN_AGENT_BINARY")
        .or_else(|_| std::env::var("OHC_AGENT_BINARY"))
        .unwrap_or_else(|_| "ohc_builtin_agent".to_string());

    let mut envs = Vec::new();
    for key in [
        "OHC_AGENT_ADDRESS",
        "OHC_AGENT_WORKSPACE",
        "OHC_LLM_PROVIDER",
        "OHC_LLM_MODEL",
        "OHC_LLM_BASE_URL",
        "OHC_LLM_ENDPOINT",
        "OHC_LLM_API_KEY",
        "OPENAI_API_KEY",
        "OPENAI_BASE_URL",
        "ANTHROPIC_API_KEY",
        "ANTHROPIC_MODEL",
        "MINIMAX_API_KEY",
        "MINIMAX_MODEL",
        "MINIMAX_BASE_URL",
        "OHC_LOCAL_LLM_ENDPOINT",
    ] {
        if let Ok(value) = std::env::var(key) {
            envs.push((key.to_string(), value));
        }
    }
    if disable_tools {
        envs.push(("OHC_AGENT_DISABLE_TOOLS".to_string(), "true".to_string()));
        envs.push(("OHC_AGENT_TASK_TIMEOUT_SECS".to_string(), "240".to_string()));
        envs.push(("OHC_LLM_TIMEOUT_SECS".to_string(), "180".to_string()));
        envs.push(("OHC_MAX_TOKENS".to_string(), "1200".to_string()));
        if std::env::var("TEST_WORKSPACE").is_ok() || std::env::var("BAZEL_TEST").is_ok() {
            envs.push(("OHC_AGENT_SPECIALIST_EXIT_HOLD_SECS".to_string(), "20".to_string()));
        }
    }

    let output = runner
        .run(&program, &["--task", task], None, envs)
        .await
        .map_err(|e| ToolError::LlmRecoverable(format!("Workflow agent launch failed: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(truncate_report(stdout))
    } else {
        Err(ToolError::LlmRecoverable(format!(
            "Workflow agent failed: {}",
            truncate_report(stderr)
        )))
    }
}

fn truncate_report(report: String) -> String {
    const MAX_CHARS: usize = 16_000;
    if report.chars().count() <= MAX_CHARS {
        return report;
    }

    let truncated = report.chars().take(MAX_CHARS).collect::<String>();
    format!(
        "{}\n\n[Output truncated by RunWorkflow after {} chars.]",
        truncated, MAX_CHARS
    )
}


// SOTA Harness Pattern: AutoGPT Unique Harness Innovations: Visual/low-code orchestration & Save workflow for reuse
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveWorkflowArgs {
    /// The exact name of the workflow to save.
    pub name: String,

    /// The script body.
    pub script: String,

    /// Whether to save globally (true) or per-project (false)
    pub is_global: bool,
}

struct SaveWorkflowExecutor {}

#[async_trait::async_trait]
impl PydanticToolExecutor<SaveWorkflowArgs> for SaveWorkflowExecutor {
    async fn execute_typed(&self, args: SaveWorkflowArgs) -> Result<String, ToolError> {
        let manager = ohc_builtin_agent_core::dynamic_workflows::WorkflowManager::new(std::env::current_dir().unwrap_or_default());
        manager.save_workflow(&args.name, &args.script, args.is_global).await.map_err(|e| ToolError::LlmRecoverable(e))?;
        Ok(format!("Successfully saved workflow '{}'.", args.name))
    }
}

pub fn save_workflow_tool() -> Tool {
    Tool {
        name: "SaveWorkflow".to_string(),
        description: "Save a dynamic workflow script for reuse. Used when you want to save an orchestration script so it can be called directly. (Claude Code Mechanic: Save the workflow for reuse)".to_string(),
        is_read_only: false,
        parameters: serde_json::json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "The exact name of the workflow to save."
                },
                "script": {
                    "type": "string",
                    "description": "The script body."
                },
                "is_global": {
                    "type": "boolean",
                    "description": "Whether to save globally (true) or per-project (false)"
                }
            },
            "required": ["name", "script", "is_global"]
        }),
        execute: Arc::new(PydanticAdapter::new(SaveWorkflowExecutor {})),
    }
}

pub fn workflow_tool(runner: Arc<dyn crate::runner::CommandRunner>) -> Tool {
    Tool {
        name: "RunWorkflow".to_string(),
        description: "Run an OHC built-in multi-agent workflow. The workflow coordinates phases, spawns subagents, verifies findings, and returns one final report.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "workflow": {
                    "type": "string",
                    "enum": ["ohc_review_branch", "review_branch", "ohc_business_swarm", "business_swarm"],
                    "description": "The built-in workflow to run."
                },
                "task": {
                    "type": "string",
                    "description": "Optional task or focus area for the workflow."
                }
            },
            "required": ["workflow"]
        }),
        execute: Arc::new(PydanticAdapter::new(WorkflowExecutor { runner })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_prompt_contains_read_only_guardrail() {
        let shard = WorkflowShard {
            label: "test",
            title: "Test shard",
            scope: "src/**",
        };
        let prompt = review_prompt("review branch", &shard);
        assert!(prompt.contains("Do a read-only review"));
        assert!(prompt.contains("Every finding must cite an existing file and line"));
    }

    #[tokio::test]
    async fn run_workflow_rejects_unknown_workflow() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());
        let executor = WorkflowExecutor { runner };

        let result = executor.execute_typed(serde_json::from_value(json!({"workflow": "unknown"})).unwrap()).await;

        assert!(matches!(result, Err(ToolError::LlmRecoverable(_))));
    }

    #[tokio::test]
    async fn run_workflow_executes_shards_verifier_and_synthesizer() {
        let runner = Arc::new(crate::runner::mock::MockCommandRunner::new());

        for label in [
            "rust report",
            "frontend report",
            "deploy report",
            "docs report",
            "tests report",
            "verified findings",
            "final report",
        ] {
            runner.push_response(Ok(crate::runner::mock::mock_output(0, label, "")));
        }

        let executor = WorkflowExecutor { runner };

        let result = executor
            .execute_typed(serde_json::from_value(json!({
                "workflow": "ohc_review_branch",
                "task": "review the branch"
            })).unwrap())
            .await
            .unwrap();

        assert!(result.contains("[Workflow: ohc_review_branch]"));
        assert!(result.contains("Phase 1 - shard review: completed with 5 shard reports"));
        assert!(result.contains("final report"));
    }
}
