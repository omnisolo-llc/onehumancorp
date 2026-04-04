use anyhow::{anyhow, Context};
use serde_json::Value;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::hub::{HubMessage, HubServiceClient, PublishMessageRequest};
use crate::llm::ToolDefinition;

// ─── ToolExecutor ─────────────────────────────────────────────────────────────

pub struct ToolExecutor {
    pub todos: Arc<Mutex<Vec<Value>>>,
    pub grpc_endpoint: String,
    #[allow(dead_code)]
    pub agent_id: String,
    pub from_agent: String,
}

impl ToolExecutor {
    pub fn new(grpc_endpoint: String, agent_id: String, agent_name: String) -> Self {
        Self {
            todos: Arc::new(Mutex::new(vec![])),
            grpc_endpoint,
            agent_id,
            from_agent: agent_name,
        }
    }
}

// ─── Tool dispatch ────────────────────────────────────────────────────────────

pub async fn execute_tool(
    name: &str,
    input: Value,
    executor: &mut ToolExecutor,
) -> anyhow::Result<String> {
    match name {
        "bash" => tool_bash(&input).await,
        "file_read" => tool_file_read(&input),
        "file_write" => tool_file_write(&input),
        "file_edit" => tool_file_edit(&input),
        "grep" => tool_grep(&input).await,
        "glob" => tool_glob(&input),
        "ls" => tool_ls(&input),
        "web_fetch" => tool_web_fetch(&input).await,
        "web_search" => tool_web_search(&input).await,
        "todo_write" => tool_todo_write(&input, &executor.todos),
        "tool_search" => tool_tool_search(&input),
        "send_message" => tool_send_message(&input, executor).await,
        "notebook_edit" => tool_notebook_edit(&input),
        _ => Err(anyhow!("unknown tool: {}", name)),
    }
}

// ─── Tool definitions ─────────────────────────────────────────────────────────

pub fn all_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "bash".into(),
            description: "Run a shell command with a timeout. Returns stdout+stderr.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {"type": "string", "description": "Shell command to run"},
                    "timeout_ms": {"type": "integer", "description": "Timeout in milliseconds (default 60000, max 600000)"}
                },
                "required": ["command"]
            }),
        },
        ToolDefinition {
            name: "file_read".into(),
            description: "Read a file, optionally with offset/limit lines.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {"type": "string"},
                    "offset": {"type": "integer", "description": "Line offset (0-based)"},
                    "limit": {"type": "integer", "description": "Max lines to return (default 2000)"}
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "file_write".into(),
            description: "Write content to a file, creating directories as needed.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {"type": "string"},
                    "content": {"type": "string"}
                },
                "required": ["path", "content"]
            }),
        },
        ToolDefinition {
            name: "file_edit".into(),
            description: "Replace first occurrence of old_str with new_str in a file.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {"type": "string"},
                    "old_str": {"type": "string"},
                    "new_str": {"type": "string"}
                },
                "required": ["path", "old_str", "new_str"]
            }),
        },
        ToolDefinition {
            name: "grep".into(),
            description: "Search files using ripgrep. Supports pattern, glob, flags.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "pattern": {"type": "string"},
                    "glob": {"type": "string"},
                    "case_insensitive": {"type": "boolean"},
                    "line_numbers": {"type": "boolean"},
                    "context_before": {"type": "integer"},
                    "context_after": {"type": "integer"},
                    "context": {"type": "integer"},
                    "head_limit": {"type": "integer"},
                    "output_mode": {"type": "string", "enum": ["files_with_matches", "content", "count"]}
                },
                "required": ["pattern"]
            }),
        },
        ToolDefinition {
            name: "glob".into(),
            description: "Find files by glob pattern (max 200 results).".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "pattern": {"type": "string"},
                    "path": {"type": "string"}
                },
                "required": ["pattern"]
            }),
        },
        ToolDefinition {
            name: "ls".into(),
            description: "List directory contents with file sizes and types.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {"type": "string"}
                }
            }),
        },
        ToolDefinition {
            name: "web_fetch".into(),
            description: "HTTP GET a URL and return the text content (max 500KB).".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {"type": "string"}
                },
                "required": ["url"]
            }),
        },
        ToolDefinition {
            name: "web_search".into(),
            description: "Search the web using DuckDuckGo or SerpAPI.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string"}
                },
                "required": ["query"]
            }),
        },
        ToolDefinition {
            name: "todo_write".into(),
            description: "Replace the in-memory todo list with the given todos array.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "todos": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "id": {"type": "string"},
                                "content": {"type": "string"},
                                "status": {"type": "string"},
                                "priority": {"type": "string"}
                            }
                        }
                    }
                },
                "required": ["todos"]
            }),
        },
        ToolDefinition {
            name: "tool_search".into(),
            description: "List available tools, optionally filtered by a query string.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {"type": "string"}
                }
            }),
        },
        ToolDefinition {
            name: "send_message".into(),
            description: "Publish a message to another agent via the Hub.".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "to_agent": {"type": "string"},
                    "type": {"type": "string"},
                    "content": {"type": "string"}
                },
                "required": ["to_agent", "type", "content"]
            }),
        },
        ToolDefinition {
            name: "notebook_edit".into(),
            description: "Edit Jupyter notebook JSON cells (replace/insert/delete).".into(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {"type": "string"},
                    "operations": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "op": {"type": "string", "enum": ["replace", "insert", "delete"]},
                                "index": {"type": "integer"},
                                "source": {"type": "string"},
                                "cell_type": {"type": "string"}
                            },
                            "required": ["op", "index"]
                        }
                    }
                },
                "required": ["path", "operations"]
            }),
        },
    ]
}

