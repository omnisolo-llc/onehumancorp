pub mod local_services;
mod protected_process;

use std::collections::BTreeSet;
use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{
    Router,
    http::{StatusCode, Uri},
    routing::get,
};
use server_harness::middleware::grpc::HarnessWorkerGrpcService;
use server_harness::middleware::harness::{HarnessProtocolKind, ProcessHarnessSpec};
#[cfg(test)]
use server_harness::middleware::provider_facade::ProviderFacade;
use server_harness::middleware::provider_facade::ProviderFacadeConfig;
use server_harness::middleware::types::{ModelApiDialect, ReasoningEffort, ResolvedModelSelection};
use server_omnisolo::harness_middleware::harness_worker_service_server::HarnessWorkerServiceServer;
use tokio::net::TcpListener;
use tonic::transport::Server;

pub const OPENAI_API_KEY: &str = "OPENAI_API_KEY";
pub const OPENAI_API_BASE_URL: &str = "OPENAI_API_BASE_URL";
pub const OPENAI_MODEL: &str = "OPENAI_MODEL";
pub const OPENAI_REASONING_EFFORT: &str = "OPENAI_REASONING_EFFORT";

const DEFAULT_MODEL: &str = "gpt-5.6-luna";
const DEFAULT_OPENAI_API_BASE_URL: &str = "https://api.openai.com/v1";
const DEPRECATED_API_KEY: &str = "OMNISOLO_HARNESS_API_KEY";
const DEPRECATED_BASE_URL: &str = "OMNISOLO_HARNESS_BASE_URL";

#[derive(Clone, Default, PartialEq, Eq)]
pub struct WorkerConfigInput {
    pub worker_id: String,
    pub harness_id: String,
    pub pool_id: String,
    pub grpc_addr: String,
    pub health_addr: String,
    pub executable: Option<String>,
    pub args_json: Option<String>,
    pub protocol: Option<String>,
    pub timeout_secs: Option<String>,
    pub external_sandbox: bool,
    pub openai_api_key: Option<String>,
    pub openai_api_base_url: Option<String>,
    pub openai_model: Option<String>,
    pub openai_reasoning_effort: Option<String>,
    pub deprecated_api_key: Option<String>,
    pub deprecated_base_url: Option<String>,
}

