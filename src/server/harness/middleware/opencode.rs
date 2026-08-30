use std::collections::{BTreeMap, HashMap};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Map, Value, json};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{Instant, sleep, timeout};
use uuid::Uuid;

use crate::middleware::harness::{HarnessAdapterError, HarnessEvent};
use crate::middleware::http_runtime::{HttpProcessConfig, HttpProcessError, HttpProcessRuntime};
use crate::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};

pub const OPENCODE_PROVIDER_ID: &str = "omnisolo-openai-compatible";
pub const OPENCODE_API_KEY_REFERENCE: &str = "OPENAI_API_KEY";
pub const OPENCODE_CONFIG_HOME_ENV: &str = "XDG_CONFIG_HOME";
pub const OPENCODE_DATA_HOME_ENV: &str = "XDG_DATA_HOME";

const OPENCODE_CONFIG_ENV: &str = "OPENCODE_CONFIG";
const OPENCODE_CONFIG_CONTENT_ENV: &str = "OPENCODE_CONFIG_CONTENT";
const MAX_HTTP_BODY_BYTES: usize = 8 * 1024 * 1024;
const MAX_HEADER_BYTES: usize = 64 * 1024;
const MAX_SSE_LINE_BYTES: usize = 64 * 1024;
const MAX_SSE_EVENT_BYTES: usize = 256 * 1024;
const MAX_READINESS_PROBE_TIMEOUT: Duration = Duration::from_secs(1);

pub fn build_opencode_config(
    selection: &ResolvedModelSelection,
    base_url: &str,
) -> Result<Value, HarnessAdapterError> {
    let provider_route = selection.provider_route.trim();
    let model_id = selection.model_id.trim();
    if provider_route != "openai-compatible" {
        return Err(HarnessAdapterError::InvalidRequest(
            "OpenCode only supports the openai-compatible provider route".to_owned(),
        ));
    }
    if model_id.is_empty() || model_id.chars().any(char::is_control) {
        return Err(HarnessAdapterError::InvalidRequest(
            "OpenCode configuration requires a provider route and model id".to_owned(),
        ));
    }
    if selection.api_dialect != ModelApiDialect::OpenAiResponses {
        return Err(HarnessAdapterError::InvalidRequest(
            "OpenCode integration requires the OpenAI Responses API dialect".to_owned(),
        ));
    }
    let base_url = validated_base_url(base_url)?;

    let mut model = Map::new();
    model.insert("name".to_owned(), Value::String(model_id.to_owned()));
    model.insert("reasoning".to_owned(), Value::Bool(false));
    if let Some(effort) = selection.reasoning_effort.as_ref() {
        let variant = reasoning_variant(effort)?;
        model.insert(
            "reasoning".to_owned(),
            Value::Bool(!matches!(effort, ReasoningEffort::None)),
        );
        model.insert(
            "variants".to_owned(),
            json!({variant: {"reasoningEffort": variant}}),
        );
    }
    if selection.context_window.is_some() || selection.max_output_tokens.is_some() {
        let mut limit = Map::new();
        if let Some(context) = selection.context_window {
            limit.insert("context".to_owned(), Value::from(context));
        }
        if let Some(output) = selection.max_output_tokens {
            limit.insert("output".to_owned(), Value::from(output));
        }
        model.insert("limit".to_owned(), Value::Object(limit));
    }

    let mut models = Map::new();
    models.insert(model_id.to_owned(), Value::Object(model));
    let provider = json!({
        "npm": "@ai-sdk/openai",
        "name": "OmniSolo OpenAI-compatible Responses",
        "options": {
            "baseURL": base_url,
            "apiKey": format!("{{env:{OPENCODE_API_KEY_REFERENCE}}}"),
            "includeUsage": true,
        },
        "models": Value::Object(models),
    });
    Ok(json!({
        "$schema": "https://opencode.ai/config.json",
        "provider": {OPENCODE_PROVIDER_ID: provider},
    }))
}

fn validated_base_url(base_url: &str) -> Result<String, HarnessAdapterError> {
    let base_url = base_url.trim().trim_end_matches('/');
    if base_url.chars().any(char::is_whitespace) {
        return Err(HarnessAdapterError::InvalidRequest(
            "OpenCode base URL must not contain whitespace".to_owned(),
        ));
    }
    let parsed = reqwest::Url::parse(base_url).map_err(|_| {
        HarnessAdapterError::InvalidRequest("OpenCode base URL is malformed".to_owned())
    })?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err(HarnessAdapterError::InvalidRequest(
            "OpenCode base URL must use HTTP or HTTPS".to_owned(),
        ));
    }
    let original_authority = base_url
        .split_once("://")
        .map(|(_, remainder)| remainder.split('/').next().unwrap_or_default())
        .unwrap_or_default();
    if original_authority.is_empty()
        || parsed.host().is_none()
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.query().is_some()
        || parsed.fragment().is_some()
    {
        return Err(HarnessAdapterError::InvalidRequest(
            "OpenCode base URL must not contain credentials, query parameters, fragments, or whitespace"
                .to_owned(),
        ));
    }
    Ok(base_url.to_owned())
}

fn reasoning_variant(effort: &ReasoningEffort) -> Result<&'static str, HarnessAdapterError> {
    match effort {
        ReasoningEffort::None => Ok("none"),
        ReasoningEffort::Minimal => Ok("minimal"),
        ReasoningEffort::Low => Ok("low"),
        ReasoningEffort::Medium => Ok("medium"),
        ReasoningEffort::High => Ok("high"),
        ReasoningEffort::Max => Ok("max"),
        ReasoningEffort::Custom => Err(HarnessAdapterError::InvalidRequest(
            "OpenCode cannot translate a custom reasoning effort".to_owned(),
        )),
    }
}

#[derive(Debug)]
pub struct OpenCodeIsolatedHome {
    root: PathBuf,
    config_home: PathBuf,
    data_home: PathBuf,
    config_path: PathBuf,
    environment: BTreeMap<String, PathBuf>,
    config_content: String,
}

impl OpenCodeIsolatedHome {
    pub fn create_in(
        parent: &Path,
        selection: &ResolvedModelSelection,
        base_url: &str,
    ) -> Result<Self, HarnessAdapterError> {
        let descriptor = build_opencode_config(selection, base_url)?;
        fs::create_dir_all(parent).map_err(HarnessAdapterError::Io)?;
        let root = parent.join(format!("omnisolo-opencode-{}", Uuid::new_v4()));
        fs::create_dir(&root).map_err(HarnessAdapterError::Io)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
                .map_err(HarnessAdapterError::Io)?;
        }

