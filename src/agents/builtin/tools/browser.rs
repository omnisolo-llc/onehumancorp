use crate::{
    Tool,
    binary_dist::{ManagedBinaryKind, ensure_managed_binary},
    browser_proxy::SharedBrowserProxy,
    network_policy,
    pydantic::{PydanticAdapter, PydanticToolExecutor},
    runner::CommandRunner,
};
use dashmap::DashMap;
use omnisolo_builtin_agent_core::types::ToolError;
use serde::Deserialize;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;

const ANTI_BOT_MARKERS: &[&str] = &[
    "challenges.cloudflare.com",
    "cf-turnstile",
    "g-recaptcha",
    "hcaptcha.com",
    "h-captcha",
    "verify you are human",
    "arkoselabs",
];

#[derive(Clone, Debug)]
pub struct BrowserSessionMetrics {
    pub started_at_ms: i64,
    pub commands_executed: u64,
    pub reconciliation_required: bool,
    pub profile_dir: PathBuf,
}

/// Shared state & executor around `agent-browser` (`vercel-labs/agent-browser` v0.43.0).
#[derive(Clone)]
pub struct AgentBrowserController {
    working_dir: Option<PathBuf>,
    runner: Arc<dyn CommandRunner>,
    proxy: SharedBrowserProxy,
    sessions: Arc<DashMap<String, BrowserSessionMetrics>>,
}

impl AgentBrowserController {
    pub fn new(working_dir: Option<PathBuf>, runner: Arc<dyn CommandRunner>) -> Self {
        Self {
            working_dir,
            runner,
            proxy: SharedBrowserProxy::new(),
            sessions: Arc::new(DashMap::new()),
        }
    }

    pub fn validate_relative_output_path(path: &str) -> Result<(), ToolError> {
        let trimmed = path.trim();
        if trimmed.is_empty()
            || trimmed.contains("..")
            || trimmed.contains('\0')
            || trimmed.starts_with('/')
            || trimmed.starts_with('\\')
            || trimmed.starts_with('-')
            || trimmed.contains(":\\")
            || trimmed.contains(":/")
            || trimmed.as_bytes().get(1) == Some(&b':')
        {
            return Err(ToolError::LlmRecoverable(
                "browser: output path must be a non-empty relative path without '..'".to_string(),
            ));
        }
        Ok(())
    }

