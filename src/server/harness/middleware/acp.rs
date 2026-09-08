use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::{Arc, RwLock};

use serde_json::{Map, Value, json};

use super::harness::{HarnessAdapterError, HarnessEvent, HarnessSessionRequest, NativeSession};
use super::json_rpc::{JsonRpcErrorObject, JsonRpcNotification, JsonRpcServerRequest};
use super::protocol::{
    AttemptOperation, HarnessProtocolCodec, JsonRpcRequestSpec, NativeTurnState,
    ProtocolAttemptInstruction, ProtocolEvent, ProtocolSessionConfiguration, ServerResponse,
    SessionOperation,
};
use super::types::{ReasoningEffort, sanitize_credential_value};

pub const ACP_PROTOCOL_VERSION: u16 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpServerInfo {
    pub protocol_version: u16,
    pub agent_name: Option<String>,
    pub agent_version: Option<String>,
    pub load_session: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpModelCompatibility {
    StableConfigOptions,
    LegacySetModel,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcpModelConfigurationPlan {
    pub compatibility: AcpModelCompatibility,
    pub requests: Vec<JsonRpcRequestSpec>,
    pub events: Vec<HarnessEvent>,
}

#[derive(Clone, Debug)]
struct AcpSelectOption {
    config_id: String,
    values: BTreeSet<String>,
}

#[derive(Clone, Debug)]
enum AcpModelRoute {
    Stable {
        model: AcpSelectOption,
        thought: Option<AcpSelectOption>,
    },
    Legacy {
        available_models: BTreeSet<String>,
    },
    Unsupported,
}

#[derive(Clone, Debug)]
pub struct AcpV1Codec {
    cwd: String,
    model_id: Option<String>,
    reasoning_effort: Option<ReasoningEffort>,
    model_routes: Arc<RwLock<BTreeMap<String, AcpModelRoute>>>,
}

impl AcpV1Codec {
    pub fn for_kimi(
        cwd: impl Into<String>,
        request: &HarnessSessionRequest,
    ) -> Result<Self, HarnessAdapterError> {
        let cwd = cwd.into();
        if cwd.trim().is_empty() || !Path::new(&cwd).is_absolute() {
            return Err(invalid_request(
                "ACP session cwd must be a non-empty absolute path",
            ));
        }

        let (model_id, reasoning_effort) = request
            .resolved_model
            .as_ref()
            .map(|selection| {
                (
                    Some(selection.model_id.trim().to_owned()),
                    selection.reasoning_effort.clone(),
                )
            })
            .unwrap_or((None, None));
        if model_id.as_deref().is_some_and(str::is_empty) {
            return Err(invalid_request("ACP resolved model id must not be empty"));
        }

        Ok(Self {
            cwd,
            model_id,
            reasoning_effort,
            model_routes: Arc::new(RwLock::new(BTreeMap::new())),
        })
    }

    pub fn initialize_request(&self) -> JsonRpcRequestSpec {
        JsonRpcRequestSpec {
            method: "initialize".to_owned(),
            params: json!({
                "protocolVersion": ACP_PROTOCOL_VERSION,
                "clientCapabilities": {
                    "fs": {
                        "readTextFile": false,
                        "writeTextFile": false,
                    },
                    "terminal": false,
                    "auth": {"terminal": false},
                },
                "clientInfo": {
                    "name": "omnisolo",
                    "title": "OmniSolo Harness Middleware",
                    "version": env!("CARGO_PKG_VERSION"),
                },
            }),
        }
    }

    pub fn decode_initialize_result(
        &self,
        result: Value,
    ) -> Result<AcpServerInfo, HarnessAdapterError> {
        let result = required_object(&result, "ACP initialize result")?;
        let protocol_version = result
            .get("protocolVersion")
            .and_then(Value::as_u64)
            .ok_or_else(|| invalid_response("ACP initialize protocolVersion must be an integer"))?;
        if protocol_version != u64::from(ACP_PROTOCOL_VERSION) {
            return Err(invalid_response(format!(
                "ACP protocol version {protocol_version} is incompatible with version {ACP_PROTOCOL_VERSION}"
            )));
        }
        let agent_capabilities = optional_object(
            result.get("agentCapabilities"),
            "ACP initialize agentCapabilities",
        )?;
        let load_session = match agent_capabilities
            .as_ref()
            .and_then(|capabilities| capabilities.get("loadSession"))
        {
            Some(value) => value
                .as_bool()
                .ok_or_else(|| invalid_response("ACP initialize loadSession must be a boolean"))?,
            None => false,
        };
        let agent_info = optional_object(result.get("agentInfo"), "ACP initialize agentInfo")?;
        let agent_name = agent_info
            .as_ref()
            .and_then(|info| info.get("name"))
            .map(|value| required_string(Some(value), "ACP initialize agentInfo.name"))
            .transpose()?
            .map(str::to_owned);
        let agent_version = agent_info
            .as_ref()
            .and_then(|info| info.get("version"))
            .map(|value| required_string(Some(value), "ACP initialize agentInfo.version"))
            .transpose()?
            .map(str::to_owned);

        Ok(AcpServerInfo {
            protocol_version: ACP_PROTOCOL_VERSION,
            agent_name,
            agent_version,
            load_session,
        })
    }

    pub fn model_configuration_plan(
        &self,
        native_session_id: &str,
    ) -> Result<AcpModelConfigurationPlan, HarnessAdapterError> {
        let session_id = required_non_empty(
            native_session_id,
            "ACP native session id is required for model configuration",
        )?;
        let model_id = self
            .model_id
            .as_deref()
            .ok_or_else(|| invalid_request("ACP resolved model is required for configuration"))?;
        let route = self
            .model_routes
            .read()
            .map_err(|_| invalid_request("ACP model capability state is unavailable"))?
            .get(session_id)
            .cloned()
            .ok_or_else(|| {
                invalid_request("ACP session has not advertised model configuration capabilities")
            })?;

        match route {
            AcpModelRoute::Stable { model, thought } => {
                if !model.values.contains(model_id) {
                    return Err(unadvertised_model(model_id, "ACP configOptions"));
                }
                let mut requests = vec![set_config_option_request(
                    session_id,
                    &model.config_id,
                    model_id,
                )];
                let mut events = Vec::new();
                let has_thought_option = thought.is_some();
                if let Some(thought) = thought {
                    if let Some(value) =
                        select_reasoning_value(self.reasoning_effort.as_ref(), &thought.values)
                    {
                        requests.push(set_config_option_request(
                            session_id,
                            &thought.config_id,
                            &value,
                        ));
                        if let Some(requested) = self.reasoning_effort.as_ref()
                            && reasoning_rank(&value)
                                .is_some_and(|rank| rank < reasoning_effort_rank(requested))
                        {
                            events.push(self.downgrade_event(
                                session_id,
                                "reasoning_effort",
                                reasoning_effort_name(requested),
                                &value,
                                "agent_advertised_no_matching_thought_level",
                            ));
                        }
                    } else if let Some(requested) = self.reasoning_effort.as_ref()
                        && !matches!(requested, ReasoningEffort::None | ReasoningEffort::Custom)
                    {
                        events.push(self.downgrade_event(
                            session_id,
                            "reasoning_effort",
                            reasoning_effort_name(requested),
                            "none",
                            "agent_advertised_only_stronger_thought_levels",
                        ));
                    }
                }
                if !has_thought_option
                    && self.reasoning_effort.as_ref().is_some_and(|effort| {
                        !matches!(effort, ReasoningEffort::None | ReasoningEffort::Custom)
                    })
                {
                    let requested = reasoning_effort_name(
                        self.reasoning_effort
                            .as_ref()
                            .expect("reasoning effort was checked above"),
                    );
                    events.push(self.downgrade_event(
                        session_id,
                        "reasoning_effort",
                        requested,
                        "none",
                        "agent_advertised_no_reasoning_config_option",
                    ));
                }
                Ok(AcpModelConfigurationPlan {
                    compatibility: AcpModelCompatibility::StableConfigOptions,
                    requests,
                    events,
                })
            }
            AcpModelRoute::Legacy { available_models } => {
                let thinking = self.reasoning_effort.as_ref().is_some_and(|effort| {
                    !matches!(effort, ReasoningEffort::None | ReasoningEffort::Custom)
                });
                let effective_model_id = if thinking && !model_id.ends_with(",thinking") {
                    format!("{model_id},thinking")
                } else {
                    model_id.to_owned()
                };
                if !available_models.contains(&effective_model_id) {
                    return Err(unadvertised_model(&effective_model_id, "legacy ACP models"));
                }

                let mut events = vec![self.compatibility_downgrade_event(session_id)];
                if let Some(requested) = self.reasoning_effort.as_ref()
                    && !matches!(requested, ReasoningEffort::None | ReasoningEffort::Custom)
                {
                    events.push(self.downgrade_event(
                        session_id,
                        "reasoning_effort",
                        reasoning_effort_name(requested),
                        "thinking",
                        "kimi_acp_1_49_supports_boolean_thinking_only",
                    ));
                }
                Ok(AcpModelConfigurationPlan {
                    compatibility: AcpModelCompatibility::LegacySetModel,
                    requests: vec![JsonRpcRequestSpec {
                        method: "session/set_model".to_owned(),
                        params: json!({
                            "sessionId": session_id,
                            "modelId": effective_model_id,
                        }),
                    }],
                    events,
                })
            }
            AcpModelRoute::Unsupported => Err(invalid_request(
                "ACP session did not advertise model configuration capabilities",
            )),
        }
    }

    pub fn cancel_notification(
        &self,
        native_session_id: &str,
    ) -> Result<JsonRpcNotification, HarnessAdapterError> {
        Ok(JsonRpcNotification {
            method: "session/cancel".to_owned(),
            params: json!({
                "sessionId": required_non_empty(
                    native_session_id,
                    "ACP native session id is required for cancellation",
                )?
            }),
        })
    }

    /// ACP v1 defines no shutdown request; the client closes the transport instead.
    pub fn shutdown_request(&self) -> Option<JsonRpcRequestSpec> {
        None
    }

    fn record_model_route(
        &self,
        session_id: &str,
        result: &Map<String, Value>,
    ) -> Result<(), HarnessAdapterError> {
        let route = parse_model_route(result)?;
        self.model_routes
            .write()
            .map_err(|_| invalid_response("ACP model capability state is unavailable"))?
            .insert(session_id.to_owned(), route);
        Ok(())
    }

    fn compatibility_downgrade_event(&self, session_id: &str) -> HarnessEvent {
        self.downgrade_event(
            session_id,
            "acp_model_configuration",
            "session/set_config_option",
            "session/set_model",
            "agent_advertised_legacy_session_model_state",
        )
    }

    fn downgrade_event(
        &self,
        session_id: &str,
        capability: &str,
        requested: &str,
        effective: &str,
        reason: &str,
    ) -> HarnessEvent {
        HarnessEvent {
            event_type: "capability.downgraded".to_owned(),
            durable: true,
            payload: json!({
                "capability": capability,
                "requested": requested,
                "effective": effective,
                "reason": reason,
                "protocol": "acp_v1",
                "session_id": session_id,
                "model_id": self.model_id,
            }),
            native_cursor: None,
        }
    }

    pub fn decode_prompt_result(
        &self,
        native_session_id: &str,
        result: Value,
    ) -> Result<ProtocolEvent, HarnessAdapterError> {
        let session_id = required_non_empty(
            native_session_id,
            "ACP native session id is required for prompt completion",
        )?;
        let result = required_object(&result, "ACP session/prompt result")?;
        let stop_reason =
            required_string(result.get("stopReason"), "ACP session/prompt stopReason")?;
        let event_type = match stop_reason {
            "end_turn" => "turn.completed",
            "cancelled" => "turn.cancelled",
            "max_tokens" | "max_turn_requests" | "refusal" => "turn.failed",
            _ => {
                return Err(invalid_response(format!(
                    "unsupported ACP session/prompt stopReason: {stop_reason}"
                )));
            }
        };
        let usage = optional_object(result.get("usage"), "ACP session/prompt usage")?
            .map(Value::Object)
            .map(|value| sanitize_credential_value(&value));
        let native = sanitize_credential_value(&Value::Object(result.clone()));
        let cursor = format!("{session_id}:prompt");
        Ok(ProtocolEvent {
            event: HarnessEvent {
                event_type: event_type.to_owned(),
                durable: true,
                payload: json!({
                    "session_id": session_id,
                    "stop_reason": stop_reason,
                    "usage": usage,
                    "native": native,
                }),
                native_cursor: Some(cursor.clone()),
            },
            native_cursor: Some(cursor),
            final_text: None,
            usage,
            terminal: true,
        })
    }

    fn session_params(&self) -> Value {
        json!({"cwd": self.cwd, "mcpServers": []})
    }

    fn unsupported(&self, operation: &str) -> HarnessAdapterError {
        invalid_request(format!(
            "ACP v1 operation is not supported by this codec: {operation}"
        ))
    }
}

impl HarnessProtocolCodec for AcpV1Codec {
    fn initialize_request(&self) -> JsonRpcRequestSpec {
        Self::initialize_request(self)
    }

    fn session_request(
        &self,
        operation: SessionOperation,
        _request: &HarnessSessionRequest,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        match operation {
            SessionOperation::Create => Ok(JsonRpcRequestSpec {
                method: "session/new".to_owned(),
                params: self.session_params(),
            }),
            SessionOperation::Resume { native_session_id } => {
                let mut params = self
                    .session_params()
                    .as_object()
                    .cloned()
                    .expect("session params are always an object");
                params.insert(
                    "sessionId".to_owned(),
                    Value::String(
                        required_non_empty(
                            &native_session_id,
                            "ACP native session id is required for session/load",
                        )?
                        .to_owned(),
                    ),
                );
                Ok(JsonRpcRequestSpec {
                    method: "session/load".to_owned(),
                    params: Value::Object(params),
                })
            }
            SessionOperation::Cancel | SessionOperation::Quiesce => {
                Err(self
                    .unsupported("session cancellation must be emitted with cancel_notification"))
            }
            SessionOperation::Import(_) => Err(self.unsupported("session import")),
            SessionOperation::Fork { .. } => Err(self.unsupported("session fork")),
            SessionOperation::Snapshot => Err(self.unsupported("session snapshot")),
            SessionOperation::Close => Err(self.unsupported("session close")),
            SessionOperation::Delete => Err(self.unsupported("session delete")),
        }
    }

    fn attempt_request(
        &self,
        operation: AttemptOperation,
        _request: &HarnessSessionRequest,
        prompt: &str,
        native_session_id: Option<&str>,
        _native_turn_id: Option<&str>,
    ) -> Result<JsonRpcRequestSpec, HarnessAdapterError> {
        let session_id = required_non_empty(
            native_session_id.unwrap_or_default(),
            "ACP native session id is required",
        )?;
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => {
                if prompt.trim().is_empty() {
                    return Err(invalid_request("ACP session/prompt content is required"));
                }
                Ok(JsonRpcRequestSpec {
                    method: "session/prompt".to_owned(),
                    params: json!({
                        "sessionId": session_id,
                        "prompt": [{"type": "text", "text": prompt}],
                    }),
                })
            }
            AttemptOperation::Cancel | AttemptOperation::Quiesce => {
                Err(self
                    .unsupported("session cancellation is a notification; use attempt_instruction"))
            }
            AttemptOperation::Steer => Err(self.unsupported("prompt steering")),
            AttemptOperation::Reconcile => Err(self.unsupported("attempt reconcile")),
        }
    }

    fn attempt_instruction(
        &self,
        operation: AttemptOperation,
        request: &HarnessSessionRequest,
        prompt: &str,
        native_session_id: Option<&str>,
        native_turn_id: Option<&str>,
    ) -> Result<ProtocolAttemptInstruction, HarnessAdapterError> {
        match operation {
            AttemptOperation::Cancel | AttemptOperation::Quiesce => {
                let session_id = required_non_empty(
                    native_session_id.unwrap_or_default(),
                    "ACP native session id is required for cancellation",
                )?;
                self.cancel_notification(session_id)
                    .map(ProtocolAttemptInstruction::Notification)
            }
            _ => self
                .attempt_request(
                    operation,
                    request,
                    prompt,
                    native_session_id,
                    native_turn_id,
                )
                .map(ProtocolAttemptInstruction::Request),
        }
    }

    fn decode_session_result(
        &self,
        operation: SessionOperation,
        result: Value,
    ) -> Result<NativeSession, HarnessAdapterError> {
        match operation {
            SessionOperation::Create => {
                let result = required_object(&result, "ACP session/new result")?;
                let session_id =
                    required_string(result.get("sessionId"), "ACP session/new result.sessionId")?;
                self.record_model_route(session_id, result)?;
                Ok(NativeSession {
                    native_session_id: session_id.to_owned(),
                    native_cursor: None,
                })
            }
            SessionOperation::Resume { native_session_id } => {
                let result = required_object(&result, "ACP session/load result")?;
                let native_session_id = required_non_empty(
                    &native_session_id,
                    "ACP session/load native session id is required",
                )?;
                self.record_model_route(native_session_id, result)?;
                Ok(NativeSession {
                    native_session_id: native_session_id.to_owned(),
                    native_cursor: None,
                })
            }
            _ => Err(invalid_response(
                "ACP session result does not match session/new or session/load",
            )),
        }
    }

    fn session_configuration(
        &self,
        native_session_id: &str,
    ) -> Result<ProtocolSessionConfiguration, HarnessAdapterError> {
        if self.model_id.is_none() {
            return Ok(ProtocolSessionConfiguration::default());
        }
        let plan = self.model_configuration_plan(native_session_id)?;
        Ok(ProtocolSessionConfiguration {
            requests: plan.requests,
            events: plan.events,
        })
    }

    fn decode_attempt_result(
        &self,
        operation: AttemptOperation,
        native_session_id: &str,
        result: Value,
    ) -> Result<Option<ProtocolEvent>, HarnessAdapterError> {
        match operation {
            AttemptOperation::Start | AttemptOperation::Execute | AttemptOperation::Resume => self
                .decode_prompt_result(native_session_id, result)
                .map(Some),
            AttemptOperation::Cancel | AttemptOperation::Quiesce => Ok(None),
            AttemptOperation::Steer | AttemptOperation::Reconcile => {
                Err(self.unsupported("ACP attempt result for unsupported operation"))
            }
        }
    }

    fn decode_notification(
        &self,
        notification: &JsonRpcNotification,
        state: &mut NativeTurnState,
    ) -> Result<ProtocolEvent, HarnessAdapterError> {
        if notification.method != "session/update" {
            return Err(invalid_response(format!(
                "unsupported ACP notification method: {}",
                notification.method
            )));
        }
        let params = required_object(&notification.params, "ACP session/update params")?;
        let session_id = required_string(params.get("sessionId"), "ACP session/update sessionId")?;
        if session_id != state.thread_id {
            return Err(invalid_response(
                "ACP session/update session id does not match the active session",
            ));
        }
        let update = params
            .get("update")
            .and_then(Value::as_object)
            .ok_or_else(|| invalid_response("ACP session/update update must be an object"))?;
        let update_type = required_string(
            update.get("sessionUpdate"),
            "ACP session/update update.sessionUpdate",
        )?;
        let native = sanitize_credential_value(&json!({
            "method": notification.method,
            "params": notification.params,
        }));
        let (event_type, durable, payload, usage) = decode_update(update_type, update)?;
        let mut payload = sanitize_credential_value(&payload);
        let usage = usage.map(|value| sanitize_credential_value(&value));
        payload
            .as_object_mut()
            .expect("ACP canonical event payloads are objects")
            .insert("native".to_owned(), native);

        state.notification_sequence = state.notification_sequence.saturating_add(1);
        let cursor = format!("{session_id}:{}", state.notification_sequence);
        state.native_cursor = Some(cursor.clone());
        Ok(ProtocolEvent {
            event: HarnessEvent {
                event_type: event_type.to_owned(),
                durable,
                payload,
                native_cursor: Some(cursor.clone()),
            },
            native_cursor: Some(cursor),
            final_text: None,
            usage,
            terminal: false,
        })
    }

    fn server_request_event(
        &self,
        request: &JsonRpcServerRequest,
    ) -> Result<HarnessEvent, HarnessAdapterError> {
        if request.method != "session/request_permission" {
            return Err(invalid_response(format!(
                "unsupported ACP server-to-client request method: {}",
                request.method
            )));
        }
        validate_permission_request(&request.params)?;
        Ok(HarnessEvent {
            event_type: "interaction.required".to_owned(),
            durable: true,
            payload: json!({
                "native_request_id": request.id.as_value(),
                "native_method": request.method,
                "native_params": sanitize_credential_value(&request.params),
            }),
            native_cursor: None,
        })
    }

    fn encode_server_response(
        &self,
        method: &str,
        response: &Value,
    ) -> Result<ServerResponse, HarnessAdapterError> {
        if method != "session/request_permission" {
            return Err(invalid_request(format!(
                "unsupported ACP server-to-client response method: {method}"
            )));
        }
        if let Some(error) = response.get("error") {
            let error = required_object(error, "ACP permission response error")?;
            return Ok(ServerResponse::Error(JsonRpcErrorObject {
                code: error.get("code").and_then(Value::as_i64).unwrap_or(-32000),
                message: error
                    .get("message")
                    .and_then(Value::as_str)
                    .unwrap_or("OmniSolo rejected ACP permission request")
                    .to_owned(),
                data: error.get("data").cloned(),
            }));
        }
        let result = response.get("result").ok_or_else(|| {
            invalid_request("ACP permission response must contain result or error")
        })?;
        validate_permission_response(result)?;
        Ok(ServerResponse::Result(result.clone()))
    }
}

fn parse_model_route(result: &Map<String, Value>) -> Result<AcpModelRoute, HarnessAdapterError> {
    let mut stable_model = None;
    let mut stable_thought = None;
    if let Some(config_options) = result.get("configOptions") {
        let config_options = required_array(Some(config_options), "ACP configOptions")?;
        for option in config_options {
            let option = option
                .as_object()
                .ok_or_else(|| invalid_response("ACP config option must be an object"))?;
            let category = required_string(option.get("category"), "ACP config option category")?;
            if !matches!(category, "model" | "thought_level" | "reasoning_effort") {
                continue;
            }
            let option_type = required_string(option.get("type"), "ACP config option type")?;
            if option_type != "select" {
                return Err(invalid_response(format!(
                    "ACP {category} config option must be a select"
                )));
            }
            let config_id = required_string(option.get("id"), "ACP config option id")?.to_owned();
            let values = required_array(option.get("options"), "ACP config option options")?
                .iter()
                .map(|value| {
                    let value = value.as_object().ok_or_else(|| {
                        invalid_response("ACP config option value must be an object")
                    })?;
                    required_string(value.get("value"), "ACP config option value")
                        .map(str::to_owned)
                })
                .collect::<Result<BTreeSet<_>, _>>()?;
            if values.is_empty() {
                return Err(invalid_response(
                    "ACP config option values must not be empty",
                ));
            }
            if matches!(category, "thought_level" | "reasoning_effort")
                && values.iter().any(|value| reasoning_rank(value).is_none())
            {
                return Err(invalid_response(
                    "ACP reasoning config option advertised an unknown level",
                ));
            }
            let parsed = AcpSelectOption { config_id, values };
            match category {
                "model" => {
                    if stable_model.replace(parsed).is_some() {
                        return Err(invalid_response(
                            "ACP advertised duplicate model config options",
                        ));
                    }
                }
                "thought_level" | "reasoning_effort" => {
                    if stable_thought.replace(parsed).is_some() {
                        return Err(invalid_response(
                            "ACP advertised duplicate reasoning config options",
                        ));
                    }
                }
                _ => unreachable!("non-routing categories were skipped"),
            }
        }
    }
    if let Some(model) = stable_model {
        return Ok(AcpModelRoute::Stable {
            model,
            thought: stable_thought,
        });
    }

    if let Some(models) = result.get("models") {
        let models = required_object(models, "ACP legacy models")?;
        let available_models =
            required_array(models.get("availableModels"), "ACP legacy availableModels")?
                .iter()
                .map(|model| {
                    let model = model
                        .as_object()
                        .ok_or_else(|| invalid_response("ACP legacy model must be an object"))?;
                    required_string(model.get("modelId"), "ACP legacy modelId").map(str::to_owned)
                })
                .collect::<Result<BTreeSet<_>, _>>()?;
        if available_models.is_empty() {
            return Err(invalid_response(
                "ACP legacy availableModels must not be empty",
            ));
        }
        return Ok(AcpModelRoute::Legacy { available_models });
    }

    Ok(AcpModelRoute::Unsupported)
}

fn set_config_option_request(session_id: &str, config_id: &str, value: &str) -> JsonRpcRequestSpec {
    JsonRpcRequestSpec {
        method: "session/set_config_option".to_owned(),
        params: json!({
            "sessionId": session_id,
            "configId": config_id,
            "value": value,
        }),
    }
}

fn select_reasoning_value(
    effort: Option<&ReasoningEffort>,
    advertised: &BTreeSet<String>,
) -> Option<String> {
    let requested = match effort {
        Some(ReasoningEffort::None) => "none",
        Some(ReasoningEffort::Minimal) => "minimal",
        Some(ReasoningEffort::Low) => "low",
        Some(ReasoningEffort::Medium) => "medium",
        Some(ReasoningEffort::High) => "high",
        Some(ReasoningEffort::Max) => "max",
        Some(ReasoningEffort::Custom) | None => return None,
    };
    let requested_rank = reasoning_rank(requested).expect("portable reasoning names are known");
    advertised
        .iter()
        .filter(|value| reasoning_rank(value).is_some_and(|rank| rank <= requested_rank))
        .max_by_key(|value| reasoning_rank(value).expect("advertised values were validated"))
        .cloned()
}

fn reasoning_effort_name(effort: &ReasoningEffort) -> &'static str {
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

fn reasoning_effort_rank(effort: &ReasoningEffort) -> u8 {
    reasoning_rank(reasoning_effort_name(effort)).unwrap_or(0)
}

fn reasoning_rank(value: &str) -> Option<u8> {
    match value {
        "none" | "off" | "disabled" => Some(0),
        "minimal" => Some(1),
        "low" => Some(2),
        "medium" | "thinking" => Some(3),
        "high" => Some(4),
        "max" | "xhigh" => Some(5),
        _ => None,
    }
}

fn unadvertised_model(model_id: &str, source: &str) -> HarnessAdapterError {
    invalid_request(format!(
        "resolved model {model_id} was not advertised by {source}"
    ))
}

fn decode_update(
    update_type: &str,
    update: &Map<String, Value>,
) -> Result<(&'static str, bool, Value, Option<Value>), HarnessAdapterError> {
    let decoded = match update_type {
        "agent_message_chunk" | "agent_thought_chunk" | "user_message_chunk" => {
            let content = update
                .get("content")
                .and_then(Value::as_object)
                .ok_or_else(|| invalid_response("ACP content chunk content must be an object"))?;
            let content_type = required_string(content.get("type"), "ACP content chunk type")?;
            if content_type != "text" {
                return Err(invalid_response(format!(
                    "unsupported ACP streamed content type: {content_type}"
                )));
            }
            // Streaming chunks may be empty or contain only whitespace. Unlike
            // identifiers, their contents must be preserved without trimming.
            let text = content
                .get("text")
                .and_then(Value::as_str)
                .ok_or_else(|| invalid_response("ACP text content must be a string"))?;
            let event_type = match update_type {
                "agent_message_chunk" => "assistant.text_chunk",
                "agent_thought_chunk" => "assistant.reasoning",
                _ => "user.text_chunk",
            };
            (event_type, false, json!({"content": text}), None)
        }
        "tool_call" => {
            let tool_call_id = required_string(update.get("toolCallId"), "ACP toolCallId")?;
            let title = required_string(update.get("title"), "ACP tool call title")?;
            validate_tool_status(update.get("status"))?;
            validate_tool_content(update.get("content"))?;
            (
                "tool.started",
                true,
                json!({
                    "tool_call_id": tool_call_id,
                    "title": title,
                    "tool": Value::Object(update.clone()),
                }),
                None,
            )
        }
        "tool_call_update" => {
            let tool_call_id = required_string(update.get("toolCallId"), "ACP toolCallId")?;
            let status = validate_tool_status(update.get("status"))?;
            let terminal_ids = validate_tool_content(update.get("content"))?;
            if !terminal_ids.is_empty() {
                (
                    "terminal.attached",
                    true,
                    json!({
                        "tool_call_id": tool_call_id,
                        "terminal_ids": terminal_ids,
                        "tool": Value::Object(update.clone()),
                    }),
                    None,
                )
            } else {
                let event_type = match status {
                    Some("completed") => "tool.completed",
                    Some("failed") => "tool.failed",
                    _ => "tool.updated",
                };
                (
                    event_type,
                    true,
                    json!({
                        "tool_call_id": tool_call_id,
                        "tool": Value::Object(update.clone()),
                    }),
                    None,
                )
            }
        }
        "usage_update" => {
            required_u64(update.get("used"), "ACP usage_update used")?;
            required_u64(update.get("size"), "ACP usage_update size")?;
            if let Some(cost) = update.get("cost")
                && !cost.is_null()
                && !cost.is_object()
            {
                return Err(invalid_response(
                    "ACP usage_update cost must be an object or null",
                ));
            }
            let usage = Value::Object(update.clone());
            ("usage.recorded", true, json!({"usage": usage}), Some(usage))
        }
        "plan" => {
            required_array(update.get("entries"), "ACP plan entries")?;
            (
                "plan.updated",
                true,
                json!({"plan": Value::Object(update.clone())}),
                None,
            )
        }
        "available_commands_update" => {
            required_array(update.get("availableCommands"), "ACP availableCommands")?;
            (
                "commands.updated",
                true,
                json!({"commands": update.get("availableCommands")}),
                None,
            )
        }
        "current_mode_update" => {
            required_string(update.get("currentModeId"), "ACP currentModeId")?;
            (
                "mode.updated",
                true,
                json!({"mode": Value::Object(update.clone())}),
                None,
            )
        }
        "config_option_update" => {
            required_array(update.get("configOptions"), "ACP configOptions")?;
            (
                "config.updated",
                true,
                json!({"config": Value::Object(update.clone())}),
                None,
            )
        }
        "session_info_update" => (
            "session.updated",
            true,
            json!({"session": Value::Object(update.clone())}),
            None,
        ),
        _ => {
            return Err(invalid_response(format!(
                "unsupported ACP session update type: {update_type}"
            )));
        }
    };
    Ok(decoded)
}

fn validate_tool_status(value: Option<&Value>) -> Result<Option<&str>, HarnessAdapterError> {
    let Some(value) = value else {
        return Ok(None);
    };
    let status = value
        .as_str()
        .ok_or_else(|| invalid_response("ACP tool status must be a string"))?;
    if !matches!(status, "pending" | "in_progress" | "completed" | "failed") {
        return Err(invalid_response(format!(
            "unsupported ACP tool status: {status}"
        )));
    }
    Ok(Some(status))
}

fn validate_tool_content(value: Option<&Value>) -> Result<Vec<String>, HarnessAdapterError> {
    let Some(value) = value else {
        return Ok(Vec::new());
    };
    let content = value
        .as_array()
        .ok_or_else(|| invalid_response("ACP tool content must be an array"))?;
    let mut terminal_ids = Vec::new();
    for item in content {
        let item = item
            .as_object()
            .ok_or_else(|| invalid_response("ACP tool content item must be an object"))?;
        let content_type = required_string(item.get("type"), "ACP tool content item.type")?;
        if content_type == "terminal" {
            terminal_ids.push(
                required_string(item.get("terminalId"), "ACP terminal content terminalId")?
                    .to_owned(),
            );
        }
    }
    Ok(terminal_ids)
}

fn validate_permission_request(params: &Value) -> Result<(), HarnessAdapterError> {
    let params = required_object(params, "ACP permission request params")?;
    required_string(params.get("sessionId"), "ACP permission request sessionId")?;
    let tool_call = params
        .get("toolCall")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_response("ACP permission request toolCall must be an object"))?;
    required_string(
        tool_call.get("toolCallId"),
        "ACP permission request toolCall.toolCallId",
    )?;
    validate_tool_status(tool_call.get("status"))?;
    validate_tool_content(tool_call.get("content"))?;

    let options = required_array(params.get("options"), "ACP permission request options")?;
    if options.is_empty() {
        return Err(invalid_response(
            "ACP permission request options must not be empty",
        ));
    }
    for option in options {
        let option = option
            .as_object()
            .ok_or_else(|| invalid_response("ACP permission request option must be an object"))?;
        required_string(option.get("optionId"), "ACP permission option optionId")?;
        required_string(option.get("name"), "ACP permission option name")?;
        let kind = required_string(option.get("kind"), "ACP permission option kind")?;
        if !matches!(
            kind,
            "allow_once" | "allow_always" | "reject_once" | "reject_always"
        ) {
            return Err(invalid_response(format!(
                "unsupported ACP permission option kind: {kind}"
            )));
        }
    }
    Ok(())
}