        let config_home = root.join("config");
        let data_home = root.join("data");
        let config_directory = config_home.join("opencode");
        if let Err(error) =
            fs::create_dir_all(&config_directory).and_then(|_| fs::create_dir_all(&data_home))
        {
            let _ = fs::remove_dir_all(&root);
            return Err(HarnessAdapterError::Io(error));
        }
        let config_path = config_directory.join("opencode.json");
        let config_content =
            serde_json::to_string_pretty(&descriptor).map_err(HarnessAdapterError::Json)?;
        if let Err(error) = write_private_config(&config_path, &config_content) {
            let _ = fs::remove_dir_all(&root);
            return Err(error);
        }
        let environment = BTreeMap::from([
            (OPENCODE_CONFIG_HOME_ENV.to_owned(), config_home.clone()),
            (OPENCODE_DATA_HOME_ENV.to_owned(), data_home.clone()),
        ]);
        Ok(Self {
            root,
            config_home,
            data_home,
            config_path,
            environment,
            config_content,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn config_home(&self) -> &Path {
        &self.config_home
    }

    pub fn data_home(&self) -> &Path {
        &self.data_home
    }

    pub fn config_path(&self) -> &Path {
        &self.config_path
    }

    pub fn environment(&self) -> &BTreeMap<String, PathBuf> {
        &self.environment
    }
}

impl Drop for OpenCodeIsolatedHome {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn write_private_config(path: &Path, content: &str) -> Result<(), HarnessAdapterError> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path).map_err(HarnessAdapterError::Io)?;
    file.write_all(content.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .and_then(|_| file.sync_all())
        .map_err(HarnessAdapterError::Io)
}

#[derive(Clone, PartialEq)]
pub struct OpenCodeProcessConfig {
    executable: String,
    prefix_args: Vec<String>,
    environment: BTreeMap<String, String>,
    working_directory: Option<PathBuf>,
    preferred_address: SocketAddr,
    readiness_timeout: Duration,
    poll_interval: Duration,
    request_timeout: Duration,
    isolation_parent: Option<PathBuf>,
}

impl std::fmt::Debug for OpenCodeProcessConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenCodeProcessConfig")
            .field("executable", &self.executable)
            .field("prefix_arg_count", &self.prefix_args.len())
            .field(
                "environment_keys",
                &self.environment.keys().collect::<Vec<_>>(),
            )
            .field("working_directory", &self.working_directory)
            .field("preferred_address", &self.preferred_address)
            .field("readiness_timeout", &self.readiness_timeout)
            .field("poll_interval", &self.poll_interval)
            .field("request_timeout", &self.request_timeout)
            .field("isolation_parent", &self.isolation_parent)
            .finish()
    }
}

impl OpenCodeProcessConfig {
    pub fn new(executable: impl Into<String>) -> Self {
        Self {
            executable: executable.into(),
            prefix_args: Vec::new(),
            environment: BTreeMap::new(),
            working_directory: None,
            preferred_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
            readiness_timeout: Duration::from_secs(30),
            poll_interval: Duration::from_millis(50),
            request_timeout: Duration::from_secs(30),
            isolation_parent: None,
        }
    }