    fn sanitize_session_id(raw: &str) -> Result<String, ToolError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() || trimmed.len() > 96 {
            return Err(ToolError::LlmRecoverable(
                "browser: session id must be 1..=96 ASCII characters".to_string(),
            ));
        }
        if !trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(ToolError::LlmRecoverable(
                "browser: session id may only contain [A-Za-z0-9_-]".to_string(),
            ));
        }
        Ok(trimmed.to_string())
    }

    fn base_profile_root(&self) -> PathBuf {
        if let Some(wd) = &self.working_dir {
            wd.join(".omnisolo-browser-sessions")
        } else {
            std::env::temp_dir().join("omnisolo-browser-sessions")
        }
    }

    fn ensure_session_entry(&self, session_id: &str) -> BrowserSessionMetrics {
        if let Some(existing) = self.sessions.get(session_id) {
            return existing.clone();
        }
        let profile_dir = self.base_profile_root().join(session_id);
        let metrics = BrowserSessionMetrics {
            started_at_ms: chrono::Utc::now().timestamp_millis(),
            commands_executed: 0,
            reconciliation_required: false,
            profile_dir,
        };
        self.sessions
            .insert(session_id.to_string(), metrics.clone());
        metrics
    }

    pub fn session_metrics(&self, session_id: &str) -> Option<BrowserSessionMetrics> {
        self.sessions.get(session_id).map(|entry| entry.clone())
    }

    fn detect_anti_bot_gate(text: &str) -> Option<&'static str> {
        let lower = text.to_ascii_lowercase();
        ANTI_BOT_MARKERS
            .iter()
            .copied()
            .find(|marker| lower.contains(marker))
    }

    async fn resolve_binary(&self) -> Result<String, ToolError> {
        let path = ensure_managed_binary(ManagedBinaryKind::AgentBrowser).await?;
        Ok(path.to_string_lossy().into_owned())
    }

    pub async fn run_browser_command(
        &self,
        raw_session_id: &str,
        command_args: &[String],
        is_mutating: bool,
        clear_reconciliation_on_success: bool,
    ) -> Result<String, ToolError> {
        let session_id = Self::sanitize_session_id(raw_session_id)?;
        let entry = self.ensure_session_entry(&session_id);

        if is_mutating && entry.reconciliation_required {
            return Err(ToolError::LlmRecoverable(
                "browser: previous mutating action had an unknown outcome; run a read-only 'snapshot' or 'get' observation to reconcile page state before retrying a mutating command".to_string(),
            ));
        }

        tokio::fs::create_dir_all(&entry.profile_dir)
            .await
            .map_err(|e| {
                ToolError::Unexpected(format!(
                    "browser: failed to create isolated session profile: {e}"
                ))
            })?;

        let allow_private = network_policy::private_network_allowed();
        let proxy_url = self.proxy.proxy_url(allow_private).await?;
        let binary = self.resolve_binary().await?;

        let profile_str = entry.profile_dir.to_string_lossy().into_owned();
        let mut full_args = vec![
            "--json".to_string(),
            "--session".to_string(),
            session_id.clone(),
            "--profile".to_string(),
            profile_str,
            "--proxy".to_string(),
            proxy_url,
            "--content-boundaries".to_string(),
            "--no-auto-dialog".to_string(),
            "--idle-timeout".to_string(),
            "5m".to_string(),
        ];

        if let Ok(chrome_bin) = std::env::var("OMNISOLO_CHROMIUM_EXECUTABLE")
            && !chrome_bin.trim().is_empty()
        {
            full_args.push("--executable-path".to_string());
            full_args.push(chrome_bin.trim().to_string());
        }
        if let Ok(domains) = std::env::var("OMNISOLO_BROWSER_ALLOWED_DOMAINS")
            && !domains.trim().is_empty()
        {
            full_args.push("--allowed-domains".to_string());
            full_args.push(domains.trim().to_string());
        }

        full_args.extend(command_args.iter().cloned());
        let arg_refs: Vec<&str> = full_args.iter().map(String::as_str).collect();

        let run_result = tokio::time::timeout(
            std::time::Duration::from_secs(45),
            self.runner
                .run(&binary, &arg_refs, self.working_dir.as_deref(), Vec::new()),
        )
        .await;

        let output = match run_result {
            Ok(Ok(out)) => out,
            Ok(Err(err)) => {
                if is_mutating && let Some(mut session) = self.sessions.get_mut(&session_id) {
                    session.reconciliation_required = true;
                }
                return Err(ToolError::LlmRecoverable(format!(
                    "browser: failed to execute agent-browser: {err}"
                )));
            }
            Err(_) => {
                if is_mutating && let Some(mut session) = self.sessions.get_mut(&session_id) {
                    session.reconciliation_required = true;
                }
                return Err(ToolError::LlmRecoverable(
                    "browser: command timed out after 45s; reconcile state with a read-only snapshot before retrying".to_string(),
                ));
            }
        };

        if let Some(mut session) = self.sessions.get_mut(&session_id) {
            session.commands_executed = session.commands_executed.saturating_add(1);
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

        if !output.status.success() {
            let combined = if stdout.is_empty() {
                stderr.clone()
            } else if stderr.is_empty() {
                stdout.clone()
            } else {
                format!("{stdout}\n{stderr}")
            };
            let lower = combined.to_ascii_lowercase();
            if is_mutating && (lower.contains("timed out") || lower.contains("timeout")) {
                if let Some(mut session) = self.sessions.get_mut(&session_id) {
                    session.reconciliation_required = true;
                }
            }
            if let Some(marker) = Self::detect_anti_bot_gate(&combined) {
                return Err(ToolError::UserFixable(format!(
                    "browser blocked by anti-bot/CAPTCHA gate ({marker}). Automated solving or bypass is forbidden; surface this URL to the owner for manual completion."
                )));
            }
            return Err(ToolError::LlmRecoverable(format!(
                "browser command failed (exit {}): {}",
                output.status.code().unwrap_or(-1),
                combined
            )));
        }

        if let Some(marker) =
            Self::detect_anti_bot_gate(&stdout).or_else(|| Self::detect_anti_bot_gate(&stderr))
        {
            return Err(ToolError::UserFixable(format!(
                "browser blocked by anti-bot/CAPTCHA gate ({marker}). Automated solving or bypass is forbidden; surface this URL to the owner for manual completion."
            )));
        }

        if clear_reconciliation_on_success
            && let Some(mut session) = self.sessions.get_mut(&session_id)
        {
            session.reconciliation_required = false;
        }

        Ok(stdout)
    }

    pub async fn validate_navigation_target(url: &str) -> Result<Url, ToolError> {
        let parsed = Url::parse(url.trim())
            .map_err(|e| ToolError::LlmRecoverable(format!("browser: invalid URL '{url}': {e}")))?;
        let allow_private = network_policy::private_network_allowed();
        network_policy::validate_and_resolve(&parsed, allow_private).await?;
        Ok(parsed)
    }

    pub async fn close_session(&self, raw_session_id: &str) {
        let Ok(session_id) = Self::sanitize_session_id(raw_session_id) else {
            return;
        };
        if let Ok(binary) = self.resolve_binary().await {
            let args = ["--json", "--session", session_id.as_str(), "close"];
            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                self.runner
                    .run(&binary, &args, self.working_dir.as_deref(), Vec::new()),
            )
            .await;
        }
        if let Some((_, removed)) = self.sessions.remove(&session_id) {
            let _ = tokio::fs::remove_dir_all(removed.profile_dir).await;
        }
    }

    pub fn working_dir(&self) -> Option<&Path> {
        self.working_dir.as_deref()
    }
}