impl WorkerConfigInput {
    fn from_env() -> Self {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    fn from_lookup(mut lookup: impl FnMut(&str) -> Option<String>) -> Self {
        Self {
            worker_id: lookup("OMNISOLO_HARNESS_WORKER_ID").unwrap_or_default(),
            harness_id: lookup("OMNISOLO_HARNESS_ID").unwrap_or_else(|| "omnisolo".to_owned()),
            pool_id: lookup("OMNISOLO_HARNESS_POOL_ID").unwrap_or_else(|| "omnisolo".to_owned()),
            grpc_addr: lookup("OMNISOLO_HARNESS_GRPC_ADDR")
                .unwrap_or_else(|| "0.0.0.0:8090".to_owned()),
            health_addr: lookup("OMNISOLO_HARNESS_HEALTH_ADDR")
                .unwrap_or_else(|| "0.0.0.0:8091".to_owned()),
            executable: lookup("OMNISOLO_HARNESS_EXECUTABLE"),
            args_json: lookup("OMNISOLO_HARNESS_ARGS_JSON"),
            protocol: lookup("OMNISOLO_HARNESS_PROTOCOL"),
            timeout_secs: lookup("OMNISOLO_HARNESS_REQUEST_TIMEOUT_SECS"),
            external_sandbox: lookup("OMNISOLO_HARNESS_EXTERNAL_SANDBOX").as_deref() == Some("1"),
            openai_api_key: lookup(OPENAI_API_KEY),
            openai_api_base_url: lookup(OPENAI_API_BASE_URL),
            openai_model: lookup(OPENAI_MODEL),
            openai_reasoning_effort: lookup(OPENAI_REASONING_EFFORT),
            deprecated_api_key: lookup(DEPRECATED_API_KEY),
            deprecated_base_url: lookup(DEPRECATED_BASE_URL),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorkerConfig {
    pub worker_id: String,
    pub harness_id: String,
    pub pool_id: String,
    pub grpc_addr: SocketAddr,
    pub health_addr: SocketAddr,
    pub default_resolved_model: Option<ResolvedModelSelection>,
    pub api_base_url: Option<String>,
    pub process_spec: Option<ProcessHarnessSpec>,
    pub request_timeout: Duration,
    provider_api_key: Option<SecretValue>,
}

#[derive(Clone, PartialEq, Eq)]
struct SecretValue(String);

impl SecretValue {
    fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SecretValue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}

#[derive(Clone)]
struct WorkerModelRouting {
    api_key: Option<String>,
    api_base_url: Option<String>,
    resolved_model: ResolvedModelSelection,
}

#[derive(Debug)]
pub enum WorkerConfigError {
    EmptyIdentity(&'static str),
    InvalidAddress {
        field: &'static str,
        value: String,
        source: std::net::AddrParseError,
    },
    AddressConflict {
        grpc_addr: SocketAddr,
        health_addr: SocketAddr,
    },
    InvalidArguments(String),
    InvalidProtocol(String),
    InvalidTimeout(String),
    InvalidBaseUrl,
    InvalidReasoningEffort(String),
}

impl std::fmt::Display for WorkerConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyIdentity(field) => write!(formatter, "{field} must not be empty"),
            Self::InvalidAddress { field, value, .. } => {
                write!(formatter, "{field} is not a socket address: {value}")
            }
            Self::AddressConflict {
                grpc_addr,
                health_addr,
            } => write!(
                formatter,
                "gRPC and health addresses must differ: {grpc_addr} = {health_addr}"
            ),
            Self::InvalidArguments(error) => {
                write!(formatter, "invalid harness argument JSON: {error}")
            }
            Self::InvalidProtocol(value) => {
                write!(formatter, "unsupported harness protocol: {value}")
            }
            Self::InvalidTimeout(value) => {
                write!(formatter, "invalid harness request timeout: {value}")
            }
            Self::InvalidBaseUrl => formatter.write_str(
                "OPENAI_API_BASE_URL must be an absolute http(s) URL without credentials",
            ),
            Self::InvalidReasoningEffort(value) => write!(
                formatter,
                "OPENAI_REASONING_EFFORT must be one of none, minimal, low, medium, high, max: {value}"
            ),
        }
    }
}

impl std::error::Error for WorkerConfigError {}

impl WorkerConfig {
    pub fn from_values(
        worker_id: impl Into<String>,
        harness_id: impl Into<String>,
        pool_id: impl Into<String>,
        grpc_addr: impl Into<String>,
        health_addr: impl Into<String>,
    ) -> Result<Self, WorkerConfigError> {
        let worker_id = required(worker_id.into(), "worker_id")?;
        let harness_id = required(harness_id.into(), "harness_id")?;
        let pool_id = required(pool_id.into(), "pool_id")?;
        let grpc_addr = parse_addr(grpc_addr.into(), "grpc_addr")?;
        let health_addr = parse_addr(health_addr.into(), "health_addr")?;
        if grpc_addr == health_addr {
            return Err(WorkerConfigError::AddressConflict {
                grpc_addr,
                health_addr,
            });
        }
        Ok(Self {
            worker_id,
            harness_id,
            pool_id,
            grpc_addr,
            health_addr,
            default_resolved_model: None,
            api_base_url: None,
            process_spec: None,
            request_timeout: Duration::from_secs(30),
            provider_api_key: None,
        })
    }

    pub fn from_env() -> Result<Self, WorkerConfigError> {
        Self::from_input(WorkerConfigInput::from_env())
    }

    pub fn from_input(input: WorkerConfigInput) -> Result<Self, WorkerConfigError> {
        let mut config = Self::from_values(
            input.worker_id.clone(),
            input.harness_id.clone(),
            input.pool_id.clone(),
            input.grpc_addr.clone(),
            input.health_addr.clone(),
        )?;
        let routing = model_routing_from_input(&input)?;
        config.default_resolved_model = Some(routing.resolved_model.clone());
        config.api_base_url = routing.api_base_url.clone();
        config.provider_api_key = routing.api_key.clone().map(SecretValue);
        config.request_timeout = parse_request_timeout(input.timeout_secs.as_deref())?;
        config.process_spec = process_spec_from_input(&input, routing)?;
        if input.external_sandbox
            && let Some(spec) = config.process_spec.as_mut()
        {
            spec.environment
                .insert("OMNISOLO_HARNESS_EXTERNAL_SANDBOX".into(), "1".into());
        }
        Ok(config)
    }
}

pub async fn run(config: WorkerConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_until_shutdown(config, shutdown_signal()).await
}

// Tonic interceptors must return the generated service's Status value directly.
#[allow(clippy::result_large_err)]
pub async fn run_until_shutdown(
    config: WorkerConfig,
    shutdown: impl Future<Output = ()> + Send + 'static,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut service = match config.process_spec.clone() {
        Some(spec) => HarnessWorkerGrpcService::with_process_spec_for_worker(
            config.worker_id.clone(),
            config.pool_id.clone(),
            spec,
        ),
        None => HarnessWorkerGrpcService::new(
            config.worker_id.clone(),
            config.harness_id.clone(),
            config.pool_id.clone(),
        ),
    };
    if let Some(api_key) = config.provider_api_key.as_ref() {
        protected_process::protect_credentials()?;
        let selection = match config.default_resolved_model.clone() {
            Some(selection) => selection,
            None => {
                resolve_model_routing(None, config.api_base_url.clone(), None, None, None, None)?
                    .resolved_model
            }
        };
        service = service.with_provider_facade(
            ProviderFacadeConfig::new(
                config
                    .api_base_url
                    .as_deref()
                    .unwrap_or(DEFAULT_OPENAI_API_BASE_URL),
                api_key.expose(),
                selection,
            )
            .with_timeout(config.request_timeout),
        );
    }
    if let Some(default_resolved_model) = config.default_resolved_model.clone() {
        service = service.with_default_resolved_model(default_resolved_model);
    }
    let (service, control_token) =
        local_services::configure_worker(service, config.provider_api_key.is_some()).await?;
    service.preflight().await?;
    let shutdown_service = service.clone();
    let health_service = service.clone();
    let ready_service = service.clone();
    let health_app = Router::new()
        .route(
            "/healthz",
            get(move || {
                let health_service = health_service.clone();
                async move { health_response(health_service.health_snapshot().is_ok()) }
            }),
        )
        .route(
            "/readyz",
            get(move || {
                let service = ready_service.clone();
                async move {
                    readiness_response(
                        service
                            .health_snapshot()
                            .map(|health| (health.ready, health.accepting_new_attempts)),
                    )
                }
            }),
        );
    let health_listener = TcpListener::bind(config.health_addr).await?;
    let grpc_server = Server::builder().add_service(HarnessWorkerServiceServer::with_interceptor(
        service,
        move |request: tonic::Request<()>| {
            if let Some(expected) = control_token.as_ref() {
                let supplied = request
                    .metadata()
                    .get("authorization")
                    .and_then(|value| value.to_str().ok())
                    .and_then(|value| value.strip_prefix("Bearer "));
                let valid = supplied.is_some_and(|supplied| {
                    supplied.len() == expected.len()
                        && supplied
                            .bytes()
                            .zip(expected.bytes())
                            .fold(0_u8, |difference, (a, b)| difference | (a ^ b))
                            == 0
                });
                if !valid {
                    return Err(tonic::Status::unauthenticated(
                        "worker control credential required",
                    ));
                }
            }
            Ok(request)
        },
    ));
    let (shutdown_sender, shutdown_receiver) = tokio::sync::watch::channel(false);
    let authority_shutdown = shutdown_service.clone();
    let shutdown_task = tokio::spawn(async move {
        shutdown.await;
        authority_shutdown.revoke_provider_routes();
        let _ = shutdown_sender.send(true);
    });
    let grpc_shutdown = shutdown_receiver.clone();
    let health_shutdown = shutdown_receiver;
    let grpc = async move {
        grpc_server
            .serve_with_shutdown(config.grpc_addr, wait_for_shutdown(grpc_shutdown))
            .await
            .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { Box::new(error) })
    };
    let health = async move {
        axum::serve(health_listener, health_app)
            .with_graceful_shutdown(wait_for_shutdown(health_shutdown))
            .await
            .map_err(|error| -> Box<dyn std::error::Error + Send + Sync> { Box::new(error) })
    };
    let result = tokio::try_join!(grpc, health);
    shutdown_task.abort();
    shutdown_service.revoke_provider_routes();
    result?;
    Ok(())
}

async fn wait_for_shutdown(mut receiver: tokio::sync::watch::Receiver<bool>) {
    if *receiver.borrow() {
        return;
    }
    while receiver.changed().await.is_ok() {
        if *receiver.borrow() {
            return;
        }
    }
}

fn health_response(healthy: bool) -> (StatusCode, &'static str) {
    if healthy {
        (StatusCode::OK, "ok")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "unavailable")
    }
}