    pub fn with_prefix_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.prefix_args = args.into_iter().map(Into::into).collect();
        self
    }

    pub fn with_environment(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    pub fn with_working_directory(mut self, working_directory: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(working_directory.into());
        self
    }

    pub fn with_preferred_address(mut self, preferred_address: SocketAddr) -> Self {
        self.preferred_address = preferred_address;
        self
    }

    pub fn with_readiness(mut self, timeout: Duration, poll_interval: Duration) -> Self {
        self.readiness_timeout = timeout;
        self.poll_interval = poll_interval;
        self
    }

    pub fn with_request_timeout(mut self, request_timeout: Duration) -> Self {
        self.request_timeout = request_timeout;
        self
    }

    pub fn with_isolation_parent(mut self, isolation_parent: impl Into<PathBuf>) -> Self {
        self.isolation_parent = Some(isolation_parent.into());
        self
    }

    pub fn http_process_config(&self, home: &OpenCodeIsolatedHome) -> HttpProcessConfig {
        let mut args = self.prefix_args.clone();
        args.extend(
            [
                "serve",
                "--hostname",
                "{host}",
                "--port",
                "{port}",
                "--pure",
            ]
            .map(str::to_owned),
        );
        let mut environment = self.environment.clone();
        for (key, value) in home.environment() {
            environment.insert(key.clone(), value.to_string_lossy().into_owned());
        }
        environment.insert(
            OPENCODE_CONFIG_ENV.to_owned(),
            home.config_path().to_string_lossy().into_owned(),
        );
        environment.insert(
            OPENCODE_CONFIG_CONTENT_ENV.to_owned(),
            home.config_content.clone(),
        );
        environment.insert("OPENCODE_DISABLE_AUTOUPDATE".to_owned(), "true".to_owned());

        let mut config = HttpProcessConfig::new(&self.executable, args)
            .with_preferred_address(self.preferred_address)
            .with_readiness(self.readiness_timeout, self.poll_interval);
        config.environment = environment;
        if let Some(working_directory) = &self.working_directory {
            config = config.with_working_directory(working_directory);
        }
        config
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenCodeHealth {
    pub version: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenCodeSession {
    pub id: String,
    pub native: Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenCodePromptResult {
    pub final_text: Option<String>,
    pub usage: Option<Value>,
    pub native: Value,
}

pub struct OpenCodeHttpAdapter {
    runtime: Option<HttpProcessRuntime>,
    home: Option<OpenCodeIsolatedHome>,
    address: SocketAddr,
    request_timeout: Duration,
    active_turns: Arc<Mutex<HashMap<String, String>>>,
    secret_values: Vec<String>,
}

impl std::fmt::Debug for OpenCodeHttpAdapter {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenCodeHttpAdapter")
            .field("address", &self.address)
            .field("request_timeout", &self.request_timeout)
            .field("managed_process", &self.runtime.is_some())
            .field("isolated", &self.home.is_some())
            .finish()
    }
}

impl OpenCodeHttpAdapter {
    pub async fn spawn(
        config: OpenCodeProcessConfig,
        selection: ResolvedModelSelection,
        base_url: &str,
    ) -> Result<Self, HarnessAdapterError> {
        let deadline = Instant::now() + config.readiness_timeout;
        let secret_values = config
            .environment
            .iter()
            .filter(|(key, _)| sensitive_key(key))
            .map(|(_, value)| value.clone())
            .filter(|value| !value.is_empty())
            .collect();
        let parent = config
            .isolation_parent
            .clone()
            .unwrap_or_else(std::env::temp_dir);
        let home = OpenCodeIsolatedHome::create_in(&parent, &selection, base_url)?;
        let mut launch = config.http_process_config(&home);
        launch.readiness_timeout = deadline.saturating_duration_since(Instant::now());
        let runtime = HttpProcessRuntime::spawn(launch)
            .await
            .map_err(map_process_error)?;
        let address = runtime.address();
        let mut adapter = Self {
            runtime: Some(runtime),
            home: Some(home),
            address,
            request_timeout: config
                .request_timeout
                .min(deadline.saturating_duration_since(Instant::now())),
            active_turns: Arc::new(Mutex::new(HashMap::new())),
            secret_values,
        };
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(HarnessAdapterError::Timeout);
            }
            adapter.request_timeout = config
                .request_timeout
                .min(MAX_READINESS_PROBE_TIMEOUT)
                .min(remaining / 2);
            match adapter.health().await {
                Ok(_) => {
                    adapter.request_timeout = config.request_timeout;
                    return Ok(adapter);
                }
                Err(HarnessAdapterError::InvalidResponse(message)) => {
                    return Err(HarnessAdapterError::InvalidResponse(message));
                }
                Err(_) if Instant::now() < deadline => sleep(config.poll_interval).await,
                Err(_) => return Err(HarnessAdapterError::Timeout),
            }
        }
    }

    pub fn connect(
        address: SocketAddr,
        request_timeout: Duration,
    ) -> Result<Self, HarnessAdapterError> {
        Self::connect_with_secret_values(address, request_timeout, std::iter::empty::<String>())
    }

    pub fn connect_with_secret_values<I>(
        address: SocketAddr,
        request_timeout: Duration,
        secret_values: I,
    ) -> Result<Self, HarnessAdapterError>
    where
        I: IntoIterator<Item = String>,
    {
        if !address.ip().is_loopback() {
            return Err(HarnessAdapterError::InvalidRequest(format!(
                "OpenCode HTTP address is not loopback: {address}"
            )));
        }
        Ok(Self {
            runtime: None,
            home: None,
            address,
            request_timeout,
            active_turns: Arc::new(Mutex::new(HashMap::new())),
            secret_values: secret_values
                .into_iter()
                .filter(|value| !value.is_empty())
                .collect(),
        })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn process_id(&self) -> Option<u32> {
        self.runtime
            .as_ref()
            .and_then(HttpProcessRuntime::process_id)
    }

    pub fn isolated_root(&self) -> Option<&Path> {
        self.home.as_ref().map(OpenCodeIsolatedHome::root)
    }

    pub async fn health(&self) -> Result<OpenCodeHealth, HarnessAdapterError> {
        let value = self
            .json_request("GET", "/global/health", None, &[200])
            .await?;
        let object = required_object(&value, "OpenCode health response")?;
        if object.get("healthy").and_then(Value::as_bool) != Some(true) {
            return Err(invalid_response(
                "OpenCode health response healthy must be true",
            ));
        }
        let version = required_string(object.get("version"), "OpenCode health version")?;
        Ok(OpenCodeHealth {
            version: version.to_owned(),
        })
    }

    pub async fn create_session(
        &self,
        title: &str,
        selection: &ResolvedModelSelection,
    ) -> Result<OpenCodeSession, HarnessAdapterError> {
        let model = create_model(selection)?;
        let value = self
            .json_request(
                "POST",
                "/session",
                Some(json!({"title": title, "model": model})),
                &[200],
            )
            .await?;
        parse_session(value)
    }

    pub async fn get_session(
        &self,
        native_session_id: &str,
    ) -> Result<OpenCodeSession, HarnessAdapterError> {
        let native_session_id = validated_session_id(native_session_id)?;
        let value = self
            .json_request(
                "GET",
                &format!("/session/{native_session_id}"),
                None,
                &[200],
            )
            .await?;
        let session = parse_session(value)?;
        if session.id != native_session_id {
            return Err(invalid_response(
                "OpenCode get-session response id did not match the requested session",
            ));
        }
        Ok(session)
    }

    pub async fn delete_session(&self, native_session_id: &str) -> Result<(), HarnessAdapterError> {
        let native_session_id = validated_session_id(native_session_id)?;
        let value = self
            .json_request(
                "DELETE",
                &format!("/session/{native_session_id}"),
                None,
                &[200],
            )
            .await?;
        if value != Value::Bool(true) {
            return Err(invalid_response(
                "OpenCode delete-session response must be true",
            ));
        }
        Ok(())
    }

    pub async fn prompt(
        &self,
        native_session_id: &str,
        prompt: &str,
        selection: &ResolvedModelSelection,
    ) -> Result<OpenCodePromptResult, HarnessAdapterError> {
        let native_session_id = validated_session_id(native_session_id)?;
        let body = prompt_body(prompt, selection)?;
        let value = self
            .json_request(
                "POST",
                &format!("/session/{native_session_id}/message"),
                Some(body),
                &[200],
            )
            .await?;
        decode_prompt_result(value, native_session_id)
    }

    pub async fn prompt_async(
        &self,
        mut correlation: OpenCodeEventCorrelation,
        prompt: &str,
        selection: &ResolvedModelSelection,
    ) -> Result<OpenCodeEventStream, HarnessAdapterError> {
        let native_session_id = validated_session_id(&correlation.native_session_id)?.to_owned();
        let message_id = format!("msg_{}", Uuid::new_v4().simple());
        correlation.admitted_user_message_id = Some(message_id.clone());
        let turn = ActiveTurnLease::acquire(
            Arc::clone(&self.active_turns),
            native_session_id.clone(),
            message_id.clone(),
        )?;
        let mut stream = self.subscribe(correlation).await?;
        let mut body = prompt_body(prompt, selection)?;
        body.as_object_mut()
            .expect("OpenCode prompt bodies are objects")
            .insert("messageID".to_owned(), Value::String(message_id));
        self.json_request(
            "POST",
            &format!("/session/{native_session_id}/prompt_async"),
            Some(body),
            &[204],
        )
        .await?;
        stream.turn = Some(turn);
        Ok(stream)
    }

    pub async fn abort(&self, native_session_id: &str) -> Result<(), HarnessAdapterError> {
        let native_session_id = validated_session_id(native_session_id)?;
        let value = self
            .json_request(
                "POST",
                &format!("/session/{native_session_id}/abort"),
                None,
                &[200],
            )
            .await?;
        if value != Value::Bool(true) {
            return Err(invalid_response("OpenCode abort response must be true"));
        }
        Ok(())
    }

    pub async fn shutdown(mut self) -> Result<(), HarnessAdapterError> {
        let active_sessions = self
            .active_turns
            .lock()
            .map_err(|_| invalid_response("OpenCode active-turn lock was poisoned"))?
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let mut first_error = None;
        for native_session_id in active_sessions {
            if let Err(error) = self.abort(&native_session_id).await
                && first_error.is_none()
            {
                first_error = Some(error);
            }
        }
        if let Some(runtime) = self.runtime.take()
            && let Err(error) = runtime.shutdown().await.map_err(HarnessAdapterError::Io)
            && first_error.is_none()
        {
            first_error = Some(error);
        }
        // The isolated home must outlive the managed process because OpenCode can
        // still flush session state while exiting.
        drop(self.home.take());
        first_error.map_or(Ok(()), Err)
    }

    async fn subscribe(
        &self,
        correlation: OpenCodeEventCorrelation,
    ) -> Result<OpenCodeEventStream, HarnessAdapterError> {
        let operation = async {
            let mut stream = TcpStream::connect(self.address)
                .await
                .map_err(|_| HarnessAdapterError::ProcessExited)?;
            let request = format!(
                "GET /event HTTP/1.0\r\nHost: {}\r\nAccept: text/event-stream\r\nConnection: close\r\n\r\n",
                self.address
            );
            stream
                .write_all(request.as_bytes())
                .await
                .map_err(|_| HarnessAdapterError::ProcessExited)?;
            let mut reader = BufReader::new(stream);
            let (status, headers) = read_response_head(&mut reader).await?;
            if status != 200 {
                return Err(remote_http_error(status, Value::Null));
            }
            let content_type = headers
                .get("content-type")
                .map(String::as_str)
                .unwrap_or("");
            if !content_type
                .to_ascii_lowercase()
                .starts_with("text/event-stream")
            {
                return Err(invalid_response(
                    "OpenCode event subscription did not return text/event-stream",
                ));
            }
            let native_session_id = correlation.native_session_id.clone();
            Ok(OpenCodeEventStream {
                reader,
                decoder: OpenCodeEventDecoder::new_with_secrets(
                    correlation,
                    self.secret_values.clone(),
                ),
                event_timeout: self.request_timeout,
                turn: None,
                address: self.address,
                native_session_id,
                cancellation_sent: false,
            })
        };
        timeout(self.request_timeout, operation)
            .await
            .map_err(|_| HarnessAdapterError::Timeout)?
    }

    async fn json_request(
        &self,
        method: &str,
        path: &str,
        body: Option<Value>,
        expected_statuses: &[u16],
    ) -> Result<Value, HarnessAdapterError> {
        let response = timeout(
            self.request_timeout,
            raw_http_request(self.address, method, path, body.as_ref()),
        )
        .await
        .map_err(|_| HarnessAdapterError::Timeout)??;
        let value = if response.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&response.body)
                .map_err(|_| invalid_response("OpenCode HTTP response body was not valid JSON"))?
        };
        if !expected_statuses.contains(&response.status) {
            return Err(remote_http_error(response.status, value));
        }
        Ok(value)
    }
}