fn default_session_id() -> String {
    "default".to_string()
}

#[derive(Deserialize)]
struct BrowserObserveArgs {
    /// One of: "open", "snapshot", "get", "screenshot"
    action: String,
    #[serde(default = "default_session_id")]
    session: String,
    #[serde(default)]
    url: Option<String>,
    /// For "get": "url" | "title" | "text"
    #[serde(default)]
    field: Option<String>,
    /// Relative file path for "screenshot"
    #[serde(default)]
    path: Option<String>,
}

struct BrowserObserveExecutor {
    controller: AgentBrowserController,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BrowserObserveArgs> for BrowserObserveExecutor {
    async fn execute_typed(&self, args: BrowserObserveArgs) -> Result<String, ToolError> {
        match args.action.trim().to_ascii_lowercase().as_str() {
            "open" => {
                let raw_url = args.url.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable("BrowserObserve 'open' requires 'url'".to_string())
                })?;
                let validated = AgentBrowserController::validate_navigation_target(raw_url).await?;
                self.controller
                    .run_browser_command(
                        &args.session,
                        &["open".to_string(), validated.to_string()],
                        false,
                        false,
                    )
                    .await
            }
            "snapshot" => {
                self.controller
                    .run_browser_command(
                        &args.session,
                        &["snapshot".to_string(), "-i".to_string()],
                        false,
                        true,
                    )
                    .await
            }
            "get" => {
                let field = args.field.as_deref().unwrap_or("url").trim();
                if !matches!(field, "url" | "title" | "text") {
                    return Err(ToolError::LlmRecoverable(
                        "BrowserObserve 'get' field must be 'url', 'title', or 'text'".to_string(),
                    ));
                }
                self.controller
                    .run_browser_command(
                        &args.session,
                        &["get".to_string(), field.to_string()],
                        false,
                        true,
                    )
                    .await
            }
            "screenshot" => {
                let path = args.path.as_deref().unwrap_or("screenshot.png").trim();
                AgentBrowserController::validate_relative_output_path(path)?;
                if let Some(raw_url) = args.url.as_deref().filter(|u| !u.trim().is_empty()) {
                    let validated =
                        AgentBrowserController::validate_navigation_target(raw_url).await?;
                    self.controller
                        .run_browser_command(
                            &args.session,
                            &["open".to_string(), validated.to_string()],
                            false,
                            false,
                        )
                        .await?;
                }
                self.controller
                    .run_browser_command(
                        &args.session,
                        &["screenshot".to_string(), path.to_string()],
                        false,
                        false,
                    )
                    .await
            }
            other => Err(ToolError::LlmRecoverable(format!(
                "BrowserObserve: unsupported action '{other}' (expected open, snapshot, get, or screenshot)"
            ))),
        }
    }
}