fn readiness_response<E>(health: Result<(bool, bool), E>) -> (StatusCode, &'static str) {
    match health {
        Ok((true, true)) => (StatusCode::OK, "ready"),
        Ok(_) => (StatusCode::SERVICE_UNAVAILABLE, "draining"),
        Err(_) => (StatusCode::SERVICE_UNAVAILABLE, "unavailable"),
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    {
        let terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .ok()
            .map(|mut signal| async move {
                let _ = signal.recv().await;
            });
        wait_for_process_signal(ctrl_c, terminate).await;
    }
    #[cfg(not(unix))]
    wait_for_process_signal(ctrl_c, None::<std::future::Ready<()>>).await;
}

async fn wait_for_process_signal<C, T>(ctrl_c: C, terminate: Option<T>)
where
    C: Future<Output = ()>,
    T: Future<Output = ()>,
{
    if let Some(terminate) = terminate {
        tokio::select! {
            () = ctrl_c => {}
            () = terminate => {}
        }
    } else {
        ctrl_c.await;
    }
}

fn required(value: String, field: &'static str) -> Result<String, WorkerConfigError> {
    if value.trim().is_empty() {
        Err(WorkerConfigError::EmptyIdentity(field))
    } else {
        Ok(value)
    }
}

fn parse_addr(value: String, field: &'static str) -> Result<SocketAddr, WorkerConfigError> {
    value
        .parse()
        .map_err(|source| WorkerConfigError::InvalidAddress {
            field,
            value,
            source,
        })
}

#[cfg(test)]
fn process_spec_from_env(
    harness_id: &str,
    executable: Option<String>,
    args_json: Option<String>,
    protocol: Option<String>,
    timeout_secs: Option<String>,
    api_key: Option<String>,
    base_url: Option<String>,
) -> Result<Option<ProcessHarnessSpec>, WorkerConfigError> {
    let routing = resolve_model_routing(api_key, base_url, None, None, None, None)?;
    build_process_spec(
        harness_id,
        executable,
        args_json,
        protocol,
        timeout_secs,
        routing,
    )
}

fn process_spec_from_input(
    input: &WorkerConfigInput,
    routing: WorkerModelRouting,
) -> Result<Option<ProcessHarnessSpec>, WorkerConfigError> {
    build_process_spec(
        &input.harness_id,
        input.executable.clone(),
        input.args_json.clone(),
        input.protocol.clone(),
        input.timeout_secs.clone(),
        routing,
    )
}

fn parse_request_timeout(value: Option<&str>) -> Result<Duration, WorkerConfigError> {
    match value.map(str::trim).filter(|value| !value.is_empty()) {
        Some(value) => value
            .parse::<u64>()
            .ok()
            .filter(|seconds| *seconds > 0)
            .map(Duration::from_secs)
            .ok_or_else(|| WorkerConfigError::InvalidTimeout(value.to_owned())),
        None => Ok(Duration::from_secs(30)),
    }
}

fn build_process_spec(
    harness_id: &str,
    executable: Option<String>,
    args_json: Option<String>,
    protocol: Option<String>,
    timeout_secs: Option<String>,
    routing: WorkerModelRouting,
) -> Result<Option<ProcessHarnessSpec>, WorkerConfigError> {
    let Some(executable) = executable.filter(|value| !value.trim().is_empty()) else {
        return Ok(None);
    };
    let args = match args_json {
        Some(value) if !value.trim().is_empty() => serde_json::from_str::<Vec<String>>(&value)
            .map_err(|error| WorkerConfigError::InvalidArguments(error.to_string()))?,
        _ => Vec::new(),
    };
    let protocol_kind = match protocol {
        Some(value) if !value.trim().is_empty() => parse_protocol(&value)?,
        _ => {
            server_harness::middleware::harness::ExternalHarnessPreset::from_harness_id(harness_id)
                .map(|preset| preset.protocol_kind())
                .unwrap_or(HarnessProtocolKind::Custom)
        }
    };
    let timeout = parse_request_timeout(timeout_secs.as_deref())?;
    let model_id = routing.resolved_model.model_id.clone();
    let reasoning_effort = routing
        .resolved_model
        .reasoning_effort
        .as_ref()
        .map(reasoning_effort_env_name)
        .map(str::to_owned);
    let direct_api_base_url = routing
        .api_key
        .is_none()
        .then(|| routing.api_base_url.clone())
        .flatten();
    let mut spec = ProcessHarnessSpec::command(executable, args, harness_id)
        .with_protocol(protocol_kind.clone())
        .with_timeout(timeout)
        .with_model_routing(routing.resolved_model, direct_api_base_url.clone());
    if let Some(base_url) = direct_api_base_url.as_ref() {
        spec = spec.with_environment(OPENAI_API_BASE_URL, base_url.clone());
        if matches!(protocol_kind, HarnessProtocolKind::KimiAcp) {
            spec = spec.with_environment("OPENAI_BASE_URL", base_url.clone());
        }
    }
    if harness_id == "plandex" && protocol_kind == HarnessProtocolKind::OpenAiCompatibleShim {
        // The native server shares the worker's network namespace. Credentials
        // belong to that server; only its fixed loopback endpoint reaches the CLI.
        spec = spec
            .with_environment("PLANDEX_ENV", "development")
            .with_environment("PLANDEX_API_HOST", "http://127.0.0.1:8099");
    }
    spec = spec.with_environment(OPENAI_MODEL, model_id);
    if let Some(reasoning_effort) = reasoning_effort {
        spec = spec.with_environment(OPENAI_REASONING_EFFORT, reasoning_effort);
    }
    if matches!(protocol_kind, HarnessProtocolKind::CodexAppServer)
        && let Some(base_url) = direct_api_base_url
    {
        spec.args = codex_provider_args(spec.args, &base_url);
    }
    Ok(Some(spec))
}

fn reasoning_effort_env_name(effort: &ReasoningEffort) -> &'static str {
    match effort {
        ReasoningEffort::None => "none",
        ReasoningEffort::Minimal => "minimal",
        ReasoningEffort::Low => "low",
        ReasoningEffort::Medium => "medium",
        ReasoningEffort::High => "high",
        ReasoningEffort::Max => "max",
        ReasoningEffort::Custom => "custom",
    }
}

fn model_routing_from_input(
    input: &WorkerConfigInput,
) -> Result<WorkerModelRouting, WorkerConfigError> {
    resolve_model_routing(
        input.openai_api_key.clone(),
        input.openai_api_base_url.clone(),
        input.openai_model.clone(),
        input.openai_reasoning_effort.clone(),
        input.deprecated_api_key.clone(),
        input.deprecated_base_url.clone(),
    )
}

