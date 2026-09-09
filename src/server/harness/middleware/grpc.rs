// Tonic service methods return Status by value as required by the generated API.
#![allow(clippy::result_large_err)]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

use serde_json::Value;
use server_omnisolo::harness_middleware::harness_worker_service_server::HarnessWorkerService;
use server_omnisolo::harness_middleware::{
    AttemptCommandEnvelope as WireAttemptCommand, EventDeliveryEnvelope,
    SessionOperationEnvelope as WireSessionOperation, SessionOperationResponse,
    WorkerControlEnvelope as WireWorkerControl, WorkerControlResponse, WorkerExchangeEnvelope,
    WorkerExchangeResponse, WorkerHealthRequest, WorkerHealthResponse,
};
use tokio_stream::{StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Status};
use uuid::Uuid;

use super::harness::{
    HarnessAdapter, HarnessAdapterError, HarnessEvent, HarnessRegistry, HarnessSessionRequest,
    OmniSoloHarnessAdapterBridge, ProcessHarnessAdapter, ProcessHarnessSpec,
};
use super::inference::OpenAiResponsesClient;
use super::local_services::{
    LOCAL_SERVICE_BUNDLE_SCHEMA, LocalServiceBundle, LocalServiceRegistry, LocalServiceScopeContext,
};
use super::openhands::OpenHandsProviderErrorKind;
use super::protocol::{AttemptOperation, HarnessExecutionItem};
use super::types::{ResolvedModelSelection, sanitize_credential_value};
use super::worker::{
    AttemptCommandEnvelope, AttemptCommandKind, SessionOperationEnvelope, SessionOperationKind,
    WorkerControlEnvelope, WorkerControlKind,
};
use super::worker_runtime::HarnessWorkerRuntime;

type SharedSessionAdapter = Arc<tokio::sync::Mutex<Box<dyn HarnessAdapter>>>;
type SessionAdapterMap = Arc<tokio::sync::Mutex<BTreeMap<Uuid, SharedSessionAdapter>>>;

#[derive(Clone)]
pub struct HarnessWorkerGrpcService {
    runtime: Arc<Mutex<HarnessWorkerRuntime>>,
    adapter_factory: Option<Arc<HarnessAdapterFactory>>,
    session_adapters: SessionAdapterMap,
    durable_sequences: Arc<tokio::sync::Mutex<BTreeMap<Uuid, i64>>>,
    event_history: Arc<tokio::sync::Mutex<BTreeMap<Uuid, Vec<EventDeliveryEnvelope>>>>,
    delivery_acknowledgements: Arc<tokio::sync::Mutex<BTreeMap<String, i64>>>,
    session_tenants: Arc<Mutex<BTreeMap<Uuid, String>>>,
    default_resolved_model: Option<ResolvedModelSelection>,
    provider_config: Option<super::provider_facade::ProviderFacadeConfig>,
    provider_service_backend: Arc<super::local_service_gateway::ProviderRouteBackend>,
    provider_redactions: Arc<Mutex<BTreeSet<String>>>,
    provider_admission: Arc<tokio::sync::Mutex<()>>,
    provider_capsules: Arc<Mutex<BTreeMap<Uuid, super::capsule::SessionCapsule>>>,
    provider_native_sessions: Arc<Mutex<BTreeMap<Uuid, String>>>,
    provider_attempts: Arc<Mutex<BTreeMap<Uuid, ProviderAttemptLease>>>,
    retired_provider_attempts: Arc<Mutex<BTreeSet<Uuid>>>,
    service_attempts: Arc<Mutex<BTreeMap<Uuid, ServiceAttemptLease>>>,
    local_service_gateway: Option<Arc<super::local_service_gateway::LocalServiceGateway>>,
    trusted_service_scopes: Arc<Mutex<BTreeMap<(String, Uuid), LocalServiceScopeContext>>>,
}

struct ServiceAttemptLease {
    tenant_id: String,
    session_id: Uuid,
    _listener: super::local_service_route::LocalServiceListener,
    cancelled: tokio::sync::watch::Sender<bool>,
}

struct ProviderAttemptLease {
    tenant_id: String,
    session_id: Uuid,
    task_id: Option<Uuid>,
    selection: ResolvedModelSelection,
    facade: super::provider_facade::ProviderFacade,
    native_session_id: Option<String>,
    _local_service_listener: Option<super::local_service_route::LocalServiceListener>,
    service_backend: Arc<super::local_service_gateway::ProviderRouteBackend>,
    attempt_id: Uuid,
}
impl Drop for ProviderAttemptLease {
    fn drop(&mut self) {
        self.service_backend
            .revoke(&self.tenant_id, self.attempt_id);
    }
}

// Drop fences provider authority even if a client disconnects or an execution
// future is cancelled before it can report a terminal event.
struct ProviderAttemptGuard {
    attempts: Arc<Mutex<BTreeMap<Uuid, ProviderAttemptLease>>>,
    attempt_id: Uuid,
    tenant_id: String,
    service: HarnessWorkerGrpcService,
}
impl Drop for ProviderAttemptGuard {
    fn drop(&mut self) {
        if let Ok(mut runtime) = self.service.runtime.lock() {
            runtime.retire_attempt(self.attempt_id);
        }
        self.service
            .service_attempts
            .lock()
            .expect("service attempts")
            .remove(&self.attempt_id);
        self.service
            .retired_provider_attempts
            .lock()
            .expect("provider retired lock")
            .insert(self.attempt_id);
        if let Ok(mut attempts) = self.attempts.lock()
            && let Some(lease) = attempts.remove(&self.attempt_id)
        {
            lease.facade.revoke();
        }
        if let Some(gateway) = &self.service.local_service_gateway {
            gateway
                .registry()
                .revoke_attempt(&self.tenant_id, self.attempt_id);
        }
        let service = self.service.clone();
        let tenant = self.tenant_id.clone();
        let attempt = self.attempt_id;
        if let Ok(runtime) = tokio::runtime::Handle::try_current() {
            runtime.spawn(async move {
                service.revoke_local_services(&tenant, attempt).await;
            });
        }
    }
}

struct ProviderAdmissionGuard {
    service: HarnessWorkerGrpcService,
    attempt_id: Uuid,
    armed: bool,
}
impl Drop for ProviderAdmissionGuard {
    fn drop(&mut self) {
        if self.armed {
            self.service.revoke_provider_attempt(self.attempt_id);
        }
    }
}

fn adapter_status(error: HarnessAdapterError) -> Status {
    let message = error.to_string();
    match error {
        HarnessAdapterError::InvalidRequest(_) | HarnessAdapterError::Capsule(_) => {
            Status::invalid_argument(message)
        }
        HarnessAdapterError::Timeout => Status::deadline_exceeded(message),
        HarnessAdapterError::ProcessExited | HarnessAdapterError::Spawn(_) => {
            Status::unavailable(message)
        }
        HarnessAdapterError::InvalidResponse(_)
        | HarnessAdapterError::Json(_)
        | HarnessAdapterError::RequestMismatch { .. } => Status::data_loss(message),
        HarnessAdapterError::OpenHandsProvider(provider) => match provider.kind {
            OpenHandsProviderErrorKind::Authentication => Status::unauthenticated(message),
            OpenHandsProviderErrorKind::Quota | OpenHandsProviderErrorKind::RateLimited => {
                Status::resource_exhausted(message)
            }
            OpenHandsProviderErrorKind::ContextLengthExceeded
            | OpenHandsProviderErrorKind::Configuration
            | OpenHandsProviderErrorKind::InvalidRequest => Status::invalid_argument(message),
            OpenHandsProviderErrorKind::Timeout => Status::deadline_exceeded(message),
            OpenHandsProviderErrorKind::ModelUnavailable
            | OpenHandsProviderErrorKind::Transient
            | OpenHandsProviderErrorKind::Unavailable => Status::unavailable(message),
            OpenHandsProviderErrorKind::AgentAction
            | OpenHandsProviderErrorKind::Internal
            | OpenHandsProviderErrorKind::Unknown => Status::internal(message),
        },
        HarnessAdapterError::OpenHandsRouter(router) => match router.status {
            404 => Status::not_found(message),
            405 | 501 => Status::unimplemented(message),
            _ => Status::internal(message),
        },
        HarnessAdapterError::Io(_) | HarnessAdapterError::Remote(_) => Status::internal(message),
    }
}

#[derive(Clone)]
enum HarnessAdapterFactory {
    OmniSolo {
        descriptor: super::harness::HarnessDescriptor,
        provider_client: Option<OpenAiResponsesClient>,
    },
    Process(ProcessHarnessSpec),
}

impl HarnessAdapterFactory {
    fn build(&self) -> Box<dyn HarnessAdapter> {
        match self {
            Self::OmniSolo {
                descriptor,
                provider_client,
            } => match provider_client.clone() {
                Some(provider_client) => {
                    Box::new(OmniSoloHarnessAdapterBridge::with_provider_client(
                        descriptor.clone(),
                        provider_client,
                    ))
                }
                None => Box::new(OmniSoloHarnessAdapterBridge::new(descriptor.clone())),
            },
            Self::Process(spec) => Box::new(ProcessHarnessAdapter::new(spec.clone())),
        }
    }
}

impl HarnessWorkerGrpcService {
    pub fn new(
        worker_id: impl Into<String>,
        harness_id: impl Into<String>,
        pool_id: impl Into<String>,
    ) -> Self {
        let worker_id = worker_id.into();
        let harness_id = harness_id.into();
        let pool_id = pool_id.into();
        let adapter_factory = if harness_id == "omnisolo" {
            HarnessRegistry::with_defaults()
                .descriptor(&harness_id)
                .cloned()
                .map(|descriptor| {
                    Arc::new(HarnessAdapterFactory::OmniSolo {
                        descriptor,
                        provider_client: None,
                    })
                })
        } else {
            None
        };
        Self {
            runtime: Arc::new(Mutex::new(HarnessWorkerRuntime::new(
                worker_id, harness_id, pool_id,
            ))),
            adapter_factory,
            session_adapters: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            durable_sequences: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            event_history: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            delivery_acknowledgements: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            session_tenants: Arc::new(Mutex::new(BTreeMap::new())),
            default_resolved_model: None,
            provider_config: None,
            provider_service_backend: Arc::new(
                super::local_service_gateway::ProviderRouteBackend::default(),
            ),
            provider_redactions: Arc::new(Mutex::new(BTreeSet::new())),
            provider_admission: Arc::new(tokio::sync::Mutex::new(())),
            provider_capsules: Arc::new(Mutex::new(BTreeMap::new())),
            provider_native_sessions: Arc::new(Mutex::new(BTreeMap::new())),
            provider_attempts: Arc::new(Mutex::new(BTreeMap::new())),
            retired_provider_attempts: Arc::new(Mutex::new(BTreeSet::new())),
            service_attempts: Arc::new(Mutex::new(BTreeMap::new())),
            local_service_gateway: None,
            trusted_service_scopes: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn with_process_spec(spec: ProcessHarnessSpec) -> Self {
        let harness_id = spec.harness_id.clone();
        let pool_id = harness_id.clone();
        Self::with_process_spec_for_worker("worker-1", pool_id, spec)
    }

    pub fn with_omnisolo_provider_client(
        worker_id: impl Into<String>,
        pool_id: impl Into<String>,
        provider_client: OpenAiResponsesClient,
    ) -> Self {
        let mut service = Self::new(worker_id, "omnisolo", pool_id);
        let descriptor = HarnessRegistry::with_defaults()
            .descriptor("omnisolo")
            .cloned()
            .expect("the built-in OmniSolo descriptor is registered");
        service.adapter_factory = Some(Arc::new(HarnessAdapterFactory::OmniSolo {
            descriptor,
            provider_client: Some(provider_client),
        }));
        service
    }

    pub fn with_process_spec_for_worker(
        worker_id: impl Into<String>,
        pool_id: impl Into<String>,
        spec: ProcessHarnessSpec,
    ) -> Self {
        let harness_id = spec.harness_id.clone();
        let default_resolved_model = spec.resolved_model.clone();
        Self {
            runtime: Arc::new(Mutex::new(HarnessWorkerRuntime::new(
                worker_id, harness_id, pool_id,
            ))),
            adapter_factory: Some(Arc::new(HarnessAdapterFactory::Process(spec))),
            session_adapters: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            durable_sequences: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            event_history: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            delivery_acknowledgements: Arc::new(tokio::sync::Mutex::new(BTreeMap::new())),
            session_tenants: Arc::new(Mutex::new(BTreeMap::new())),
            default_resolved_model,
            provider_config: None,
            provider_service_backend: Arc::new(
                super::local_service_gateway::ProviderRouteBackend::default(),
            ),
            provider_redactions: Arc::new(Mutex::new(BTreeSet::new())),
            provider_admission: Arc::new(tokio::sync::Mutex::new(())),
            provider_capsules: Arc::new(Mutex::new(BTreeMap::new())),
            provider_native_sessions: Arc::new(Mutex::new(BTreeMap::new())),
            provider_attempts: Arc::new(Mutex::new(BTreeMap::new())),
            retired_provider_attempts: Arc::new(Mutex::new(BTreeSet::new())),
            service_attempts: Arc::new(Mutex::new(BTreeMap::new())),
            local_service_gateway: None,
            trusted_service_scopes: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn with_default_resolved_model(
        mut self,
        default_resolved_model: ResolvedModelSelection,
    ) -> Self {
        self.default_resolved_model = Some(default_resolved_model);
        self
    }

    fn apply_default_resolved_model(
        &self,
        mut request: HarnessSessionRequest,
    ) -> HarnessSessionRequest {
        if request.resolved_model.is_none() {
            request.resolved_model = self.default_resolved_model.clone();
        }
        request
    }

    fn has_adapter_factory(&self) -> bool {
        self.adapter_factory.is_some()
    }

    async fn adapter_for_session(
        &self,
        session_id: Uuid,
    ) -> Option<Arc<tokio::sync::Mutex<Box<dyn HarnessAdapter>>>> {
        let factory = self.adapter_factory.as_ref()?;
        let mut adapters = self.session_adapters.lock().await;
        Some(
            adapters
                .entry(session_id)
                .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(factory.build())))
                .clone(),
        )
    }

    async fn remove_session_adapter(
        &self,
        session_id: Uuid,
    ) -> Option<Arc<tokio::sync::Mutex<Box<dyn HarnessAdapter>>>> {
        self.session_adapters.lock().await.remove(&session_id)
    }

    fn runtime(&self) -> Result<std::sync::MutexGuard<'_, HarnessWorkerRuntime>, Status> {
        self.runtime
            .lock()
            .map_err(|_| Status::internal("worker runtime lock poisoned"))
    }

    pub fn health_snapshot(&self) -> Result<super::worker_runtime::WorkerHealth, Status> {
        Ok(self.runtime()?.health())
    }

    pub async fn preflight(&self) -> Result<(), HarnessAdapterError> {
        let Some(factory) = self.adapter_factory.as_ref() else {
            return Ok(());
        };
        let request = self.apply_default_resolved_model(HarnessSessionRequest::new(
            "__omnisolo_preflight__",
            Uuid::new_v4(),
            Uuid::new_v4(),
        ));
        let facade = match self.provider_config.clone() {
            Some(mut config) => Some({
                config.inference_allowed = false;
                super::provider_facade::ProviderFacade::start_with_config(config)
                    .await
                    .map_err(|error| HarnessAdapterError::Remote(error.to_string()))?
            }),
            None => None,
        };
        let mut adapter = match &facade {
            Some(facade) => factory
                .build_with_provider(
                    facade.route(),
                    request.resolved_model.as_ref().expect("provider default"),
                    self.provider_config
                        .as_ref()
                        .expect("provider config")
                        .request_timeout,
                    None,
                )
                .map_err(|error| HarnessAdapterError::Remote(error.to_string()))?,
            None => factory.build(),
        };
        adapter.preflight(request).await
    }

    pub async fn acknowledged_delivery_sequence(&self, stream_id: &str) -> Option<i64> {
        self.delivery_acknowledgements
            .lock()
            .await
            .get(stream_id)
            .copied()
    }

    async fn append_attempt_event(
        &self,
        command: &AttemptCommandEnvelope,
        event: HarnessEvent,
        events: &mut Vec<EventDeliveryEnvelope>,
    ) {
        let payload = sanitize_credential_value(&self.redact_provider_value(event.payload));
        let durable_sequence = if event.durable {
            let mut sequences = self.durable_sequences.lock().await;
            let sequence = sequences.entry(command.session_id).or_insert(0);
            *sequence += 1;
            *sequence
        } else {
            0
        };
        let delivery = EventDeliveryEnvelope {
            protocol_version: 1,
            tenant_id: command.tenant_id.clone(),
            session_id: command.session_id.to_string(),
            task_id: command.task_id.map(|id| id.to_string()).unwrap_or_default(),
            turn_id: command.turn_id.map(|id| id.to_string()).unwrap_or_default(),
            source_attempt_id: command.attempt_id.to_string(),
            ingest_attempt_id: command.attempt_id.to_string(),
            event_id: Uuid::new_v4().to_string(),
            durable_sequence,
            delivery_stream_id: command.command_id.to_string(),
            delivery_sequence: (events.len() + 1) as i64,
            payload_schema: "omnisolo.harness.event.v1".to_owned(),
            payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({
                "event_type": self.redact_provider_text(&event.event_type),
                "payload": payload,
                "native_cursor": event.native_cursor.as_deref().map(|cursor| self.redact_provider_text(cursor)),
            }))
            .expect("harness event JSON is always serializable"),
            lease_generation: command.lease_generation,
            fencing_token: command.fencing_token.clone(),
        };
        self.event_history
            .lock()
            .await
            .entry(command.session_id)
            .or_default()
            .push(delivery.clone());
        events.push(delivery);
    }