fn map_process_error(error: HttpProcessError) -> HarnessAdapterError {
    match error {
        HttpProcessError::NonLoopbackAddress(address) => HarnessAdapterError::InvalidRequest(
            format!("OpenCode HTTP address is not loopback: {address}"),
        ),
        HttpProcessError::Spawn(error) => HarnessAdapterError::Spawn(error),
        HttpProcessError::Bind(error) | HttpProcessError::Poll(error) => {
            HarnessAdapterError::Io(error)
        }
        HttpProcessError::ReadinessTimeout { .. } => HarnessAdapterError::Timeout,
        HttpProcessError::EarlyExit { .. } => HarnessAdapterError::ProcessExited,
    }
}

fn validated_session_id(value: &str) -> Result<&str, HarnessAdapterError> {
    if value.starts_with("ses")
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '-'))
    {
        Ok(value)
    } else {
        Err(HarnessAdapterError::InvalidRequest(
            "OpenCode session id is malformed".to_owned(),
        ))
    }
}

fn create_model(selection: &ResolvedModelSelection) -> Result<Value, HarnessAdapterError> {
    validate_selection(selection)?;
    let mut model = Map::new();
    model.insert(
        "providerID".to_owned(),
        Value::String(OPENCODE_PROVIDER_ID.to_owned()),
    );
    model.insert(
        "id".to_owned(),
        Value::String(selection.model_id.trim().to_owned()),
    );
    if let Some(effort) = selection.reasoning_effort.as_ref() {
        model.insert(
            "variant".to_owned(),
            Value::String(reasoning_variant(effort)?.to_owned()),
        );
    }
    Ok(Value::Object(model))
}

fn prompt_body(
    prompt: &str,
    selection: &ResolvedModelSelection,
) -> Result<Value, HarnessAdapterError> {
    validate_selection(selection)?;
    let mut body = Map::new();
    body.insert(
        "model".to_owned(),
        json!({
            "providerID": OPENCODE_PROVIDER_ID,
            "modelID": selection.model_id.trim(),
        }),
    );
    if let Some(effort) = selection.reasoning_effort.as_ref() {
        body.insert(
            "variant".to_owned(),
            Value::String(reasoning_variant(effort)?.to_owned()),
        );
    }
    body.insert("parts".to_owned(), json!([{"type":"text", "text":prompt}]));
    Ok(Value::Object(body))
}

fn validate_selection(selection: &ResolvedModelSelection) -> Result<(), HarnessAdapterError> {
    build_opencode_config(selection, "http://127.0.0.1")?;
    Ok(())
}

fn parse_session(value: Value) -> Result<OpenCodeSession, HarnessAdapterError> {
    let object = required_object(&value, "OpenCode session response")?;
    let id = required_string(object.get("id"), "OpenCode session id")?;
    validated_session_id(id).map_err(|_| {
        invalid_response("OpenCode session response id did not match the native id contract")
    })?;
    for (field, label) in [
        ("slug", "slug"),
        ("projectID", "project id"),
        ("directory", "directory"),
        ("title", "title"),
        ("version", "version"),
    ] {
        required_string(object.get(field), &format!("OpenCode session {label}"))?;
    }
    let time = required_object_value(object.get("time"), "OpenCode session time")?;
    required_number(time.get("created"), "OpenCode session created time")?;
    required_number(time.get("updated"), "OpenCode session updated time")?;
    Ok(OpenCodeSession {
        id: id.to_owned(),
        native: sanitize_native(value),
    })
}

fn decode_prompt_result(
    value: Value,
    native_session_id: &str,
) -> Result<OpenCodePromptResult, HarnessAdapterError> {
    let object = required_object(&value, "OpenCode prompt response")?;
    let info = required_object_value(object.get("info"), "OpenCode assistant message info")?;
    if required_string(info.get("role"), "OpenCode assistant role")? != "assistant" {
        return Err(invalid_response(
            "OpenCode prompt response role must be assistant",
        ));
    }
    if required_string(info.get("sessionID"), "OpenCode assistant session id")? != native_session_id
    {
        return Err(invalid_response(
            "OpenCode prompt response session id did not match",
        ));
    }
    validate_assistant_info(info)?;
    if let Some(error) = info.get("error").filter(|value| !value.is_null()) {
        required_object_value(Some(error), "OpenCode assistant error")?;
        return Err(remote_native_error(error));
    }
    let usage = decode_usage(info)?;
    let parts = object
        .get("parts")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_response("OpenCode prompt response parts must be an array"))?;
    let mut text = String::new();
    for part in parts {
        let part = required_object(part, "OpenCode prompt response part")?;
        required_string(part.get("id"), "OpenCode prompt part id")?;
        if required_string(part.get("sessionID"), "OpenCode prompt part session id")?
            != native_session_id
        {
            return Err(invalid_response(
                "OpenCode prompt part session id did not match",
            ));
        }
        required_string(part.get("messageID"), "OpenCode prompt part message id")?;
        let part_type = required_string(part.get("type"), "OpenCode prompt part type")?;
        if matches!(part_type, "text" | "reasoning") {
            let part_text = required_string_allow_empty(
                part.get("text"),
                "OpenCode prompt text or reasoning part",
            )?;
            if part_type == "text" {
                text.push_str(part_text);
            }
        }
    }
    Ok(OpenCodePromptResult {
        final_text: (!text.is_empty()).then_some(text),
        usage: Some(usage),
        native: sanitize_native(value),
    })
}