fn resolve_model_routing(
    api_key: Option<String>,
    base_url: Option<String>,
    model: Option<String>,
    reasoning_effort: Option<String>,
    deprecated_api_key: Option<String>,
    deprecated_base_url: Option<String>,
) -> Result<WorkerModelRouting, WorkerConfigError> {
    let api_key = preferred_non_empty(api_key, deprecated_api_key);
    let api_base_url = preferred_non_empty(base_url, deprecated_base_url)
        .map(validate_base_url)
        .transpose()?;
    let model_id = trimmed_non_empty(model).unwrap_or_else(|| DEFAULT_MODEL.to_owned());
    let reasoning_effort =
        parse_reasoning_effort(non_empty(reasoning_effort).as_deref().unwrap_or("max"))?;
    let provider_route = if api_base_url.is_some() {
        "openai-compatible"
    } else {
        "openai"
    };
    let resolved_model = ResolvedModelSelection {
        provider_route: provider_route.to_owned(),
        model_id,
        reasoning_effort: Some(reasoning_effort),
        api_dialect: ModelApiDialect::OpenAiResponses,
        context_window: None,
        max_output_tokens: None,
        capabilities: BTreeSet::from(["reasoning".to_owned(), "tools".to_owned()]),
        binding_revision: "worker-env-v1".to_owned(),
        binding_digest: "sha256:runtime-unbound".to_owned(),
        metadata: Default::default(),
    };
    Ok(WorkerModelRouting {
        api_key,
        api_base_url,
        resolved_model,
    })
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.trim().is_empty())
}

fn trimmed_non_empty(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_owned())
    })
}

fn preferred_non_empty(primary: Option<String>, fallback: Option<String>) -> Option<String> {
    non_empty(primary).or_else(|| non_empty(fallback))
}

fn validate_base_url(value: String) -> Result<String, WorkerConfigError> {
    let trimmed = value.trim();
    let uri = trimmed
        .parse::<Uri>()
        .map_err(|_| WorkerConfigError::InvalidBaseUrl)?;
    let valid_scheme = matches!(uri.scheme_str(), Some("http" | "https"));
    let valid_authority = uri
        .authority()
        .is_some_and(|authority| !authority.as_str().contains('@'));
    if !valid_scheme || !valid_authority || uri.query().is_some() {
        return Err(WorkerConfigError::InvalidBaseUrl);
    }
    Ok(trimmed.to_owned())
}

fn parse_reasoning_effort(value: &str) -> Result<ReasoningEffort, WorkerConfigError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "none" => Ok(ReasoningEffort::None),
        "minimal" => Ok(ReasoningEffort::Minimal),
        "low" => Ok(ReasoningEffort::Low),
        "medium" => Ok(ReasoningEffort::Medium),
        "high" => Ok(ReasoningEffort::High),
        "max" => Ok(ReasoningEffort::Max),
        _ => Err(WorkerConfigError::InvalidReasoningEffort(value.to_owned())),
    }
}