    async fn append_local_services_bound(
        &self,
        command: &AttemptCommandEnvelope,
        request: &HarnessSessionRequest,
        events: &mut Vec<EventDeliveryEnvelope>,
    ) {
        let Some(bundle) = &request.local_service_bundle else {
            return;
        };
        let bindings = bundle
            .bindings
            .iter()
            .map(|binding| {
                serde_json::json!({
                    "binding_id": binding.binding_id,
                    "service_id": binding.service_id,
                    "kind": binding.kind,
                    "scope": binding.scope,
                    "generation": binding.generation,
                    "granted_capabilities": binding.granted_capabilities,
                })
            })
            .collect::<Vec<_>>();
        self.append_attempt_event(
            command,
            HarnessEvent {
                event_type: "local_services.bound".to_owned(),
                durable: true,
                payload: serde_json::json!({
                    "schema": LOCAL_SERVICE_BUNDLE_SCHEMA,
                    "bindings": bindings,
                }),
                native_cursor: None,
            },
            events,
        )
        .await;
    }

    // This task owns the complete admitted execution and its delivery channel.
    #[allow(clippy::too_many_arguments)]
    async fn run_concurrent_attempt(
        &self,
        command: AttemptCommandEnvelope,
        adapter: Arc<tokio::sync::Mutex<Box<dyn HarnessAdapter>>>,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: String,
        prompt: String,
        native_session_id: Option<String>,
        mut events: Vec<EventDeliveryEnvelope>,
        sender: tokio::sync::mpsc::Sender<Result<EventDeliveryEnvelope, Status>>,
    ) {
        let initial_events = events.len();
        self.append_local_services_bound(&command, &request, &mut events)
            .await;
        if events.len() > initial_events
            && let Some(delivery) = events.last().cloned()
            && sender.send(Ok(delivery)).await.is_err()
        {
            return;
        }
        let stream_result = {
            let mut adapter = adapter.lock().await;
            tokio::select! {
                biased;
                _ = sender.closed() => return,
                result = adapter.attempt_stream(
                    operation, request, &attempt_id, &prompt, native_session_id.as_deref(),
                ) => result,
            }
        };
        let mut stream = match stream_result {
            Ok(stream) => stream,
            Err(error) => {
                if let Ok(mut runtime) = self.runtime() {
                    runtime.rollback_attempt_command(&command);
                }
                let _ = sender.send(Err(self.provider_adapter_status(error))).await;
                return;
            }
        };
        let mut final_text = None;
        let mut usage = None;
        loop {
            let item = tokio::select! {
                biased;
                _ = sender.closed() => return,
                item = stream.next() => item,
            };
            let Some(item) = item else { break };
            match item {
                Ok(HarnessExecutionItem::Event(event)) => {
                    self.append_attempt_event(&command, event, &mut events)
                        .await;
                    if let Some(delivery) = events.last().cloned()
                        && sender.send(Ok(delivery)).await.is_err()
                    {
                        return;
                    }
                }
                Ok(HarnessExecutionItem::Completed {
                    final_text: completed_text,
                    usage: completed_usage,
                }) => {
                    final_text = completed_text;
                    usage = completed_usage;
                }
                Err(error) => {
                    if let Ok(mut runtime) = self.runtime() {
                        runtime.rollback_attempt_command(&command);
                    }
                    let _ = sender.send(Err(self.provider_adapter_status(error))).await;
                    return;
                }
            }
        }
        if let Some(final_text) = final_text {
            let already_emitted = events.iter().any(|event| {
                serde_json::from_slice::<Value>(&event.payload)
                    .ok()
                    .is_some_and(|payload| {
                        payload.get("event_type").and_then(Value::as_str) == Some("assistant.final")
                            && payload
                                .get("payload")
                                .and_then(|payload| payload.get("text"))
                                == Some(&Value::String(final_text.clone()))
                    })
            });
            if !already_emitted {
                self.append_attempt_event(
                    &command,
                    HarnessEvent {
                        event_type: "assistant.final".to_owned(),
                        durable: true,
                        payload: serde_json::json!({"text": final_text}),
                        native_cursor: None,
                    },
                    &mut events,
                )
                .await;
                if let Some(delivery) = events.last().cloned()
                    && sender.send(Ok(delivery)).await.is_err()
                {
                    return;
                }
            }
        }
        if let Some(usage) = usage {
            let already_emitted = events.iter().any(|event| {
                serde_json::from_slice::<Value>(&event.payload)
                    .ok()
                    .is_some_and(|payload| {
                        payload.get("event_type").and_then(Value::as_str) == Some("usage.recorded")
                            && (payload.get("payload") == Some(&usage)
                                || payload
                                    .get("payload")
                                    .and_then(|payload| payload.get("usage"))
                                    == Some(&usage))
                    })
            });
            if !already_emitted {
                self.append_attempt_event(
                    &command,
                    HarnessEvent {
                        event_type: "usage.recorded".to_owned(),
                        durable: true,
                        payload: usage,
                        native_cursor: None,
                    },
                    &mut events,
                )
                .await;
                if let Some(delivery) = events.last().cloned()
                    && sender.send(Ok(delivery)).await.is_err()
                {
                    return;
                }
            }
        }
        if events.is_empty() {
            let _ = sender
                .send(Ok(attempt_ack_event(&command, true, false)))
                .await;
        }
    }

    async fn exchange(
        &self,
        wire: WorkerExchangeEnvelope,
    ) -> Result<Response<WorkerExchangeResponse>, Status> {
        let command = parse_exchange(wire)?;
        let request = self.apply_default_resolved_model(session_request_for_command(&command, "")?);
        let result = self
            .runtime()?
            .handle_attempt_command(command.clone())
            .map_err(runtime_status)?;
        let mut payload = Vec::new();
        if !result.duplicate && command.payload.get("local_service").is_some() {
            let response = self
                .execute_local_service_exchange(&request, &command.payload)
                .await;
            match response {
                Ok(response) => {
                    payload = serde_json::to_vec(&response).expect("service result serializes")
                }
                Err(error) => {
                    self.runtime()?.rollback_attempt_command(&command);
                    return Err(error);
                }
            }
        } else if !result.duplicate
            && let Some(adapter) = self.adapter_for_session(command.session_id).await
        {
            let kind = command
                .extensions
                .get("exchange_kind")
                .and_then(Value::as_str)
                .unwrap_or("exchange");
            let response = adapter
                .lock()
                .await
                .exchange(request, kind, command.payload.clone())
                .await;
            let response = match response {
                Ok(response) => response,
                Err(error) => {
                    self.runtime()?.rollback_attempt_command(&command);
                    return Err(self.provider_adapter_status(error));
                }
            };
            payload = serde_json::to_vec(&response).expect("JSON values are always serializable");
        }
        Ok(Response::new(WorkerExchangeResponse {
            protocol_version: 1,
            accepted: result.accepted,
            duplicate: result.duplicate,
            error: String::new(),
            payload,
        }))
    }
}

impl HarnessWorkerGrpcService {
    pub fn with_local_service_gateway(
        mut self,
        gateway: Arc<super::local_service_gateway::LocalServiceGateway>,
    ) -> Self {
        self.local_service_gateway = Some(gateway);
        self
    }

    /// Register scope from the authenticated control plane's session record.
    /// This method is intentionally not exposed through caller request context.
    pub fn with_trusted_service_scope(self, scope: LocalServiceScopeContext) -> Self {
        self.trusted_service_scopes
            .lock()
            .expect("trusted service scope state poisoned")
            .insert((scope.tenant_id.clone(), scope.session_id), scope);
        self
    }

