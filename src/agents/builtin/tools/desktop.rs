use crate::{
    Tool,
    binary_dist::{ManagedBinaryKind, ensure_managed_binary},
    browser::AgentBrowserController,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
    runner::CommandRunner,
};
use omnisolo_builtin_agent_core::types::ToolError;
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

static SECRET_REDACTION_REGEX: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
    Regex::new(
        r"(?i)(bearer\s+[A-Za-z0-9._\-]{16,}|sk-[A-Za-z0-9_\-]{16,}|gh[pousr]_[A-Za-z0-9]{20,})",
    )
    .expect("valid redaction regex")
});

fn redact_sensitive_desktop_output(raw: &str) -> String {
    SECRET_REDACTION_REGEX
        .replace_all(raw, "[REDACTED_SECRET]")
        .into_owned()
}

pub fn desktop_tools_enabled() -> bool {
    let opted_in = std::env::var("OMNISOLO_ENABLE_AGENT_DESKTOP").is_ok_and(|v| {
        matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        )
    });
    if !opted_in {
        return false;
    }
    let mode = std::env::var("OMNISOLO_AGENT_EXECUTION_MODE")
        .or_else(|_| std::env::var("OMNISOLO_EXECUTION_MODE"))
        .or_else(|_| std::env::var("OMNISOLO_SOURCE_MODE"))
        .unwrap_or_else(|_| "standalone".to_string())
        .trim()
        .to_ascii_lowercase();
    let is_container_backend = |var_name: &str| {
        std::env::var(var_name).is_ok_and(|raw| {
            matches!(
                raw.trim().to_ascii_lowercase().as_str(),
                "container"
                    | "docker"
                    | "podman"
                    | "ssh"
                    | "singularity"
                    | "modal"
                    | "daytona"
                    | "vercal"
            )
        })
    };

    mode == "standalone"
        && !is_container_backend("OMNISOLO_AGENT_COMMAND_BACKEND")
        && !is_container_backend("OMNISOLO_AGENT_SANDBOX_BACKEND")
}

fn verify_allowed_app(
    app: Option<&str>,
    require_when_allowlist_set: bool,
) -> Result<(), ToolError> {
    if let Some(name) = app
        && name.trim().starts_with('-')
    {
        return Err(ToolError::LlmRecoverable(
            "desktop: application name must not start with '-'".to_string(),
        ));
    }

    let Ok(raw_allowlist) = std::env::var("OMNISOLO_AGENT_DESKTOP_ALLOWED_APPS") else {
        return Ok(());
    };
    let allowed: Vec<String> = raw_allowlist
        .split(',')
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    if allowed.is_empty() {
        return Ok(());
    }

    let Some(requested) = app.map(str::trim).filter(|s| !s.is_empty()) else {
        if require_when_allowlist_set {
            return Err(ToolError::LlmRecoverable(
                "desktop: 'app' is required because OMNISOLO_AGENT_DESKTOP_ALLOWED_APPS is enforced"
                    .to_string(),
            ));
        }
        return Ok(());
    };

    let req_lower = requested.to_ascii_lowercase();
    if !allowed.iter().any(|item| item == &req_lower) {
        return Err(ToolError::LlmRecoverable(format!(
            "desktop: application '{requested}' is not in OMNISOLO_AGENT_DESKTOP_ALLOWED_APPS"
        )));
    }
    Ok(())
}

fn ensure_safe_cli_token(
    label: &str,
    value: &str,
    reject_leading_dash: bool,
) -> Result<(), ToolError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(ToolError::LlmRecoverable(format!(
            "desktop: '{label}' must not be empty"
        )));
    }
    if trimmed == "--headed" || trimmed.starts_with("--headed=") {
        return Err(ToolError::Fatal(
            "desktop: physical '--headed' cursor mode is forbidden by OmniSolo safety policy"
                .to_string(),
        ));
    }
    if reject_leading_dash && trimmed.starts_with('-') {
        return Err(ToolError::LlmRecoverable(format!(
            "desktop: '{label}' must not start with '-'"
        )));
    }
    Ok(())
}