fn validate_permission_response(result: &Value) -> Result<(), HarnessAdapterError> {
    let result = required_object(result, "ACP permission response result")?;
    let outcome = result
        .get("outcome")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_request("ACP permission response outcome must be an object"))?;
    match required_string(
        outcome.get("outcome"),
        "ACP permission response outcome.outcome",
    )? {
        "cancelled" => Ok(()),
        "selected" => {
            required_string(
                outcome.get("optionId"),
                "ACP selected permission outcome optionId",
            )?;
            Ok(())
        }
        value => Err(invalid_request(format!(
            "unsupported ACP permission outcome: {value}"
        ))),
    }
}

fn required_object<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a Map<String, Value>, HarnessAdapterError> {
    value
        .as_object()
        .ok_or_else(|| invalid_response(format!("{field} must be an object")))
}

fn optional_object(
    value: Option<&Value>,
    field: &str,
) -> Result<Option<Map<String, Value>>, HarnessAdapterError> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Object(value)) => Ok(Some(value.clone())),
        Some(_) => Err(invalid_response(format!(
            "{field} must be an object or null"
        ))),
    }
}

fn required_array<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a Vec<Value>, HarnessAdapterError> {
    value
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_response(format!("{field} must be an array")))
}

fn required_string<'a>(
    value: Option<&'a Value>,
    field: &str,
) -> Result<&'a str, HarnessAdapterError> {
    let value = value
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_response(format!("{field} must be a string")))?;
    if value.trim().is_empty() {
        return Err(invalid_response(format!("{field} must not be empty")));
    }
    Ok(value)
}

fn required_u64(value: Option<&Value>, field: &str) -> Result<u64, HarnessAdapterError> {
    value
        .and_then(Value::as_u64)
        .ok_or_else(|| invalid_response(format!("{field} must be an unsigned integer")))
}

fn required_non_empty<'a>(value: &'a str, message: &str) -> Result<&'a str, HarnessAdapterError> {
    non_empty(value).ok_or_else(|| invalid_request(message))
}

fn non_empty(value: &str) -> Option<&str> {
    (!value.trim().is_empty()).then_some(value)
}

fn invalid_request(message: impl Into<String>) -> HarnessAdapterError {
    HarnessAdapterError::InvalidRequest(message.into())
}

fn invalid_response(message: impl Into<String>) -> HarnessAdapterError {
    HarnessAdapterError::InvalidResponse(message.into())
}