    fn trusted_scope_for_request(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<LocalServiceScopeContext, Status> {
        let mut scope = self
            .trusted_service_scopes
            .lock()
            .map_err(|_| Status::internal("service scope state unavailable"))?
            .get(&(request.tenant_id.clone(), request.session_id))
            .cloned()
            .ok_or_else(|| {
                Status::permission_denied("no admitted local service scope for session")
            })?;
        if scope.task_id.is_some() && scope.task_id != request.task_id {
            return Err(Status::permission_denied(
                "local service task is not admitted",
            ));
        }
        scope.task_id = request.task_id;
        scope.attempt_id = request.attempt_id;
        Ok(scope)
    }

    fn admit_local_services(
        &self,
        mut request: HarnessSessionRequest,
    ) -> Result<HarnessSessionRequest, Status> {
        if request.local_service_bundle.is_none() && self.local_service_gateway.is_none() {
            return Ok(request);
        }
        let gateway = self.local_service_gateway.as_ref().ok_or_else(|| {
            Status::failed_precondition("local service gateway is not configured")
        })?;
        let scope = self.trusted_scope_for_request(&request)?;
        if let Some(portable) = request.local_service_bundle.take() {
            let available = gateway.available_capabilities();
            for binding in &portable.bindings {
                let configured = available.get(&binding.kind).ok_or_else(|| {
                    Status::unimplemented("requested local service is unavailable")
                })?;
                if !binding.granted_capabilities.is_subset(configured) {
                    return Err(Status::permission_denied(
                        "requested local service capability is unavailable",
                    ));
                }
            }
            request.local_service_bundle = Some(
                gateway
                    .registry()
                    .rebind(&portable, scope)
                    .map_err(|_| Status::permission_denied("local service rebinding rejected"))?,
            );
        } else {
            request.local_service_bundle = Some(gateway.resolve(scope).map_err(|_| {
                Status::failed_precondition("local services could not be resolved")
            })?);
        }
        Ok(request)
    }

    pub async fn local_service_listener_for_request(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<Option<super::local_service_route::LocalServiceListener>, Status> {
        let Some(bundle) = &request.local_service_bundle else {
            return Ok(None);
        };
        let gateway = self.local_service_gateway.as_ref().ok_or_else(|| {
            Status::failed_precondition("local service gateway is not configured")
        })?;
        let scope = self.trusted_scope_for_request(request)?;
        super::local_service_route::LocalServiceListener::start(
            gateway.clone(),
            bundle.clone(),
            scope,
        )
        .await
        .map(Some)
        .map_err(|_| Status::permission_denied("local service route requires issued bindings"))
    }

    async fn execute_local_service_exchange(
        &self,
        request: &HarnessSessionRequest,
        payload: &Value,
    ) -> Result<Value, Status> {
        #[derive(serde::Deserialize)]
        struct Call {
            binding_id: Uuid,
            operation: super::local_service_gateway::LocalServiceOperation,
        }
        let call: Call =
            serde_json::from_value(payload.get("local_service").cloned().unwrap_or(Value::Null))
                .map_err(|_| Status::invalid_argument("invalid typed local service call"))?;
        let gateway = self.local_service_gateway.as_ref().ok_or_else(|| {
            Status::failed_precondition("local service gateway is not configured")
        })?;
        let scope = self.trusted_scope_for_request(request)?;
        let binding = gateway
            .registry()
            .issued_binding(call.binding_id)
            .ok_or_else(|| Status::permission_denied("local service binding was not issued"))?;
        gateway
            .execute(&binding, &scope, call.operation)
            .await
            .map_err(|error| {
                use super::local_service_gateway::LocalServiceGatewayError as Error;
                match error {
                    Error::Authorization(_) => {
                        Status::permission_denied("local service authorization rejected")
                    }
                    Error::UnsupportedCapability(_) => {
                        Status::unimplemented("local service operation unavailable")
                    }
                    Error::InvalidOperation => {
                        Status::invalid_argument("invalid local service operation")
                    }
                    Error::BackendFailure => {
                        Status::unavailable("local service backend unavailable")
                    }
                }
            })
    }

    async fn revoke_local_services(&self, tenant: &str, attempt: Uuid) {
        if let Some(gateway) = &self.local_service_gateway {
            gateway.registry().revoke_attempt(tenant, attempt);
            let scopes = self
                .trusted_service_scopes
                .lock()
                .expect("service scope state poisoned")
                .values()
                .cloned()
                .collect::<Vec<_>>();
            for scope in scopes.into_iter().filter(|scope| scope.tenant_id == tenant) {
                let mut bundle = gateway
                    .registry()
                    .issued_for_session(tenant, scope.session_id);
                bundle
                    .bindings
                    .retain(|binding| binding.attempt_id == Some(attempt));
                gateway.release(&bundle).await;
            }
        }
    }

    async fn revoke_session_local_services(&self, tenant: &str, session: Uuid) {
        if let Some(gateway) = &self.local_service_gateway {
            let bundle = gateway.registry().issued_for_session(tenant, session);
            gateway.registry().revoke_session(tenant, session);
            gateway.release(&bundle).await;
        }
    }
}

#[tonic::async_trait]
impl HarnessWorkerService for HarnessWorkerGrpcService {
    async fn health(
        &self,
        request: Request<WorkerHealthRequest>,
    ) -> Result<Response<WorkerHealthResponse>, Status> {
        let request = request.into_inner();
        let health = self.runtime()?.health();
        if request.worker_id != health.worker_id
            || request.harness_id != health.harness_id
            || request.pool_id != health.pool_id
        {
            return Err(Status::not_found("worker identity does not match"));
        }
        Ok(Response::new(WorkerHealthResponse {
            protocol_version: 1,
            worker_id: health.worker_id,
            ready: health.ready,
            accepting_new_attempts: health.accepting_new_attempts,
            active_attempts: health.active_attempts as u64,
            capability_version: 1,
        }))
    }

    async fn control(
        &self,
        request: Request<WireWorkerControl>,
    ) -> Result<Response<WorkerControlResponse>, Status> {
        let envelope = parse_control(request.into_inner())?;
        let shutdown = envelope.kind == WorkerControlKind::Shutdown;
        let result = self
            .runtime()?
            .handle_control(envelope)
            .map_err(runtime_status)?;
        if shutdown && !result.duplicate {
            self.revoke_provider_routes();
        }
        Ok(Response::new(WorkerControlResponse {
            protocol_version: 1,
            accepted: result.accepted,
            duplicate: result.duplicate,
            error: String::new(),
        }))
    }

    async fn session_operation(
        &self,
        request: Request<WireSessionOperation>,
    ) -> Result<Response<SessionOperationResponse>, Status> {
        let envelope = parse_session_operation(request.into_inner())?;
        let adapter_request =
            self.apply_default_resolved_model(session_request_for_operation(&envelope)?);
        let adapter_capsule = match &envelope.kind {
            SessionOperationKind::Import | SessionOperationKind::Handoff => {
                Some(capsule_from_operation(&envelope)?)
            }
            _ => None,
        };
        let result = self
            .runtime()?
            .handle_session_operation(envelope.clone())
            .map_err(runtime_status)?;
        self.admit_session_tenant(&envelope.tenant_id, envelope.session_id)?;
        if !result.duplicate
            && let Some(capsule) = adapter_capsule.clone()
        {
            self.provider_capsules
                .lock()
                .expect("provider capsule lock")
                .insert(envelope.session_id, capsule);
        }
        let mut payload = Vec::new();
        let mut payload_schema = String::new();
        let mut payload_version = 0;
        if !result.duplicate {
            if matches!(
                envelope.kind,
                SessionOperationKind::Close
                    | SessionOperationKind::Delete
                    | SessionOperationKind::Cancel
                    | SessionOperationKind::Quiesce
            ) {
                self.revoke_provider_session(&envelope.tenant_id, envelope.session_id);
                self.revoke_session_local_services(&envelope.tenant_id, envelope.session_id)
                    .await;
            }
            let has_native_session = self
                .session_adapters
                .lock()
                .await
                .contains_key(&envelope.session_id);
            let needs_setup_route = !has_native_session
                || matches!(
                    envelope.kind,
                    SessionOperationKind::Create
                        | SessionOperationKind::Import
                        | SessionOperationKind::Handoff
                );
            let _operation_facade = if needs_setup_route
                && matches!(
                    envelope.kind,
                    SessionOperationKind::Create
                        | SessionOperationKind::Import
                        | SessionOperationKind::Handoff
                        | SessionOperationKind::Fork
                        | SessionOperationKind::Resume
                ) {
                if let Some(mut config) = self.provider_config.clone() {
                    config.inference_allowed = false;
                    config.selection = adapter_request.resolved_model.clone().ok_or_else(|| {
                        Status::invalid_argument("session requires resolved model")
                    })?;
                    let facade = super::provider_facade::ProviderFacade::start_with_config(config)
                        .await
                        .map_err(|error| Status::unavailable(error.to_string()))?;
                    self.provider_redactions
                        .lock()
                        .expect("provider redactions")
                        .insert(facade.route().token().to_owned());
                    let adapter = self
                        .adapter_factory
                        .as_ref()
                        .expect("provider factory")
                        .build_with_provider(
                            facade.route(),
                            adapter_request.resolved_model.as_ref().unwrap(),
                            self.provider_config
                                .as_ref()
                                .expect("provider config")
                                .request_timeout,
                            None,
                        )?;
                    self.session_adapters.lock().await.insert(
                        envelope.session_id,
                        Arc::new(tokio::sync::Mutex::new(adapter)),
                    );
                    Some(facade)
                } else {
                    None
                }
            } else {
                None
            };
            if let Some(adapter) = self.adapter_for_session(envelope.session_id).await {
                let mut request = adapter_request.clone();
                if let Some(native_session_id) = self
                    .provider_native_sessions
                    .lock()
                    .expect("native session aliases")
                    .get(&envelope.session_id)
                    .cloned()
                {
                    request.extensions.insert(
                        "native_session_id".to_owned(),
                        Value::String(native_session_id),
                    );
                }
                let operation_result = match &envelope.kind {
                    SessionOperationKind::Create => {
                        adapter.lock().await.create_session(request).await
                    }
                    SessionOperationKind::Fork => {
                        let native_session_id = envelope
                            .payload
                            .get("native_session_id")
                            .and_then(Value::as_str);
                        adapter
                            .lock()
                            .await
                            .fork_session(request, native_session_id)
                            .await
                    }
                    SessionOperationKind::Import | SessionOperationKind::Handoff => {
                        adapter
                            .lock()
                            .await
                            .import_session(request, adapter_capsule.expect("capsule prepared"))
                            .await
                    }
                    SessionOperationKind::Resume => {
                        let native_session_id = match envelope
                            .payload
                            .get("native_session_id")
                            .and_then(Value::as_str)
                        {
                            Some(native_session_id) => native_session_id,
                            None => {
                                self.runtime()?.rollback_session_operation(&envelope);
                                return Err(Status::invalid_argument(
                                    "resume operation needs native_session_id",
                                ));
                            }
                        };
                        adapter
                            .lock()
                            .await
                            .resume_session(request, native_session_id)
                            .await
                    }
                    SessionOperationKind::Close => {
                        adapter.lock().await.close_session(request).await.map(|_| {
                            super::harness::NativeSession {
                                native_session_id: String::new(),
                                native_cursor: None,
                            }
                        })
                    }
                    SessionOperationKind::Delete => {
                        adapter.lock().await.delete_session(request).await.map(|_| {
                            super::harness::NativeSession {
                                native_session_id: String::new(),
                                native_cursor: None,
                            }
                        })
                    }
                    SessionOperationKind::Quiesce
                    | SessionOperationKind::Cancel
                    | SessionOperationKind::Snapshot => {
                        let operation = session_operation_name(&envelope.kind);
                        let mut payload = envelope.payload.clone();
                        if let Some(native) = request.extensions.get("native_session_id")
                            && let Some(object) = payload.as_object_mut()
                        {
                            object.insert("native_session_id".to_owned(), native.clone());
                        }
                        adapter
                            .lock()
                            .await
                            .control_session(request, operation, payload)
                            .await
                    }
                };
                let native_session = match operation_result {
                    Ok(native_session) => native_session,
                    Err(error) => {
                        self.runtime()?.rollback_session_operation(&envelope);
                        return Err(self.provider_adapter_status(error));
                    }
                };
                payload = serde_json::to_vec(&self.redact_provider_value(
                    serde_json::to_value(&native_session).expect("native session JSON"),
                ))
                .expect("NativeSession is always serializable");
                payload_schema = "omnisolo.harness.native_session.v1".to_owned();
                payload_version = 1;
                if matches!(
                    envelope.kind,
                    SessionOperationKind::Close | SessionOperationKind::Delete
                ) {
                    self.remove_session_adapter(envelope.session_id).await;
                    self.provider_native_sessions
                        .lock()
                        .expect("native session aliases")
                        .remove(&envelope.session_id);
                    self.provider_capsules
                        .lock()
                        .expect("provider capsule lock")
                        .remove(&envelope.session_id);
                }
            }
        }
        Ok(Response::new(SessionOperationResponse {
            protocol_version: 1,
            accepted: result.accepted,
            duplicate: result.duplicate,
            error: String::new(),
            payload_schema,
            payload_version,
            payload,
        }))
    }

    type AttemptCommandStream = ReceiverStream<Result<EventDeliveryEnvelope, Status>>;

    async fn attempt_command(
        &self,
        request: Request<WireAttemptCommand>,
    ) -> Result<Response<Self::AttemptCommandStream>, Status> {
        let command = parse_attempt_command(request.into_inner())?;
        let prompt = command
            .payload
            .get("prompt")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if self.has_adapter_factory()
            && matches!(
                command.kind,
                AttemptCommandKind::Start
                    | AttemptCommandKind::Execute
                    | AttemptCommandKind::Resume
                    | AttemptCommandKind::Steer
            )
            && prompt.trim().is_empty()
        {
            return Err(Status::invalid_argument("attempt payload needs prompt"));
        }
        let mut adapter_request = if self.has_adapter_factory() {
            Some(self.apply_default_resolved_model(session_request_for_command(&command, prompt)?))
        } else {
            None
        };
        let delivery_acknowledgement = parse_delivery_ack(&command.payload)?;
        let replay_range = replay_range(&command.payload)?;
        let accepted = self
            .runtime()?
            .handle_attempt_command(command.clone())
            .map_err(runtime_status)?;
        self.admit_session_tenant(&command.tenant_id, command.session_id)?;
        {
            let attempts = self.provider_attempts.lock().expect("provider lease lock");
            if let Some(lease) = attempts.get(&command.attempt_id) {
                if lease.tenant_id != command.tenant_id
                    || lease.session_id != command.session_id
                    || lease.task_id != command.task_id
                {
                    self.runtime()?.rollback_attempt_command(&command);
                    return Err(Status::permission_denied(
                        "provider command does not match the admitted attempt scope",
                    ));
                }
                if let Some(request) = adapter_request.as_mut() {
                    let explicit_selection = command
                        .payload
                        .pointer("/context/resolved_model")
                        .is_some_and(|value| !value.is_null());
                    if explicit_selection
                        && request.resolved_model.as_ref() != Some(&lease.selection)
                    {
                        self.runtime()?.rollback_attempt_command(&command);
                        return Err(Status::permission_denied(
                            "attempt model selection is immutable",
                        ));
                    }
                    request.resolved_model = Some(lease.selection.clone());
                }
                if !accepted.duplicate
                    && matches!(
                        command.kind,
                        AttemptCommandKind::Start
                            | AttemptCommandKind::Execute
                            | AttemptCommandKind::Resume
                    )
                {
                    self.runtime()?.rollback_attempt_command(&command);
                    return Err(Status::failed_precondition(
                        "provider attempt is already executing",
                    ));
                }
            }
        }
        let mut provider_guard = None;
        let execution_kind = matches!(
            command.kind,
            AttemptCommandKind::Start | AttemptCommandKind::Execute | AttemptCommandKind::Resume
        );
        if !accepted.duplicate && replay_range.is_none() {
            if execution_kind {
                if let Some(request) = adapter_request.take() {
                    let request = match self.admit_local_services(request) {
                        Ok(request) => request,
                        Err(error) => {
                            self.runtime()?.rollback_attempt_command(&command);
                            return Err(error);
                        }
                    };
                    provider_guard = self.provider_guard(command.attempt_id, &command.tenant_id);
                    if let Err(error) = self.prepare_provider_attempt(&request).await {
                        self.revoke_local_services(&command.tenant_id, command.attempt_id)
                            .await;
                        self.runtime()?.rollback_attempt_command(&command);
                        return Err(error);
                    }
                    adapter_request = Some(request);
                }
            } else if matches!(
                command.kind,
                AttemptCommandKind::Cancel | AttemptCommandKind::Quiesce
            ) {
                self.revoke_provider_attempt(command.attempt_id);
                self.revoke_local_services(&command.tenant_id, command.attempt_id)
                    .await;
            }
        }
        let provider_prompt = self.provider_prompt(command.session_id, prompt).await;
        let prompt = provider_prompt.as_str();
        let mut events = Vec::new();
        if execution_kind
            && !accepted.duplicate
            && (self.provider_config.is_some() || self.local_service_gateway.is_some())
        {
            self.append_attempt_event(&command, HarnessEvent {
                event_type: "user.message".to_owned(), durable: true,
                payload: serde_json::json!({"text": command.payload.get("prompt").and_then(Value::as_str).unwrap_or_default()}),
                native_cursor: None,
            }, &mut events).await;
        }
        if !accepted.duplicate {
            if let Some((stream_id, sequence)) = delivery_acknowledgement {
                self.delivery_acknowledgements
                    .lock()
                    .await
                    .entry(stream_id)
                    .and_modify(|current| *current = (*current).max(sequence))
                    .or_insert(sequence);
            }
            if replay_range.is_none()
                && let Some(adapter) = self.adapter_for_session(command.session_id).await
            {
                let concurrent = adapter.lock().await.supports_concurrent_streaming();
                if concurrent && command.kind != AttemptCommandKind::Checkpoint {
                    let operation = attempt_stream_operation(&command.kind).ok_or_else(|| {
                        Status::invalid_argument("attempt kind cannot produce a stream")
                    })?;
                    let request = adapter_request
                        .clone()
                        .expect("adapter request prepared when an adapter is present");
                    let native_session_id = self.provider_native_session(
                        command.attempt_id,
                        command.session_id,
                        command
                            .payload
                            .get("native_session_id")
                            .and_then(Value::as_str)
                            .map(str::to_owned),
                    );
                    let (sender, receiver) = tokio::sync::mpsc::channel(64);
                    for event in &events {
                        let _ = sender.send(Ok(event.clone())).await;
                    }
                    let service = self.clone();
                    let adapter = Arc::clone(&adapter);
                    let attempt_id = command.attempt_id.to_string();
                    let prompt = prompt.to_owned();
                    let command_for_task = command.clone();
                    tokio::spawn(async move {
                        let _provider_guard = provider_guard;
                        let tenant_id = command_for_task.tenant_id.clone();
                        let completed_attempt_id = command_for_task.attempt_id;
                        tokio::select! {
                            biased;
                            _ = service.wait_provider_revoked(completed_attempt_id) => {},
                            _ = service.run_concurrent_attempt(
                                command_for_task,
                                adapter,
                                operation,
                                request,
                                attempt_id,
                                prompt,
                                native_session_id,
                                events,
                                sender,
                            )
                            => {},
                        }
                        service
                            .revoke_local_services(&tenant_id, completed_attempt_id)
                            .await;
                    });
                    return Ok(Response::new(ReceiverStream::new(receiver)));
                }
            }
            if replay_range.is_none()
                && let Some(adapter) = self.adapter_for_session(command.session_id).await
            {
                let native_session_id = self.provider_native_session(
                    command.attempt_id,
                    command.session_id,
                    command
                        .payload
                        .get("native_session_id")
                        .and_then(Value::as_str)
                        .map(str::to_owned),
                );
                let native_session_id = native_session_id.as_deref();
                let request = adapter_request
                    .clone()
                    .expect("adapter request prepared when an adapter is present");
                let attempt_id = command.attempt_id.to_string();
                self.append_local_services_bound(&command, &request, &mut events)
                    .await;
                if command.kind == AttemptCommandKind::Checkpoint {
                    let checkpoint = adapter.lock().await.checkpoint(request, &attempt_id).await;
                    let checkpoint = match checkpoint {
                        Ok(checkpoint) => checkpoint,
                        Err(error) => {
                            self.runtime()?.rollback_attempt_command(&command);
                            return Err(self.provider_adapter_status(error));
                        }
                    };
                    self.append_attempt_event(
                        &command,
                        HarnessEvent {
                            event_type: "context.checkpointed".to_owned(),
                            durable: true,
                            payload: serde_json::json!({
                                "checkpoint_ref": checkpoint.checkpoint_ref,
                                "native_cursor": checkpoint.native_cursor,
                            }),
                            native_cursor: checkpoint.native_cursor,
                        },
                        &mut events,
                    )
                    .await;
                } else {
                    let operation = attempt_stream_operation(&command.kind).ok_or_else(|| {
                        Status::invalid_argument("attempt kind cannot produce a stream")
                    })?;
                    let stream_result = {
                        let mut adapter = adapter.lock().await;
                        adapter
                            .attempt_stream(
                                operation,
                                request,
                                &attempt_id,
                                prompt,
                                native_session_id,
                            )
                            .await
                    };
                    let mut stream = match stream_result {
                        Ok(stream) => stream,
                        Err(error) => {
                            self.runtime()?.rollback_attempt_command(&command);
                            return Err(self.provider_adapter_status(error));
                        }
                    };
                    let mut final_text = None;
                    let mut usage = None;
                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(HarnessExecutionItem::Event(event)) => {
                                self.append_attempt_event(&command, event, &mut events)
                                    .await;
                            }
                            Ok(HarnessExecutionItem::Completed {
                                final_text: completed_text,
                                usage: completed_usage,
                            }) => {
                                final_text = completed_text;
                                usage = completed_usage;
                            }
                            Err(error) => {
                                self.runtime()?.rollback_attempt_command(&command);
                                return Err(self.provider_adapter_status(error));
                            }
                        }
                    }
                    if let Some(final_text) = final_text {
                        let already_emitted = events.iter().any(|event| {
                            event.payload_schema == "omnisolo.harness.event.v1"
                                && serde_json::from_slice::<Value>(&event.payload)
                                    .ok()
                                    .and_then(|payload| {
                                        Some((
                                            payload.get("event_type")?.as_str()?.to_owned(),
                                            payload.get("payload")?.get("text")?.clone(),
                                        ))
                                    })
                                    .is_some_and(|(event_type, text)| {
                                        event_type == "assistant.final"
                                            && text == Value::String(final_text.clone())
                                    })
                        });
                        if !already_emitted {
                            self.append_attempt_event(
                                &command,
                                HarnessEvent {
                                    event_type: "assistant.final".to_owned(),
                                    durable: true,
                                    payload: serde_json::json!({"text": final_text}),
                                    native_cursor: None,
                                },
                                &mut events,
                            )
                            .await;
                        }
                    }
                    if let Some(usage) = usage {
                        let already_emitted = events.iter().any(|event| {
                            serde_json::from_slice::<Value>(&event.payload)
                                .ok()
                                .and_then(|payload| {
                                    Some((
                                        payload.get("event_type")?.as_str()?.to_owned(),
                                        payload.get("payload")?.clone(),
                                    ))
                                })
                                .is_some_and(|(event_type, payload)| {
                                    event_type == "usage.recorded"
                                        && (payload == usage
                                            || payload.get("usage") == Some(&usage))
                                })
                        });
                        if !already_emitted {
                            self.append_attempt_event(
                                &command,
                                HarnessEvent {
                                    event_type: "usage.recorded".to_owned(),
                                    durable: true,
                                    payload: usage,
                                    native_cursor: None,
                                },
                                &mut events,
                            )
                            .await;
                        }
                    }
                }
            }
        }
        if !accepted.duplicate
            && let Some((from_sequence, to_sequence)) = replay_range
        {
            let history = self.event_history.lock().await;
            if let Some(history) = history.get(&command.session_id) {
                for (index, original) in history
                    .iter()
                    .filter(|event| {
                        event.durable_sequence >= from_sequence
                            && to_sequence.is_none_or(|to| event.durable_sequence <= to)
                    })
                    .enumerate()
                {
                    let mut replayed = original.clone();
                    replayed.delivery_stream_id = command.command_id.to_string();
                    replayed.delivery_sequence = (index + 1) as i64;
                    replayed.lease_generation = command.lease_generation;
                    replayed.fencing_token = command.fencing_token.clone();
                    events.push(replayed);
                }
            }
        }
        if events.is_empty() {
            events.push(EventDeliveryEnvelope {
                protocol_version: 1,
                tenant_id: command.tenant_id.clone(),
                session_id: command.session_id.to_string(),
                task_id: command.task_id.map(|id| id.to_string()).unwrap_or_default(),
                turn_id: command.turn_id.map(|id| id.to_string()).unwrap_or_default(),
                source_attempt_id: command.attempt_id.to_string(),
                ingest_attempt_id: command.attempt_id.to_string(),
                event_id: command.command_id.to_string(),
                durable_sequence: 0,
                delivery_stream_id: command.command_id.to_string(),
                delivery_sequence: 1,
                payload_schema: "omnisolo.worker.command.accepted.v1".to_owned(),
                payload_version: 1,
                payload: serde_json::to_vec(&serde_json::json!({
                    "accepted": accepted.accepted,
                    "duplicate": accepted.duplicate,
                }))
                .expect("acknowledgement JSON is always serializable"),
                lease_generation: command.lease_generation,
                fencing_token: command.fencing_token,
            });
        }
        if execution_kind && !accepted.duplicate {
            self.revoke_local_services(&command.tenant_id, command.attempt_id)
                .await;
        }
        let (sender, receiver) = tokio::sync::mpsc::channel(events.len());
        for event in events {
            sender
                .send(Ok(event))
                .await
                .expect("event receiver remains owned until the stream is returned");
        }
        Ok(Response::new(ReceiverStream::new(receiver)))
    }

    async fn interaction(
        &self,
        request: Request<WorkerExchangeEnvelope>,
    ) -> Result<Response<WorkerExchangeResponse>, Status> {
        self.exchange(request.into_inner()).await
    }

    async fn artifact(
        &self,
        request: Request<WorkerExchangeEnvelope>,
    ) -> Result<Response<WorkerExchangeResponse>, Status> {
        self.exchange(request.into_inner()).await
    }

    async fn workspace(
        &self,
        request: Request<WorkerExchangeEnvelope>,
    ) -> Result<Response<WorkerExchangeResponse>, Status> {
        self.exchange(request.into_inner()).await
    }

    async fn native_record(
        &self,
        request: Request<WorkerExchangeEnvelope>,
    ) -> Result<Response<WorkerExchangeResponse>, Status> {
        self.exchange(request.into_inner()).await
    }
}

fn session_request_for_command(
    command: &AttemptCommandEnvelope,
    prompt: &str,
) -> Result<HarnessSessionRequest, Status> {
    let mut request =
        HarnessSessionRequest::new(&command.tenant_id, command.session_id, command.command_id)
            .with_attempt_id(command.attempt_id)
            .with_task(
                command
                    .task_id
                    .ok_or_else(|| Status::invalid_argument("task id is required"))?,
                prompt,
            );
    if let Some(turn_id) = command.turn_id {
        request = request.with_turn(turn_id);
    }
    request.extensions.extend(command.extensions.clone());
    apply_request_context(request, &command.payload)
}