fn validate_assistant_info(info: &Map<String, Value>) -> Result<(), HarnessAdapterError> {
    for (field, label) in [
        ("id", "message id"),
        ("parentID", "parent id"),
        ("modelID", "model id"),
        ("providerID", "provider id"),
        ("mode", "mode"),
        ("agent", "agent"),
    ] {
        required_string(info.get(field), &format!("OpenCode assistant {label}"))?;
    }
    let time = required_object_value(info.get("time"), "OpenCode assistant time")?;
    required_number(time.get("created"), "OpenCode assistant created time")?;
    let path = required_object_value(info.get("path"), "OpenCode assistant path")?;
    required_string(path.get("cwd"), "OpenCode assistant cwd")?;
    required_string(path.get("root"), "OpenCode assistant root")?;
    Ok(())
}

fn decode_usage(info: &Map<String, Value>) -> Result<Value, HarnessAdapterError> {
    let tokens = required_object_value(info.get("tokens"), "OpenCode assistant tokens")?;
    for field in ["input", "output", "reasoning"] {
        required_number(tokens.get(field), &format!("OpenCode token field {field}"))?;
    }
    if let Some(total) = tokens.get("total") {
        required_number(Some(total), "OpenCode token field total")?;
    }
    let cache = required_object_value(tokens.get("cache"), "OpenCode token cache")?;
    required_number(cache.get("read"), "OpenCode cache read tokens")?;
    required_number(cache.get("write"), "OpenCode cache write tokens")?;
    let cost = required_number(info.get("cost"), "OpenCode assistant cost")?;
    Ok(json!({
        "tokens": sanitize_native(Value::Object(tokens.clone())),
        "cost": cost,
    }))
}

#[derive(Debug)]
struct HttpResponse {
    status: u16,
    body: Vec<u8>,
}

async fn raw_http_request(
    address: SocketAddr,
    method: &str,
    path: &str,
    body: Option<&Value>,
) -> Result<HttpResponse, HarnessAdapterError> {
    let body = body
        .map(serde_json::to_vec)
        .transpose()
        .map_err(HarnessAdapterError::Json)?
        .unwrap_or_default();
    let mut stream = TcpStream::connect(address)
        .await
        .map_err(|_| HarnessAdapterError::ProcessExited)?;
    let mut request = format!(
        "{method} {path} HTTP/1.0\r\nHost: {address}\r\nAccept: application/json\r\nConnection: close\r\n"
    );
    if !body.is_empty() {
        request.push_str("Content-Type: application/json\r\n");
        request.push_str(&format!("Content-Length: {}\r\n", body.len()));
    }
    request.push_str("\r\n");
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|_| HarnessAdapterError::ProcessExited)?;
    if !body.is_empty() {
        stream
            .write_all(&body)
            .await
            .map_err(|_| HarnessAdapterError::ProcessExited)?;
    }
    let mut reader = BufReader::new(stream);
    let (status, headers) = read_response_head(&mut reader).await?;
    let content_length = headers
        .get("content-length")
        .map(|value| {
            value
                .parse::<usize>()
                .map_err(|_| invalid_response("OpenCode Content-Length was malformed"))
        })
        .transpose()?;
    if content_length.is_some_and(|length| length > MAX_HTTP_BODY_BYTES) {
        return Err(invalid_response(
            "OpenCode HTTP response exceeded the body limit",
        ));
    }
    let mut response_body = Vec::new();
    if let Some(content_length) = content_length {
        response_body.resize(content_length, 0);
        reader
            .read_exact(&mut response_body)
            .await
            .map_err(|_| HarnessAdapterError::ProcessExited)?;
    } else {
        reader
            .take((MAX_HTTP_BODY_BYTES + 1) as u64)
            .read_to_end(&mut response_body)
            .await
            .map_err(|_| HarnessAdapterError::ProcessExited)?;
        if response_body.len() > MAX_HTTP_BODY_BYTES {
            return Err(invalid_response(
                "OpenCode HTTP response exceeded the body limit",
            ));
        }
    }
    Ok(HttpResponse {
        status,
        body: response_body,
    })
}

async fn read_response_head(
    reader: &mut BufReader<TcpStream>,
) -> Result<(u16, BTreeMap<String, String>), HarnessAdapterError> {
    let mut status_line = String::new();
    if reader
        .read_line(&mut status_line)
        .await
        .map_err(|_| HarnessAdapterError::ProcessExited)?
        == 0
    {
        return Err(HarnessAdapterError::ProcessExited);
    }
    let mut parts = status_line.split_whitespace();
    let protocol = parts.next().unwrap_or_default();
    let status = parts
        .next()
        .and_then(|value| value.parse::<u16>().ok())
        .ok_or_else(|| invalid_response("OpenCode HTTP status line was malformed"))?;
    if !protocol.starts_with("HTTP/1.") {
        return Err(invalid_response("OpenCode HTTP protocol was malformed"));
    }

    let mut headers = BTreeMap::new();
    let mut header_bytes = status_line.len();
    loop {
        let mut line = String::new();
        if reader
            .read_line(&mut line)
            .await
            .map_err(|_| HarnessAdapterError::ProcessExited)?
            == 0
        {
            return Err(HarnessAdapterError::ProcessExited);
        }
        header_bytes += line.len();
        if header_bytes > MAX_HEADER_BYTES {
            return Err(invalid_response("OpenCode HTTP headers exceeded the limit"));
        }
        if line == "\r\n" || line == "\n" {
            break;
        }
        let (name, value) = line
            .trim_end_matches(['\r', '\n'])
            .split_once(':')
            .ok_or_else(|| invalid_response("OpenCode HTTP header was malformed"))?;
        headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
    }
    Ok((status, headers))
}

fn remote_http_error(status: u16, body: Value) -> HarnessAdapterError {
    let class = classify_error(Some(status), &body);
    let sanitized = sanitize_native(body);
    HarnessAdapterError::Remote(format!(
        "{} (HTTP {status}): {}",
        class,
        compact_error(&sanitized)
    ))
}