// ─── bash ─────────────────────────────────────────────────────────────────────

async fn tool_bash(input: &Value) -> anyhow::Result<String> {
    let command = input["command"]
        .as_str()
        .ok_or_else(|| anyhow!("bash: missing command"))?;

    let default_timeout_ms = env::var("OHC_BASH_TIMEOUT_MS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60_000);

    let timeout_ms = input["timeout_ms"]
        .as_u64()
        .unwrap_or(default_timeout_ms)
        .min(600_000);

    let output = tokio::time::timeout(
        Duration::from_millis(timeout_ms),
        tokio::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output(),
    )
    .await
    .context("bash timeout")?
    .context("bash exec")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{}{}", stdout, stderr);

    if !output.status.success() {
        let code = output.status.code().unwrap_or(-1);
        Ok(format!("[exit {}]\n{}", code, combined))
    } else {
        Ok(combined)
    }
}

// ─── file_read ────────────────────────────────────────────────────────────────

fn tool_file_read(input: &Value) -> anyhow::Result<String> {
    let path = input["path"]
        .as_str()
        .ok_or_else(|| anyhow!("file_read: missing path"))?;
    let offset = input["offset"].as_u64().unwrap_or(0) as usize;
    let limit = input["limit"].as_u64().unwrap_or(2000) as usize;

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("file_read: {}", path))?;

    let lines: Vec<&str> = content.lines().collect();
    let start = offset.min(lines.len());
    let end = (start + limit).min(lines.len());

    let result: String = lines[start..end]
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{}: {}\n", start + i + 1, line))
        .collect();

    Ok(result)
}

// ─── file_write ───────────────────────────────────────────────────────────────

fn tool_file_write(input: &Value) -> anyhow::Result<String> {
    let path = input["path"]
        .as_str()
        .ok_or_else(|| anyhow!("file_write: missing path"))?;
    let content = input["content"]
        .as_str()
        .ok_or_else(|| anyhow!("file_write: missing content"))?;

    let p = Path::new(path);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("file_write: create dirs for {}", path))?;
    }
    std::fs::write(path, content).with_context(|| format!("file_write: {}", path))?;
    Ok(format!("wrote {} bytes to {}", content.len(), path))
}

// ─── file_edit ────────────────────────────────────────────────────────────────

fn tool_file_edit(input: &Value) -> anyhow::Result<String> {
    let path = input["path"]
        .as_str()
        .ok_or_else(|| anyhow!("file_edit: missing path"))?;
    let old_str = input["old_str"]
        .as_str()
        .ok_or_else(|| anyhow!("file_edit: missing old_str"))?;
    let new_str = input["new_str"]
        .as_str()
        .ok_or_else(|| anyhow!("file_edit: missing new_str"))?;

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("file_edit: read {}", path))?;

    let count = content.matches(old_str).count();
    if count == 0 {
        return Err(anyhow!("file_edit: old_str not found in {}", path));
    }
    if count > 1 {
        return Err(anyhow!(
            "file_edit: old_str found {} times in {} (must be unique)",
            count,
            path
        ));
    }

    let new_content = content.replacen(old_str, new_str, 1);
    std::fs::write(path, &new_content).with_context(|| format!("file_edit: write {}", path))?;
    Ok(format!("edited {}", path))
}

