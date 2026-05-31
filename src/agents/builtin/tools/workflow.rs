use crate::{Tool, ToolExecutor};
use ohc_builtin_agent_core::types::ToolError;
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Clone)]
struct WorkflowShard {
    label: &'static str,
    title: &'static str,
    scope: &'static str,
}

pub struct WorkflowExecutor {
    pub runner: Arc<dyn crate::runner::CommandRunner>,
}

#[async_trait::async_trait]
impl ToolExecutor for WorkflowExecutor {
    async fn execute(&self, args: Value) -> Result<String, ToolError> {
        let workflow = args
            .get("workflow")
            .and_then(Value::as_str)
            .unwrap_or("ohc_review_branch");
        let task = args
            .get("task")
            .and_then(Value::as_str)
            .unwrap_or("Review the current branch for correctness, security, deployment, and test coverage issues.");

        match workflow {
            "ohc_review_branch" | "review_branch" => self.run_review_branch(task).await,
            other => Err(ToolError::LlmRecoverable(format!(
                "Unknown workflow '{}'. Supported workflows: ohc_review_branch",
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
                scope: "package.json, pnpm-workspace.yaml, playwright.config.ts, vitest.config.ts, src/cli/**, verification_tests/**",
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
                scope: "all BUILD.bazel targets, verification_tests/**, deploy/tests/**, scripts/*test*, package test scripts",
            },
        ];

        let mut set = tokio::task::JoinSet::new();
        for shard in shards {
            let runner = self.runner.clone();
            let prompt = review_prompt(task, &shard);
            let label = shard.label.to_string();
            set.spawn(async move {
                let result = run_builtin_agent(runner, &prompt).await;
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
        let verification = run_builtin_agent(self.runner.clone(), &verification_prompt).await?;

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
        let final_report = run_builtin_agent(self.runner.clone(), &synthesis_prompt).await?;

        Ok(format!(
            "[Workflow: ohc_review_branch]\n\
             Phase 1 - shard review: completed with {} shard reports\n\
             Phase 2 - adversarial verification: completed\n\
             Phase 3 - synthesis: completed\n\n{}",
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

async fn run_builtin_agent(
    runner: Arc<dyn crate::runner::CommandRunner>,
    task: &str,
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
        "OHC_OPENAI_API_KEY",
        "OHC_OPENAI_BASE_URL",
        "OHC_ANTHROPIC_API_KEY",
        "OHC_ANTHROPIC_MODEL",
        "OHC_MINIMAX_API_KEY",
        "OHC_MINIMAX_MODEL",
        "OHC_MINIMAX_BASE_URL",
        "OHC_LOCAL_LLM_ENDPOINT",
    ] {
        if let Ok(value) = std::env::var(key) {
            envs.push((key.to_string(), value));
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
                    "enum": ["ohc_review_branch", "review_branch"],
                    "description": "The built-in workflow to run."
                },
                "task": {
                    "type": "string",
                    "description": "Optional task or focus area for the workflow."
                }
            },
            "required": ["workflow"]
        }),
        execute: Arc::new(WorkflowExecutor { runner }),
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
        let result = executor.execute(json!({"workflow": "unknown"})).await;

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
            .execute(json!({
                "workflow": "ohc_review_branch",
                "task": "review the branch"
            }))
            .await
            .unwrap();

        assert!(result.contains("[Workflow: ohc_review_branch]"));
        assert!(result.contains("Phase 1 - shard review: completed with 5 shard reports"));
        assert!(result.contains("final report"));
    }
}