fn remote_native_error(error: &Value) -> HarnessAdapterError {
    let class = classify_error(None, error);
    HarnessAdapterError::Remote(format!(
        "{}: {}",
        class,
        compact_error(&sanitize_native(error.clone()))
    ))
}

fn compact_error(value: &Value) -> String {
    value
        .get("message")
        .and_then(Value::as_str)
        .or_else(|| value.get("name").and_then(Value::as_str))
        .unwrap_or("provider rejected request")
        .chars()
        .take(512)
        .collect()
}

fn classify_error(status: Option<u16>, value: &Value) -> &'static str {
    let name = value.get("name").and_then(Value::as_str).unwrap_or("");
    let message = value.get("message").and_then(Value::as_str).unwrap_or("");
    let combined = format!("{name} {message}").to_ascii_lowercase();
    if matches!(status, Some(401 | 403)) || combined.contains("auth") {
        "authentication"
    } else if status == Some(429) || combined.contains("rate limit") {
        "rate_limit"
    } else if combined.contains("model")
        && (combined.contains("unknown") || combined.contains("not found"))
    {
        "unknown_model"
    } else if combined.contains("reasoning") && combined.contains("unsupported") {
        "unsupported_reasoning"
    } else if matches!(status, Some(408 | 504)) || combined.contains("timeout") {
        "timeout"
    } else {
        "provider_rejection"
    }
}