fn session_request_for_operation(
    operation: &SessionOperationEnvelope,
) -> Result<HarnessSessionRequest, Status> {
    let objective = operation
        .payload
        .get("objective")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let mut request = HarnessSessionRequest::new(
        &operation.tenant_id,
        operation.session_id,
        operation.operation_id,
    );
    if let Some(task_id) = operation.task_id {
        request = request.with_task(task_id, objective);
    } else {
        request.objective = objective.to_owned();
    }
    if let Some(turn_id) = operation
        .payload
        .get("turn_id")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
    {
        request = request.with_turn(
            Uuid::parse_str(turn_id)
                .map_err(|_| Status::invalid_argument("invalid turn_id in operation payload"))?,
        );
    }
    request.extensions = operation
        .extensions
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    if let Some(native_session_id) = operation
        .payload
        .get("native_session_id")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
    {
        request.extensions.insert(
            "native_session_id".to_owned(),
            Value::String(native_session_id.to_owned()),
        );
    }
    apply_request_context(request, &operation.payload)
}

fn apply_request_context(
    mut request: HarnessSessionRequest,
    payload: &Value,
) -> Result<HarnessSessionRequest, Status> {
    if let Some(context) = payload.get("context") {
        let context = context
            .as_object()
            .ok_or_else(|| Status::invalid_argument("request context must be an object"))?;
        request.model_binding_id = context_uuid(context, "model_binding_id")?;
        request.runtime_config_snapshot_id = context_uuid(context, "runtime_config_snapshot_id")?;
        request.workspace_snapshot_id = context_uuid(context, "workspace_snapshot_id")?;
        request.capability_snapshot_id = context_uuid(context, "capability_snapshot_id")?;
        request.durable_sequence = context.get("durable_sequence").and_then(Value::as_i64);
        if let Some(resolved_model) = context.get("resolved_model") {
            request.resolved_model =
                if resolved_model.is_null() {
                    None
                } else {
                    Some(serde_json::from_value(resolved_model.clone()).map_err(|_| {
                        Status::invalid_argument("context resolved_model is invalid")
                    })?)
                };
        }
        if let Some(local_services) = context.get("local_services") {
            request.local_service_bundle = if local_services.is_null() {
                None
            } else {
                Some(
                    serde_json::from_value::<LocalServiceBundle>(local_services.clone()).map_err(
                        |_| Status::invalid_argument("context local_services is invalid"),
                    )?,
                )
            };
            if let Some(bundle) = &request.local_service_bundle {
                // Historical task/attempt associations are portable context. Trusted
                // admission checks durable namespace membership and issues new leases.
                let first_binding = bundle.bindings.first();
                let scope = LocalServiceScopeContext::new(
                    request.tenant_id.clone(),
                    first_binding.and_then(|binding| binding.project_id.clone()),
                    first_binding.and_then(|binding| binding.workspace_id.clone()),
                    request.session_id,
                    first_binding.and_then(|binding| binding.task_id),
                    first_binding.and_then(|binding| binding.attempt_id),
                );
                LocalServiceRegistry::with_defaults()
                    .validate_portable(bundle, &scope)
                    .map_err(|_| Status::invalid_argument("context local_services is invalid"))?;
            }
        }
        if let Some(artifact_ids) = context.get("artifact_ids") {
            let artifact_ids = artifact_ids
                .as_array()
                .ok_or_else(|| Status::invalid_argument("context artifact_ids must be an array"))?;
            request.artifact_ids = artifact_ids
                .iter()
                .map(|value| {
                    let value = value.as_str().ok_or_else(|| {
                        Status::invalid_argument("context artifact id must be a UUID")
                    })?;
                    Uuid::parse_str(value)
                        .map_err(|_| Status::invalid_argument("context artifact id must be a UUID"))
                })
                .collect::<Result<Vec<_>, _>>()?;
        }
    }
    if let Some(metadata) = payload.get("metadata") {
        let metadata = metadata
            .as_object()
            .ok_or_else(|| Status::invalid_argument("request metadata must be an object"))?;
        for (key, value) in metadata {
            let value = value.as_str().ok_or_else(|| {
                Status::invalid_argument("request metadata values must be strings")
            })?;
            request.metadata.insert(key.clone(), value.to_owned());
        }
    }
    if let Some(context) = payload.get("context")
        && let Some(extensions) = context
            .as_object()
            .ok_or_else(|| Status::invalid_argument("request context must be an object"))?
            .get("extensions")
    {
        let extensions = extensions
            .as_object()
            .ok_or_else(|| Status::invalid_argument("request extensions must be an object"))?;
        request.extensions.extend(
            extensions
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }
    Ok(request)
}

fn context_uuid(
    context: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<Uuid>, Status> {
    match context.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) if value.is_empty() => Ok(None),
        Some(Value::String(value)) => Uuid::parse_str(value)
            .map(Some)
            .map_err(|_| Status::invalid_argument(format!("context {field} must be a UUID"))),
        Some(_) => Err(Status::invalid_argument(format!(
            "context {field} must be a UUID"
        ))),
    }
}

fn capsule_from_operation(
    operation: &SessionOperationEnvelope,
) -> Result<super::capsule::SessionCapsule, Status> {
    let capsule = operation
        .payload
        .get("capsule")
        .cloned()
        .unwrap_or_else(|| operation.payload.clone());
    serde_json::from_value(capsule)
        .map_err(|_| Status::invalid_argument("operation payload needs a session capsule"))
}

fn parse_control(wire: WireWorkerControl) -> Result<WorkerControlEnvelope, Status> {
    Ok(WorkerControlEnvelope {
        protocol_version: wire.protocol_version,
        worker_id: wire.worker_id,
        runtime_id: wire.runtime_id,
        kind: parse_control_kind(&wire.kind)?,
        session_id: None,
        correlation_id: parse_optional_uuid(&wire.correlation_id)?,
        idempotency_key: non_empty(wire.idempotency_key),
        capability_version: wire.capability_version,
        payload_schema: wire.payload_schema,
        payload_version: wire.payload_version,
        payload: parse_payload(wire.payload)?,
        extensions: string_map(wire.extensions),
    })
}

fn parse_session_operation(wire: WireSessionOperation) -> Result<SessionOperationEnvelope, Status> {
    let session_id = parse_uuid(&wire.session_id, "session_id")?;
    let operation_id = parse_uuid(&wire.operation_id, "operation_id")?;
    let mut envelope = SessionOperationEnvelope::new(
        wire.tenant_id,
        session_id,
        operation_id,
        wire.operation_generation,
        wire.fencing_token,
        parse_session_operation_kind(&wire.kind)?,
    );
    if !wire.worker_id.is_empty() || !wire.harness_id.is_empty() || !wire.pool_id.is_empty() {
        envelope = envelope.with_worker(
            wire.worker_id,
            wire.harness_id,
            wire.pool_id,
            wire.capability_version,
        );
    }
    if !wire.binding_id.is_empty() {
        envelope = envelope.with_binding(
            parse_uuid(&wire.binding_id, "binding_id")?,
            wire.binding_generation,
        );
    }
    envelope.task_id = parse_optional_uuid(&wire.task_id)?;
    envelope.correlation_id = parse_optional_uuid(&wire.correlation_id)?;
    envelope.idempotency_key = non_empty(wire.idempotency_key);
    envelope.payload_schema = wire.payload_schema;
    envelope.payload_version = wire.payload_version;
    envelope.payload = parse_payload(wire.payload)?;
    envelope.extensions = string_map(wire.extensions);
    envelope.workspace_mutation_scope_id = non_empty(wire.workspace_mutation_scope_id);
    Ok(envelope)
}

fn parse_attempt_command(wire: WireAttemptCommand) -> Result<AttemptCommandEnvelope, Status> {
    let mut envelope = AttemptCommandEnvelope::new(
        wire.tenant_id,
        parse_uuid(&wire.session_id, "session_id")?,
        parse_uuid(&wire.task_id, "task_id")?,
        parse_uuid(&wire.attempt_id, "attempt_id")?,
        parse_optional_uuid(&wire.turn_id)?,
        parse_uuid(&wire.lease_id, "lease_id")?,
        wire.lease_generation,
        wire.fencing_token,
        parse_attempt_command_kind(&wire.kind)?,
    );
    if !wire.worker_id.is_empty() || !wire.harness_id.is_empty() {
        envelope = envelope.with_worker(wire.worker_id, wire.harness_id, wire.capability_version);
    }
    if !wire.binding_id.is_empty() {
        envelope = envelope.with_binding(
            parse_uuid(&wire.binding_id, "binding_id")?,
            wire.binding_generation,
        );
    }
    envelope.command_id = parse_uuid(&wire.command_id, "command_id")?;
    envelope.correlation_id = parse_optional_uuid(&wire.correlation_id)?;
    envelope.idempotency_key = non_empty(wire.idempotency_key);
    envelope.payload_schema = wire.payload_schema;
    envelope.payload_version = wire.payload_version;
    envelope.payload = parse_payload(wire.payload)?;
    envelope.extensions = string_map(wire.extensions);
    Ok(envelope)
}

fn parse_exchange(wire: WorkerExchangeEnvelope) -> Result<AttemptCommandEnvelope, Status> {
    let exchange_kind = wire.kind.clone();
    let idempotency_key = non_empty(wire.idempotency_key.clone());
    let command_id = idempotency_key.as_deref().map_or_else(Uuid::new_v4, |key| {
        Uuid::new_v5(
            &Uuid::NAMESPACE_URL,
            format!(
                "omnisolo:exchange:{}:{}:{}:{}",
                wire.tenant_id, wire.session_id, wire.attempt_id, key
            )
            .as_bytes(),
        )
    });
    let mut envelope = AttemptCommandEnvelope::new(
        wire.tenant_id,
        parse_uuid(&wire.session_id, "session_id")?,
        parse_uuid(&wire.task_id, "task_id")?,
        parse_uuid(&wire.attempt_id, "attempt_id")?,
        None,
        parse_uuid(&wire.lease_id, "lease_id")?,
        wire.lease_generation,
        wire.fencing_token,
        AttemptCommandKind::Reconcile,
    )
    .with_worker(wire.worker_id, wire.harness_id, 1)
    .with_pool(wire.pool_id);
    envelope.command_id = command_id;
    envelope.idempotency_key = idempotency_key;
    envelope.payload_schema = wire.payload_schema;
    envelope.payload_version = wire.payload_version;
    envelope.payload = parse_payload(wire.payload)?;
    if let Some(digest) = non_empty(wire.artifact_digest) {
        envelope
            .extensions
            .insert("artifact_digest".to_owned(), Value::String(digest));
    }
    envelope.extensions.extend(string_map(wire.extensions));
    envelope
        .extensions
        .insert("exchange_kind".to_owned(), Value::String(exchange_kind));
    Ok(envelope)
}

fn parse_control_kind(kind: &str) -> Result<WorkerControlKind, Status> {
    match kind {
        "register" => Ok(WorkerControlKind::Register),
        "heartbeat" => Ok(WorkerControlKind::Heartbeat),
        "capability_snapshot" => Ok(WorkerControlKind::CapabilitySnapshot),
        "drain" => Ok(WorkerControlKind::Drain),
        "shutdown" => Ok(WorkerControlKind::Shutdown),
        _ => Err(Status::invalid_argument("unknown worker control kind")),
    }
}

fn parse_session_operation_kind(kind: &str) -> Result<SessionOperationKind, Status> {
    match kind {
        "create" => Ok(SessionOperationKind::Create),
        "import" => Ok(SessionOperationKind::Import),
        "handoff" => Ok(SessionOperationKind::Handoff),
        "quiesce" => Ok(SessionOperationKind::Quiesce),
        "resume" => Ok(SessionOperationKind::Resume),
        "cancel" => Ok(SessionOperationKind::Cancel),
        "snapshot" => Ok(SessionOperationKind::Snapshot),
        "close" => Ok(SessionOperationKind::Close),
        "delete" => Ok(SessionOperationKind::Delete),
        "fork" => Ok(SessionOperationKind::Fork),
        _ => Err(Status::invalid_argument("unknown session operation kind")),
    }
}

fn session_operation_name(kind: &SessionOperationKind) -> &'static str {
    match kind {
        SessionOperationKind::Create => "create_session",
        SessionOperationKind::Import => "import_session",
        SessionOperationKind::Handoff => "handoff_session",
        SessionOperationKind::Quiesce => "quiesce",
        SessionOperationKind::Resume => "resume_session",
        SessionOperationKind::Cancel => "cancel",
        SessionOperationKind::Snapshot => "snapshot",
        SessionOperationKind::Close => "close_session",
        SessionOperationKind::Delete => "delete_session",
        SessionOperationKind::Fork => "fork_session",
    }
}

fn parse_attempt_command_kind(kind: &str) -> Result<AttemptCommandKind, Status> {
    match kind {
        "start" => Ok(AttemptCommandKind::Start),
        "execute" => Ok(AttemptCommandKind::Execute),
        "resume" => Ok(AttemptCommandKind::Resume),
        "steer" => Ok(AttemptCommandKind::Steer),
        "wait" => Ok(AttemptCommandKind::Wait),
        "quiesce" => Ok(AttemptCommandKind::Quiesce),
        "cancel" => Ok(AttemptCommandKind::Cancel),
        "checkpoint" => Ok(AttemptCommandKind::Checkpoint),
        "reconcile" => Ok(AttemptCommandKind::Reconcile),
        _ => Err(Status::invalid_argument("unknown attempt command kind")),
    }
}

fn attempt_stream_operation(kind: &AttemptCommandKind) -> Option<AttemptOperation> {
    match kind {
        AttemptCommandKind::Start => Some(AttemptOperation::Start),
        AttemptCommandKind::Execute => Some(AttemptOperation::Execute),
        AttemptCommandKind::Resume => Some(AttemptOperation::Resume),
        AttemptCommandKind::Steer => Some(AttemptOperation::Steer),
        AttemptCommandKind::Wait => Some(AttemptOperation::Reconcile),
        AttemptCommandKind::Quiesce => Some(AttemptOperation::Quiesce),
        AttemptCommandKind::Cancel => Some(AttemptOperation::Cancel),
        AttemptCommandKind::Reconcile => Some(AttemptOperation::Reconcile),
        AttemptCommandKind::Checkpoint => None,
    }
}

#[cfg(test)]
fn attempt_operation_name(kind: &AttemptCommandKind) -> &'static str {
    match kind {
        AttemptCommandKind::Start => "start",
        AttemptCommandKind::Execute => "execute",
        AttemptCommandKind::Resume => "resume",
        AttemptCommandKind::Steer => "steer",
        AttemptCommandKind::Wait => "wait",
        AttemptCommandKind::Quiesce => "quiesce",
        AttemptCommandKind::Cancel => "cancel",
        AttemptCommandKind::Checkpoint => "checkpoint",
        AttemptCommandKind::Reconcile => "reconcile",
    }
}

fn attempt_ack_event(
    command: &AttemptCommandEnvelope,
    accepted: bool,
    duplicate: bool,
) -> EventDeliveryEnvelope {
    EventDeliveryEnvelope {
        protocol_version: 1,
        tenant_id: command.tenant_id.clone(),
        session_id: command.session_id.to_string(),
        task_id: command.task_id.map(|id| id.to_string()).unwrap_or_default(),
        turn_id: command.turn_id.map(|id| id.to_string()).unwrap_or_default(),
        source_attempt_id: command.attempt_id.to_string(),
        ingest_attempt_id: command.attempt_id.to_string(),
        event_id: command.command_id.to_string(),
        durable_sequence: 0,
        delivery_stream_id: command.command_id.to_string(),
        delivery_sequence: 1,
        payload_schema: "omnisolo.worker.command.accepted.v1".to_owned(),
        payload_version: 1,
        payload: serde_json::to_vec(&serde_json::json!({
            "accepted": accepted,
            "duplicate": duplicate,
        }))
        .expect("acknowledgement JSON is always serializable"),
        lease_generation: command.lease_generation,
        fencing_token: command.fencing_token.clone(),
    }
}

fn parse_uuid(value: &str, field: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(value).map_err(|_| Status::invalid_argument(format!("invalid {field}")))
}

fn parse_optional_uuid(value: &str) -> Result<Option<Uuid>, Status> {
    if value.is_empty() {
        Ok(None)
    } else {
        parse_uuid(value, "uuid").map(Some)
    }
}

fn parse_payload(payload: Vec<u8>) -> Result<Value, Status> {
    if payload.is_empty() {
        Ok(Value::Null)
    } else {
        serde_json::from_slice(&payload)
            .map_err(|_| Status::invalid_argument("payload is not valid JSON"))
    }
}

fn replay_range(payload: &Value) -> Result<Option<(i64, Option<i64>)>, Status> {
    let Some(from) = payload.get("replay_from") else {
        return Ok(None);
    };
    let from = from
        .as_i64()
        .filter(|sequence| *sequence > 0)
        .ok_or_else(|| Status::invalid_argument("replay_from must be a positive sequence"))?;
    let to = match payload.get("replay_to") {
        None | Some(Value::Null) => None,
        Some(value) => Some(
            value
                .as_i64()
                .filter(|sequence| *sequence >= from)
                .ok_or_else(|| {
                    Status::invalid_argument("replay_to must be a sequence after replay_from")
                })?,
        ),
    };
    Ok(Some((from, to)))
}