#[derive(Deserialize)]
struct BrowserActArgs {
    /// One of: "click", "fill", "select", "press", "hover", "scroll", "wait", "back", "forward", "reload", "close"
    action: String,
    #[serde(default = "default_session_id")]
    session: String,
    /// Element reference (e.g. "@e1") or key/wait target
    #[serde(default)]
    target: Option<String>,
    /// Value for "fill", "select", or scroll amount
    #[serde(default)]
    value: Option<String>,
}

struct BrowserActExecutor {
    controller: AgentBrowserController,
}

#[async_trait::async_trait]
impl PydanticToolExecutor<BrowserActArgs> for BrowserActExecutor {
    async fn execute_typed(&self, args: BrowserActArgs) -> Result<String, ToolError> {
        let action = args.action.trim().to_ascii_lowercase();
        match action.as_str() {
            "close" => {
                self.controller.close_session(&args.session).await;
                Ok(json!({"success": true, "session": args.session, "closed": true}).to_string())
            }
            "back" | "forward" | "reload" => {
                self.controller
                    .run_browser_command(&args.session, &[action], true, false)
                    .await
            }
            "click" | "hover" | "press" | "wait" => {
                let target = args
                    .target
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| {
                        ToolError::LlmRecoverable(format!(
                            "BrowserAct '{action}' requires a non-empty 'target' (e.g., '@e1')"
                        ))
                    })?;
                if target.starts_with('-') {
                    return Err(ToolError::LlmRecoverable(format!(
                        "BrowserAct '{action}' target must not start with '-'"
                    )));
                }
                let is_mutating = action != "wait";
                self.controller
                    .run_browser_command(
                        &args.session,
                        &[action, target.to_string()],
                        is_mutating,
                        false,
                    )
                    .await
            }
            "fill" | "select" => {
                let target = args
                    .target
                    .as_deref()
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .ok_or_else(|| {
                        ToolError::LlmRecoverable(format!(
                            "BrowserAct '{action}' requires a non-empty 'target' (e.g., '@e2')"
                        ))
                    })?;
                if target.starts_with('-') {
                    return Err(ToolError::LlmRecoverable(format!(
                        "BrowserAct '{action}' target must not start with '-'"
                    )));
                }
                let value = args.value.as_deref().ok_or_else(|| {
                    ToolError::LlmRecoverable(format!("BrowserAct '{action}' requires 'value'"))
                })?;
                self.controller
                    .run_browser_command(
                        &args.session,
                        &[action, target.to_string(), value.to_string()],
                        true,
                        false,
                    )
                    .await
            }
            "scroll" => {
                let direction = args.target.as_deref().unwrap_or("down").trim();
                if !matches!(direction, "up" | "down" | "left" | "right") {
                    return Err(ToolError::LlmRecoverable(
                        "BrowserAct 'scroll' target must be 'up', 'down', 'left', or 'right'"
                            .to_string(),
                    ));
                }
                let mut cmd = vec!["scroll".to_string(), direction.to_string()];
                if let Some(amount) = args.value.as_deref().filter(|v| !v.trim().is_empty()) {
                    let trimmed = amount.trim();
                    if trimmed.starts_with('-') {
                        return Err(ToolError::LlmRecoverable(
                            "BrowserAct 'scroll' value must not start with '-'".to_string(),
                        ));
                    }
                    cmd.push(trimmed.to_string());
                }
                self.controller
                    .run_browser_command(&args.session, &cmd, true, false)
                    .await
            }
            other => Err(ToolError::LlmRecoverable(format!(
                "BrowserAct: unsupported action '{other}' (expected click, fill, select, press, hover, scroll, wait, back, forward, reload, or close)"
            ))),
        }
    }
}