fn decode_native_error(error: &Value) -> Result<(&'static str, &'static str), HarnessAdapterError> {
    let object = required_object(error, "OpenCode native error")?;
    let name = required_string(object.get("name"), "OpenCode native error name")?;
    let data = required_object_value(object.get("data"), "OpenCode native error data")?;
    required_string(data.get("message"), "OpenCode native error message")?;
    let outcome = match name {
        "MessageAbortedError" => ("aborted", "turn.cancelled"),
        "ProviderAuthError" => {
            required_string(
                data.get("providerID"),
                "OpenCode provider authentication error provider id",
            )?;
            ("authentication", "turn.failed")
        }
        "APIError" => {
            data.get("isRetryable")
                .and_then(Value::as_bool)
                .ok_or_else(|| {
                    invalid_response("OpenCode API error isRetryable must be a boolean")
                })?;
            let status = data
                .get("statusCode")
                .map(|value| {
                    value
                        .as_u64()
                        .filter(|status| *status <= u16::MAX as u64)
                        .ok_or_else(|| {
                            invalid_response(
                                "OpenCode API error statusCode must be a valid HTTP status",
                            )
                        })
                })
                .transpose()?;
            match status {
                Some(401 | 403) => ("authentication", "turn.failed"),
                Some(429) => ("rate_limit", "turn.failed"),
                Some(408 | 504) => ("timeout", "turn.failed"),
                _ => ("provider_rejection", "turn.failed"),
            }
        }
        "ContextOverflowError" => ("context_length", "turn.failed"),
        "MessageOutputLengthError" => ("output_length", "turn.failed"),
        "ContentFilterError" => ("content_filter", "turn.failed"),
        "StructuredOutputError" => ("structured_output", "turn.failed"),
        "UnknownError" => ("provider_rejection", "turn.failed"),
        _ => {
            return Err(invalid_response(format!(
                "OpenCode native error name is unsupported: {name}"
            )));
        }
    };
    Ok(outcome)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodeEventCorrelation {
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: String,
    pub native_session_id: String,
    pub admitted_user_message_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenCodeDecodedEvent {
    pub event: HarnessEvent,
    pub final_text: Option<String>,
    pub usage: Option<Value>,
    pub error_class: Option<String>,
    pub terminal: bool,
}

#[derive(Clone, Debug)]
pub struct OpenCodeEventDecoder {
    correlation: OpenCodeEventCorrelation,
    assistant_message_id: Option<String>,
    secret_values: Vec<String>,
    text: String,
    terminal_emitted: bool,
}

impl OpenCodeEventDecoder {
    pub fn new(correlation: OpenCodeEventCorrelation) -> Self {
        Self::new_with_secrets(correlation, std::iter::empty::<String>())
    }

    pub fn new_with_secrets<I>(correlation: OpenCodeEventCorrelation, secret_values: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        Self {
            correlation,
            assistant_message_id: None,
            secret_values: secret_values
                .into_iter()
                .filter(|value| !value.is_empty())
                .collect(),
            text: String::new(),
            terminal_emitted: false,
        }
    }

    pub fn decode(
        &mut self,
        native: Value,
    ) -> Result<Option<OpenCodeDecodedEvent>, HarnessAdapterError> {
        let object = required_object(&native, "OpenCode event")?;
        let event_id = required_string(object.get("id"), "OpenCode event id")?.to_owned();
        let event_type = required_string(object.get("type"), "OpenCode event type")?;
        let properties =
            required_object_value(object.get("properties"), "OpenCode event properties")?;

        let mut final_text = None;
        let mut usage = None;
        let mut error_class = None;
        let mut terminal = false;
        let mut data = Map::new();
        let (canonical_type, durable) = match event_type {
            "server.connected" => ("harness.connected", false),
            "message.part.delta" => {
                if !self.matches_session(properties)? {
                    return Ok(None);
                }
                let message_id =
                    required_string(properties.get("messageID"), "OpenCode delta message id")?;
                if self.assistant_message_id.as_deref() != Some(message_id) {
                    return Ok(None);
                }
                required_string(properties.get("partID"), "OpenCode delta part id")?;
                let field = required_string(properties.get("field"), "OpenCode delta field")?;
                let delta = required_string_allow_empty(
                    properties.get("delta"),
                    "OpenCode message part delta",
                )?;
                data.insert("content".to_owned(), Value::String(delta.to_owned()));
                match field {
                    "text" => {
                        self.text.push_str(delta);
                        ("assistant.text_chunk", false)
                    }
                    "reasoning" => ("assistant.reasoning", false),
                    _ => ("native.opencode.event", false),
                }
            }
            "message.part.updated" => {
                if !self.matches_session(properties)? {
                    return Ok(None);
                }
                let part =
                    required_object_value(properties.get("part"), "OpenCode updated message part")?;
                let message_id =
                    required_string(part.get("messageID"), "OpenCode updated part message id")?;
                if self.assistant_message_id.as_deref() != Some(message_id) {
                    return Ok(None);
                }
                let part_type = required_string(part.get("type"), "OpenCode message part type")?;
                match part_type {
                    "text" => {
                        let content = required_string_allow_empty(
                            part.get("text"),
                            "OpenCode updated text part",
                        )?;
                        data.insert("content".to_owned(), Value::String(content.to_owned()));
                        ("assistant.text_snapshot", false)
                    }
                    "reasoning" => {
                        required_string_allow_empty(
                            part.get("text"),
                            "OpenCode updated reasoning part",
                        )?;
                        ("assistant.reasoning", false)
                    }
                    "tool" => ("tool.state_changed", true),
                    _ => ("native.opencode.event", false),
                }
            }
            "message.updated" => {
                if !self.matches_session(properties)? {
                    return Ok(None);
                }
                let info =
                    required_object_value(properties.get("info"), "OpenCode updated message info")?;
                let role = required_string(info.get("role"), "OpenCode updated message role")?;
                if role != "assistant" {
                    let message_id =
                        required_string(info.get("id"), "OpenCode updated user message id")?;
                    if self.correlation.admitted_user_message_id.as_deref() != Some(message_id) {
                        return Ok(None);
                    }
                    ("conversation.message_updated", true)
                } else {
                    let message_id =
                        required_string(info.get("id"), "OpenCode assistant message id")?;
                    let parent_id =
                        required_string(info.get("parentID"), "OpenCode assistant parent id")?;
                    if self.correlation.admitted_user_message_id.as_deref() != Some(parent_id) {
                        return Ok(None);
                    }
                    if let Some(active_message_id) = self.assistant_message_id.as_deref()
                        && active_message_id != message_id
                    {
                        return Ok(None);
                    }
                    self.assistant_message_id = Some(message_id.to_owned());
                    if let Some(error) = info.get("error").filter(|value| !value.is_null()) {
                        let (class, event_type) = decode_native_error(error)?;
                        error_class = Some(class.to_owned());
                        if !self.terminal_emitted {
                            self.terminal_emitted = true;
                            terminal = true;
                        }
                        (event_type, true)
                    } else {
                        usage = Some(decode_usage(info)?);
                        let time = required_object_value(
                            info.get("time"),
                            "OpenCode assistant message time",
                        )?;
                        let completed = time.get("completed");
                        let finish = info.get("finish");
                        match (completed, finish) {
                            (None, None) => ("assistant.message_updated", true),
                            (Some(completed), Some(finish)) => {
                                required_number(
                                    Some(completed),
                                    "OpenCode assistant completion time",
                                )?;
                                required_string(Some(finish), "OpenCode assistant finish reason")?;
                                final_text = (!self.text.is_empty()).then(|| self.text.clone());
                                if !self.terminal_emitted {
                                    self.terminal_emitted = true;
                                    terminal = true;
                                }
                                ("assistant.final", true)
                            }
                            _ => {
                                return Err(invalid_response(
                                    "OpenCode assistant completion requires both time.completed and finish",
                                ));
                            }
                        }
                    }
                }
            }
            "session.idle" => {
                if !self.matches_session(properties)? {
                    return Ok(None);
                }
                if self.terminal_emitted {
                    ("native.opencode.event", true)
                } else {
                    self.terminal_emitted = true;
                    terminal = true;
                    ("turn.completed", true)
                }
            }
            "session.error" => {
                if !self.matches_session(properties)? {
                    return Ok(None);
                }
                let error =
                    required_object_value(properties.get("error"), "OpenCode session error")?;
                let (class, event_type) = decode_native_error(&Value::Object(error.clone()))?;
                error_class = Some(class.to_owned());
                if !self.terminal_emitted {
                    self.terminal_emitted = true;
                    terminal = true;
                }
                (event_type, true)
            }
            "session.status" => {
                if !self.matches_session(properties)? {
                    return Ok(None);
                }
                let status =
                    required_object_value(properties.get("status"), "OpenCode session status")?;
                match required_string(status.get("type"), "OpenCode session status type")? {
                    "busy" | "retry" => ("turn.started", true),
                    "idle" => ("session.idle", true),
                    _ => ("session.state_changed", true),
                }
            }
            _ => ("native.opencode.event", false),
        };

        data.insert(
            "session_id".to_owned(),
            Value::String(self.correlation.session_id.to_string()),
        );
        data.insert(
            "task_id".to_owned(),
            self.correlation
                .task_id
                .map(|id| Value::String(id.to_string()))
                .unwrap_or(Value::Null),
        );
        data.insert(
            "turn_id".to_owned(),
            self.correlation
                .turn_id
                .map(|id| Value::String(id.to_string()))
                .unwrap_or(Value::Null),
        );
        data.insert(
            "attempt_id".to_owned(),
            Value::String(self.correlation.attempt_id.clone()),
        );
        data.insert(
            "native_session_id".to_owned(),
            Value::String(self.correlation.native_session_id.clone()),
        );
        data.insert(
            "native".to_owned(),
            sanitize_native_with_secrets(native, &self.secret_values),
        );
        Ok(Some(OpenCodeDecodedEvent {
            event: HarnessEvent {
                event_type: canonical_type.to_owned(),
                durable,
                payload: Value::Object(data),
                native_cursor: Some(event_id),
            },
            final_text,
            usage,
            error_class,
            terminal,
        }))
    }

    fn matches_session(
        &self,
        properties: &Map<String, Value>,
    ) -> Result<bool, HarnessAdapterError> {
        let session_id = required_string(
            properties.get("sessionID"),
            "OpenCode event native session id",
        )?;
        Ok(session_id == self.correlation.native_session_id)
    }
}

pub struct OpenCodeEventStream {
    reader: BufReader<TcpStream>,
    decoder: OpenCodeEventDecoder,
    event_timeout: Duration,
    turn: Option<ActiveTurnLease>,
    address: SocketAddr,
    native_session_id: String,
    cancellation_sent: bool,
}

impl OpenCodeEventStream {
    pub async fn cancel(&mut self) -> Result<(), HarnessAdapterError> {
        if !self.cancellation_sent {
            abort_at(self.address, self.event_timeout, &self.native_session_id).await?;
            self.cancellation_sent = true;
        }
        self.turn.take();
        Ok(())
    }

    pub async fn next_event(&mut self) -> Result<OpenCodeDecodedEvent, HarnessAdapterError> {
        let event = match timeout(self.event_timeout, self.next_event_inner()).await {
            Ok(result) => result?,
            Err(_) => {
                let _ = self.cancel().await;
                return Err(HarnessAdapterError::Timeout);
            }
        };
        if event.terminal {
            self.turn.take();
        }
        Ok(event)
    }

    async fn next_event_inner(&mut self) -> Result<OpenCodeDecodedEvent, HarnessAdapterError> {
        loop {
            let native = read_sse_event(&mut self.reader).await?;
            if let Some(event) = self.decoder.decode(native)? {
                return Ok(event);
            }
        }
    }
}

impl Drop for OpenCodeEventStream {
    fn drop(&mut self) {
        if self.turn.is_none() || self.cancellation_sent {
            return;
        }
        self.cancellation_sent = true;
        self.turn.take();
        let address = self.address;
        let request_timeout = self.event_timeout;
        let native_session_id = self.native_session_id.clone();
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(async move {
                let _ = abort_at(address, request_timeout, &native_session_id).await;
            });
        }
    }
}

async fn abort_at(
    address: SocketAddr,
    request_timeout: Duration,
    native_session_id: &str,
) -> Result<(), HarnessAdapterError> {
    let response = timeout(
        request_timeout,
        raw_http_request(
            address,
            "POST",
            &format!("/session/{native_session_id}/abort"),
            None,
        ),
    )
    .await
    .map_err(|_| HarnessAdapterError::Timeout)??;
    if response.status != 200
        || serde_json::from_slice::<Value>(&response.body)
            .ok()
            .as_ref()
            != Some(&Value::Bool(true))
    {
        return Err(invalid_response("OpenCode abort response must be true"));
    }
    Ok(())
}

struct ActiveTurnLease {
    active_turns: Arc<Mutex<HashMap<String, String>>>,
    native_session_id: String,
    message_id: String,
}

impl ActiveTurnLease {
    fn acquire(
        active_turns: Arc<Mutex<HashMap<String, String>>>,
        native_session_id: String,
        message_id: String,
    ) -> Result<Self, HarnessAdapterError> {
        let mut turns = active_turns.lock().map_err(|_| {
            HarnessAdapterError::InvalidRequest(
                "OpenCode active-turn registry is poisoned".to_owned(),
            )
        })?;
        if turns.contains_key(&native_session_id) {
            return Err(HarnessAdapterError::InvalidRequest(format!(
                "OpenCode session {native_session_id} already has an active turn"
            )));
        }
        turns.insert(native_session_id.clone(), message_id.clone());
        drop(turns);
        Ok(Self {
            active_turns,
            native_session_id,
            message_id,
        })
    }
}

impl Drop for ActiveTurnLease {
    fn drop(&mut self) {
        if let Ok(mut turns) = self.active_turns.lock()
            && turns.get(&self.native_session_id) == Some(&self.message_id)
        {
            turns.remove(&self.native_session_id);
        }
    }
}

async fn read_sse_event(reader: &mut BufReader<TcpStream>) -> Result<Value, HarnessAdapterError> {
    let mut data = String::new();
    loop {
        let Some(line) = read_bounded_sse_line(reader).await? else {
            return Err(HarnessAdapterError::ProcessExited);
        };
        if line == b"\r\n" || line == b"\n" {
            if data.is_empty() {
                continue;
            }
            break;
        }
        let line = std::str::from_utf8(&line)
            .map_err(|_| invalid_response("OpenCode SSE line was not valid UTF-8"))?
            .trim_end_matches(['\r', '\n']);
        if line.starts_with(':') {
            continue;
        }
        if let Some(value) = line.strip_prefix("data:") {
            let value = value.strip_prefix(' ').unwrap_or(value);
            let separator_bytes = usize::from(!data.is_empty());
            if data.len() + separator_bytes + value.len() > MAX_SSE_EVENT_BYTES {
                return Err(invalid_response(
                    "OpenCode SSE event exceeded the byte limit",
                ));
            }
            if !data.is_empty() {
                data.push('\n');
            }
            data.push_str(value);
        }
    }
    serde_json::from_str(&data)
        .map_err(|_| invalid_response("OpenCode SSE data was not valid JSON"))
}

async fn read_bounded_sse_line(
    reader: &mut BufReader<TcpStream>,
) -> Result<Option<Vec<u8>>, HarnessAdapterError> {
    let mut line = Vec::new();
    loop {
        let available = reader
            .fill_buf()
            .await
            .map_err(|_| HarnessAdapterError::ProcessExited)?;
        if available.is_empty() {
            return Ok((!line.is_empty()).then_some(line));
        }
        let newline = available.iter().position(|byte| *byte == b'\n');
        let consumed = newline.map_or(available.len(), |index| index + 1);
        if line.len() + consumed > MAX_SSE_LINE_BYTES {
            return Err(invalid_response(
                "OpenCode SSE line exceeded the byte limit",
            ));
        }
        line.extend_from_slice(&available[..consumed]);
        reader.consume(consumed);
        if newline.is_some() {
            return Ok(Some(line));
        }
    }
}

fn sanitize_native(value: Value) -> Value {
    sanitize_native_with_secrets(value, &[])
}

fn sanitize_native_with_secrets(value: Value, secret_values: &[String]) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .into_iter()
                .map(|(key, value)| {
                    let value = if key.eq_ignore_ascii_case("error") {
                        sanitize_error_value(value, secret_values)
                    } else if sensitive_key(&key) {
                        Value::String("<redacted>".to_owned())
                    } else {
                        sanitize_native_with_secrets(value, secret_values)
                    };
                    (key, value)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(|value| sanitize_native_with_secrets(value, secret_values))
                .collect(),
        ),
        Value::String(mut value) => {
            for secret in secret_values {
                value = value.replace(secret, "<redacted>");
            }
            Value::String(value)
        }
        value => value,
    }
}