#[derive(Clone)]
pub struct AgentDesktopController {
    working_dir: Option<PathBuf>,
    runner: Arc<dyn CommandRunner>,
    reconciliation_required: Arc<AtomicBool>,
}

impl AgentDesktopController {
    pub fn new(working_dir: Option<PathBuf>, runner: Arc<dyn CommandRunner>) -> Self {
        Self {
            working_dir,
            runner,
            reconciliation_required: Arc::new(AtomicBool::new(false)),
        }
    }

    async fn run_desktop_command(
        &self,
        args: &[String],
        is_mutating: bool,
        clear_reconciliation_on_success: bool,
    ) -> Result<String, ToolError> {
        if !desktop_tools_enabled() {
            return Err(ToolError::Fatal(
                "desktop: agent-desktop is disabled outside standalone local opt-in mode (set OMNISOLO_ENABLE_AGENT_DESKTOP=true on a standalone host)".to_string(),
            ));
        }
        for arg in args {
            if arg == "--headed" || arg.starts_with("--headed=") {
                return Err(ToolError::Fatal(
                    "desktop: '--headed' physical input hijacking is forbidden".to_string(),
                ));
            }
        }
        if is_mutating && self.reconciliation_required.load(Ordering::SeqCst) {
            return Err(ToolError::LlmRecoverable(
                "desktop: previous mutating desktop action had an unknown outcome; run DesktopObserve 'snapshot' to reconcile UI state before retrying".to_string(),
            ));
        }

        let binary = ensure_managed_binary(ManagedBinaryKind::AgentDesktop).await?;
        let binary_str = binary.to_string_lossy().into_owned();
        let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();

        let exec_res = tokio::time::timeout(
            std::time::Duration::from_secs(30),
            self.runner.run(
                &binary_str,
                &arg_refs,
                self.working_dir.as_deref(),
                Vec::new(),
            ),
        )
        .await;

        let output = match exec_res {
            Ok(Ok(out)) => out,
            Ok(Err(err)) => {
                if is_mutating {
                    self.reconciliation_required.store(true, Ordering::SeqCst);
                }
                return Err(ToolError::LlmRecoverable(format!(
                    "desktop: failed to execute agent-desktop: {err}"
                )));
            }
            Err(_) => {
                if is_mutating {
                    self.reconciliation_required.store(true, Ordering::SeqCst);
                }
                return Err(ToolError::LlmRecoverable(
                    "desktop: command timed out after 30s; run DesktopObserve 'snapshot' before retrying".to_string(),
                ));
            }
        };

        let stdout = redact_sensitive_desktop_output(&String::from_utf8_lossy(&output.stdout));
        let stderr = redact_sensitive_desktop_output(&String::from_utf8_lossy(&output.stderr));

        if !output.status.success() {
            let combined = if stdout.trim().is_empty() {
                stderr.trim().to_string()
            } else {
                stdout.trim().to_string()
            };
            let lower = combined.to_ascii_lowercase();
            if is_mutating && (lower.contains("timed out") || lower.contains("timeout")) {
                self.reconciliation_required.store(true, Ordering::SeqCst);
            }
            return Err(ToolError::LlmRecoverable(format!(
                "desktop command failed (exit {}): {}",
                output.status.code().unwrap_or(-1),
                combined
            )));
        }

        if clear_reconciliation_on_success {
            self.reconciliation_required.store(false, Ordering::SeqCst);
        }

        Ok(stdout.trim().to_string())
    }
}

#[derive(Deserialize)]
struct DesktopObserveArgs {
    /// One of: "permissions", "list-apps", "list-windows", "list-surfaces", "snapshot", "find", "get", "is", "screenshot"
    action: String,
    #[serde(default)]
    app: Option<String>,
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    property: Option<String>,
    #[serde(default)]
    max_depth: Option<u32>,
    #[serde(default)]
    path: Option<String>,
}