// ─── grep ─────────────────────────────────────────────────────────────────────

async fn tool_grep(input: &Value) -> anyhow::Result<String> {
    let pattern = input["pattern"]
        .as_str()
        .ok_or_else(|| anyhow!("grep: missing pattern"))?;

    let mut args: Vec<String> = Vec::new();

    let output_mode = input["output_mode"].as_str().unwrap_or("files_with_matches");
    match output_mode {
        "files_with_matches" => args.push("-l".into()),
        "count" => args.push("-c".into()),
        _ => {} // content mode: no extra flag
    }

    if input["case_insensitive"].as_bool().unwrap_or(false) {
        args.push("-i".into());
    }
    if input["line_numbers"].as_bool().unwrap_or(false) {
        args.push("-n".into());
    }
    if let Some(c) = input["context"].as_i64() {
        args.push(format!("-C{}", c));
    }
    if let Some(b) = input["context_before"].as_i64() {
        args.push(format!("-B{}", b));
    }
    if let Some(a) = input["context_after"].as_i64() {
        args.push(format!("-A{}", a));
    }
    if let Some(g) = input["glob"].as_str() {
        args.push("--glob".into());
        args.push(g.into());
    }

    args.push(pattern.into());
    args.push(".".into());

    // Try rg first, fall back to grep
    let rg_result = tokio::process::Command::new("rg")
        .args(&args)
        .output()
        .await;

    let output = match rg_result {
        Ok(o) => o,
        Err(_) => {
            // fallback grep
            let mut grep_args: Vec<String> = vec!["-r".into()];
            if input["case_insensitive"].as_bool().unwrap_or(false) {
                grep_args.push("-i".into());
            }
            if input["line_numbers"].as_bool().unwrap_or(false) {
                grep_args.push("-n".into());
            }
            if output_mode == "files_with_matches" {
                grep_args.push("-l".into());
            }
            if output_mode == "count" {
                grep_args.push("-c".into());
            }
            grep_args.push(pattern.into());
            grep_args.push(".".into());
            tokio::process::Command::new("grep")
                .args(&grep_args)
                .output()
                .await
                .context("grep exec")?
        }
    };

    let mut result = String::from_utf8_lossy(&output.stdout).to_string();

    if let Some(head) = input["head_limit"].as_u64() {
        let lines: Vec<&str> = result.lines().take(head as usize).collect();
        result = lines.join("\n");
        if !result.is_empty() {
            result.push('\n');
        }
    }

    Ok(result)
}

// ─── glob ─────────────────────────────────────────────────────────────────────

fn tool_glob(input: &Value) -> anyhow::Result<String> {
    let pattern = input["pattern"]
        .as_str()
        .ok_or_else(|| anyhow!("glob: missing pattern"))?;

    let base = input["path"].as_str().unwrap_or(".");

    let full_pattern = if pattern.starts_with('/') {
        pattern.to_string()
    } else {
        format!("{}/{}", base.trim_end_matches('/'), pattern)
    };

    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in glob::glob(&full_pattern).context("glob pattern")? {
        match entry {
            Ok(p) => {
                paths.push(p);
                if paths.len() >= 200 {
                    break;
                }
            }
            Err(_) => continue,
        }
    }

    let result = paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join("\n");
    Ok(result)
}

// ─── ls ───────────────────────────────────────────────────────────────────────

fn tool_ls(input: &Value) -> anyhow::Result<String> {
    let path = input["path"].as_str().unwrap_or(".");

    let entries = std::fs::read_dir(path).with_context(|| format!("ls: {}", path))?;
    let mut lines: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry.context("ls entry")?;
        let meta = entry.metadata().context("ls metadata")?;
        let name = entry.file_name().to_string_lossy().to_string();
        let kind = if meta.is_dir() { "dir" } else { "file" };
        let size = meta.len();
        lines.push(format!("{}\t{}\t{}", kind, size, name));
    }
    lines.sort();
    Ok(lines.join("\n"))
}

// ─── web_fetch ────────────────────────────────────────────────────────────────