fn parse_delivery_ack(payload: &Value) -> Result<Option<(String, i64)>, Status> {
    let Some(stream_id) = payload.get("ack_delivery_stream_id") else {
        return Ok(None);
    };
    let stream_id = stream_id
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| Status::invalid_argument("ack delivery stream id must be non-empty"))?;
    let sequence = payload
        .get("ack_delivery_sequence")
        .and_then(Value::as_i64)
        .filter(|sequence| *sequence > 0)
        .ok_or_else(|| Status::invalid_argument("ack delivery sequence must be positive"))?;
    Ok(Some((stream_id.to_owned(), sequence)))
}

fn string_map<I>(values: I) -> BTreeMap<String, Value>
where
    I: IntoIterator<Item = (String, String)>,
{
    values
        .into_iter()
        .map(|(key, value)| (key, Value::String(value)))
        .collect()
}

fn non_empty(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}

fn runtime_status(error: super::worker_runtime::WorkerRuntimeError) -> Status {
    match error {
        super::worker_runtime::WorkerRuntimeError::WrongWorker
        | super::worker_runtime::WorkerRuntimeError::WrongHarness
        | super::worker_runtime::WorkerRuntimeError::WrongPool => {
            Status::permission_denied(format!("worker routing rejected: {error:?}"))
        }
        super::worker_runtime::WorkerRuntimeError::StaleLease
        | super::worker_runtime::WorkerRuntimeError::StaleOperation => {
            Status::aborted(format!("stale fenced request: {error:?}"))
        }
        super::worker_runtime::WorkerRuntimeError::Draining => {
            Status::unavailable("worker is draining")
        }
        super::worker_runtime::WorkerRuntimeError::OperationConflict
        | super::worker_runtime::WorkerRuntimeError::LeaseConflict => {
            Status::already_exists(format!("fence conflict: {error:?}"))
        }
        super::worker_runtime::WorkerRuntimeError::InvalidEnvelope => {
            Status::invalid_argument("invalid worker envelope")
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tokio_stream::StreamExt;
    use uuid::Uuid;

    use super::super::harness::HarnessProtocolKind;
    use super::*;

    fn uuid(value: u128) -> String {
        Uuid::from_u128(value).to_string()
    }

    fn control_wire(kind: &str) -> WireWorkerControl {
        WireWorkerControl {
            protocol_version: 1,
            worker_id: "worker-1".to_owned(),
            runtime_id: "codex".to_owned(),
            kind: kind.to_owned(),
            correlation_id: uuid(10),
            idempotency_key: "control-key".to_owned(),
            capability_version: 3,
            payload_schema: "worker.control.v1".to_owned(),
            payload_version: 1,
            payload: br#"{"ok":true}"#.to_vec(),
            extensions: Default::default(),
        }
    }

    fn session_wire(kind: &str) -> WireSessionOperation {
        WireSessionOperation {
            protocol_version: 1,
            tenant_id: "tenant-1".to_owned(),
            session_id: uuid(11),
            operation_id: uuid(12),
            operation_generation: 2,
            fencing_token: "operation-fence".to_owned(),
            kind: kind.to_owned(),
            task_id: uuid(13),
            correlation_id: uuid(14),
            idempotency_key: "operation-key".to_owned(),
            payload_schema: "session.operation.v1".to_owned(),
            payload_version: 1,
            payload:
                br#"{"objective":"objective","turn_id":"00000000-0000-0000-0000-000000000015"}"#
                    .to_vec(),
            extensions: Default::default(),
            worker_id: "worker-1".to_owned(),
            pool_id: "codex-pool".to_owned(),
            harness_id: "codex".to_owned(),
            capability_version: 3,
            binding_id: uuid(16),
            binding_generation: 4,
            workspace_mutation_scope_id: "workspace-1".to_owned(),
        }
    }

    fn attempt_wire(kind: &str) -> WireAttemptCommand {
        WireAttemptCommand {
            protocol_version: 1,
            tenant_id: "tenant-1".to_owned(),
            session_id: uuid(21),
            task_id: uuid(22),
            attempt_id: uuid(23),
            turn_id: uuid(24),
            command_id: uuid(25),
            lease_id: uuid(26),
            lease_generation: 2,
            fencing_token: "lease-fence".to_owned(),
            kind: kind.to_owned(),
            correlation_id: uuid(27),
            idempotency_key: "command-key".to_owned(),
            payload_schema: "attempt.command.v1".to_owned(),
            payload_version: 1,
            payload: br#"{"prompt":"hello"}"#.to_vec(),
            extensions: Default::default(),
            worker_id: "worker-1".to_owned(),
            harness_id: "codex".to_owned(),
            capability_version: 3,
            binding_id: uuid(28),
            binding_generation: 5,
        }
    }

    #[test]
    fn wire_parsers_accept_every_versioned_kind_and_reject_unknown_values() {
        for kind in [
            "register",
            "heartbeat",
            "capability_snapshot",
            "drain",
            "shutdown",
        ] {
            assert!(parse_control(control_wire(kind)).is_ok());
        }
        assert!(parse_control(control_wire("unknown")).is_err());

        for kind in [
            "create", "import", "handoff", "quiesce", "resume", "cancel", "snapshot", "close",
            "delete", "fork",
        ] {
            assert!(parse_session_operation(session_wire(kind)).is_ok());
        }
        assert!(parse_session_operation(session_wire("unknown")).is_err());

        for kind in [
            "start",
            "execute",
            "resume",
            "steer",
            "wait",
            "quiesce",
            "cancel",
            "checkpoint",
            "reconcile",
        ] {
            assert!(parse_attempt_command(attempt_wire(kind)).is_ok());
        }
        assert!(parse_attempt_command(attempt_wire("unknown")).is_err());
    }

    #[test]
    fn parser_validation_preserves_optional_identity_and_reports_bad_wire_values() {
        assert_eq!(parse_optional_uuid("").unwrap(), None);
        assert_eq!(
            parse_optional_uuid(&uuid(30)).unwrap(),
            Some(Uuid::from_u128(30))
        );
        assert!(parse_optional_uuid("bad").is_err());
        assert!(parse_uuid("bad", "session_id").is_err());
        assert_eq!(parse_payload(Vec::new()).unwrap(), Value::Null);
        assert!(parse_payload(b"bad".to_vec()).is_err());
        assert_eq!(non_empty(String::new()), None);
        assert_eq!(non_empty("value".to_owned()), Some("value".to_owned()));
        let map = string_map([(String::from("key"), String::from("value"))]);
        assert_eq!(map.get("key"), Some(&Value::String("value".to_owned())));

        let mut control = control_wire("heartbeat");
        control.correlation_id = "bad".to_owned();
        assert!(parse_control(control).is_err());

        let mut session = session_wire("create");
        session.binding_id = "bad".to_owned();
        assert!(parse_session_operation(session).is_err());

        let mut attempt = attempt_wire("execute");
        attempt.command_id = "bad".to_owned();
        assert!(parse_attempt_command(attempt).is_err());

        let mut session_without_binding = session_wire("create");
        session_without_binding.binding_id.clear();
        assert!(parse_session_operation(session_without_binding).is_ok());

        let mut session_without_worker = session_wire("create");
        session_without_worker.worker_id.clear();
        session_without_worker.harness_id.clear();
        session_without_worker.pool_id.clear();
        assert!(parse_session_operation(session_without_worker).is_ok());

        let mut attempt_without_binding = attempt_wire("execute");
        attempt_without_binding.binding_id.clear();
        assert!(parse_attempt_command(attempt_without_binding).is_ok());

        let mut attempt_without_worker = attempt_wire("execute");
        attempt_without_worker.worker_id.clear();
        attempt_without_worker.harness_id.clear();
        assert!(parse_attempt_command(attempt_without_worker).is_ok());

        let request = apply_request_context(
            HarnessSessionRequest::new("tenant-1", Uuid::from_u128(31), Uuid::from_u128(32)),
            &json!({"context": {}}),
        )
        .unwrap();
        assert!(request.extensions.is_empty());
    }

    #[test]
    fn request_context_rejects_forged_local_service_descriptors_and_scopes() {
        use super::super::local_services::{LocalServiceRegistry, LocalServiceScopeContext};

        let scope = LocalServiceScopeContext::for_attempt(
            "tenant-services",
            Some("project-services"),
            Some("workspace-services"),
            Uuid::from_u128(70),
            Some(Uuid::from_u128(71)),
            Some(Uuid::from_u128(72)),
        );
        let bundle = LocalServiceRegistry::with_defaults()
            .resolve(scope.clone())
            .unwrap();
        let request =
            HarnessSessionRequest::new(&scope.tenant_id, scope.session_id, Uuid::from_u128(73))
                .with_task(scope.task_id.unwrap(), "services")
                .with_attempt_id(scope.attempt_id.unwrap());
        assert!(
            apply_request_context(
                request.clone(),
                &json!({"context": {"local_services": bundle}})
            )
            .is_ok()
        );

        for mutation in [
            "capability",
            "digest",
            "service",
            "missing_workspace",
            "mixed_workspace",
            "mixed_project",
            "scope",
        ] {
            let mut forged = bundle.clone();
            let binding = &mut forged.bindings[0];
            match mutation {
                "capability" => {
                    binding
                        .granted_capabilities
                        .insert("browser.navigate".to_owned());
                }
                "digest" => binding.configuration_digest = "forged".to_owned(),
                "service" => binding.service_id = "unknown-service".to_owned(),
                "missing_workspace" => binding.workspace_id = None,
                "mixed_workspace" => binding.workspace_id = Some("other-workspace".to_owned()),
                "mixed_project" => binding.project_id = Some("other-project".to_owned()),
                "scope" => binding.scope = super::super::local_services::LocalServiceScope::Tenant,
                _ => unreachable!(),
            }
            let error = apply_request_context(
                request.clone(),
                &json!({"context": {"local_services": forged}}),
            )
            .expect_err(mutation);
            assert_eq!(error.code(), tonic::Code::InvalidArgument, "{mutation}");
        }
    }

    #[test]
    fn request_context_parser_carries_all_transfer_data_and_fails_closed() {
        let ids = [
            Uuid::from_u128(40),
            Uuid::from_u128(41),
            Uuid::from_u128(42),
            Uuid::from_u128(43),
        ];
        let artifact_id = Uuid::from_u128(44);
        let request =
            HarnessSessionRequest::new("tenant-1", ids[0], ids[1]).with_task(ids[2], "task");
        let payload = json!({
            "metadata": {"source": "test"},
            "context": {
                "model_binding_id": ids[0].to_string(),
                "runtime_config_snapshot_id": ids[1].to_string(),
                "workspace_snapshot_id": ids[2].to_string(),
                "capability_snapshot_id": ids[3].to_string(),
                "artifact_ids": [artifact_id.to_string()],
                "durable_sequence": 9,
                "extensions": {"future": {"enabled": true}}
            }
        });
        let request = apply_request_context(request, &payload).unwrap();
        assert_eq!(request.model_binding_id, Some(ids[0]));
        assert_eq!(request.runtime_config_snapshot_id, Some(ids[1]));
        assert_eq!(request.workspace_snapshot_id, Some(ids[2]));
        assert_eq!(request.capability_snapshot_id, Some(ids[3]));
        assert_eq!(request.artifact_ids, vec![artifact_id]);
        assert_eq!(request.durable_sequence, Some(9));
        assert_eq!(request.metadata.get("source"), Some(&"test".to_owned()));
        assert!(request.extensions.contains_key("future"));

        let no_context = apply_request_context(request.clone(), &json!({})).unwrap();
        assert_eq!(no_context, request);
        assert!(apply_request_context(request.clone(), &json!({"context": []})).is_err());
        assert!(
            apply_request_context(request.clone(), &json!({"context": {"artifact_ids": {}}}))
                .is_err()
        );
        assert!(
            apply_request_context(request.clone(), &json!({"context": {"artifact_ids": [1]}}))
                .is_err()
        );
        assert!(
            apply_request_context(
                request.clone(),
                &json!({"context": {"artifact_ids": ["bad"]}})
            )
            .is_err()
        );
        assert!(apply_request_context(request.clone(), &json!({"metadata": []})).is_err());
        assert!(apply_request_context(request.clone(), &json!({"metadata": {"key": 1}})).is_err());
        assert!(
            apply_request_context(request.clone(), &json!({"context": {"extensions": []}}))
                .is_err()
        );
        assert!(
            apply_request_context(
                request.clone(),
                &json!({"context": {"model_binding_id": 1}})
            )
            .is_err()
        );
        assert!(
            apply_request_context(request, &json!({"context": {"model_binding_id": "bad"}}))
                .is_err()
        );
    }

    #[test]
    fn operation_and_command_request_builders_validate_payload_identity() {
        let mut operation = SessionOperationEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::from_u128(50),
            Uuid::from_u128(51),
            1,
            "fence".to_owned(),
            SessionOperationKind::Create,
        );
        operation.task_id = Some(Uuid::from_u128(52));
        operation.payload = json!({"objective":"objective","turn_id":uuid(53)});
        let request = session_request_for_operation(&operation).unwrap();
        assert_eq!(request.objective, "objective");
        assert_eq!(request.turn_id, Some(Uuid::from_u128(53)));

        operation.task_id = None;
        operation.payload = json!({"objective":"session"});
        assert_eq!(
            session_request_for_operation(&operation).unwrap().objective,
            "session"
        );
        operation.payload = json!({"turn_id":"bad"});
        assert!(session_request_for_operation(&operation).is_err());

        let command = AttemptCommandEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::from_u128(60),
            Uuid::from_u128(61),
            Uuid::from_u128(62),
            Some(Uuid::from_u128(63)),
            Uuid::from_u128(64),
            1,
            "fence".to_owned(),
            AttemptCommandKind::Execute,
        );
        let command_request = session_request_for_command(&command, "prompt").unwrap();
        assert_eq!(command_request.turn_id, Some(Uuid::from_u128(63)));
        let mut no_task = command;
        no_task.task_id = None;
        assert!(session_request_for_command(&no_task, "prompt").is_err());
    }

    #[test]
    fn capsule_and_exchange_parsers_keep_payload_metadata_and_report_invalid_capsules() {
        let operation = SessionOperationEnvelope::new(
            "tenant-1".to_owned(),
            Uuid::from_u128(70),
            Uuid::from_u128(71),
            1,
            "fence".to_owned(),
            SessionOperationKind::Import,
        );
        assert!(capsule_from_operation(&operation).is_err());

        let mut exchange = WorkerExchangeEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-1".to_owned(),
            session_id: uuid(80),
            task_id: uuid(81),
            attempt_id: uuid(82),
            worker_id: "worker-1".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "pool".to_owned(),
            lease_id: uuid(83),
            lease_generation: 1,
            fencing_token: "fence".to_owned(),
            kind: "artifact".to_owned(),
            idempotency_key: "key".to_owned(),
            payload_schema: "exchange.v1".to_owned(),
            payload_version: 1,
            payload: br#"{"ok":true}"#.to_vec(),
            artifact_digest: "digest".to_owned(),
            extensions: Default::default(),
        };
        let parsed = parse_exchange(exchange.clone()).unwrap();
        assert_eq!(
            parsed.extensions.get("artifact_digest"),
            Some(&json!("digest"))
        );
        assert_eq!(
            parsed.extensions.get("exchange_kind"),
            Some(&json!("artifact"))
        );
        assert_eq!(parsed.pool_id, "pool");
        assert_eq!(
            parsed.command_id,
            parse_exchange(exchange.clone()).unwrap().command_id
        );
        exchange.payload = b"bad".to_vec();
        assert!(parse_exchange(exchange).is_err());
        assert_eq!(replay_range(&json!({})).unwrap(), None);
        assert_eq!(
            replay_range(&json!({"replay_from": 2, "replay_to": 4})).unwrap(),
            Some((2, Some(4)))
        );
        assert!(replay_range(&json!({"replay_from": 0})).is_err());
        assert!(replay_range(&json!({"replay_from": 4, "replay_to": 2})).is_err());
        assert_eq!(parse_delivery_ack(&json!({})).unwrap(), None);
        assert_eq!(
            parse_delivery_ack(&json!({
                "ack_delivery_stream_id": "stream",
                "ack_delivery_sequence": 3
            }))
            .unwrap(),
            Some(("stream".to_owned(), 3))
        );
        assert!(parse_delivery_ack(&json!({"ack_delivery_stream_id":""})).is_err());
        assert!(parse_delivery_ack(&json!({"ack_delivery_stream_id":"stream"})).is_err());
        assert!(
            parse_delivery_ack(&json!({
                "ack_delivery_stream_id": 1,
                "ack_delivery_sequence": 1
            }))
            .is_err()
        );
        assert!(
            parse_delivery_ack(&json!({
                "ack_delivery_stream_id": "stream",
                "ack_delivery_sequence": 0
            }))
            .is_err()
        );
        assert_eq!(
            replay_range(&json!({"replay_from": 2})).unwrap(),
            Some((2, None))
        );

        for kind in [
            SessionOperationKind::Create,
            SessionOperationKind::Import,
            SessionOperationKind::Handoff,
            SessionOperationKind::Quiesce,
            SessionOperationKind::Resume,
            SessionOperationKind::Cancel,
            SessionOperationKind::Snapshot,
            SessionOperationKind::Close,
            SessionOperationKind::Delete,
            SessionOperationKind::Fork,
        ] {
            assert!(!session_operation_name(&kind).is_empty());
        }
        for kind in [
            AttemptCommandKind::Start,
            AttemptCommandKind::Execute,
            AttemptCommandKind::Resume,
            AttemptCommandKind::Steer,
            AttemptCommandKind::Wait,
            AttemptCommandKind::Quiesce,
            AttemptCommandKind::Cancel,
            AttemptCommandKind::Checkpoint,
            AttemptCommandKind::Reconcile,
        ] {
            assert!(!attempt_operation_name(&kind).is_empty());
        }

        let context = serde_json::Map::from_iter([
            ("missing".to_owned(), json!(null)),
            ("empty".to_owned(), json!("")),
            ("valid".to_owned(), json!(uuid(99))),
            ("invalid_type".to_owned(), json!(true)),
            ("invalid_value".to_owned(), json!("bad")),
        ]);
        assert_eq!(context_uuid(&context, "missing").unwrap(), None);
        assert_eq!(context_uuid(&context, "empty").unwrap(), None);
        assert_eq!(
            context_uuid(&context, "valid").unwrap(),
            Some(Uuid::from_u128(99))
        );
        assert!(context_uuid(&context, "invalid_type").is_err());
        assert!(context_uuid(&context, "invalid_value").is_err());
    }

    #[test]
    fn runtime_status_maps_each_fencing_failure_to_a_transport_status() {
        use super::super::worker_runtime::WorkerRuntimeError;

        for error in [
            WorkerRuntimeError::WrongWorker,
            WorkerRuntimeError::WrongHarness,
            WorkerRuntimeError::WrongPool,
        ] {
            assert_eq!(runtime_status(error).code(), tonic::Code::PermissionDenied);
        }
        for error in [
            WorkerRuntimeError::StaleLease,
            WorkerRuntimeError::StaleOperation,
        ] {
            assert_eq!(runtime_status(error).code(), tonic::Code::Aborted);
        }
        assert_eq!(
            runtime_status(WorkerRuntimeError::Draining).code(),
            tonic::Code::Unavailable
        );
        for error in [
            WorkerRuntimeError::OperationConflict,
            WorkerRuntimeError::LeaseConflict,
        ] {
            assert_eq!(runtime_status(error).code(), tonic::Code::AlreadyExists);
        }
        assert_eq!(
            runtime_status(WorkerRuntimeError::InvalidEnvelope).code(),
            tonic::Code::InvalidArgument
        );
    }

    #[test]
    fn adapter_status_preserves_typed_provider_and_router_failure_classes() {
        use super::super::openhands::{
            OpenHandsProviderError, OpenHandsProviderErrorKind, OpenHandsRouterError,
        };

        let rate_limit = adapter_status(HarnessAdapterError::OpenHandsProvider(
            OpenHandsProviderError {
                kind: OpenHandsProviderErrorKind::RateLimited,
                status: Some(429),
                retryable: true,
                native: json!({"type":"rate_limit"}),
            },
        ));
        assert_eq!(rate_limit.code(), tonic::Code::ResourceExhausted);
        assert!(!rate_limit.message().contains("rate_limit"));

        let router = adapter_status(HarnessAdapterError::OpenHandsRouter(OpenHandsRouterError {
            status: 404,
            native: json!({"detail":"missing"}),
        }));
        assert_eq!(router.code(), tonic::Code::NotFound);
        assert!(!router.message().contains("missing"));
    }

    #[tokio::test]
    async fn grpc_unit_service_covers_native_health_ack_and_exchange_boundaries() {
        let omnisolo = HarnessWorkerGrpcService::new("worker-omni", "omnisolo", "omni-pool");
        let health = omnisolo.health_snapshot().unwrap();
        assert_eq!(health.harness_id, "omnisolo");
        assert_eq!(
            omnisolo.acknowledged_delivery_sequence("missing").await,
            None
        );

        let service = HarnessWorkerGrpcService::new("worker-1", "codex", "codex-pool");
        let health = service
            .health(Request::new(WorkerHealthRequest {
                protocol_version: 1,
                worker_id: "worker-1".to_owned(),
                harness_id: "codex".to_owned(),
                pool_id: "codex-pool".to_owned(),
            }))
            .await
            .unwrap()
            .into_inner();
        assert!(health.ready);
        assert!(
            service
                .health(Request::new(WorkerHealthRequest {
                    protocol_version: 1,
                    worker_id: "wrong".to_owned(),
                    harness_id: "codex".to_owned(),
                    pool_id: "codex-pool".to_owned(),
                }))
                .await
                .is_err()
        );

        let control = service
            .control(Request::new(control_wire("heartbeat")))
            .await
            .unwrap()
            .into_inner();
        assert!(control.accepted);

        let session = service
            .session_operation(Request::new(session_wire("create")))
            .await
            .unwrap()
            .into_inner();
        assert!(session.accepted);
        assert!(session.payload.is_empty());

        let mut attempt = attempt_wire("reconcile");
        attempt.payload = serde_json::to_vec(&json!({
            "ack_delivery_stream_id": "stream-1",
            "ack_delivery_sequence": 3
        }))
        .unwrap();
        let mut stream = service
            .attempt_command(Request::new(attempt.clone()))
            .await
            .unwrap()
            .into_inner();
        let acknowledgement = stream.next().await.unwrap().unwrap();
        assert_eq!(
            acknowledgement.payload_schema,
            "omnisolo.worker.command.accepted.v1"
        );
        assert!(stream.next().await.is_none());

        attempt.command_id = uuid(29);
        attempt.payload = serde_json::to_vec(&json!({
            "ack_delivery_stream_id": "stream-1",
            "ack_delivery_sequence": 2
        }))
        .unwrap();
        let mut stream = service
            .attempt_command(Request::new(attempt))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(
            stream.next().await.unwrap().unwrap().payload_schema,
            "omnisolo.worker.command.accepted.v1"
        );
        assert!(stream.next().await.is_none());
        assert_eq!(
            service.acknowledged_delivery_sequence("stream-1").await,
            Some(3)
        );

        let exchange = WorkerExchangeEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-1".to_owned(),
            session_id: uuid(90),
            task_id: uuid(91),
            attempt_id: uuid(92),
            worker_id: "worker-1".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "codex-pool".to_owned(),
            lease_id: uuid(93),
            lease_generation: 1,
            fencing_token: "exchange-fence".to_owned(),
            kind: "artifact".to_owned(),
            idempotency_key: "exchange-key".to_owned(),
            payload_schema: "exchange.v1".to_owned(),
            payload_version: 1,
            payload: br#"{"ok":true}"#.to_vec(),
            artifact_digest: String::new(),
            extensions: Default::default(),
        };
        let response = service
            .exchange(exchange.clone())
            .await
            .unwrap()
            .into_inner();
        assert!(response.accepted);
        assert!(response.payload.is_empty());

        let mut interaction = exchange.clone();
        interaction.idempotency_key = "wrapper-interaction".to_owned();
        assert!(
            HarnessWorkerService::interaction(&service, Request::new(interaction))
                .await
                .unwrap()
                .into_inner()
                .accepted
        );
        let mut artifact = exchange.clone();
        artifact.idempotency_key = "wrapper-artifact".to_owned();
        assert!(
            HarnessWorkerService::artifact(&service, Request::new(artifact))
                .await
                .unwrap()
                .into_inner()
                .accepted
        );
        let mut workspace = exchange.clone();
        workspace.idempotency_key = "wrapper-workspace".to_owned();
        assert!(
            HarnessWorkerService::workspace(&service, Request::new(workspace))
                .await
                .unwrap()
                .into_inner()
                .accepted
        );
        let mut native_record = exchange;
        native_record.idempotency_key = "wrapper-native-record".to_owned();
        assert!(
            HarnessWorkerService::native_record(&service, Request::new(native_record))
                .await
                .unwrap()
                .into_inner()
                .accepted
        );
    }

    #[tokio::test]
    async fn grpc_process_service_routes_native_lifecycle_execution_replay_and_exchange() {
        let script = r#"while IFS= read -r line; do
id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
op=$(printf '%s' "$line" | sed -n 's/.*"operation":"\([^"]*\)".*/\1/p')
case "$op" in
create_session|fork_session|import_session|resume_session|close_session|delete_session|snapshot|quiesce|cancel)
payload='{"native_session_id":"native","native_cursor":"cursor"}' ;;
checkpoint)
payload='{"checkpoint_ref":"checkpoint","native_cursor":"cursor"}' ;;
*)
payload='{"events":[{"event_type":"assistant.final","durable":true,"payload":{"text":"same"}},{"event_type":"usage.recorded","durable":true,"payload":{"input_tokens":1}}],"final_text":"same","usage":{"input_tokens":1}}' ;;
esac
printf '{"request_id":"%s","ok":true,"payload":%s}\n' "$id" "$payload"
done"#;
        let spec = ProcessHarnessSpec::command("/bin/sh", ["-c", script], "codex")
            .with_protocol(HarnessProtocolKind::Custom);
        let _default_pool_service = HarnessWorkerGrpcService::with_process_spec(spec.clone());
        let service =
            HarnessWorkerGrpcService::with_process_spec_for_worker("worker-1", "codex-pool", spec);

        let mut create = session_wire("create");
        create.operation_id = uuid(101);
        create.correlation_id = uuid(102);
        create.idempotency_key = "process-create".to_owned();
        let created = service
            .session_operation(Request::new(create))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(created.payload_schema, "omnisolo.harness.native_session.v1");
        assert!(!created.payload.is_empty());

        let mut execute = attempt_wire("execute");
        execute.command_id = uuid(103);
        execute.idempotency_key = "process-execute".to_owned();
        execute.payload = serde_json::to_vec(&json!({"prompt":"hello"})).unwrap();
        let mut execution_stream = service
            .attempt_command(Request::new(execute))
            .await
            .unwrap()
            .into_inner();
        let mut execution_events = Vec::new();
        while let Some(event) = execution_stream.next().await {
            execution_events.push(event.unwrap());
        }
        assert_eq!(execution_events.len(), 2);
        let adapters = service.session_adapters.lock().await;
        assert_eq!(adapters.len(), 2);
        assert!(!Arc::ptr_eq(
            adapters.get(&Uuid::from_u128(11)).unwrap(),
            adapters.get(&Uuid::from_u128(21)).unwrap(),
        ));
        drop(adapters);

        let mut first_ack = attempt_wire("wait");
        first_ack.command_id = uuid(104);
        first_ack.idempotency_key.clear();
        first_ack.payload = serde_json::to_vec(&json!({
            "ack_delivery_stream_id": "process-stream",
            "ack_delivery_sequence": 3
        }))
        .unwrap();
        let mut ack_stream = service
            .attempt_command(Request::new(first_ack))
            .await
            .unwrap()
            .into_inner();
        while ack_stream.next().await.is_some() {}

        let mut second_ack = attempt_wire("wait");
        second_ack.command_id = uuid(105);
        second_ack.idempotency_key.clear();
        second_ack.payload = serde_json::to_vec(&json!({
            "ack_delivery_stream_id": "process-stream",
            "ack_delivery_sequence": 2
        }))
        .unwrap();
        let mut ack_stream = service
            .attempt_command(Request::new(second_ack))
            .await
            .unwrap()
            .into_inner();
        while ack_stream.next().await.is_some() {}
        assert_eq!(
            service
                .acknowledged_delivery_sequence("process-stream")
                .await,
            Some(3)
        );

        let mut replay = attempt_wire("reconcile");
        replay.command_id = uuid(106);
        replay.idempotency_key.clear();
        replay.payload = serde_json::to_vec(&json!({
            "replay_from": 1,
            "replay_to": 100
        }))
        .unwrap();
        let mut replay_stream = service
            .attempt_command(Request::new(replay))
            .await
            .unwrap()
            .into_inner();
        let mut replayed = Vec::new();
        while let Some(event) = replay_stream.next().await {
            replayed.push(event.unwrap());
        }
        assert!(!replayed.is_empty());

        let exchange = WorkerExchangeEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-1".to_owned(),
            session_id: uuid(107),
            task_id: uuid(108),
            attempt_id: uuid(109),
            worker_id: "worker-1".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "codex-pool".to_owned(),
            lease_id: uuid(110),
            lease_generation: 1,
            fencing_token: "process-exchange".to_owned(),
            kind: "artifact".to_owned(),
            idempotency_key: "process-exchange".to_owned(),
            payload_schema: "exchange.v1".to_owned(),
            payload_version: 1,
            payload: br#"{"ok":true}"#.to_vec(),
            artifact_digest: String::new(),
            extensions: [("exchange_kind".to_owned(), "artifact".to_owned())]
                .into_iter()
                .collect(),
        };
        let exchange_response = service
            .exchange(exchange.clone())
            .await
            .unwrap()
            .into_inner();
        assert!(exchange_response.accepted);
        assert!(!exchange_response.payload.is_empty());
        let duplicate_exchange = service.exchange(exchange).await.unwrap().into_inner();
        assert!(duplicate_exchange.accepted);
        assert!(duplicate_exchange.duplicate);

        let mut close = session_wire("close");
        close.operation_id = uuid(111);
        close.correlation_id = uuid(112);
        close.idempotency_key = "process-close".to_owned();
        let closed = service
            .session_operation(Request::new(close))
            .await
            .unwrap()
            .into_inner();
        assert_eq!(closed.payload_schema, "omnisolo.harness.native_session.v1");

        let mut delete = session_wire("delete");
        delete.operation_id = uuid(113);
        delete.correlation_id = uuid(114);
        delete.idempotency_key = "process-delete".to_owned();
        service
            .session_operation(Request::new(delete))
            .await
            .unwrap();

        let rejecting_script = r#"while IFS= read -r line; do