pub fn browser_tools(
    working_dir: Option<PathBuf>,
    runner: Arc<dyn CommandRunner>,
) -> (Tool, Tool, AgentBrowserController) {
    let controller = AgentBrowserController::new(working_dir, runner);
    let observe = Tool {
        name: "BrowserObserve".to_string(),
        description: "Observe web pages via agent-browser (Headless Chromium CDP + compact @ref accessibility trees). \
            Supports 'open' (navigate to http/https URL), 'snapshot' (interactive accessibility tree with @e1 refs), \
            'get' (url/title/text), and 'screenshot' (save PNG to relative workspace path).".to_string(),
        is_read_only: true,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["open", "snapshot", "get", "screenshot"],
                    "description": "Read-only browser observation command."
                },
                "session": {
                    "type": "string",
                    "description": "Isolated browser session identifier (defaults to 'default')."
                },
                "url": {
                    "type": "string",
                    "description": "Target http/https URL for 'open' or optional navigation before 'screenshot'."
                },
                "field": {
                    "type": "string",
                    "enum": ["url", "title", "text"],
                    "description": "Property to inspect when action is 'get'."
                },
                "path": {
                    "type": "string",
                    "description": "Relative output PNG path when action is 'screenshot'."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(PydanticAdapter::new(BrowserObserveExecutor {
            controller: controller.clone(),
        })),
    };

    let act = Tool {
        name: "BrowserAct".to_string(),
        description: "Interact with web page elements via agent-browser @ref IDs obtained from BrowserObserve 'snapshot' \
            (e.g., click '@e1', fill '@e2' with text, select, press, scroll, wait, or close session). \
            State-changing actions require authorization and reconciliation before retry after timeouts.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["click", "fill", "select", "press", "hover", "scroll", "wait", "back", "forward", "reload", "close"],
                    "description": "Browser interaction command."
                },
                "session": {
                    "type": "string",
                    "description": "Browser session identifier matching BrowserObserve."
                },
                "target": {
                    "type": "string",
                    "description": "Element @ref (e.g. '@e1'), key name for 'press', direction for 'scroll', or selector/ms for 'wait'."
                },
                "value": {
                    "type": "string",
                    "description": "Input value for 'fill'/'select' or pixel amount for 'scroll'."
                }
            },
            "required": ["action"]
        }),
        execute: Arc::new(PydanticAdapter::new(BrowserActExecutor {
            controller: controller.clone(),
        })),
    };

    (observe, act, controller)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::mock::{MockCommandRunner, mock_output};

    #[tokio::test]
    async fn browser_observe_rejects_private_urls_and_path_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let fake_bin = dir.path().join("fake-agent-browser");
        std::fs::write(&fake_bin, b"#!/bin/sh\n").unwrap();

        temp_env::async_with_vars(
            [
                (
                    "OMNISOLO_AGENT_BROWSER_BIN",
                    Some(fake_bin.to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("false")),
            ],
            async {
                let runner = Arc::new(MockCommandRunner::new());
                let (observe, _, _) = browser_tools(Some(dir.path().to_path_buf()), runner);

                let private_res = observe
                    .execute
                    .execute(json!({"action": "open", "url": "http://127.0.0.1:8080/admin"}))
                    .await;
                assert!(private_res.is_err());

                let file_res = observe
                    .execute
                    .execute(json!({"action": "open", "url": "file:///etc/passwd"}))
                    .await;
                assert!(file_res.is_err());

                let traversal_res = observe
                    .execute
                    .execute(json!({"action": "screenshot", "path": "../escape.png"}))
                    .await;
                assert!(traversal_res.is_err());
            },
        )
        .await;
    }

    #[tokio::test]
    async fn browser_fails_closed_on_captcha_and_enforces_reconciliation_after_mutating_failure() {
        let dir = tempfile::tempdir().unwrap();
        let fake_bin = dir.path().join("fake-agent-browser");
        std::fs::write(&fake_bin, b"#!/bin/sh\n").unwrap();

        temp_env::async_with_vars(
            [(
                "OMNISOLO_AGENT_BROWSER_BIN",
                Some(fake_bin.to_str().unwrap()),
            )],
            async {
                let runner = Arc::new(MockCommandRunner::new());
                // 1. Snapshot containing a Cloudflare Turnstile gate must fail closed with UserFixable
                runner.push_response(Ok(mock_output(
                    0,
                    r#"{"success":true,"data":{"tree":"iframe challenges.cloudflare.com cf-turnstile"}}"#,
                    "",
                )));
                // 2. Mutating click that times out sets reconciliation_required = true
                runner.push_response(Err(std::io::Error::new(
                    std::io::ErrorKind::TimedOut,
                    "ipc timeout",
                )));
                // 3. Reconciling snapshot clears reconciliation_required
                runner.push_response(Ok(mock_output(
                    0,
                    r#"{"success":true,"data":{"tree":"@e1 button Submit"}}"#,
                    "",
                )));
                // 4. Subsequent click succeeds after reconciliation
                runner.push_response(Ok(mock_output(0, r#"{"success":true}"#, "")));

                let (observe, act, _) = browser_tools(Some(dir.path().to_path_buf()), runner);

                let captcha_err = observe
                    .execute
                    .execute(json!({"action": "snapshot", "session": "s1"}))
                    .await
                    .unwrap_err();
                assert!(matches!(captcha_err, ToolError::UserFixable(_)));

                let click_timeout = act
                    .execute
                    .execute(json!({"action": "click", "session": "s1", "target": "@e1"}))
                    .await;
                assert!(click_timeout.is_err());

                // Blind retry without snapshot must be blocked before invoking the runner!
                let blind_retry = act
                    .execute
                    .execute(json!({"action": "click", "session": "s1", "target": "@e1"}))
                    .await
                    .unwrap_err();
                assert!(blind_retry.to_string().contains("reconcile"));

                // Reconcile via snapshot, then retry succeeds
                observe
                    .execute
                    .execute(json!({"action": "snapshot", "session": "s1"}))
                    .await
                    .unwrap();
                let after_reconcile = act
                    .execute
                    .execute(json!({"action": "click", "session": "s1", "target": "@e1"}))
                    .await;
                assert!(after_reconcile.is_ok());
            },
        )
        .await;
    }

    #[tokio::test]
    async fn browser_tools_cover_all_actions_validation_env_flags_and_close_cleanup() {
        let dir = tempfile::tempdir().unwrap();
        let fake_bin = dir.path().join("fake-agent-browser");
        std::fs::write(&fake_bin, b"#!/bin/sh\n").unwrap();

        temp_env::async_with_vars(
            [
                (
                    "OMNISOLO_AGENT_BROWSER_BIN",
                    Some(fake_bin.to_str().unwrap()),
                ),
                ("OMNISOLO_AGENT_ALLOW_PRIVATE_NETWORK", Some("false")),
                ("OMNISOLO_CHROMIUM_EXECUTABLE", Some("/usr/bin/chromium")),
                (
                    "OMNISOLO_BROWSER_ALLOWED_DOMAINS",
                    Some("example.com,*.example.org"),
                ),
            ],
            async {
                let runner = Arc::new(MockCommandRunner::new());
                let (observe, act, controller) =
                    browser_tools(Some(dir.path().to_path_buf()), runner.clone());

                // Validation rejections
                for bad_session in ["", "../escape", &"a".repeat(97)] {
                    assert!(
                        observe
                            .execute
                            .execute(json!({"action": "snapshot", "session": bad_session}))
                            .await
                            .is_err()
                    );
                }
                assert!(
                    observe
                        .execute
                        .execute(json!({"action": "get", "field": "cookies"}))
                        .await
                        .is_err()
                );
                assert!(
                    observe
                        .execute
                        .execute(json!({"action": "unknown"}))
                        .await
                        .is_err()
                );
                for bad_act in [
                    json!({"action": "click", "target": "--headed"}),
                    json!({"action": "fill", "target": "-bad", "value": "x"}),
                    json!({"action": "scroll", "target": "diagonal"}),
                    json!({"action": "scroll", "target": "down", "value": "--eval"}),
                    json!({"action": "unknown"}),
                ] {
                    assert!(act.execute.execute(bad_act).await.is_err());
                }

                // Happy paths for get, screenshot, fill, select, press, hover, scroll, wait, back, forward, reload
                for req in [
                    (
                        &observe,
                        json!({"action": "get", "session": "s2", "field": "title"}),
                    ),
                    (
                        &observe,
                        json!({"action": "screenshot", "session": "s2", "path": "shot.png"}),
                    ),
                    (
                        &act,
                        json!({"action": "fill", "session": "s2", "target": "@e1", "value": "hi"}),
                    ),
                    (
                        &act,
                        json!({"action": "select", "session": "s2", "target": "@e2", "value": "opt"}),
                    ),
                    (
                        &act,
                        json!({"action": "press", "session": "s2", "target": "Enter"}),
                    ),
                    (
                        &act,
                        json!({"action": "hover", "session": "s2", "target": "@e3"}),
                    ),
                    (
                        &act,
                        json!({"action": "scroll", "session": "s2", "target": "down", "value": "300"}),
                    ),
                    (
                        &act,
                        json!({"action": "wait", "session": "s2", "target": "100"}),
                    ),
                    (&act, json!({"action": "back", "session": "s2"})),
                    (&act, json!({"action": "forward", "session": "s2"})),
                    (&act, json!({"action": "reload", "session": "s2"})),
                ] {
                    runner.push_response(Ok(mock_output(0, r#"{"success":true}"#, "")));
                    assert!(req.0.execute.execute(req.1).await.is_ok());
                }

                let metrics = controller.session_metrics("s2").unwrap();
                assert_eq!(metrics.commands_executed, 11);
                assert!(metrics.profile_dir.exists());

                let recorded = runner.last_command.lock().unwrap().clone().unwrap();
                assert!(recorded.1.windows(2).any(|w| w == ["--executable-path", "/usr/bin/chromium"]));
                assert!(recorded.1.windows(2).any(|w| w == ["--allowed-domains", "example.com,*.example.org"]));

                // Non-zero exit with anti-bot marker fails closed with UserFixable
                runner.push_response(Ok(mock_output(1, "", "blocked by hcaptcha.com")));
                let anti_bot = observe
                    .execute
                    .execute(json!({"action": "snapshot", "session": "s2"}))
                    .await
                    .unwrap_err();
                assert!(matches!(anti_bot, ToolError::UserFixable(_)));

                // Mutating command failing with timeout string requires reconciliation before retry
                runner.push_response(Ok(mock_output(1, "", "operation timed out")));
                assert!(
                    act.execute
                        .execute(json!({"action": "click", "session": "s2", "target": "@e1"}))
                        .await
                        .is_err()
                );
                assert!(controller.session_metrics("s2").unwrap().reconciliation_required);

                // Close removes profile directory and session metrics
                runner.push_response(Ok(mock_output(0, r#"{"success":true}"#, "")));
                let closed = act
                    .execute
                    .execute(json!({"action": "close", "session": "s2"}))
                    .await
                    .unwrap();
                assert!(closed.contains("\"closed\":true"));
                assert!(controller.session_metrics("s2").is_none());
                assert!(!metrics.profile_dir.exists());
            },
        )
        .await;
    }
}
