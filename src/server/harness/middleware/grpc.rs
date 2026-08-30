use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use serde_json::Value;
use server_ohc::harness_middleware::harness_worker_service_server::HarnessWorkerService;
use server_ohc::harness_middleware::{
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
use super::local_services::{LOCAL_SERVICE_BUNDLE_SCHEMA, LocalServiceBundle};
use super::openhands::OpenHandsProviderErrorKind;
use super::protocol::{AttemptOperation, HarnessExecutionItem};
use super::types::{ResolvedModelSelection, sanitize_credential_value};
use super::worker::{
    AttemptCommandEnvelope, AttemptCommandKind, SessionOperationEnvelope, SessionOperationKind,
    WorkerControlEnvelope, WorkerControlKind,
};
use super::worker_runtime::HarnessWorkerRuntime;

#[derive(Clone)]
pub struct HarnessWorkerGrpcService {
    runtime: Arc<Mutex<HarnessWorkerRuntime>>,
    adapter_factory: Option<Arc<HarnessAdapterFactory>>,
    session_adapters:
        Arc<tokio::sync::Mutex<BTreeMap<Uuid, Arc<tokio::sync::Mutex<Box<dyn HarnessAdapter>>>>>>,
    durable_sequences: Arc<tokio::sync::Mutex<BTreeMap<Uuid, i64>>>,
    event_history: Arc<tokio::sync::Mutex<BTreeMap<Uuid, Vec<EventDeliveryEnvelope>>>>,
    delivery_acknowledgements: Arc<tokio::sync::Mutex<BTreeMap<String, i64>>>,
    default_resolved_model: Option<ResolvedModelSelection>,
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
            default_resolved_model: None,
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
            default_resolved_model,
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
        let mut adapter = factory.build();
        let request = self.apply_default_resolved_model(HarnessSessionRequest::new(
            "__omnisolo_preflight__",
            Uuid::new_v4(),
            Uuid::new_v4(),
        ));
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
        let payload = sanitize_credential_value(&event.payload);
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
                "event_type": event.event_type,
                "payload": payload,
                "native_cursor": event.native_cursor,
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

    async fn run_concurrent_attempt(
        &self,
        command: AttemptCommandEnvelope,
        adapter: Arc<tokio::sync::Mutex<Box<dyn HarnessAdapter>>>,
        operation: AttemptOperation,
        request: HarnessSessionRequest,
        attempt_id: String,
        prompt: String,
        native_session_id: Option<String>,
        sender: tokio::sync::mpsc::Sender<Result<EventDeliveryEnvelope, Status>>,
    ) {
        let mut events = Vec::new();
        self.append_local_services_bound(&command, &request, &mut events)
            .await;
        if let Some(delivery) = events.last().cloned()
            && sender.send(Ok(delivery)).await.is_err()
        {
            return;
        }
        let stream_result = {
            let mut adapter = adapter.lock().await;
            adapter
                .attempt_stream(
                    operation,
                    request,
                    &attempt_id,
                    &prompt,
                    native_session_id.as_deref(),
                )
                .await
        };
        let mut stream = match stream_result {
            Ok(stream) => stream,
            Err(error) => {
                if let Ok(mut runtime) = self.runtime() {
                    runtime.rollback_attempt_command(&command);
                }
                let _ = sender.send(Err(adapter_status(error))).await;
                return;
            }
        };
        let mut final_text = None;
        let mut usage = None;
        while let Some(item) = stream.next().await {
            match item {
                Ok(HarnessExecutionItem::Event(event)) => {
                    self.append_attempt_event(&command, event, &mut events)
                        .await;
                    if let Some(delivery) = events.last().cloned() {
                        if sender.send(Ok(delivery)).await.is_err() {
                            return;
                        }
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
                    let _ = sender.send(Err(adapter_status(error))).await;
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
                if let Some(delivery) = events.last().cloned() {
                    if sender.send(Ok(delivery)).await.is_err() {
                        return;
                    }
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
                if let Some(delivery) = events.last().cloned() {
                    if sender.send(Ok(delivery)).await.is_err() {
                        return;
                    }
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
        if !result.duplicate {
            if let Some(adapter) = self.adapter_for_session(command.session_id).await {
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
                        return Err(adapter_status(error));
                    }
                };
                payload =
                    serde_json::to_vec(&response).expect("JSON values are always serializable");
            }
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
        let result = self
            .runtime()?
            .handle_control(envelope)
            .map_err(runtime_status)?;
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
        let mut payload = Vec::new();
        let mut payload_schema = String::new();
        let mut payload_version = 0;
        if !result.duplicate {
            if let Some(adapter) = self.adapter_for_session(envelope.session_id).await {
                let request = adapter_request.clone();
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
                        adapter
                            .lock()
                            .await
                            .control_session(request, operation, envelope.payload.clone())
                            .await
                    }
                };
                let native_session = match operation_result {
                    Ok(native_session) => native_session,
                    Err(error) => {
                        self.runtime()?.rollback_session_operation(&envelope);
                        return Err(adapter_status(error));
                    }
                };
                payload = serde_json::to_vec(&native_session)
                    .expect("NativeSession is always serializable");
                payload_schema = "omnisolo.harness.native_session.v1".to_owned();
                payload_version = 1;
                if matches!(
                    envelope.kind,
                    SessionOperationKind::Close | SessionOperationKind::Delete
                ) {
                    self.remove_session_adapter(envelope.session_id).await;
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
        let adapter_request = if self.has_adapter_factory() {
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
        let mut events = Vec::new();
        if !accepted.duplicate {
            if let Some((stream_id, sequence)) = delivery_acknowledgement {
                self.delivery_acknowledgements
                    .lock()
                    .await
                    .entry(stream_id)
                    .and_modify(|current| *current = (*current).max(sequence))
                    .or_insert(sequence);
            }
            if replay_range.is_none() {
                if let Some(adapter) = self.adapter_for_session(command.session_id).await {
                    let concurrent = adapter.lock().await.supports_concurrent_streaming();
                    if concurrent && command.kind != AttemptCommandKind::Checkpoint {
                        let operation =
                            attempt_stream_operation(&command.kind).ok_or_else(|| {
                                Status::invalid_argument("attempt kind cannot produce a stream")
                            })?;
                        let request = adapter_request
                            .clone()
                            .expect("adapter request prepared when an adapter is present");
                        let native_session_id = command
                            .payload
                            .get("native_session_id")
                            .and_then(Value::as_str)
                            .map(str::to_owned);
                        let (sender, receiver) = tokio::sync::mpsc::channel(64);
                        let service = self.clone();
                        let adapter = Arc::clone(&adapter);
                        let attempt_id = command.attempt_id.to_string();
                        let prompt = prompt.to_owned();
                        let command_for_task = command.clone();
                        tokio::spawn(async move {
                            service
                                .run_concurrent_attempt(
                                    command_for_task,
                                    adapter,
                                    operation,
                                    request,
                                    attempt_id,
                                    prompt,
                                    native_session_id,
                                    sender,
                                )
                                .await;
                        });
                        return Ok(Response::new(ReceiverStream::new(receiver)));
                    }
                }
            }
            if replay_range.is_none() {
                if let Some(adapter) = self.adapter_for_session(command.session_id).await {
                    let native_session_id = command
                        .payload
                        .get("native_session_id")
                        .and_then(Value::as_str);
                    let request = adapter_request
                        .clone()
                        .expect("adapter request prepared when an adapter is present");
                    let attempt_id = command.attempt_id.to_string();
                    self.append_local_services_bound(&command, &request, &mut events)
                        .await;
                    if command.kind == AttemptCommandKind::Checkpoint {
                        let checkpoint =
                            adapter.lock().await.checkpoint(request, &attempt_id).await;
                        let checkpoint = match checkpoint {
                            Ok(checkpoint) => checkpoint,
                            Err(error) => {
                                self.runtime()?.rollback_attempt_command(&command);
                                return Err(adapter_status(error));
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
                        let operation =
                            attempt_stream_operation(&command.kind).ok_or_else(|| {
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
                                return Err(adapter_status(error));
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
                                    return Err(adapter_status(error));
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
        }
        if !accepted.duplicate {
            if let Some((from_sequence, to_sequence)) = replay_range {
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
        }
        if events.is_empty() {
            events.push(EventDeliveryEnvelope {
                protocol_version: 1,
                tenant_id: command.tenant_id,
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
                Some(serde_json::from_value::<LocalServiceBundle>(local_services.clone()).map_err(
                    |_| Status::invalid_argument("context local_services is invalid"),
                )?)
            };
            if let Some(bundle) = &request.local_service_bundle {
                bundle
                    .validate()
                    .map_err(|_| Status::invalid_argument("context local_services is invalid"))?;
                if bundle.bindings.iter().any(|binding| {
                    binding.tenant_id != request.tenant_id
                        || binding.session_id != request.session_id
                        || binding.task_id != request.task_id
                        || request
                            .attempt_id
                            .is_some_and(|attempt_id| binding.attempt_id != Some(attempt_id))
                }) {
                    return Err(Status::invalid_argument(
                        "context local_services identity does not match request",
                    ));
                }
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
    if let Some(context) = payload.get("context") {
        if let Some(extensions) = context
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