id=$(printf '%s' "$line" | sed -n 's/.*"request_id":"\([^"]*\)".*/\1/p')
printf '{"request_id":"%s","ok":false}\n' "$id"
done"#;
        let rejecting = HarnessWorkerGrpcService::with_process_spec_for_worker(
            "worker-1",
            "codex-pool",
            ProcessHarnessSpec::command("/bin/sh", ["-c", rejecting_script], "codex")
                .with_protocol(HarnessProtocolKind::Custom),
        );
        let rejected_exchange = WorkerExchangeEnvelope {
            protocol_version: 1,
            tenant_id: "tenant-1".to_owned(),
            session_id: uuid(115),
            task_id: uuid(116),
            attempt_id: uuid(117),
            worker_id: "worker-1".to_owned(),
            harness_id: "codex".to_owned(),
            pool_id: "codex-pool".to_owned(),
            lease_id: uuid(118),
            lease_generation: 1,
            fencing_token: "rejected-exchange".to_owned(),
            kind: "artifact".to_owned(),
            idempotency_key: "rejected-exchange".to_owned(),
            payload_schema: "exchange.v1".to_owned(),
            payload_version: 1,
            payload: br#"{"ok":true}"#.to_vec(),
            artifact_digest: String::new(),
            extensions: Default::default(),
        };
        assert_eq!(
            rejecting
                .exchange(rejected_exchange)
                .await
                .unwrap_err()
                .code(),
            tonic::Code::Internal
        );
    }
}

#[cfg(test)]
mod provider_attempt_tests {
    use super::super::provider_facade::ProviderFacadeConfig;
    use super::super::types::{ModelApiDialect, ReasoningEffort};
    use super::*;

    fn selection(model: &str) -> ResolvedModelSelection {
        ResolvedModelSelection {
            provider_route: "openai-compatible".into(),
            model_id: model.into(),
            reasoning_effort: Some(ReasoningEffort::Max),
            api_dialect: ModelApiDialect::OpenAiResponses,
            context_window: None,
            max_output_tokens: Some(128),
            capabilities: Default::default(),
            binding_revision: "test-v1".into(),
            binding_digest: "sha256:test".into(),
            metadata: Default::default(),
        }
    }

    #[tokio::test]
    async fn service_only_attempt_listener_is_revoked_with_execution() {
        use super::super::local_service_gateway::{LocalServiceGateway, SqliteAgentMemoryBackend};
        use super::super::local_services::LocalServiceKind;
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let mut gateway = LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SqliteAgentMemoryBackend::new(pool)),
        );
        let session = Uuid::new_v4();
        let task = Uuid::new_v4();
        let attempt = Uuid::new_v4();
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            session,
            Some(task),
            Some(attempt),
        );
        let script = r#"import os,sys,json
assert os.environ['OMNISOLO_LOCAL_SERVICE_URL'].startswith('http://127.0.0.1:')
assert os.environ['OMNISOLO_LOCAL_SERVICE_TOKEN']
assert 'OMNISOLO_LOCAL_SERVICE_CONTROL_TOKEN' not in os.environ
for line in sys.stdin:
    request=json.loads(line)
    print(json.dumps({'request_id':request['request_id'],'ok':True,'payload':{'native_session_id':'scoped-services','native_cursor':'cursor'}}),flush=True)