fn sanitize_error_value(value: Value, secret_values: &[String]) -> Value {
    let Some(error) = value.as_object() else {
        return Value::String("<redacted>".to_owned());
    };
    let mut safe = Map::new();
    if let Some(Value::String(name)) = error.get("name") {
        safe.insert("name".to_owned(), Value::String(name.clone()));
    }
    if let Some(Value::Object(data)) = error.get("data") {
        let mut safe_data = Map::new();
        for key in ["providerID", "message", "statusCode", "isRetryable", "ref"] {
            if let Some(value) = data.get(key) {
                safe_data.insert(
                    key.to_owned(),
                    sanitize_native_with_secrets(value.clone(), secret_values),
                );
            }
        }
        safe.insert("data".to_owned(), Value::Object(safe_data));
    }
    Value::Object(safe)
}

fn sensitive_key(key: &str) -> bool {
    let normalized = key
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    matches!(
        normalized.as_str(),
        "apikey"
            | "authorization"
            | "credential"
            | "credentials"
            | "password"
            | "secret"
            | "token"
            | "accesstoken"
            | "refreshtoken"
            | "idtoken"
            | "bearertoken"
    )
}

fn required_object<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a Map<String, Value>, HarnessAdapterError> {
    value
        .as_object()
        .ok_or_else(|| invalid_response(format!("{field} must be an object")))
}

fn required_object_value<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a Map<String, Value>, HarnessAdapterError> {
    value
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_response(format!("{field} must be an object")))
}

fn required_string<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a str, HarnessAdapterError> {
    value
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| invalid_response(format!("{field} must be a non-empty string")))
}

fn required_string_allow_empty<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a str, HarnessAdapterError> {
    value
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_response(format!("{field} must be a string")))
}

fn required_number(value: Option<&Value>, field: &str) -> Result<f64, HarnessAdapterError> {
    value
        .and_then(Value::as_f64)
        .ok_or_else(|| invalid_response(format!("{field} must be a number")))
}

fn invalid_response(message: impl Into<String>) -> HarnessAdapterError {
    HarnessAdapterError::InvalidResponse(message.into())
}