async fn tool_web_fetch(input: &Value) -> anyhow::Result<String> {
    let url = input["url"]
        .as_str()
        .ok_or_else(|| anyhow!("web_fetch: missing url"))?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let resp = client.get(url).send().await.context("web_fetch request")?;
    let status = resp.status();
    if !status.is_success() {
        return Err(anyhow!("web_fetch: HTTP {}", status));
    }

    let bytes = resp.bytes().await.context("web_fetch read")?;
    let limit = 500 * 1024;
    let truncated = if bytes.len() > limit {
        &bytes[..limit]
    } else {
        &bytes[..]
    };
    let text = String::from_utf8_lossy(truncated).to_string();

    // Simple HTML tag stripping
    let stripped = strip_html(&text);
    Ok(stripped)
}

fn strip_html(html: &str) -> String {
    let re = regex::Regex::new(r"<[^>]+>").unwrap();
    let no_tags = re.replace_all(html, "");
    // Collapse whitespace
    let ws_re = regex::Regex::new(r"\n{3,}").unwrap();
    ws_re.replace_all(&no_tags, "\n\n").to_string()
}

// ─── web_search ───────────────────────────────────────────────────────────────

async fn tool_web_search(input: &Value) -> anyhow::Result<String> {
    let query = input["query"]
        .as_str()
        .ok_or_else(|| anyhow!("web_search: missing query"))?;

    if let Ok(api_key) = env::var("SERPAPI_API_KEY") {
        if !api_key.is_empty() {
            return serpapi_search(query, &api_key).await;
        }
    }

    duckduckgo_search(query).await
}

async fn serpapi_search(query: &str, api_key: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()?;

    let url = format!(
        "https://serpapi.com/search?q={}&api_key={}&engine=google&format=json",
        urlencoding::encode(query),
        api_key
    );

    let resp = client.get(&url).send().await.context("serpapi request")?;
    let text = resp.text().await?;
    let v: Value = serde_json::from_str(&text)?;

    let mut results = Vec::new();
    if let Some(organic) = v["organic_results"].as_array() {
        for item in organic.iter().take(5) {
            let title = item["title"].as_str().unwrap_or("");
            let link = item["link"].as_str().unwrap_or("");
            let snippet = item["snippet"].as_str().unwrap_or("");
            results.push(format!("**{}**\n{}\n{}", title, link, snippet));
        }
    }

    Ok(results.join("\n\n"))
}