struct DesktopObserveExecutor {
    controller: AgentDesktopController,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<DesktopObserveArgs> for DesktopObserveExecutor {
    async fn execute_typed(&self, args: DesktopObserveArgs) -> Result<String, ToolError> {
        let action = args.action.trim().to_ascii_lowercase();
        match action.as_str() {
            "permissions" | "list-apps" | "list-surfaces" => {
                verify_allowed_app(args.app.as_deref(), false)?;
                self.controller
                    .run_desktop_command(&[action], false, false)
                    .await
            }
            "list-windows" => {
                verify_allowed_app(args.app.as_deref(), false)?;
                let mut cmd = vec!["list-windows".to_string()];
                if let Some(app) = args.app.as_deref().filter(|s| !s.trim().is_empty()) {
                    cmd.push("--app".to_string());
                    cmd.push(app.trim().to_string());
                }
                self.controller
                    .run_desktop_command(&cmd, false, false)
                    .await
            }
            "snapshot" => {
                verify_allowed_app(args.app.as_deref(), true)?;
                let mut cmd = vec!["snapshot".to_string()];
                if let Some(app) = args.app.as_deref().filter(|s| !s.trim().is_empty()) {
                    cmd.push("--app".to_string());
                    cmd.push(app.trim().to_string());
                }
                if let Some(depth) = args.max_depth {
                    cmd.push("--max-depth".to_string());
                    cmd.push(depth.min(12).to_string());
                }
                self.controller.run_desktop_command(&cmd, false, true).await
            }
            "find" => {
                verify_allowed_app(args.app.as_deref(), true)?;
                let query = args.target.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(
                        "DesktopObserve 'find' requires 'target' query text or role".to_string(),
                    )
                })?;
                ensure_safe_cli_token("target", query, false)?;
                let mut cmd = vec!["find".to_string()];
                if let Some(app) = args.app.as_deref().filter(|s| !s.trim().is_empty()) {
                    cmd.push("--app".to_string());
                    cmd.push(app.trim().to_string());
                }
                cmd.push("--".to_string());
                cmd.push(query.trim().to_string());
                self.controller.run_desktop_command(&cmd, false, true).await
            }
            "get" | "is" => {
                verify_allowed_app(args.app.as_deref(), false)?;
                let target = args.target.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(format!(
                        "DesktopObserve '{action}' requires 'target' @ref"
                    ))
                })?;
                let prop = args.property.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(format!(
                        "DesktopObserve '{action}' requires 'property'"
                    ))
                })?;
                ensure_safe_cli_token("target", target, true)?;
                ensure_safe_cli_token("property", prop, true)?;
                self.controller
                    .run_desktop_command(
                        &[action, target.trim().to_string(), prop.trim().to_string()],
                        false,
                        true,
                    )
                    .await
            }
            "screenshot" => {
                let app = args
                    .app
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| {
                        ToolError::LlmRecoverable(
                            "DesktopObserve 'screenshot' requires an explicit 'app' name (unscoped full-screen capture is blocked)".to_string(),
                        )
                    })?;
                verify_allowed_app(Some(app), true)?;
                let path = args.path.as_deref().unwrap_or("desktop-window.png").trim();
                AgentBrowserController::validate_relative_output_path(path)?;
                self.controller
                    .run_desktop_command(
                        &[
                            "screenshot".to_string(),
                            "--app".to_string(),
                            app.to_string(),
                            "--output".to_string(),
                            path.to_string(),
                        ],
                        false,
                        false,
                    )
                    .await
            }
            other => Err(ToolError::LlmRecoverable(format!(
                "DesktopObserve: unsupported or policy-blocked action '{other}'"
            ))),
        }
    }
}

#[derive(Deserialize)]
struct DesktopActArgs {
    /// Semantic headless actions only (physical mouse/headed/clipboard actions are blocked)
    action: String,
    #[serde(default)]
    app: Option<String>,
    #[serde(default)]
    target: Option<String>,
    #[serde(default)]
    value: Option<String>,
}