"#;
        let spec = ProcessHarnessSpec::command("python3", ["-u", "-c", script], "codex")
            .with_protocol(super::super::harness::HarnessProtocolKind::Custom);
        let service =
            HarnessWorkerGrpcService::with_process_spec_for_worker("worker", "pool", spec)
                .with_local_service_gateway(Arc::new(gateway))
                .with_trusted_service_scope(scope);
        let request = service
            .admit_local_services(
                HarnessSessionRequest::new("tenant", session, Uuid::new_v4())
                    .with_task(task, "services")
                    .with_attempt_id(attempt),
            )
            .unwrap();
        service.prepare_provider_attempt(&request).await.unwrap();
        assert!(
            service
                .service_attempts
                .lock()
                .unwrap()
                .contains_key(&attempt)
        );
        let waiting_service = service.clone();
        let waiter =
            tokio::spawn(async move { waiting_service.wait_provider_revoked(attempt).await });
        tokio::task::yield_now().await;
        let guard = service.provider_guard(attempt, "tenant");
        drop(guard);
        tokio::time::timeout(std::time::Duration::from_secs(1), waiter)
            .await
            .unwrap()
            .unwrap();
        assert!(service.service_attempts.lock().unwrap().is_empty());
        assert!(service.prepare_provider_attempt(&request).await.is_err());
    }

    #[tokio::test]
    async fn local_service_listener_shares_the_attempt_provider_and_revocation() {
        use super::super::local_service_gateway::{LocalServiceGateway, LocalServiceOperation};
        use super::super::local_services::LocalServiceKind;
        let upstream = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let upstream_url = format!("http://{}/v1", upstream.local_addr().unwrap());
        let upstream_task = tokio::spawn(async move {
            axum::serve(
                upstream,
                axum::Router::new().route(
                    "/v1/models",
                    axum::routing::get(|| async {
                        axum::Json(serde_json::json!({"data":[{"id":"attempt-model"}]}))
                    }),
                ),
            )
            .await
            .unwrap();
        });
        let session = Uuid::new_v4();
        let attempt = Uuid::new_v4();
        let task = Uuid::new_v4();
        let service =
            HarnessWorkerGrpcService::new("worker-1", "omnisolo", "pool").with_provider_facade(
                ProviderFacadeConfig::new(upstream_url, "protected-key", selection("default")),
            );
        let mut gateway = LocalServiceGateway::new(LocalServiceRegistry::with_defaults());
        gateway.register(
            LocalServiceKind::ProviderFacade,
            service.provider_service_backend(),
        );
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            session,
            Some(task),
            Some(attempt),
        );
        let service = service
            .with_local_service_gateway(Arc::new(gateway))
            .with_trusted_service_scope(scope);
        let request = service
            .admit_local_services(
                HarnessSessionRequest::new("tenant", session, Uuid::new_v4())
                    .with_task(task, "service test")
                    .with_attempt_id(attempt)
                    .with_resolved_model(selection("attempt-model")),
            )
            .unwrap();
        service.prepare_provider_attempt(&request).await.unwrap();
        let route = service.provider_attempts.lock().unwrap()[&attempt]
            ._local_service_listener
            .as_ref()
            .unwrap()
            .route()
            .clone();
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/operations", route.base_url()))
            .bearer_auth(route.token())
            .json(&LocalServiceOperation::ProviderModels)
            .send()
            .await
            .unwrap();
        let status = response.status();
        let models: Value = response.json().await.unwrap();
        assert!(status.is_success(), "{status}: {models}");
        assert_eq!(models["data"][0]["id"], "attempt-model");
        assert_eq!(service.redact_provider_text(route.token()), "[REDACTED]");
        service.revoke_provider_attempt(attempt);
        let result = client
            .post(format!("{}/operations", route.base_url()))
            .bearer_auth(route.token())
            .json(&LocalServiceOperation::ProviderModels)
            .send()
            .await;
        assert!(result.is_err() || !result.unwrap().status().is_success());
        upstream_task.abort();
    }

    #[tokio::test]
    async fn provider_attempt_routes_use_request_models_and_distinct_revocable_tokens() {
        let service = HarnessWorkerGrpcService::new("worker-1", "omnisolo", "pool")
            .with_provider_facade(ProviderFacadeConfig::new(
                "http://127.0.0.1:9/v1",
                "protected-key",
                selection("worker-default"),
            ));
        let session = Uuid::new_v4();
        let first = HarnessSessionRequest::new("tenant-a", session, Uuid::new_v4())
            .with_task(Uuid::new_v4(), "first")
            .with_attempt_id(Uuid::new_v4())
            .with_resolved_model(selection("model-a"));
        let second = HarnessSessionRequest::new("tenant-a", Uuid::new_v4(), Uuid::new_v4())
            .with_task(Uuid::new_v4(), "second")
            .with_attempt_id(Uuid::new_v4())
            .with_resolved_model(selection("model-b"));
        service.prepare_provider_attempt(&first).await.unwrap();
        service.prepare_provider_attempt(&second).await.unwrap();
        let first_route = service.provider_attempts.lock().unwrap()[&first.attempt_id.unwrap()]
            .facade
            .route()
            .clone();
        let second_route = service.provider_attempts.lock().unwrap()[&second.attempt_id.unwrap()]
            .facade
            .route()
            .clone();
        assert_ne!(first_route.token(), second_route.token());
        let client = reqwest::Client::new();
        let cross = client
            .post(format!("{}/responses", second_route.base_url()))
            .bearer_auth(first_route.token())
            .json(&serde_json::json!({"model":"model-b"}))
            .send()
            .await
            .unwrap();
        assert_eq!(cross.status(), reqwest::StatusCode::UNAUTHORIZED);
        let wrong_model = client
            .post(format!("{}/responses", first_route.base_url()))
            .bearer_auth(first_route.token())
            .json(&serde_json::json!({"model":"worker-default"}))
            .send()
            .await
            .unwrap();
        assert_eq!(wrong_model.status(), reqwest::StatusCode::BAD_REQUEST);
        let mut changed = first.clone();
        changed.resolved_model = Some(selection("model-b"));
        assert_eq!(
            service
                .prepare_provider_attempt(&changed)
                .await
                .unwrap_err()
                .code(),
            tonic::Code::PermissionDenied
        );
        let mut conflicting_control = command(first.session_id, "model-b");
        conflicting_control.attempt_id = first.attempt_id.unwrap().to_string();
        conflicting_control.task_id = first.task_id.unwrap().to_string();
        conflicting_control.kind = "cancel".to_owned();
        assert_eq!(
            service
                .attempt_command(Request::new(conflicting_control))
                .await
                .unwrap_err()
                .code(),
            tonic::Code::PermissionDenied
        );
        assert!(
            service
                .provider_attempts
                .lock()
                .unwrap()
                .contains_key(&first.attempt_id.unwrap())
        );
        service.revoke_provider_attempt(first.attempt_id.unwrap());
        assert_eq!(
            service
                .prepare_provider_attempt(&first)
                .await
                .unwrap_err()
                .code(),
            tonic::Code::PermissionDenied
        );
        let mut foreign = first.clone();
        foreign.tenant_id = "tenant-b".to_owned();
        foreign.attempt_id = Some(Uuid::new_v4());
        assert_eq!(
            service
                .prepare_provider_attempt(&foreign)
                .await
                .unwrap_err()
                .code(),
            tonic::Code::PermissionDenied
        );
        if let Ok(response) = client
            .get(format!("{}/models", first_route.base_url()))
            .bearer_auth(first_route.token())
            .send()
            .await
        {
            assert_eq!(response.status(), reqwest::StatusCode::SERVICE_UNAVAILABLE);
        }
        assert_eq!(
            client
                .get(format!("{}/models", second_route.base_url()))
                .bearer_auth("wrong-token")
                .send()
                .await
                .unwrap()
                .status(),
            reqwest::StatusCode::UNAUTHORIZED
        );
        service.revoke_provider_attempt(second.attempt_id.unwrap());
    }
    fn command(session: Uuid, model: &str) -> WireAttemptCommand {
        WireAttemptCommand {
            protocol_version: 1, tenant_id: "tenant-a".into(), session_id: session.to_string(),
            task_id: Uuid::new_v4().to_string(), attempt_id: Uuid::new_v4().to_string(),
            turn_id: Uuid::new_v4().to_string(), command_id: Uuid::new_v4().to_string(),
            lease_id: Uuid::new_v4().to_string(), lease_generation: 1, fencing_token: "fence".into(),
            kind: "execute".into(), correlation_id: Uuid::new_v4().to_string(), idempotency_key: Uuid::new_v4().to_string(),
            payload_schema: "attempt.command.v1".into(), payload_version: 1,
            payload: serde_json::to_vec(&serde_json::json!({"prompt":"hello", "context":{"resolved_model": selection(model)}})).unwrap(),
            extensions: Default::default(), worker_id: "worker-1".into(), harness_id: "omnisolo".into(),
            capability_version: 1, binding_id: Uuid::new_v4().to_string(), binding_generation: 1,
        }
    }

    #[tokio::test]
    async fn completed_worker_attempts_release_routes_and_preserve_history_between_models() {
        let seen = Arc::new(tokio::sync::Mutex::new(Vec::<Value>::new()));
        let captured = seen.clone();
        let app = axum::Router::new().route("/v1/responses", axum::routing::post(move |axum::Json(body): axum::Json<Value>| {
            let captured = captured.clone();
            async move {
                captured.lock().await.push(body.clone());
                axum::Json(serde_json::json!({"id":"resp_test", "object":"response", "status":"completed", "model":body["model"],
                    "output":[{"type":"message", "content":[{"type":"output_text", "text":"history-sentinel"}]}],
                    "usage":{"input_tokens":3,"output_tokens":2,"total_tokens":5}}))
            }
        }));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let service = HarnessWorkerGrpcService::new("worker-1", "omnisolo", "pool")
            .with_provider_facade(ProviderFacadeConfig::new(
                format!("http://{address}/v1"),
                "protected",
                selection("default"),
            ));
        let session = Uuid::new_v4();
        for model in ["model-a", "model-b"] {
            let mut stream = service
                .attempt_command(Request::new(command(session, model)))
                .await
                .unwrap()
                .into_inner();
            let mut events = Vec::new();
            while let Some(event) = stream.next().await {
                events.push(event.unwrap());
            }
            assert!(
                events
                    .iter()
                    .any(|event| String::from_utf8_lossy(&event.payload)
                        .contains("history-sentinel"))
            );
            assert!(
                service.provider_attempts.lock().unwrap().is_empty(),
                "terminal execution must revoke authority"
            );
        }
        let seen = seen.lock().await;
        assert_eq!(seen[0]["model"], "model-a");
        assert_eq!(seen[1]["model"], "model-b");
        assert!(seen[1]["input"].to_string().contains("history-sentinel"));
        server.abort();
    }

    #[tokio::test]
    async fn cancellation_revokes_route_before_waiting_for_busy_adapter() {
        let entered = Arc::new(tokio::sync::Notify::new());
        let notify = entered.clone();
        let app = axum::Router::new().route(
            "/v1/responses",
            axum::routing::post(move || {
                let notify = notify.clone();
                async move {
                    notify.notify_one();
                    std::future::pending::<String>().await
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        let service = HarnessWorkerGrpcService::new("worker-1", "omnisolo", "pool")
            .with_provider_facade(ProviderFacadeConfig::new(
                format!("http://{address}/v1"),
                "protected",
                selection("default"),
            ));
        let execute = command(Uuid::new_v4(), "model-a");
        let mut cancel = execute.clone();
        cancel.kind = "cancel".into();
        cancel.command_id = Uuid::new_v4().to_string();
        cancel.idempotency_key = Uuid::new_v4().to_string();
        let executing_service = service.clone();
        let executing = tokio::spawn(async move {
            executing_service
                .attempt_command(Request::new(execute))
                .await
        });
        tokio::time::timeout(std::time::Duration::from_secs(2), entered.notified())
            .await
            .unwrap();
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            service.attempt_command(Request::new(cancel)),
        )
        .await
        .unwrap()
        .unwrap();
        assert!(
            tokio::time::timeout(std::time::Duration::from_secs(2), executing)
                .await
                .unwrap()
                .unwrap()
                .is_err()
        );
        assert!(service.provider_attempts.lock().unwrap().is_empty());
        server.abort();
    }
    #[tokio::test]
    async fn provider_credentials_are_redacted_from_native_event_text() {
        let service = HarnessWorkerGrpcService::new("worker-1", "omnisolo", "pool")
            .with_provider_facade(ProviderFacadeConfig::new(
                "http://127.0.0.1:9/v1",
                "protected-key-canary",
                selection("model-a"),
            ));
        let wire = command(Uuid::new_v4(), "model-a");
        let command = parse_attempt_command(wire).unwrap();
        let request = session_request_for_command(&command, "hello").unwrap();
        service.prepare_provider_attempt(&request).await.unwrap();
        let token = service.provider_attempts.lock().unwrap()[&command.attempt_id]
            .facade
            .route()
            .token()
            .to_owned();
        service.revoke_provider_attempt(command.attempt_id);
        let mut events = Vec::new();
        service.append_attempt_event(&command, HarnessEvent { event_type:"assistant.final".into(), durable:true,
            payload: serde_json::json!({"text":format!("key protected-key-canary token {token}")}), native_cursor:None }, &mut events).await;
        let text = String::from_utf8_lossy(&events[0].payload);
        assert!(!text.contains("protected-key-canary"));
        assert!(!text.contains(&token));
    }
}

impl HarnessWorkerGrpcService {
    /// The configuration remains in the protected worker; every child gets a
    /// separately generated opaque route instead of the upstream credential.
    pub fn with_provider_facade(
        mut self,
        config: super::provider_facade::ProviderFacadeConfig,
    ) -> Self {
        if self.default_resolved_model.is_none() {
            self.default_resolved_model = Some(config.selection.clone());
        }
        self.provider_redactions
            .lock()
            .expect("provider redactions")
            .insert(config.upstream_api_key.clone());
        self.provider_config = Some(config);
        self
    }

    fn redact_provider_text(&self, text: &str) -> String {
        let mut text = text.to_owned();
        for secret in self
            .provider_redactions
            .lock()
            .expect("provider redactions")
            .iter()
            .filter(|secret| !secret.is_empty())
        {
            text = text.replace(secret, "[REDACTED]");
        }
        text
    }

    fn redact_provider_value(&self, value: Value) -> Value {
        match value {
            Value::String(text) => Value::String(self.redact_provider_text(&text)),
            Value::Array(values) => Value::Array(
                values
                    .into_iter()
                    .map(|value| self.redact_provider_value(value))
                    .collect(),
            ),
            Value::Object(values) => Value::Object(
                values
                    .into_iter()
                    .map(|(key, value)| {
                        (
                            self.redact_provider_text(&key),
                            self.redact_provider_value(value),
                        )
                    })
                    .collect(),
            ),
            value => value,
        }
    }

    fn provider_adapter_status(&self, error: HarnessAdapterError) -> Status {
        let status = adapter_status(error);
        Status::new(status.code(), self.redact_provider_text(status.message()))
    }

    fn admit_session_tenant(&self, tenant_id: &str, session_id: Uuid) -> Result<(), Status> {
        let mut owners = self
            .session_tenants
            .lock()
            .map_err(|_| Status::internal("session identity state unavailable"))?;
        if let Some(owner) = owners.get(&session_id) {
            if owner != tenant_id {
                return Err(Status::permission_denied(
                    "session belongs to a different tenant",
                ));
            }
        } else {
            owners.insert(session_id, tenant_id.to_owned());
        }
        Ok(())
    }

    pub fn provider_service_backend(
        &self,
    ) -> Arc<super::local_service_gateway::ProviderRouteBackend> {
        self.provider_service_backend.clone()
    }

    fn revoke_provider_attempt(&self, attempt_id: Uuid) {
        self.service_attempts
            .lock()
            .expect("service attempts")
            .remove(&attempt_id);
        if let Ok(mut runtime) = self.runtime.lock() {
            runtime.retire_attempt(attempt_id);
        }
        self.retired_provider_attempts
            .lock()
            .expect("provider retired lock")
            .insert(attempt_id);
        self.provider_attempts
            .lock()
            .expect("provider lease lock")
            .remove(&attempt_id);
    }

    fn revoke_provider_session(&self, tenant_id: &str, session_id: Uuid) {
        self.service_attempts
            .lock()
            .expect("service attempts")
            .retain(|_, lease| lease.tenant_id != tenant_id || lease.session_id != session_id);
        self.provider_attempts
            .lock()
            .expect("provider lease lock")
            .retain(|_, lease| lease.tenant_id != tenant_id || lease.session_id != session_id);
    }

    pub fn revoke_provider_routes(&self) {
        self.service_attempts
            .lock()
            .expect("service attempts")
            .clear();
        self.provider_attempts
            .lock()
            .expect("provider lease lock")
            .clear();
    }

    async fn wait_provider_revoked(&self, attempt_id: Uuid) {
        if self.provider_config.is_none() {
            let receiver = self
                .service_attempts
                .lock()
                .expect("service attempts")
                .get(&attempt_id)
                .map(|lease| lease.cancelled.subscribe());
            if let Some(mut receiver) = receiver {
                let _ = receiver.changed().await;
            } else if self.local_service_gateway.is_none() {
                std::future::pending::<()>().await;
            }
            return;
        }
        let receiver = self
            .provider_attempts
            .lock()
            .expect("provider lease lock")
            .get(&attempt_id)
            .map(|lease| lease.facade.revocation());
        let Some(mut receiver) = receiver else { return };
        loop {
            if *receiver.borrow_and_update() {
                return;
            }
            if receiver.changed().await.is_err() {
                return;
            }
        }
    }

    fn provider_guard(&self, attempt_id: Uuid, tenant_id: &str) -> Option<ProviderAttemptGuard> {
        Some(ProviderAttemptGuard {
            attempts: self.provider_attempts.clone(),
            attempt_id,
            tenant_id: tenant_id.to_owned(),
            service: self.clone(),
        })
    }

    async fn prepare_service_attempt(&self, request: &HarnessSessionRequest) -> Result<(), Status> {
        let _admission = self.provider_admission.lock().await;
        self.admit_session_tenant(&request.tenant_id, request.session_id)?;
        let Some(listener) = self.local_service_listener_for_request(request).await? else {
            return Ok(());
        };
        let attempt = request
            .attempt_id
            .ok_or_else(|| Status::invalid_argument("service route requires attempt"))?;
        let route = listener.route().clone();
        self.provider_redactions
            .lock()
            .expect("provider redactions")
            .insert(route.token().to_owned());
        {
            let retired = self
                .retired_provider_attempts
                .lock()
                .expect("provider retired lock");
            if retired.contains(&attempt) {
                return Err(Status::cancelled("service attempt revoked"));
            }
            let mut attempts = self.service_attempts.lock().expect("service attempts");
            if attempts
                .values()
                .any(|lease| lease.session_id == request.session_id)
            {
                return Err(Status::failed_precondition(
                    "session has active service attempt",
                ));
            }
            let (cancelled, _) = tokio::sync::watch::channel(false);
            attempts.insert(
                attempt,
                ServiceAttemptLease {
                    tenant_id: request.tenant_id.clone(),
                    session_id: request.session_id,
                    _listener: listener,
                    cancelled,
                },
            );
        }
        let mut guard = ProviderAdmissionGuard {
            service: self.clone(),
            attempt_id: attempt,
            armed: true,
        };
        if let Some(HarnessAdapterFactory::Process(spec)) = self.adapter_factory.as_deref() {
            let mut spec = spec.clone();
            spec.environment.insert(
                "OMNISOLO_LOCAL_SERVICE_URL".into(),
                route.base_url().to_owned(),
            );
            spec.environment.insert(
                "OMNISOLO_LOCAL_SERVICE_TOKEN".into(),
                route.token().to_owned(),
            );
            let mut adapter = ProcessHarnessAdapter::new(spec);
            let native = if let Some(capsule) = request.capsule.clone() {
                adapter.import_session(request.clone(), capsule).await
            } else {
                adapter.create_session(request.clone()).await
            }
            .map_err(|error| self.provider_adapter_status(error))?;
            if !self
                .service_attempts
                .lock()
                .expect("service attempts")
                .contains_key(&attempt)
            {
                return Err(Status::cancelled(
                    "service attempt revoked during native admission",
                ));
            }
            self.provider_native_sessions
                .lock()
                .expect("native session aliases")
                .insert(request.session_id, native.native_session_id);
            let old = self.session_adapters.lock().await.insert(
                request.session_id,
                Arc::new(tokio::sync::Mutex::new(Box::new(adapter))),
            );
            if let Some(old) = old {
                let mut cleanup = request.clone();
                cleanup.extensions.remove("native_session_id");
                let _ = old.lock().await.delete_session(cleanup).await;
            }
        }
        guard.armed = false;
        Ok(())
    }

    async fn prepare_provider_attempt(
        &self,
        request: &HarnessSessionRequest,
    ) -> Result<(), Status> {
        let Some(mut config) = self.provider_config.clone() else {
            return self.prepare_service_attempt(request).await;
        };
        let _admission = self.provider_admission.lock().await;
        self.admit_session_tenant(&request.tenant_id, request.session_id)?;
        let attempt_id = request
            .attempt_id
            .ok_or_else(|| Status::invalid_argument("provider route requires attempt identity"))?;
        if self
            .retired_provider_attempts
            .lock()
            .expect("provider retired lock")
            .contains(&attempt_id)
        {
            return Err(Status::permission_denied(
                "provider attempt has already been revoked",
            ));
        }
        let selection = request
            .resolved_model
            .clone()
            .ok_or_else(|| Status::invalid_argument("provider route requires resolved model"))?;
        if selection.provider_route != config.selection.provider_route {
            return Err(Status::permission_denied(
                "provider route is not configured for this worker",
            ));
        }
        {
            let attempts = self.provider_attempts.lock().expect("provider lease lock");
            if let Some(lease) = attempts.get(&attempt_id) {
                if lease.tenant_id != request.tenant_id
                    || lease.session_id != request.session_id
                    || lease.task_id != request.task_id
                    || lease.selection != selection
                {
                    return Err(Status::permission_denied(
                        "attempt provider binding is immutable",
                    ));
                }
                return Err(Status::failed_precondition(
                    "provider attempt is already executing",
                ));
            }
            if attempts
                .values()
                .any(|lease| lease.session_id == request.session_id)
            {
                return Err(Status::failed_precondition(
                    "session already has an active provider attempt",
                ));
            }
        }
        config.selection = selection.clone();
        let facade = super::provider_facade::ProviderFacade::start_with_config(config)
            .await
            .map_err(|error| Status::unavailable(error.to_string()))?;
        let route = facade.route().clone();
        self.provider_redactions
            .lock()
            .expect("provider redactions")
            .insert(route.token().to_owned());
        if self
            .retired_provider_attempts
            .lock()
            .expect("provider retired lock")
            .contains(&attempt_id)
        {
            return Err(Status::cancelled(
                "provider attempt revoked during route admission",
            ));
        }
        let local_listener = self.local_service_listener_for_request(request).await?;
        let local_route = local_listener
            .as_ref()
            .map(|listener| listener.route().clone());
        if let Some(route) = &local_route {
            self.provider_redactions
                .lock()
                .expect("provider redactions")
                .insert(route.token().to_owned());
        }
        {
            let retired = self
                .retired_provider_attempts
                .lock()
                .expect("provider retired lock");
            if retired.contains(&attempt_id) {
                return Err(Status::cancelled(
                    "attempt revoked during local service admission",
                ));
            }
            self.provider_service_backend
                .bind(&request.tenant_id, attempt_id, route.clone());
            self.provider_attempts
                .lock()
                .expect("provider lease lock")
                .insert(
                    attempt_id,
                    ProviderAttemptLease {
                        tenant_id: request.tenant_id.clone(),
                        session_id: request.session_id,
                        task_id: request.task_id,
                        selection: selection.clone(),
                        facade,
                        native_session_id: None,
                        _local_service_listener: local_listener,
                        service_backend: self.provider_service_backend.clone(),
                        attempt_id,
                    },
                );
        }
        let mut admission_guard = ProviderAdmissionGuard {
            service: self.clone(),
            attempt_id,
            armed: true,
        };
        let factory = self
            .adapter_factory
            .as_ref()
            .ok_or_else(|| Status::failed_precondition("provider worker has no adapter"))?;
        let mut adapter = match factory.build_with_provider(
            &route,
            &selection,
            self.provider_config
                .as_ref()
                .expect("provider config")
                .request_timeout,
            local_route.as_ref(),
        ) {
            Ok(adapter) => adapter,
            Err(error) => {
                self.revoke_provider_attempt(attempt_id);
                return Err(error);
            }
        };
        let capsule = request.capsule.clone().or_else(|| {
            self.provider_capsules
                .lock()
                .expect("provider capsule lock")
                .get(&request.session_id)
                .cloned()
        });
        let native = if let Some(capsule) = capsule {
            adapter.import_session(request.clone(), capsule).await
        } else {
            adapter.create_session(request.clone()).await
        };
        let native = match native {
            Ok(native) => native,
            Err(error) => {
                self.revoke_provider_attempt(attempt_id);
                return Err(self.provider_adapter_status(error));
            }
        };
        if !self
            .provider_attempts
            .lock()
            .expect("provider lease lock")
            .contains_key(&attempt_id)
        {
            let _ = adapter.delete_session(request.clone()).await;
            return Err(Status::cancelled(
                "provider attempt revoked during native admission",
            ));
        }
        // Each native child is attempt-owned. The worker's canonical history is
        // retained separately and carried into the next attempt's prompt.
        let old = self.session_adapters.lock().await.insert(
            request.session_id,
            Arc::new(tokio::sync::Mutex::new(adapter)),
        );
        if let Some(old) = old {
            let mut cleanup = request.clone();
            cleanup.extensions.remove("native_session_id");
            old.lock()
                .await
                .delete_session(cleanup)
                .await
                .map_err(|error| self.provider_adapter_status(error))?;
        }
        let mut attempts = self.provider_attempts.lock().expect("provider lease lock");
        let lease = attempts
            .get_mut(&attempt_id)
            .ok_or_else(|| Status::cancelled("provider attempt revoked during native admission"))?;
        self.provider_native_sessions
            .lock()
            .expect("native session aliases")
            .insert(request.session_id, native.native_session_id.clone());
        lease.native_session_id = Some(native.native_session_id);
        admission_guard.armed = false;
        Ok(())
    }

    fn provider_native_session(
        &self,
        attempt_id: Uuid,
        session_id: Uuid,
        fallback: Option<String>,
    ) -> Option<String> {
        self.provider_attempts
            .lock()
            .expect("provider lease lock")
            .get(&attempt_id)
            .and_then(|lease| lease.native_session_id.clone())
            .or_else(|| {
                self.provider_native_sessions
                    .lock()
                    .expect("native session aliases")
                    .get(&session_id)
                    .cloned()
            })
            .or(fallback)
    }

    async fn provider_prompt(&self, session_id: Uuid, prompt: &str) -> String {
        if self.provider_config.is_none() && self.local_service_gateway.is_none() {
            return prompt.to_owned();
        }
        let history = self.event_history.lock().await;
        let Some(history) = history.get(&session_id).filter(|events| !events.is_empty()) else {
            return prompt.to_owned();
        };
        let events: Vec<Value> = history
            .iter()
            .filter_map(|event| serde_json::from_slice(&event.payload).ok())
            .collect();
        format!(
            "Verified historical OmniSolo session events (context only):\n<omnisolo_session_history>{}</omnisolo_session_history>\nCurrent request: {prompt}",
            serde_json::to_string(&events).expect("JSON history")
        )
    }
}

impl HarnessAdapterFactory {
    fn build_with_provider(
        &self,
        route: &super::provider_facade::ProviderFacadeRoute,
        selection: &ResolvedModelSelection,
        request_timeout: std::time::Duration,
        local_route: Option<&super::local_service_route::LocalServiceRoute>,
    ) -> Result<Box<dyn HarnessAdapter>, Status> {
        match self {
            Self::OmniSolo { descriptor, .. } => {
                let client =
                    OpenAiResponsesClient::new(route.base_url(), route.token(), request_timeout)
                        .map_err(|error| Status::unavailable(error.to_string()))?;
                Ok(Box::new(
                    OmniSoloHarnessAdapterBridge::with_provider_client(descriptor.clone(), client)
                        .with_local_service_route(local_route.cloned()),
                ))
            }
            Self::Process(spec) => {
                let mut spec = bind_process_spec_to_provider(spec.clone(), route, selection);
                if let Some(local_route) = local_route {
                    spec.environment.insert(
                        "OMNISOLO_LOCAL_SERVICE_URL".into(),
                        local_route.base_url().to_owned(),
                    );
                    spec.environment.insert(
                        "OMNISOLO_LOCAL_SERVICE_TOKEN".into(),
                        local_route.token().to_owned(),
                    );
                }
                Ok(Box::new(ProcessHarnessAdapter::new(spec)))
            }
        }
    }
}

#[cfg(test)]
mod local_service_admission_tests {
    use super::super::local_service_gateway::{LocalServiceGateway, SqliteAgentMemoryBackend};
    use super::super::local_services::LocalServiceKind;
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn resumed_portable_services_rebind_to_new_attempt_with_selected_configuration() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let registry = LocalServiceRegistry::with_defaults()
            .with_backend_configuration(b"selected-memory-a")
            .unwrap();
        let mut gateway = LocalServiceGateway::new(registry.clone());
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SqliteAgentMemoryBackend::new(pool)),
        );
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
        );
        let portable = gateway.resolve(scope.clone()).unwrap();
        registry.revoke_attempt(&scope.tenant_id, scope.attempt_id.unwrap());
        let service = HarnessWorkerGrpcService::new("worker", "omnisolo", "pool")
            .with_local_service_gateway(Arc::new(gateway))
            .with_trusted_service_scope(scope.clone());
        let next_attempt = Uuid::new_v4();
        let request =
            HarnessSessionRequest::new(&scope.tenant_id, scope.session_id, Uuid::new_v4())
                .with_task(scope.task_id.unwrap(), "resume")
                .with_attempt_id(next_attempt);
        let parsed =
            apply_request_context(request, &json!({"context":{"local_services":portable}}))
                .expect("portable parser must accept configured historical references");
        let admitted = service
            .admit_local_services(parsed.clone())
            .expect("trusted admission must rebind prior attempt");
        let binding = &admitted.local_service_bundle.as_ref().unwrap().bindings[0];
        assert_eq!(binding.attempt_id, Some(next_attempt));
        assert_eq!(binding.generation, 2);
        assert_ne!(binding.binding_id, portable.bindings[0].binding_id);
        let mut current = scope.clone();
        current.attempt_id = Some(next_attempt);
        registry
            .authorize(binding, &current, "memory.search")
            .unwrap();
        assert!(
            registry
                .authorize(&portable.bindings[0], &scope, "memory.search")
                .is_err()
        );
        let mut forged = parsed;
        forged.local_service_bundle.as_mut().unwrap().bindings[0].configuration_digest =
            format!("sha256:{}", "0".repeat(64));
        assert!(service.admit_local_services(forged).is_err());
    }

    #[tokio::test]
    async fn admission_uses_trusted_namespace_and_exchange_rejects_revoked_bindings() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::query(
            "CREATE VIRTUAL TABLE agent_memory USING fts5(content,tags,created_at UNINDEXED)",
        )
        .execute(&pool)
        .await
        .unwrap();
        let registry = LocalServiceRegistry::with_defaults();
        let mut gateway = LocalServiceGateway::new(registry.clone());
        gateway.register(
            LocalServiceKind::Memory,
            Arc::new(SqliteAgentMemoryBackend::new(pool)),
        );
        let scope = LocalServiceScopeContext::for_attempt(
            "tenant",
            Some("project"),
            Some("workspace"),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Some(Uuid::new_v4()),
        );
        let service = HarnessWorkerGrpcService::new("worker", "omnisolo", "pool")
            .with_local_service_gateway(Arc::new(gateway))
            .with_trusted_service_scope(scope.clone());
        let request =
            HarnessSessionRequest::new(&scope.tenant_id, scope.session_id, Uuid::new_v4())
                .with_task(scope.task_id.unwrap(), "services")
                .with_attempt_id(scope.attempt_id.unwrap());
        let admitted = service.admit_local_services(request.clone()).unwrap();
        let binding = admitted
            .local_service_bundle
            .as_ref()
            .unwrap()
            .binding(LocalServiceKind::Memory)
            .unwrap();
        let write = json!({"local_service":{"binding_id":binding.binding_id,"operation":{"operation":"memory_write","content":"gateway-sentinel"}}});
        service
            .execute_local_service_exchange(&request, &write)
            .await
            .unwrap();
        let read = json!({"local_service":{"binding_id":binding.binding_id,"operation":{"operation":"memory_search","query":"sentinel","limit":10}}});
        assert_eq!(
            service
                .execute_local_service_exchange(&request, &read)
                .await
                .unwrap(),
            json!({"items":["gateway-sentinel"]})
        );
        let mut forged = admitted.clone();
        forged.local_service_bundle.as_mut().unwrap().bindings[0].workspace_id =
            Some("foreign".into());
        assert_eq!(
            service.admit_local_services(forged).unwrap_err().code(),
            tonic::Code::PermissionDenied
        );
        let mut foreign = request.clone();
        foreign.tenant_id = "other-tenant".into();
        assert_eq!(
            service
                .execute_local_service_exchange(&foreign, &read)
                .await
                .unwrap_err()
                .code(),
            tonic::Code::PermissionDenied
        );
        service
            .revoke_local_services(&scope.tenant_id, scope.attempt_id.unwrap())
            .await;
        assert_eq!(
            service
                .execute_local_service_exchange(&request, &read)
                .await
                .unwrap_err()
                .code(),
            tonic::Code::PermissionDenied
        );
    }
}