async fn duckduckgo_search(query: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("Mozilla/5.0")
        .build()?;

    let url = format!(
        "https://html.duckduckgo.com/html/?q={}",
        urlencoding::encode(query)
    );

    let resp = client.get(&url).send().await.context("ddg request")?;
    let text = resp.text().await?;

    // Extract result snippets with basic regex
    let title_re = regex::Regex::new(r#"class="result__title"[^>]*>.*?<a[^>]+href="([^"]+)"[^>]*>(.*?)</a>"#).unwrap();
    let snippet_re = regex::Regex::new(r#"class="result__snippet"[^>]*>(.*?)</span>"#).unwrap();

    let titles: Vec<_> = title_re.captures_iter(&text).take(5).collect();
    let snippets: Vec<_> = snippet_re.captures_iter(&text).take(5).collect();

    let tag_re = regex::Regex::new(r"<[^>]+>").unwrap();

    let mut results = Vec::new();
    for (i, cap) in titles.iter().enumerate() {
        let url = cap.get(1).map(|m| m.as_str()).unwrap_or("");
        let title = cap.get(2).map(|m| m.as_str()).unwrap_or("");
        let title_clean = tag_re.replace_all(title, "");
        let snippet = snippets
            .get(i)
            .and_then(|c| c.get(1))
            .map(|m| tag_re.replace_all(m.as_str(), "").to_string())
            .unwrap_or_default();
        results.push(format!("**{}**\n{}\n{}", title_clean.trim(), url, snippet.trim()));
    }

    if results.is_empty() {
        Ok(format!("No results found for: {}", query))
    } else {
        Ok(results.join("\n\n"))
    }
}

// ─── todo_write ───────────────────────────────────────────────────────────────

fn tool_todo_write(input: &Value, todos: &Arc<Mutex<Vec<Value>>>) -> anyhow::Result<String> {
    let new_todos = input["todos"]
        .as_array()
        .ok_or_else(|| anyhow!("todo_write: missing todos array"))?
        .clone();

    let mut guard = todos.lock().map_err(|_| anyhow!("todo mutex"))?;
    *guard = new_todos;
    Ok(format!("todo list updated ({} items)", guard.len()))
}

// ─── tool_search ──────────────────────────────────────────────────────────────

fn tool_tool_search(input: &Value) -> anyhow::Result<String> {
    let query = input["query"].as_str().unwrap_or("").to_lowercase();
    let defs = all_tool_definitions();

    let results: Vec<String> = defs
        .iter()
        .filter(|t| {
            query.is_empty()
                || t.name.to_lowercase().contains(&query)
                || t.description.to_lowercase().contains(&query)
        })
        .map(|t| format!("**{}**: {}", t.name, t.description))
        .collect();

    Ok(results.join("\n"))
}

// ─── send_message ─────────────────────────────────────────────────────────────

async fn tool_send_message(input: &Value, executor: &ToolExecutor) -> anyhow::Result<String> {
    let to_agent = input["to_agent"]
        .as_str()
        .ok_or_else(|| anyhow!("send_message: missing to_agent"))?;
    let msg_type = input["type"]
        .as_str()
        .ok_or_else(|| anyhow!("send_message: missing type"))?;
    let content = input["content"]
        .as_str()
        .ok_or_else(|| anyhow!("send_message: missing content"))?;

    let mut client = HubServiceClient::connect(executor.grpc_endpoint.clone())
        .await
        .context("send_message: connect hub")?;

    let msg = HubMessage {
        id: uuid::Uuid::new_v4().to_string(),
        from_agent: executor.from_agent.clone(),
        to_agent: to_agent.to_string(),
        r#type: msg_type.to_string(),
        content: content.to_string(),
        meeting_id: String::new(),
        occurred_at_unix: chrono::Utc::now().timestamp(),
    };

    client
        .publish(PublishMessageRequest { message: Some(msg) })
        .await
        .map_err(|s| anyhow!("send_message: publish: {}", s))?;

    Ok(format!("message sent to {}", to_agent))
}

// ─── notebook_edit ────────────────────────────────────────────────────────────

fn tool_notebook_edit(input: &Value) -> anyhow::Result<String> {
    let path = input["path"]
        .as_str()
        .ok_or_else(|| anyhow!("notebook_edit: missing path"))?;
    let ops = input["operations"]
        .as_array()
        .ok_or_else(|| anyhow!("notebook_edit: missing operations"))?;

    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("notebook_edit: read {}", path))?;
    let mut nb: Value = serde_json::from_str(&raw).context("notebook_edit: parse JSON")?;

    let cells = nb["cells"]
        .as_array_mut()
        .ok_or_else(|| anyhow!("notebook_edit: no cells array"))?;

    for op in ops {
        let operation = op["op"].as_str().unwrap_or("");
        let index = op["index"].as_u64().unwrap_or(0) as usize;

        match operation {
            "replace" => {
                if index >= cells.len() {
                    return Err(anyhow!("notebook_edit: index {} out of range", index));
                }
                let source = op["source"].as_str().unwrap_or("");
                let cell_type = op["cell_type"].as_str().unwrap_or("code");
                cells[index] = serde_json::json!({
                    "cell_type": cell_type,
                    "source": source,
                    "metadata": {},
                    "outputs": [],
                    "execution_count": null
                });
            }
            "insert" => {
                let source = op["source"].as_str().unwrap_or("");
                let cell_type = op["cell_type"].as_str().unwrap_or("code");
                let new_cell = serde_json::json!({
                    "cell_type": cell_type,
                    "source": source,
                    "metadata": {},
                    "outputs": [],
                    "execution_count": null
                });
                let idx = index.min(cells.len());
                cells.insert(idx, new_cell);
            }
            "delete" => {
                if index >= cells.len() {
                    return Err(anyhow!("notebook_edit: delete index {} out of range", index));
                }
                cells.remove(index);
            }
            _ => return Err(anyhow!("notebook_edit: unknown op: {}", operation)),
        }
    }

    let serialized = serde_json::to_string_pretty(&nb).context("notebook_edit: serialize")?;
    std::fs::write(path, &serialized)
        .with_context(|| format!("notebook_edit: write {}", path))?;

    Ok(format!("notebook {} updated", path))
}

// Need this for URL encoding in web_search
mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut out = String::new();
        for b in s.bytes() {
            match b {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    out.push(b as char)
                }
                b' ' => out.push('+'),
                b => out.push_str(&format!("%{:02X}", b)),
            }
        }
        out
    }
}