struct DesktopActExecutor {
    controller: AgentDesktopController,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<DesktopActArgs> for DesktopActExecutor {
    async fn execute_typed(&self, args: DesktopActArgs) -> Result<String, ToolError> {
        let action = args.action.trim().to_ascii_lowercase();
        if matches!(
            action.as_str(),
            "double-click"
                | "triple-click"
                | "hover"
                | "drag"
                | "mouse-move"
                | "mouse-click"
                | "mouse-down"
                | "mouse-up"
                | "mouse-wheel"
                | "cursor-overlay"
                | "clipboard-get"
                | "clipboard-set"
                | "open-system-surface"
        ) {
            return Err(ToolError::Fatal(format!(
                "DesktopAct: action '{action}' is blocked by OmniSolo safety policy (only headless accessibility actions are permitted)"
            )));
        }

        verify_allowed_app(args.app.as_deref(), true)?;

        match action.as_str() {
            "click" | "right-click" | "clear" | "focus" | "toggle" | "check" | "uncheck"
            | "expand" | "collapse" | "scroll-to" => {
                let target = args.target.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(format!(
                        "DesktopAct '{action}' requires 'target' @ref"
                    ))
                })?;
                ensure_safe_cli_token("target", target, true)?;
                self.controller
                    .run_desktop_command(&[action, target.trim().to_string()], true, false)
                    .await
            }
            "type" | "set-value" | "select" => {
                let target = args.target.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(format!(
                        "DesktopAct '{action}' requires 'target' @ref"
                    ))
                })?;
                let value = args.value.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(format!("DesktopAct '{action}' requires 'value'"))
                })?;
                ensure_safe_cli_token("target", target, true)?;
                ensure_safe_cli_token("value", value, false)?;
                self.controller
                    .run_desktop_command(
                        &[
                            action,
                            "--".to_string(),
                            target.trim().to_string(),
                            value.to_string(),
                        ],
                        true,
                        false,
                    )
                    .await
            }
            "press" | "wait" => {
                let target = args.target.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(format!("DesktopAct '{action}' requires 'target'"))
                })?;
                ensure_safe_cli_token("target", target, true)?;
                let is_mutating = action != "wait";
                self.controller
                    .run_desktop_command(&[action, target.trim().to_string()], is_mutating, false)
                    .await
            }
            "scroll" => {
                let target = args.target.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(
                        "DesktopAct 'scroll' requires 'target' @ref".to_string(),
                    )
                })?;
                ensure_safe_cli_token("target", target, true)?;
                let direction = args.value.as_deref().unwrap_or("down").trim();
                if !matches!(direction, "up" | "down" | "left" | "right") {
                    return Err(ToolError::LlmRecoverable(
                        "DesktopAct 'scroll' value must be 'up', 'down', 'left', or 'right'"
                            .to_string(),
                    ));
                }
                self.controller
                    .run_desktop_command(
                        &[
                            "scroll".to_string(),
                            target.trim().to_string(),
                            "--direction".to_string(),
                            direction.to_string(),
                        ],
                        true,
                        false,
                    )
                    .await
            }
            "launch" | "close-app" => {
                let app = args
                    .app
                    .as_deref()
                    .or(args.target.as_deref())
                    .ok_or_else(|| {
                        ToolError::LlmRecoverable(format!("DesktopAct '{action}' requires 'app'"))
                    })?;
                ensure_safe_cli_token("app", app, true)?;
                verify_allowed_app(Some(app), true)?;
                self.controller
                    .run_desktop_command(&[action, app.trim().to_string()], true, false)
                    .await
            }
            other => Err(ToolError::LlmRecoverable(format!(
                "DesktopAct: unsupported action '{other}'"
            ))),
        }
    }
}

