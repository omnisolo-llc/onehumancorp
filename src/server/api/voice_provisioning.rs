//! Standalone, instance-local voice settings and durable provisioning holds.
//! Hosted replicas have no shared tenant-bound authority for this global store.
use crate::settings::Store;
use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use server_common::Claims;
use server_integrations_twilio::client::{RealTwilioClient, TwilioClientWrapper};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
use std::{fs, io::Write, path::Path, sync::Arc};

#[derive(Clone)]
pub(crate) struct VoiceState {
    pub store: Arc<Store>,
    pub provider: Arc<dyn TwilioClientWrapper>,
    pub standalone: bool,
    pub multitenant: bool,
    pub configured: bool,
    pub account_sid: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Binding {
    tenant: String,
    actor: String,
    account: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Receipt {
    version: u8,
    operation: String,
    binding: Binding,
    state: String,
    number: Option<String>,
}
#[derive(Debug)]
enum Failure {
    Unauthenticated,
    Denied(&'static str),
    Unavailable(&'static str),
    Reconcile,
}
impl Failure {
    fn response(self) -> Response {
        let (status, reason) = match self {
            Self::Unauthenticated => (StatusCode::UNAUTHORIZED, "authentication_required"),
            Self::Denied(reason) => (StatusCode::FORBIDDEN, reason),
            Self::Unavailable(reason) => (StatusCode::SERVICE_UNAVAILABLE, reason),
            Self::Reconcile => (StatusCode::CONFLICT, "provisioning_requires_reconciliation"),
        };
        (status, Json(json!({"success":false,"error":reason,"provisioning_available":false,"provisioning_block_reason":reason}))).into_response()
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(store: Arc<Store>) -> Router<S> {
    let config = ::server_config::get();
    let account_sid = std::env::var("TWILIO_ACCOUNT_SID").unwrap_or_default();
    let provider = RealTwilioClient::new(
        account_sid.clone(),
        std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default(),
    );
    let configured = provider.provisioning_configured();
    router_with_state(VoiceState {
        store,
        provider: Arc::new(provider),
        standalone: config.standalone,
        multitenant: config.multitenant,
        configured,
        account_sid,
    })
}
pub(crate) fn router_with_state<S: Clone + Send + Sync + 'static>(state: VoiceState) -> Router<S> {
    Router::new()
        .route("/api/v1/settings/voice", get(read).post(update))
        .route(
            "/api/v1/settings/voice/provision",
            axum::routing::post(provision),
        )
        .with_state(state)
}
impl VoiceState {
    fn binding(&self, user: Option<Extension<Claims>>) -> Result<Binding, Failure> {
        let user = user.ok_or(Failure::Unauthenticated)?.0;
        if !self.standalone || self.multitenant {
            return Err(Failure::Denied("hosted_global_provisioning_unavailable"));
        }
        if !user
            .roles
            .iter()
            .any(|role| role.eq_ignore_ascii_case("ADMIN"))
        {
            return Err(Failure::Denied("admin_required"));
        }
        let tenant = user
            .organization_id
            .filter(|value| {
                !value.trim().is_empty()
                    && value.trim() == value
                    && !value.eq_ignore_ascii_case("system")
            })
            .ok_or(Failure::Denied("tenant_required"))?;
        if user.sub.trim().is_empty() {
            return Err(Failure::Denied("actor_required"));
        }
        Ok(Binding {
            tenant,
            actor: user.sub,
            account: self.account_sid.clone(),
        })
    }
    fn receipt(&self, binding: &Binding) -> Result<Option<Receipt>, Failure> {
        let path = self
            .store
            .voice_provisioning_path()
            .map_err(|_| Failure::Unavailable("persistent_storage_unavailable"))?;
        let record = read_receipt(&path)?;
        if record
            .as_ref()
            .is_some_and(|record| &record.binding != binding)
        {
            return Err(Failure::Denied("operation_owner_mismatch"));
        }
        Ok(record)
    }
}
fn valid_number(value: &str) -> bool {
    let bytes = value.as_bytes();
    (3..=16).contains(&bytes.len())
        && bytes[0] == b'+'
        && (b'1'..=b'9').contains(&bytes[1])
        && bytes[2..].iter().all(u8::is_ascii_digit)
}
fn read_receipt(path: &Path) -> Result<Option<Receipt>, Failure> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(value) => value,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(Failure::Unavailable("provisioning_storage_unavailable")),
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 8192 {
        return Err(Failure::Reconcile);
    }
    let record: Receipt = serde_json::from_slice(&fs::read(path).map_err(|_| Failure::Reconcile)?)
        .map_err(|_| Failure::Reconcile)?;
    if record.version != 1
        || uuid::Uuid::parse_str(&record.operation).is_err()
        || record.binding.actor.is_empty()
        || record.binding.tenant.is_empty()
        || !matches!(record.state.as_str(), "pending" | "confirmed")
        || record.state == "pending" && record.number.is_some()
        || record.state == "confirmed" && !record.number.as_deref().is_some_and(valid_number)
    {
        return Err(Failure::Reconcile);
    }
    Ok(Some(record))
}
fn sync_directory(path: &Path) -> Result<(), Failure> {
    #[cfg(unix)]
    fs::File::open(path.parent().ok_or(Failure::Reconcile)?)
        .and_then(|file| file.sync_all())
        .map_err(|_| Failure::Reconcile)?;
    Ok(())
}
enum Claim {
    New(Receipt),
    Confirmed(String),
}
fn claim(state: &VoiceState, binding: &Binding) -> Result<Claim, Failure> {
    let guard = state
        .store
        .lock_voice_settings()
        .map_err(|_| Failure::Unavailable("settings_storage_unavailable"))?;
    let path = state
        .store
        .voice_provisioning_path()
        .map_err(|_| Failure::Unavailable("persistent_storage_unavailable"))?;
    if let Some(record) = state.receipt(binding)? {
        if record.state == "confirmed" {
            let number = record.number.ok_or(Failure::Reconcile)?;
            if !guard.persisted
                || guard.value.voice_receptionist_number.as_deref() != Some(number.as_str())
            {
                return Err(Failure::Reconcile);
            }
            return Ok(Claim::Confirmed(number));
        }
        return Err(Failure::Reconcile);
    }
    // Existing unbound number configuration is not a provider receipt.
    if guard
        .value
        .voice_receptionist_number
        .as_deref()
        .is_some_and(|value| !value.is_empty())
    {
        return Err(Failure::Reconcile);
    }
    let record = Receipt {
        version: 1,
        operation: uuid::Uuid::new_v4().to_string(),
        binding: binding.clone(),
        state: "pending".into(),
        number: None,
    };
    fs::create_dir_all(path.parent().ok_or(Failure::Reconcile)?)
        .map_err(|_| Failure::Unavailable("provisioning_storage_unavailable"))?;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    let mut file = match options.open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            return Err(Failure::Reconcile);
        }
        Err(_) => return Err(Failure::Unavailable("provisioning_storage_unavailable")),
    };
    // A failed or interrupted marker write remains a hold. It must never be
    // removed automatically or interpreted as permission to repeat an effect.
    let bytes = serde_json::to_vec(&record).map_err(|_| Failure::Reconcile)?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| Failure::Reconcile)?;
    sync_directory(&path)?;
    Ok(Claim::New(record))
}
fn complete(state: &VoiceState, record: &Receipt, number: String) -> Result<(), Failure> {
    let mut guard = state
        .store
        .lock_voice_settings()
        .map_err(|_| Failure::Reconcile)?;
    let path = state
        .store
        .voice_provisioning_path()
        .map_err(|_| Failure::Reconcile)?;
    if read_receipt(&path)?.as_ref() != Some(record) || !valid_number(&number) {
        return Err(Failure::Reconcile);
    }
    guard.value.voice_receptionist_number = Some(number.clone());
    guard.persist().map_err(|_| Failure::Reconcile)?;
    let confirmed = Receipt {
        state: "confirmed".into(),
        number: Some(number),
        ..record.clone()
    };
    crate::utils::fs::write_file_atomic(
        &path,
        &serde_json::to_vec(&confirmed).map_err(|_| Failure::Reconcile)?,
        0o600,
    )
    .map_err(|_| Failure::Reconcile)?;
    sync_directory(&path)
}
async fn read(State(state): State<VoiceState>, user: Option<Extension<Claims>>) -> Response {
    let binding = match state.binding(user) {
        Ok(value) => value,
        Err(error) => return error.response(),
    };
    let guard = match state.store.lock_voice_settings() {
        Ok(value) => value,
        Err(_) => return Failure::Unavailable("settings_storage_unavailable").response(),
    };
    let receipt = match state.receipt(&binding) {
        Ok(value) => value,
        Err(error) => return error.response(),
    };
    let settings = &guard.value;
    let reason = if !state.configured {
        Some("provider_not_configured")
    } else if receipt.as_ref().is_some_and(|record| {
        record.state == "pending"
            || !guard.persisted
            || record.number != settings.voice_receptionist_number
    }) {
        Some("provisioning_requires_reconciliation")
    } else if receipt.is_none()
        && settings
            .voice_receptionist_number
            .as_deref()
            .is_some_and(|value| !value.is_empty())
    {
        Some("unverified_number_configuration")
    } else {
        None
    };
    Json(json!({
        "voice_receptionist_enabled":settings.voice_receptionist_enabled,
        "voice_receptionist_number":settings.voice_receptionist_number,
        "voice_receptionist_persona":settings.voice_receptionist_persona,
        "voice_receptionist_instructions":settings.voice_receptionist_instructions,
        "provisioning_available":reason.is_none(),
        "provisioning_block_reason":reason,
        "provisioning_state":receipt.as_ref().map(|record|record.state.as_str()).unwrap_or("not_started")
    })).into_response()
}
async fn update(
    State(state): State<VoiceState>,
    user: Option<Extension<Claims>>,
    Json(request): Json<Value>,
) -> Response {
    let binding = match state.binding(user) {
        Ok(value) => value,
        Err(error) => return error.response(),
    };
    let mut guard = match state.store.lock_voice_settings() {
        Ok(value) => value,
        Err(_) => return Failure::Unavailable("settings_storage_unavailable").response(),
    };
    match state.receipt(&binding) {
        Ok(Some(record)) if record.state == "confirmed" && !guard.persisted => {
            return Failure::Reconcile.response();
        }
        Ok(_) => {}
        Err(error) => return error.response(),
    }
    if let Some(value) = request
        .get("voice_receptionist_enabled")
        .and_then(Value::as_bool)
    {
        guard.value.voice_receptionist_enabled = value;
    }
    if let Some(value) = request.get("voice_receptionist_persona") {
        guard.value.voice_receptionist_persona = value.as_str().map(str::to_owned);
    }
    if let Some(value) = request.get("voice_receptionist_instructions") {
        guard.value.voice_receptionist_instructions = value.as_str().map(str::to_owned);
    }
    // Legacy clients may send a whole snapshot, but its number is never
    // authority to replace a provider receipt or clear a confirmed value.
    if guard.persist().is_err() {
        return Failure::Unavailable("settings_storage_unavailable").response();
    }
    Json(json!({"success":true})).into_response()
}
async fn provision(State(state): State<VoiceState>, user: Option<Extension<Claims>>) -> Response {
    let binding = match state.binding(user) {
        Ok(value) => value,
        Err(error) => return error.response(),
    };
    if !state.configured {
        return Failure::Unavailable("provider_not_configured").response();
    }
    let record = match claim(&state, &binding) {
        Ok(Claim::New(record)) => record,
        Ok(Claim::Confirmed(number)) => {
            return Json(json!({"success":true,"number":number})).into_response();
        }
        Err(error) => return error.response(),
    };
    let number = match state.provider.provision_number("415").await {
        Ok(number) => number,
        Err(_) => return Failure::Reconcile.response(),
    };
    if let Err(error) = complete(&state, &record, number.clone()) {
        return error.response();
    }
    Json(json!({"success":true,"number":number})).into_response()
}