#[cfg(test)]
fn optional_env(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

fn codex_provider_args(mut args: Vec<String>, base_url: &str) -> Vec<String> {
    let mut overrides = vec![
        "-c".to_owned(),
        "model_provider=\"omnisolo\"".to_owned(),
        "-c".to_owned(),
        "model_providers.omnisolo.name=\"OpenAI\"".to_owned(),
        "-c".to_owned(),
        format!(
            "model_providers.omnisolo.base_url={}",
            serde_json::to_string(base_url).expect("a string is JSON serializable")
        ),
        "-c".to_owned(),
        "model_providers.omnisolo.wire_api=\"responses\"".to_owned(),
        "-c".to_owned(),
        "model_providers.omnisolo.requires_openai_auth=true".to_owned(),
        "-c".to_owned(),
        "model_providers.omnisolo.env_key=\"OPENAI_API_KEY\"".to_owned(),
    ];
    overrides.append(&mut args);
    overrides
}

fn parse_protocol(value: &str) -> Result<HarnessProtocolKind, WorkerConfigError> {
    serde_json::from_value(serde_json::Value::String(value.to_owned()))
        .map_err(|_| WorkerConfigError::InvalidProtocol(value.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn available_addresses() -> (SocketAddr, SocketAddr) {
        let first = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let second = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        (first.local_addr().unwrap(), second.local_addr().unwrap())
    }

    async fn assert_worker_serves_health(config: WorkerConfig) {
        let health_addr = config.health_addr;
        let (shutdown_sender, shutdown_receiver) = tokio::sync::oneshot::channel();
        let task = tokio::spawn(run_until_shutdown(config, async move {
            let _ = shutdown_receiver.await;
        }));
        let client = reqwest::Client::new();
        tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                if let Ok(response) = client
                    .get(format!("http://{health_addr}/healthz"))
                    .send()
                    .await
                    && response.status() == StatusCode::OK
                {
                    break;
                }
                assert!(
                    !task.is_finished(),
                    "worker exited before health became ready"
                );
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("worker health endpoint did not become ready");
        let ready = client
            .get(format!("http://{health_addr}/readyz"))
            .send()
            .await
            .unwrap();
        assert_eq!(ready.status(), StatusCode::OK);
        assert_eq!(ready.text().await.unwrap(), "ready");
        shutdown_sender.send(()).unwrap();
        tokio::time::timeout(Duration::from_secs(5), task)
            .await
            .expect("worker did not stop after graceful shutdown")
            .unwrap()
            .unwrap();
    }

    fn worker_input() -> WorkerConfigInput {
        WorkerConfigInput {
            worker_id: "worker-1".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "codex-pool".to_owned(),
            grpc_addr: "127.0.0.1:8090".to_owned(),
            health_addr: "127.0.0.1:8091".to_owned(),
            executable: Some("codex".to_owned()),
            args_json: Some(r#"["app-server","--stdio"]"#.to_owned()),
            protocol: Some("codex_app_server".to_owned()),
            timeout_secs: None,
            external_sandbox: false,
            openai_api_key: None,
            openai_api_base_url: None,
            openai_model: None,
            openai_reasoning_effort: None,
            deprecated_api_key: None,
            deprecated_base_url: None,
        }
    }

    #[tokio::test]
    async fn worker_run_fails_preflight_before_binding_ports() {
        let grpc_reservation = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let health_reservation = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let grpc_addr = grpc_reservation.local_addr().unwrap();
        let health_addr = health_reservation.local_addr().unwrap();
        drop(grpc_reservation);
        drop(health_reservation);
        let mut config = WorkerConfig::from_values(
            "worker-preflight",
            "codex",
            "codex",
            grpc_addr.to_string(),
            health_addr.to_string(),
        )
        .unwrap();
        let routing = resolve_model_routing(
            None,
            Some("https://llmapi.omnisolo.co/v1".to_owned()),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        config.default_resolved_model = Some(routing.resolved_model.clone());
        config.process_spec = Some(
            ProcessHarnessSpec::command(
                "/definitely/missing/codex",
                std::iter::empty::<String>(),
                "codex",
            )
            .with_model_routing(routing.resolved_model, routing.api_base_url),
        );

        let error = run(config).await.unwrap_err();
        assert!(
            error.to_string().contains("No such file") || error.to_string().contains("not found")
        );
        let _grpc = std::net::TcpListener::bind(grpc_addr).expect("gRPC port was never bound");
        let _health =
            std::net::TcpListener::bind(health_addr).expect("health port was never bound");

        let (invalid_grpc_addr, invalid_health_addr) = available_addresses();
        let mut invalid_provider = WorkerConfig::from_values(
            "worker-invalid-provider",
            "omnisolo",
            "omnisolo",
            invalid_grpc_addr.to_string(),
            invalid_health_addr.to_string(),
        )
        .unwrap();
        invalid_provider.provider_api_key = Some(SecretValue("secret-canary".to_owned()));
        invalid_provider.api_base_url = Some("not-an-absolute-url".to_owned());
        let error = run_until_shutdown(invalid_provider, std::future::pending())
            .await
            .unwrap_err();
        assert!(!error.to_string().contains("secret-canary"));
    }

    #[tokio::test]
    async fn run_serves_health_for_plain_provider_and_process_worker_branches() {
        let (grpc_addr, health_addr) = available_addresses();
        let config = WorkerConfig::from_values(
            "worker-plain",
            "omnisolo",
            "omnisolo",
            grpc_addr.to_string(),
            health_addr.to_string(),
        )
        .unwrap();
        assert_worker_serves_health(config).await;

        let mut provider = worker_input();
        provider.worker_id = "worker-provider".to_owned();
        provider.harness_id = "omnisolo".to_owned();
        provider.pool_id = "omnisolo".to_owned();
        let (grpc_addr, health_addr) = available_addresses();
        provider.grpc_addr = grpc_addr.to_string();
        provider.health_addr = health_addr.to_string();
        provider.executable = None;
        provider.args_json = None;
        provider.protocol = None;
        provider.openai_api_key = Some("provider-secret-canary".to_owned());
        provider.openai_api_base_url = Some("https://llmapi.omnisolo.co/v1".to_owned());
        assert_worker_serves_health(WorkerConfig::from_input(provider).unwrap()).await;

        let mut process = worker_input();
        process.worker_id = "worker-process".to_owned();
        process.harness_id = "custom".to_owned();
        process.pool_id = "custom".to_owned();
        let (grpc_addr, health_addr) = available_addresses();
        process.grpc_addr = grpc_addr.to_string();
        process.health_addr = health_addr.to_string();
        process.executable = Some("/bin/sh".to_owned());
        process.args_json = Some(
            serde_json::to_string(&[
                "-c",
                "while IFS= read -r line; do printf '%s\\n' '{\"request_id\":\"00000000-0000-0000-0000-000000000000\",\"ok\":true,\"payload\":{}}'; done",
            ])
            .unwrap(),
        );
        process.protocol = Some("custom".to_owned());
        assert_worker_serves_health(WorkerConfig::from_input(process).unwrap()).await;
    }

    #[test]
    fn environment_lookup_defaults_and_explicit_values_are_complete() {
        let defaults = WorkerConfigInput::from_lookup(|_| None);
        assert_eq!(defaults.harness_id, "omnisolo");
        assert_eq!(defaults.pool_id, "omnisolo");
        assert_eq!(defaults.grpc_addr, "0.0.0.0:8090");
        assert_eq!(defaults.health_addr, "0.0.0.0:8091");
        assert!(defaults.worker_id.is_empty());
        assert!(defaults.executable.is_none());

        let values = std::collections::BTreeMap::from([
            ("OMNISOLO_HARNESS_WORKER_ID", "worker-env"),
            ("OMNISOLO_HARNESS_ID", "pi"),
            ("OMNISOLO_HARNESS_POOL_ID", "pi-pool"),
            ("OMNISOLO_HARNESS_GRPC_ADDR", "127.0.0.1:9100"),
            ("OMNISOLO_HARNESS_HEALTH_ADDR", "127.0.0.1:9101"),
            ("OMNISOLO_HARNESS_EXECUTABLE", "pi"),
            ("OMNISOLO_HARNESS_ARGS_JSON", "[]"),
            ("OMNISOLO_HARNESS_PROTOCOL", "pi_rpc"),
            ("OMNISOLO_HARNESS_REQUEST_TIMEOUT_SECS", "45"),
            (OPENAI_API_KEY, "secret-canary"),
            (OPENAI_API_BASE_URL, "https://llmapi.omnisolo.co/v1"),
            (OPENAI_MODEL, "gpt-5.6-luna"),
            (OPENAI_REASONING_EFFORT, "max"),
            (DEPRECATED_API_KEY, "deprecated-secret"),
            (DEPRECATED_BASE_URL, "https://deprecated.example/v1"),
        ]);
        let explicit = WorkerConfigInput::from_lookup(|name| {
            values.get(name).map(|value| (*value).to_owned())
        });
        assert_eq!(explicit.worker_id, "worker-env");
        assert_eq!(explicit.harness_id, "pi");
        assert_eq!(explicit.pool_id, "pi-pool");
        assert_eq!(explicit.executable.as_deref(), Some("pi"));
        assert_eq!(explicit.protocol.as_deref(), Some("pi_rpc"));
        assert_eq!(explicit.openai_model.as_deref(), Some("gpt-5.6-luna"));
        assert_eq!(
            explicit.deprecated_api_key.as_deref(),
            Some("deprecated-secret")
        );
    }

    #[test]
    fn configuration_error_display_and_reasoning_names_cover_every_variant() {
        let empty_identity =
            WorkerConfig::from_values("", "codex", "codex", "127.0.0.1:8090", "127.0.0.1:8091")
                .unwrap_err();
        assert_eq!(empty_identity.to_string(), "worker_id must not be empty");
        let mut empty_input = worker_input();
        empty_input.worker_id.clear();
        assert!(matches!(
            WorkerConfig::from_input(empty_input),
            Err(WorkerConfigError::EmptyIdentity("worker_id"))
        ));

        let invalid_address = WorkerConfig::from_values(
            "worker",
            "codex",
            "codex",
            "not-an-address",
            "127.0.0.1:8091",
        )
        .unwrap_err();
        assert!(invalid_address.to_string().contains("grpc_addr"));
        let conflict = WorkerConfig::from_values(
            "worker",
            "codex",
            "codex",
            "127.0.0.1:8090",
            "127.0.0.1:8090",
        )
        .unwrap_err();
        assert!(conflict.to_string().contains("addresses must differ"));

        for (field, mutation) in [("argument", 0_u8), ("protocol", 1), ("timeout", 2)] {
            let mut input = worker_input();
            if mutation == 0 {
                input.args_json = Some("[".to_owned());
            } else if mutation == 1 {
                input.protocol = Some("future-protocol".to_owned());
            } else {
                input.timeout_secs = Some("not-seconds".to_owned());
            }
            let error = WorkerConfig::from_input(input).unwrap_err();
            assert!(error.to_string().contains(field));
        }

        for (effort, expected) in [
            (ReasoningEffort::None, "none"),
            (ReasoningEffort::Minimal, "minimal"),
            (ReasoningEffort::Low, "low"),
            (ReasoningEffort::Medium, "medium"),
            (ReasoningEffort::High, "high"),
            (ReasoningEffort::Max, "max"),
            (ReasoningEffort::Custom, "custom"),
        ] {
            assert_eq!(reasoning_effort_env_name(&effort), expected);
        }
    }

    #[tokio::test]
    async fn shutdown_and_health_helpers_cover_presignaled_closed_and_draining_states() {
        assert_eq!(health_response(true), (StatusCode::OK, "ok"));
        assert_eq!(
            health_response(false),
            (StatusCode::SERVICE_UNAVAILABLE, "unavailable")
        );
        assert_eq!(
            readiness_response::<()>(Ok((true, true))),
            (StatusCode::OK, "ready")
        );
        assert_eq!(
            readiness_response::<()>(Ok((true, false))),
            (StatusCode::SERVICE_UNAVAILABLE, "draining")
        );
        assert_eq!(
            readiness_response::<()>(Err(())),
            (StatusCode::SERVICE_UNAVAILABLE, "unavailable")
        );

        let (_sender, receiver) = tokio::sync::watch::channel(true);
        wait_for_shutdown(receiver).await;
        let (sender, receiver) = tokio::sync::watch::channel(false);
        drop(sender);
        wait_for_shutdown(receiver).await;

        wait_for_process_signal(std::future::ready(()), Some(std::future::pending::<()>())).await;
        wait_for_process_signal(std::future::pending::<()>(), Some(std::future::ready(()))).await;
        wait_for_process_signal(std::future::ready(()), None::<std::future::Ready<()>>).await;
    }

    #[test]
    fn process_spec_omits_reasoning_environment_when_selection_has_no_effort() {
        let mut routing = resolve_model_routing(
            None,
            Some("https://llmapi.omnisolo.co/v1".to_owned()),
            Some("open-model".to_owned()),
            None,
            None,
            None,
        )
        .unwrap();
        routing.resolved_model.reasoning_effort = None;
        let spec = build_process_spec(
            "pi",
            Some("pi".to_owned()),
            Some("[]".to_owned()),
            Some("pi_rpc".to_owned()),
            None,
            routing,
        )
        .unwrap()
        .unwrap();
        assert!(!spec.environment.contains_key(OPENAI_REASONING_EFFORT));
    }

    #[test]
    fn plandex_worker_supplies_only_loopback_native_server_configuration() {
        let mut input = worker_input();
        input.harness_id = "plandex".to_owned();
        input.args_json = Some("[]".to_owned());
        input.executable = Some("omnisolo-openai-shim".to_owned());
        input.protocol = Some("openai_compatible_shim".to_owned());
        let config = WorkerConfig::from_input(input).unwrap();
        let spec = config.process_spec.unwrap();
        assert_eq!(
            spec.environment.get("PLANDEX_ENV").map(String::as_str),
            Some("development")
        );
        assert_eq!(
            spec.environment.get("PLANDEX_API_HOST").map(String::as_str),
            Some("http://127.0.0.1:8099")
        );
        assert!(!spec.environment.contains_key("DATABASE_URL"));
        assert!(!spec.environment.contains_key("DB_PASSWORD"));
    }

    #[test]
    fn openai_inputs_win_and_secrets_stay_out_of_the_process_spec_until_facade_binding() {
        let mut input = worker_input();
        input.openai_api_key = Some("new-key".to_owned());
        input.openai_api_base_url = Some("https://llmapi.omnisolo.co/v1".to_owned());
        input.openai_model = Some("gpt-5.6-luna".to_owned());
        input.openai_reasoning_effort = Some("max".to_owned());
        input.deprecated_api_key = Some("deprecated-key".to_owned());
        input.deprecated_base_url = Some("https://deprecated.example/v1".to_owned());

        let config = WorkerConfig::from_input(input).unwrap();
        let spec = config.process_spec.as_ref().unwrap();
        let selection = spec.resolved_model.as_ref().unwrap();
        assert!(spec.environment.get(OPENAI_API_KEY).is_none());
        assert!(spec.environment.get(OPENAI_API_BASE_URL).is_none());
        assert_eq!(
            spec.environment.get(OPENAI_MODEL),
            Some(&"gpt-5.6-luna".to_owned())
        );
        assert_eq!(
            spec.environment.get(OPENAI_REASONING_EFFORT),
            Some(&"max".to_owned())
        );
        assert!(spec.api_base_url.is_none());
        assert_eq!(selection.model_id, "gpt-5.6-luna");
        assert_eq!(selection.reasoning_effort, Some(ReasoningEffort::Max));
        assert_eq!(selection.api_dialect, ModelApiDialect::OpenAiResponses);
        assert_eq!(selection.provider_route, "openai-compatible");
        assert!(!spec.args.iter().any(|arg| arg.contains("model_provider")));
        assert!(
            !spec
                .args
                .iter()
                .any(|arg| arg.contains("llmapi.omnisolo.co"))
        );
        assert_eq!(
            config.provider_api_key.as_ref().map(SecretValue::expose),
            Some("new-key")
        );

        let debug = format!("{config:?}");
        let args = format!("{:?}", spec.args);
        let selection_json = serde_json::to_string(selection).unwrap();
        for secret in ["new-key", "deprecated-key"] {
            assert!(!debug.contains(secret));
            assert!(!args.contains(secret));
            assert!(!selection_json.contains(secret));
        }
    }

    #[test]
    fn kimi_receives_an_isolated_openai_responses_provider_configuration() {
        let mut input = worker_input();
        input.harness_id = "kimi".to_owned();
        input.pool_id = "kimi".to_owned();
        input.executable = Some("omnisolo-kimi-acp".to_owned());
        input.args_json = Some("[]".to_owned());
        input.protocol = Some("kimi_acp".to_owned());
        input.openai_api_key = Some("kimi-secret-canary".to_owned());
        input.openai_api_base_url = Some("https://llmapi.omnisolo.co/v1".to_owned());
        input.openai_model = Some("gpt-5.6-luna".to_owned());
        input.openai_reasoning_effort = Some("max".to_owned());

        let config = WorkerConfig::from_input(input).unwrap();
        let spec = config.process_spec.as_ref().unwrap();
        assert!(spec.environment.get("OPENAI_BASE_URL").is_none());
        assert!(spec.environment.get(OPENAI_API_KEY).is_none());
        assert!(spec.api_base_url.is_none());
        assert!(spec.args.is_empty());
    }

    #[test]
    fn model_and_effort_defaults_are_detached_from_credentials() {
        let config = WorkerConfig::from_input(worker_input()).unwrap();
        let spec = config.process_spec.as_ref().unwrap();
        let selection = spec.resolved_model.as_ref().unwrap();
        assert_eq!(selection.model_id, "gpt-5.6-luna");
        assert_eq!(selection.reasoning_effort, Some(ReasoningEffort::Max));
        assert_eq!(selection.provider_route, "openai");
        assert_eq!(config.default_resolved_model.as_ref(), Some(selection));
        assert_eq!(config.api_base_url, spec.api_base_url);
        assert!(spec.environment.get(OPENAI_API_KEY).is_none());
        assert!(spec.api_base_url.is_none());
    }

    #[test]
    fn openai_model_is_trimmed_and_blank_values_use_the_default() {
        for (configured, expected) in [
            ("  gpt-5.6-luna-custom  ", "gpt-5.6-luna-custom"),
            ("\tgpt-5.6-luna-tabbed\n", "gpt-5.6-luna-tabbed"),
            ("   ", "gpt-5.6-luna"),
        ] {
            let mut input = worker_input();
            input.openai_model = Some(configured.to_owned());

            let config = WorkerConfig::from_input(input).unwrap();
            let selection = config.default_resolved_model.as_ref().unwrap();
            assert_eq!(
                selection.model_id, expected,
                "configured model: {configured:?}"
            );
            assert_eq!(
                config
                    .process_spec
                    .as_ref()
                    .and_then(|spec| spec.resolved_model.as_ref()),
                Some(selection)
            );
        }
    }

    #[test]
    fn omnisolo_without_an_executable_retains_non_secret_model_routing_defaults() {
        let mut input = worker_input();
        input.harness_id = "omnisolo".to_owned();
        input.pool_id = "omnisolo".to_owned();
        input.executable = None;
        input.openai_api_key = Some("secret-canary".to_owned());
        input.openai_api_base_url = Some("https://llmapi.omnisolo.co/v1".to_owned());

        let config = WorkerConfig::from_input(input).unwrap();

        assert!(config.process_spec.is_none());
        assert_eq!(
            config.api_base_url.as_deref(),
            Some("https://llmapi.omnisolo.co/v1")
        );
        let selection = config.default_resolved_model.as_ref().unwrap();
        assert_eq!(selection.model_id, "gpt-5.6-luna");
        assert_eq!(selection.reasoning_effort, Some(ReasoningEffort::Max));
        let serialized = serde_json::to_string(selection).unwrap();
        assert!(!serialized.contains("secret-canary"));
        assert!(!format!("{config:?}").contains("secret-canary"));
    }

    #[test]
    fn omnisolo_without_an_executable_retains_its_provider_secret_privately() {
        let mut input = worker_input();
        input.harness_id = "omnisolo".to_owned();
        input.pool_id = "omnisolo".to_owned();
        input.executable = None;
        input.openai_api_key = Some("omnisolo-provider-secret-canary".to_owned());
        input.openai_api_base_url = Some("https://llmapi.omnisolo.co/v1".to_owned());

        let config = WorkerConfig::from_input(input).unwrap();

        assert!(config.process_spec.is_none());
        assert_eq!(
            config.provider_api_key.as_ref().map(SecretValue::expose),
            Some("omnisolo-provider-secret-canary")
        );
        let debug = format!("{config:?}");
        assert!(!debug.contains("omnisolo-provider-secret-canary"));
        assert!(debug.contains("[REDACTED]"));
    }

    #[test]
    fn empty_openai_values_do_not_mask_deprecated_fallbacks() {
        let mut input = worker_input();
        input.openai_api_key = Some("  ".to_owned());
        input.openai_api_base_url = Some(String::new());
        input.openai_model = Some(" ".to_owned());
        input.openai_reasoning_effort = Some("".to_owned());
        input.deprecated_api_key = Some("deprecated-key".to_owned());
        input.deprecated_base_url = Some("https://legacy.example/v1".to_owned());

        let config = WorkerConfig::from_input(input).unwrap();
        let spec = config.process_spec.as_ref().unwrap();
        let selection = spec.resolved_model.as_ref().unwrap();
        assert!(spec.environment.get(OPENAI_API_KEY).is_none());
        assert!(spec.api_base_url.is_none());
        assert_eq!(
            config.provider_api_key.as_ref().map(SecretValue::expose),
            Some("deprecated-key")
        );
        assert_eq!(selection.model_id, "gpt-5.6-luna");
        assert_eq!(selection.reasoning_effort, Some(ReasoningEffort::Max));
    }

    #[test]
    fn invalid_base_url_and_reasoning_effort_are_precise_and_secret_free() {
        let mut invalid_url = worker_input();
        invalid_url.openai_api_key = Some("secret-canary".to_owned());
        invalid_url.openai_api_base_url =
            Some("https://user:secret-canary@example.test/v1".to_owned());
        let url_error = WorkerConfig::from_input(invalid_url).unwrap_err();
        assert!(matches!(url_error, WorkerConfigError::InvalidBaseUrl));
        assert_eq!(
            url_error.to_string(),
            "OPENAI_API_BASE_URL must be an absolute http(s) URL without credentials"
        );
        assert!(!format!("{url_error:?}").contains("secret-canary"));
        assert!(!url_error.to_string().contains("secret-canary"));

        let mut invalid_effort = worker_input();
        invalid_effort.openai_reasoning_effort = Some("extreme".to_owned());
        let effort_error = WorkerConfig::from_input(invalid_effort).unwrap_err();
        assert!(matches!(
            effort_error,
            WorkerConfigError::InvalidReasoningEffort(ref value) if value == "extreme"
        ));
        assert_eq!(
            effort_error.to_string(),
            "OPENAI_REASONING_EFFORT must be one of none, minimal, low, medium, high, max: extreme"
        );
    }

    #[test]
    fn worker_config_requires_identity_and_keeps_grpc_and_health_addresses_separate() {
        let config = WorkerConfig::from_values(
            "worker-1",
            "codex",
            "codex-pool",
            "127.0.0.1:8090",
            "127.0.0.1:8091",
        )
        .unwrap();
        assert_eq!(config.worker_id, "worker-1");
        assert_eq!(config.harness_id, "codex");
        assert_ne!(config.grpc_addr, config.health_addr);
        assert!(config.process_spec.is_none());

        assert!(
            WorkerConfig::from_values(
                "",
                "codex",
                "codex-pool",
                "127.0.0.1:8090",
                "127.0.0.1:8091",
            )
            .is_err()
        );
    }

    #[test]
    fn external_sandbox_is_an_explicit_operator_setting() {
        let mut input = worker_input();
        let standard = WorkerConfig::from_input(input.clone()).unwrap();
        assert!(
            !standard
                .process_spec
                .unwrap()
                .environment
                .contains_key("OMNISOLO_HARNESS_EXTERNAL_SANDBOX")
        );
        input.external_sandbox = true;
        let container = WorkerConfig::from_input(input).unwrap();
        assert_eq!(
            container.process_spec.unwrap().environment["OMNISOLO_HARNESS_EXTERNAL_SANDBOX"],
            "1"
        );
    }

    #[test]
    fn native_worker_honors_request_timeout_without_a_process() {
        let mut input = worker_input();
        input.harness_id = "omnisolo".into();
        input.executable = None;
        input.timeout_secs = Some("300".into());
        let config = WorkerConfig::from_input(input.clone()).unwrap();
        assert!(config.process_spec.is_none());
        assert_eq!(config.request_timeout, Duration::from_secs(300));
        input.timeout_secs = Some("0".into());
        assert!(WorkerConfig::from_input(input.clone()).is_err());
        input.timeout_secs = Some("invalid".into());
        assert!(WorkerConfig::from_input(input).is_err());
    }

    #[test]
    fn process_environment_is_explicit_and_uses_harness_protocol_defaults() {
        let spec = process_spec_from_env(
            "codex",
            Some("/opt/codex".to_owned()),
            Some(r#"["app-server","--stdio"]"#.to_owned()),
            None,
            Some("45".to_owned()),
            None,
            None,
        )
        .unwrap()
        .unwrap();
        assert_eq!(spec.executable, "/opt/codex");
        assert_eq!(spec.args, ["app-server", "--stdio"]);
        assert_eq!(spec.protocol_kind, HarnessProtocolKind::CodexAppServer);
        assert_eq!(spec.request_timeout, Duration::from_secs(45));

        assert!(
            process_spec_from_env("omnisolo", None, None, None, None, None, None)
                .unwrap()
                .is_none()
        );
        assert!(matches!(
            process_spec_from_env(
                "codex",
                Some("codex".to_owned()),
                None,
                Some("unknown".to_owned()),
                None,
                None,
                None,
            ),
            Err(WorkerConfigError::InvalidProtocol(_))
        ));
    }

    #[test]
    fn protocol_parser_accepts_canonical_names_and_legacy_aliases() {
        for (value, expected) in [
            ("opencode_http", HarnessProtocolKind::OpenCodeHttp),
            ("opencode_json_rpc", HarnessProtocolKind::OpenCodeHttp),
            ("deepseek_json_rpc", HarnessProtocolKind::DeepSeekJsonRpc),
            ("pi_rpc", HarnessProtocolKind::PiRpc),
            ("pi_jsonl", HarnessProtocolKind::PiRpc),
            ("kimi_acp", HarnessProtocolKind::KimiAcp),
            ("kimi_json_rpc", HarnessProtocolKind::KimiAcp),
            ("acp_v1", HarnessProtocolKind::KimiAcp),
            ("openhands_http", HarnessProtocolKind::OpenHandsHttp),
            ("openharness_sdk", HarnessProtocolKind::OpenHarnessSdk),
            ("openharness_acp", HarnessProtocolKind::OpenHarnessSdk),
        ] {
            assert_eq!(parse_protocol(value).unwrap(), expected);
        }
    }

    #[test]
    fn process_environment_defers_provider_binding_without_codex_home() {
        assert!(optional_env("PATH").is_some());
        assert!(optional_env("OMNISOLO_TEST_MISSING_ENV").is_none());

        let spec = process_spec_from_env(
            "codex",
            Some("codex".to_owned()),
            Some(r#"["app-server","--stdio"]"#.to_owned()),
            Some("codex_app_server".to_owned()),
            None,
            Some("sub2api-test-key".to_owned()),
            Some("https://llmapi.omnisolo.co/v1".to_owned()),
        )
        .unwrap()
        .unwrap();

        assert!(spec.environment.get("OPENAI_API_KEY").is_none());
        assert!(spec.environment.get("OPENAI_API_BASE_URL").is_none());
        assert!(spec.api_base_url.is_none());
        assert!(!spec.args.iter().any(|arg| arg.contains("model_provider")));
        assert_eq!(spec.args.last().map(String::as_str), Some("--stdio"));

        let debug = format!("{spec:?}");
        assert!(!debug.contains("sub2api-test-key"));
        assert!(!format!("{:?}", spec.args).contains("sub2api-test-key"));
    }

    #[tokio::test]
    async fn facade_binding_supplies_only_the_scoped_route_to_codex_processes() {
        let routing = resolve_model_routing(
            Some("sub2api-test-key".to_owned()),
            Some("https://llmapi.omnisolo.co/v1".to_owned()),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        let spec = build_process_spec(
            "codex",
            Some("codex".to_owned()),
            Some(r#"["app-server","--stdio"]"#.to_owned()),
            Some("codex_app_server".to_owned()),
            None,
            routing,
        )
        .unwrap()
        .unwrap();
        let facade = ProviderFacade::start(
            "https://llmapi.omnisolo.co/v1",
            "sub2api-test-key",
            spec.resolved_model.clone().unwrap(),
        )
        .await
        .unwrap();
        let route = facade.route().clone();
        let selection = spec.resolved_model.clone().unwrap();
        let bound = server_harness::middleware::grpc::bind_process_spec_to_provider(
            spec, &route, &selection,
        );

        assert_eq!(
            bound.environment.get(OPENAI_API_KEY),
            Some(&route.token().to_owned())
        );
        assert_eq!(
            bound.environment.get(OPENAI_API_BASE_URL),
            Some(&route.base_url().to_owned())
        );
        assert_eq!(bound.api_base_url.as_deref(), Some(route.base_url()));
        assert!(bound.args.iter().any(|arg| {
            arg == &format!(
                "model_providers.omnisolo.base_url={}",
                serde_json::to_string(route.base_url()).unwrap()
            )
        }));
        assert!(!format!("{bound:?}").contains("sub2api-test-key"));
        facade.shutdown().await.unwrap();
    }
}