pub fn desktop_tools(working_dir: Option<PathBuf>, runner: Arc<dyn CommandRunner>) -> Vec<Tool> {
    if !desktop_tools_enabled() {
        return Vec::new();
    }
    let controller = AgentDesktopController::new(working_dir, runner);
    vec![
        Tool {
            name: "DesktopObserve".to_string(),
            description: "Observe local desktop applications headlessly via agent-desktop OS accessibility trees (AX / UIA / AT-SPI) with @ref IDs, without stealing cursor or focus.".to_string(),
            is_read_only: true,
            parameters: json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["permissions", "list-apps", "list-windows", "list-surfaces", "snapshot", "find", "get", "is", "screenshot"],
                        "description": "Read-only desktop accessibility observation command."
                    },
                    "app": {
                        "type": "string",
                        "description": "Target desktop application name."
                    },
                    "target": {
                        "type": "string",
                        "description": "Element @ref or search query for 'find'."
                    },
                    "property": {
                        "type": "string",
                        "description": "Property/state name for 'get' or 'is'."
                    },
                    "max_depth": {
                        "type": "integer",
                        "description": "Optional accessibility skeleton traversal depth (1..=12)."
                    },
                    "path": {
                        "type": "string",
                        "description": "Relative output PNG path for 'screenshot'."
                    }
                },
                "required": ["action"]
            }),
            execute: Arc::new(PydanticAdapter::new(DesktopObserveExecutor {
                controller: controller.clone(),
            })),
        },
        Tool {
            name: "DesktopAct".to_string(),
            description: "Operate local desktop applications headlessly via agent-desktop semantic accessibility actions on @ref IDs (no physical mouse hijacking or clipboard scraping).".to_string(),
            is_read_only: false,
            parameters: json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["click", "right-click", "type", "set-value", "clear", "focus", "select", "toggle", "check", "uncheck", "expand", "collapse", "scroll", "scroll-to", "press", "wait", "launch", "close-app"],
                        "description": "Headless accessibility interaction command."
                    },
                    "app": {
                        "type": "string",
                        "description": "Target application name (required when app allowlist is configured)."
                    },
                    "target": {
                        "type": "string",
                        "description": "Target element @ref or key combo for 'press'."
                    },
                    "value": {
                        "type": "string",
                        "description": "Input value for 'type'/'set-value'/'select' or direction for 'scroll'."
                    }
                },
                "required": ["action"]
            }),
            execute: Arc::new(PydanticAdapter::new(DesktopActExecutor { controller })),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::mock::{MockCommandRunner, mock_output};

    #[tokio::test]
    async fn desktop_tools_remain_disabled_by_default_and_in_cloud_mode() {
        let runner = Arc::new(MockCommandRunner::new());
        temp_env::async_with_vars(
            [
                ("OMNISOLO_ENABLE_AGENT_DESKTOP", None::<&str>),
                ("OMNISOLO_AGENT_EXECUTION_MODE", None::<&str>),
                ("OMNISOLO_EXECUTION_MODE", Some("standalone")),
                ("OMNISOLO_SOURCE_MODE", None::<&str>),
                ("OMNISOLO_AGENT_COMMAND_BACKEND", None::<&str>),
                ("OMNISOLO_AGENT_SANDBOX_BACKEND", None::<&str>),
            ],
            async {
                assert!(desktop_tools(None, runner.clone()).is_empty());
            },
        )
        .await;

        temp_env::async_with_vars(
            [
                ("OMNISOLO_ENABLE_AGENT_DESKTOP", Some("true")),
                ("OMNISOLO_AGENT_EXECUTION_MODE", None::<&str>),
                ("OMNISOLO_EXECUTION_MODE", Some("cloud")),
                ("OMNISOLO_SOURCE_MODE", None::<&str>),
                ("OMNISOLO_AGENT_COMMAND_BACKEND", None::<&str>),
                ("OMNISOLO_AGENT_SANDBOX_BACKEND", None::<&str>),
            ],
            async {
                assert!(desktop_tools(None, runner.clone()).is_empty());
            },
        )
        .await;

        temp_env::async_with_vars(
            [
                ("OMNISOLO_ENABLE_AGENT_DESKTOP", Some("true")),
                ("OMNISOLO_AGENT_EXECUTION_MODE", None::<&str>),
                ("OMNISOLO_EXECUTION_MODE", Some("standalone")),
                ("OMNISOLO_SOURCE_MODE", None::<&str>),
                ("OMNISOLO_AGENT_COMMAND_BACKEND", Some("docker")),
                ("OMNISOLO_AGENT_SANDBOX_BACKEND", None::<&str>),
            ],
            async {
                assert!(desktop_tools(None, runner).is_empty());
            },
        )
        .await;
    }

    #[tokio::test]
    async fn desktop_enforces_headless_policy_allowlist_and_secret_redaction() {
        let dir = tempfile::tempdir().unwrap();
        let fake_bin = dir.path().join("fake-agent-desktop");
        std::fs::write(&fake_bin, b"#!/bin/sh\n").unwrap();

        temp_env::async_with_vars(
            [
                ("OMNISOLO_ENABLE_AGENT_DESKTOP", Some("true")),
                ("OMNISOLO_AGENT_EXECUTION_MODE", None::<&str>),
                ("OMNISOLO_EXECUTION_MODE", Some("standalone")),
                ("OMNISOLO_SOURCE_MODE", None::<&str>),
                ("OMNISOLO_AGENT_COMMAND_BACKEND", None::<&str>),
                ("OMNISOLO_AGENT_SANDBOX_BACKEND", None::<&str>),
                (
                    "OMNISOLO_AGENT_DESKTOP_BIN",
                    Some(fake_bin.to_str().unwrap()),
                ),
                (
                    "OMNISOLO_AGENT_DESKTOP_ALLOWED_APPS",
                    Some("Calculator,Notes"),
                ),
            ],
            async {
                let runner = Arc::new(MockCommandRunner::new());
                runner.push_response(Ok(mock_output(
                    0,
                    r#"{"ok":true,"token":"sk-1234567890abcdefghijklmnop","tree":"@e1 button"}"#,
                    "",
                )));
                let tools = desktop_tools(Some(dir.path().to_path_buf()), runner.clone());
                assert_eq!(tools.len(), 2);
                let observe = &tools[0];
                let act = &tools[1];

                // Unlisted app must be rejected
                let blocked_app = observe
                    .execute
                    .execute(json!({"action": "snapshot", "app": "PasswordManager"}))
                    .await;
                assert!(blocked_app.is_err());

                // Leading-dash target on flag-sensitive actions must be rejected
                let dash_target = act
                    .execute
                    .execute(json!({"action": "click", "app": "Calculator", "target": "--flag"}))
                    .await;
                assert!(matches!(dash_target, Err(ToolError::LlmRecoverable(_))));

                // Physical mouse/clipboard commands must be rejected with Fatal
                for forbidden in [
                    "mouse-click",
                    "clipboard-get",
                    "drag",
                    "open-system-surface",
                ] {
                    let res = act
                        .execute
                        .execute(json!({"action": forbidden, "app": "Calculator", "target": "@e1"}))
                        .await;
                    assert!(matches!(res, Err(ToolError::Fatal(_))), "{forbidden}");
                }

                // Allowed app snapshot succeeds and redacts leaked secret tokens
                let snap = observe
                    .execute
                    .execute(json!({"action": "snapshot", "app": "Calculator", "max_depth": 4}))
                    .await
                    .unwrap();
                assert!(snap.contains("[REDACTED_SECRET]"));
                assert!(!snap.contains("sk-1234567890"));

                // Positional find query and type value starting with '-' must follow '--'
                runner.push_response(Ok(mock_output(0, r#"{"ok":true}"#, "")));
                observe
                    .execute
                    .execute(json!({"action": "find", "app": "Calculator", "target": "-42"}))
                    .await
                    .unwrap();
                assert_eq!(
                    runner.last_command.lock().unwrap().as_ref().unwrap().1,
                    vec!["find", "--app", "Calculator", "--", "-42"]
                );

                runner.push_response(Ok(mock_output(0, r#"{"ok":true}"#, "")));
                act.execute
                    .execute(
                        json!({"action": "type", "app": "Calculator", "target": "@e1", "value": "-42"}),
                    )
                    .await
                    .unwrap();
                assert_eq!(
                    runner.last_command.lock().unwrap().as_ref().unwrap().1,
                    vec!["type", "--", "@e1", "-42"]
                );

                // Verify remaining DesktopObserve & DesktopAct commands + reconciliation after timeout
                for (tool_ref, payload, expected_prefix) in [
                    (observe, json!({"action": "permissions"}), vec!["permissions"]),
                    (observe, json!({"action": "list-apps"}), vec!["list-apps"]),
                    (observe, json!({"action": "list-windows", "app": "Notes"}), vec!["list-windows", "--app", "Notes"]),
                    (observe, json!({"action": "list-surfaces"}), vec!["list-surfaces"]),
                    (observe, json!({"action": "get", "app": "Notes", "target": "@e1", "property": "value"}), vec!["get", "@e1", "value"]),
                    (observe, json!({"action": "is", "app": "Notes", "target": "@e1", "property": "enabled"}), vec!["is", "@e1", "enabled"]),
                    (observe, json!({"action": "screenshot", "app": "Notes", "path": "notes.png"}), vec!["screenshot", "--app", "Notes", "--output", "notes.png"]),
                    (act, json!({"action": "right-click", "app": "Notes", "target": "@e1"}), vec!["right-click", "@e1"]),
                    (act, json!({"action": "toggle", "app": "Notes", "target": "@e2"}), vec!["toggle", "@e2"]),
                    (act, json!({"action": "scroll", "app": "Notes", "target": "@e3", "value": "down"}), vec!["scroll", "@e3", "--direction", "down"]),
                    (act, json!({"action": "press", "app": "Notes", "target": "Return"}), vec!["press", "Return"]),
                    (act, json!({"action": "wait", "app": "Notes", "target": "100"}), vec!["wait", "100"]),
                    (act, json!({"action": "launch", "app": "Notes"}), vec!["launch", "Notes"]),
                    (act, json!({"action": "close-app", "app": "Notes"}), vec!["close-app", "Notes"]),
                ] {
                    runner.push_response(Ok(mock_output(0, r#"{"ok":true}"#, "")));
                    tool_ref.execute.execute(payload).await.unwrap();
                    assert_eq!(
                        runner.last_command.lock().unwrap().as_ref().unwrap().1,
                        expected_prefix
                    );
                }

                // Screenshot requires app and rejects traversal; scroll rejects invalid direction; --headed is Fatal
                assert!(
                    observe
                        .execute
                        .execute(json!({"action": "screenshot"}))
                        .await
                        .is_err()
                );
                assert!(
                    observe
                        .execute
                        .execute(json!({"action": "screenshot", "app": "Notes", "path": "../x.png"}))
                        .await
                        .is_err()
                );
                assert!(
                    act.execute
                        .execute(json!({"action": "scroll", "app": "Notes", "target": "@e1", "value": "sideways"}))
                        .await
                        .is_err()
                );
                let headed_err = act
                    .execute
                    .execute(json!({"action": "click", "app": "Notes", "target": "--headed"}))
                    .await
                    .unwrap_err();
                assert!(matches!(headed_err, ToolError::Fatal(_)));

                // Mutating failure with timeout forces snapshot reconciliation before next mutation
                runner.push_response(Ok(mock_output(1, "", "timed out waiting for accessibility")));
                assert!(
                    act.execute
                        .execute(json!({"action": "click", "app": "Notes", "target": "@e1"}))
                        .await
                        .is_err()
                );
                let blocked_retry = act
                    .execute
                    .execute(json!({"action": "click", "app": "Notes", "target": "@e1"}))
                    .await
                    .unwrap_err();
                assert!(blocked_retry.to_string().contains("reconcile"));

                runner.push_response(Ok(mock_output(0, r#"{"ok":true}"#, "")));
                observe
                    .execute
                    .execute(json!({"action": "snapshot", "app": "Notes"}))
                    .await
                    .unwrap();
                runner.push_response(Ok(mock_output(0, r#"{"ok":true}"#, "")));
                act.execute
                    .execute(json!({"action": "click", "app": "Notes", "target": "@e1"}))
                    .await
                    .unwrap();
            },
        )
        .await;
    }
}