/// Generate only the child-facing scoped provider configuration.
pub fn bind_process_spec_to_provider(
    mut spec: ProcessHarnessSpec,
    route: &super::provider_facade::ProviderFacadeRoute,
    selection: &ResolvedModelSelection,
) -> ProcessHarnessSpec {
    spec.resolved_model = Some(selection.clone());
    spec.api_base_url = Some(route.base_url().to_owned());
    spec.environment
        .insert("OPENAI_API_KEY".into(), route.token().to_owned());
    spec.environment
        .insert("OPENAI_API_BASE_URL".into(), route.base_url().to_owned());
    spec.environment
        .insert("OPENAI_BASE_URL".into(), route.base_url().to_owned());
    spec.environment
        .insert("OPENAI_MODEL".into(), selection.model_id.clone());
    if let Some(effort) = &selection.reasoning_effort {
        spec.environment.insert(
            "OPENAI_REASONING_EFFORT".into(),
            serde_json::to_value(effort)
                .expect("reasoning JSON")
                .as_str()
                .expect("reasoning string")
                .to_owned(),
        );
    } else {
        spec.environment.remove("OPENAI_REASONING_EFFORT");
    }
    if spec.protocol_kind == super::harness::HarnessProtocolKind::CodexAppServer {
        let base = serde_json::to_string(route.base_url()).expect("base URL JSON");
        let mut overrides = vec![
            "-c".into(),
            "model_provider=\"omnisolo\"".into(),
            "-c".into(),
            "model_providers.omnisolo.name=\"OpenAI\"".into(),
            "-c".into(),
            format!("model_providers.omnisolo.base_url={base}"),
            "-c".into(),
            "model_providers.omnisolo.wire_api=\"responses\"".into(),
            "-c".into(),
            "model_providers.omnisolo.env_key=\"OPENAI_API_KEY\"".into(),
            "-c".into(),
            "model_providers.omnisolo.requires_openai_auth=true".into(),
        ];
        overrides.append(&mut spec.args);
        spec.args = overrides;
    }
    spec
}
