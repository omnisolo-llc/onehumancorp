use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::types::ModelDescriptor;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerState {
    #[default]
    Ready,
    Draining,
    Unhealthy,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InferenceState {
    #[default]
    Queued,
    CapacityLeased,
    Admitted,
    Streaming,
    Completed,
    Failed,
    Cancelled,
    Uncertain,
}

impl InferenceState {
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (Self::Queued, Self::CapacityLeased)
                | (
                    Self::CapacityLeased,
                    Self::Admitted | Self::Failed | Self::Cancelled
                )
                | (
                    Self::Admitted,
                    Self::Streaming
                        | Self::Completed
                        | Self::Cancelled
                        | Self::Failed
                        | Self::Uncertain
                )
                | (
                    Self::Streaming,
                    Self::Completed | Self::Failed | Self::Cancelled | Self::Uncertain
                )
                | (
                    Self::Uncertain,
                    Self::Admitted | Self::Completed | Self::Failed | Self::Cancelled
                )
        )
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum InferenceError {
    WorkerAlreadyRegistered,
    RequestDigestConflict,
    RequestNotFound,
    NoCompatibleCapacity,
    AdmissionNotFound,
    CapacityLeaseNotFound,
    StaleCapacityFence,
    CapacityLeaseNotActive,
    GenerationOverflow,
    AdmissionTerminal,
    InvalidInferenceState,
    StaleStateVersion,
    ChunkSequence { expected: u64, actual: u64 },
    UncertainExecution,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct RuntimeWorker {
    pub worker_id: String,
    pub tenant_id: Option<String>,
    pub runtime_id: String,
    pub model_revisions: BTreeSet<String>,
    pub capabilities: BTreeSet<String>,
    pub max_context_tokens: u64,
    pub capacity_slots: u32,
    pub leased_slots: u32,
    pub state: WorkerState,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InferenceRequest {
    pub request_id: Uuid,
    pub tenant_id: Option<String>,
    pub session_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub model_binding_id: Uuid,
    pub model: ModelDescriptor,
    pub request_digest: String,
    pub required_capabilities: BTreeSet<String>,
    pub context_tokens: u64,
    pub replayable: bool,
    pub state: InferenceState,
    pub state_version: i64,
}

impl InferenceRequest {
    pub fn new(
        request_id: Uuid,
        model_binding_id: Uuid,
        model: ModelDescriptor,
        request_digest: impl Into<String>,
        required_capabilities: BTreeSet<String>,
        context_tokens: u64,
        replayable: bool,
    ) -> Self {
        Self {
            request_id,
            tenant_id: None,
            session_id: None,
            task_id: None,
            turn_id: None,
            attempt_id: None,
            model_binding_id,
            model,
            request_digest: request_digest.into(),
            required_capabilities,
            context_tokens,
            replayable,
            state: InferenceState::Queued,
            state_version: 0,
        }
    }

    pub fn with_scope(
        mut self,
        tenant_id: impl Into<String>,
        session_id: Uuid,
        task_id: Option<Uuid>,
        turn_id: Option<Uuid>,
        attempt_id: Option<Uuid>,
    ) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self.session_id = Some(session_id);
        self.task_id = task_id;
        self.turn_id = turn_id;
        self.attempt_id = attempt_id;
        self
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct SubmitResult {
    pub request_id: Uuid,
    pub duplicate: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct CapacityFence {
    pub lease_id: Uuid,
    pub worker_id: String,
    pub generation: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum CapacityLeaseState {
    Active,
    Released,
    Lost,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct CapacityLease {
    pub lease_id: Uuid,
    pub request_id: Uuid,
    pub worker_id: String,
    pub generation: i64,
    pub state: CapacityLeaseState,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InferenceAdmission {
    pub admission_id: Uuid,
    pub request_id: Uuid,
    pub tenant_id: Option<String>,
    pub session_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub turn_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub model_binding_id: Uuid,
    pub request_digest: String,
    pub worker_id: String,
    pub capacity_lease_id: Uuid,
    pub capacity_generation: i64,
    pub fence: CapacityFence,
    pub state: InferenceState,
    pub state_version: i64,
    pub chunks: Vec<StreamChunk>,
    pub final_response: Option<FinalResponse>,
    pub usage: Option<Usage>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct StreamChunk {
    pub sequence: u64,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub struct FinalResponse {
    pub content: String,
    pub finish_reason: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct Usage {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cached_tokens: u64,
}

#[derive(Clone, Debug, Default)]
pub struct InferenceGateway {
    workers: HashMap<String, RuntimeWorker>,
    requests: HashMap<Uuid, InferenceRequest>,
    admissions: HashMap<Uuid, InferenceAdmission>,
    admission_by_request: HashMap<Uuid, Uuid>,
    capacity_leases: HashMap<Uuid, CapacityLease>,
}

impl InferenceGateway {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_worker(&mut self, worker: RuntimeWorker) -> Result<(), InferenceError> {
        if self.workers.contains_key(&worker.worker_id) {
            return Err(InferenceError::WorkerAlreadyRegistered);
        }
        self.workers.insert(worker.worker_id.clone(), worker);
        Ok(())
    }

    pub fn submit(&mut self, request: InferenceRequest) -> Result<SubmitResult, InferenceError> {
        if let Some(existing) = self.requests.get(&request.request_id) {
            if existing.request_digest != request.request_digest {
                return Err(InferenceError::RequestDigestConflict);
            }
            return Ok(SubmitResult {
                request_id: request.request_id,
                duplicate: true,
            });
        }
        let request_id = request.request_id;
        self.requests.insert(request_id, request);
        Ok(SubmitResult {
            request_id,
            duplicate: false,
        })
    }

    pub fn admit(&mut self, request_id: Uuid) -> Result<InferenceAdmission, InferenceError> {
        if let Some(admission_id) = self.admission_by_request.get(&request_id) {
            return self
                .admissions
                .get(admission_id)
                .cloned()
                .ok_or(InferenceError::AdmissionNotFound);
        }
        let request = self
            .requests
            .get(&request_id)
            .cloned()
            .ok_or(InferenceError::RequestNotFound)?;
        if request.state != InferenceState::Queued {
            return Err(InferenceError::InvalidInferenceState);
        }
        let worker_id = self.select_worker(&request)?;
        let lease = self.create_capacity_lease(request_id, &worker_id)?;
        let admission_id = Uuid::new_v4();
        let admission = InferenceAdmission {
            admission_id,
            request_id,
            tenant_id: request.tenant_id.clone(),
            session_id: request.session_id,
            task_id: request.task_id,
            turn_id: request.turn_id,
            attempt_id: request.attempt_id,
            model_binding_id: request.model_binding_id,
            request_digest: request.request_digest.clone(),
            worker_id: worker_id.clone(),
            capacity_lease_id: lease.lease_id,
            capacity_generation: lease.generation,
            fence: CapacityFence {
                lease_id: lease.lease_id,
                worker_id,
                generation: lease.generation,
            },
            state: InferenceState::Admitted,
            state_version: 2,
            chunks: Vec::new(),
            final_response: None,
            usage: None,
        };
        let request = self
            .requests
            .get_mut(&request_id)
            .ok_or(InferenceError::RequestNotFound)?;
        request.state = InferenceState::Admitted;
        request.state_version = 2;
        self.admission_by_request.insert(request_id, admission_id);
        self.admissions.insert(admission_id, admission.clone());
        Ok(admission)
    }

    pub fn reassign_capacity(&mut self, lease_id: Uuid) -> Result<CapacityFence, InferenceError> {
        let lease = self
            .capacity_leases
            .get_mut(&lease_id)
            .ok_or(InferenceError::CapacityLeaseNotFound)?;
        if lease.state != CapacityLeaseState::Active {
            return Err(InferenceError::CapacityLeaseNotActive);
        }
        lease.generation = lease
            .generation
            .checked_add(1)
            .ok_or(InferenceError::GenerationOverflow)?;
        let fence = CapacityFence {
            lease_id,
            worker_id: lease.worker_id.clone(),
            generation: lease.generation,
        };
        let admission_id = self
            .admission_by_request
            .get(&lease.request_id)
            .copied()
            .ok_or(InferenceError::AdmissionNotFound)?;
        let admission = self
            .admissions
            .get_mut(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?;
        admission.capacity_generation = lease.generation;
        admission.fence = fence.clone();
        Ok(fence)
    }

    pub fn start_stream(
        &mut self,
        admission_id: Uuid,
        fence: CapacityFence,
    ) -> Result<(), InferenceError> {
        self.validate_fence(admission_id, &fence)?;
        let admission = self
            .admissions
            .get_mut(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?;
        let next = transition(
            &admission.state,
            admission.state_version,
            admission.state_version,
            InferenceState::Streaming,
        )?;
        admission.state = next.0;
        admission.state_version = next.1;
        Ok(())
    }

    pub fn append_chunk(
        &mut self,
        admission_id: Uuid,
        fence: CapacityFence,
        chunk: StreamChunk,
    ) -> Result<(), InferenceError> {
        self.validate_fence(admission_id, &fence)?;
        let admission = self
            .admissions
            .get_mut(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?;
        if admission.state != InferenceState::Streaming {
            return Err(InferenceError::InvalidInferenceState);
        }
        let expected = admission.chunks.len() as u64;
        if chunk.sequence != expected {
            if admission
                .chunks
                .get(chunk.sequence as usize)
                .is_some_and(|existing| existing == &chunk)
            {
                return Ok(());
            }
            return Err(InferenceError::ChunkSequence {
                expected,
                actual: chunk.sequence,
            });
        }
        admission.chunks.push(chunk);
        Ok(())
    }

    pub fn complete(
        &mut self,
        admission_id: Uuid,
        fence: CapacityFence,
        final_response: FinalResponse,
        usage: Usage,
    ) -> Result<(), InferenceError> {
        self.validate_fence(admission_id, &fence)?;
        let (worker_id, lease_id, request_id) = {
            let admission = self
                .admissions
                .get_mut(&admission_id)
                .ok_or(InferenceError::AdmissionNotFound)?;
            let next = transition(
                &admission.state,
                admission.state_version,
                admission.state_version,
                InferenceState::Completed,
            )?;
            admission.state = next.0;
            admission.state_version = next.1;
            admission.final_response = Some(final_response);
            admission.usage = Some(usage);
            (
                admission.worker_id.clone(),
                admission.capacity_lease_id,
                admission.request_id,
            )
        };
        self.release_capacity(lease_id, &worker_id, CapacityLeaseState::Released);
        if let Some(request) = self.requests.get_mut(&request_id) {
            request.state = InferenceState::Completed;
            request.state_version += 1;
        }
        Ok(())
    }

    pub fn cancel(
        &mut self,
        admission_id: Uuid,
        fence: CapacityFence,
    ) -> Result<(), InferenceError> {
        self.validate_fence(admission_id, &fence)?;
        let (worker_id, lease_id, request_id) = {
            let admission = self
                .admissions
                .get_mut(&admission_id)
                .ok_or(InferenceError::AdmissionNotFound)?;
            let next = transition(
                &admission.state,
                admission.state_version,
                admission.state_version,
                InferenceState::Cancelled,
            )?;
            admission.state = next.0;
            admission.state_version = next.1;
            (
                admission.worker_id.clone(),
                admission.capacity_lease_id,
                admission.request_id,
            )
        };
        self.release_capacity(lease_id, &worker_id, CapacityLeaseState::Released);
        if let Some(request) = self.requests.get_mut(&request_id) {
            request.state = InferenceState::Cancelled;
            request.state_version += 1;
        }
        Ok(())
    }

    pub fn recover_after_restart(&mut self, admission_id: Uuid) -> Result<(), InferenceError> {
        let (worker_id, lease_id, request_id) = {
            let admission = self
                .admissions
                .get_mut(&admission_id)
                .ok_or(InferenceError::AdmissionNotFound)?;
            if !matches!(
                admission.state,
                InferenceState::Admitted | InferenceState::Streaming
            ) {
                return Err(InferenceError::InvalidInferenceState);
            }
            let next = transition(
                &admission.state,
                admission.state_version,
                admission.state_version,
                InferenceState::Uncertain,
            )?;
            admission.state = next.0;
            admission.state_version = next.1;
            (
                admission.worker_id.clone(),
                admission.capacity_lease_id,
                admission.request_id,
            )
        };
        self.release_capacity(lease_id, &worker_id, CapacityLeaseState::Lost);
        if let Some(request) = self.requests.get_mut(&request_id) {
            request.state = InferenceState::Uncertain;
            request.state_version += 1;
        }
        Ok(())
    }

    pub fn retry_uncertain(
        &mut self,
        admission_id: Uuid,
        runtime_proves_safe_replay: bool,
    ) -> Result<(), InferenceError> {
        let request_id = self
            .admissions
            .get(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?
            .request_id;
        let request = self
            .requests
            .get(&request_id)
            .cloned()
            .ok_or(InferenceError::RequestNotFound)?;
        if !runtime_proves_safe_replay || !request.replayable {
            return Err(InferenceError::UncertainExecution);
        }
        let admission = self
            .admissions
            .get(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?;
        if admission.state != InferenceState::Uncertain {
            return Err(InferenceError::InvalidInferenceState);
        }
        let worker_id = self.select_worker(&request)?;
        let lease = self.create_capacity_lease(request_id, &worker_id)?;
        let admission = self
            .admissions
            .get_mut(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?;
        admission.worker_id = worker_id.clone();
        admission.capacity_lease_id = lease.lease_id;
        admission.capacity_generation = lease.generation;
        admission.fence = CapacityFence {
            lease_id: lease.lease_id,
            worker_id,
            generation: lease.generation,
        };
        admission.state = InferenceState::Admitted;
        admission.state_version += 1;
        admission.chunks.clear();
        admission.final_response = None;
        admission.usage = None;
        if let Some(request) = self.requests.get_mut(&request_id) {
            request.state = InferenceState::Admitted;
            request.state_version += 2;
        }
        Ok(())
    }

    /// Resolves an uncertain post-admission execution without reusing its lost
    /// capacity lease. Re-execution remains a separate, explicit operation.
    pub fn cancel_uncertain(&mut self, admission_id: Uuid) -> Result<(), InferenceError> {
        let request_id = self
            .admissions
            .get(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?
            .request_id;
        let admission = self
            .admissions
            .get_mut(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?;
        if admission.state != InferenceState::Uncertain {
            return Err(InferenceError::InvalidInferenceState);
        }
        admission.state = InferenceState::Cancelled;
        admission.state_version += 1;
        if let Some(request) = self.requests.get_mut(&request_id) {
            request.state = InferenceState::Cancelled;
            request.state_version += 1;
        }
        Ok(())
    }

    pub fn admission(&self, admission_id: Uuid) -> Option<&InferenceAdmission> {
        self.admissions.get(&admission_id)
    }

    pub fn request(&self, request_id: Uuid) -> Option<&InferenceRequest> {
        self.requests.get(&request_id)
    }

    pub fn worker(&self, worker_id: &str) -> Option<&RuntimeWorker> {
        self.workers.get(worker_id)
    }

    fn select_worker(&self, request: &InferenceRequest) -> Result<String, InferenceError> {
        self.workers
            .values()
            .find(|worker| {
                worker.state == WorkerState::Ready
                    && worker.leased_slots < worker.capacity_slots
                    && request.tenant_id.as_ref().is_none_or(|tenant_id| {
                        worker
                            .tenant_id
                            .as_ref()
                            .is_some_and(|worker_tenant_id| worker_tenant_id == tenant_id)
                    })
                    && request.context_tokens <= worker.max_context_tokens
                    && request
                        .model
                        .revision
                        .as_ref()
                        .is_none_or(|revision| worker.model_revisions.contains(revision))
                    && request
                        .required_capabilities
                        .iter()
                        .all(|capability| worker.capabilities.contains(capability))
            })
            .map(|worker| worker.worker_id.clone())
            .ok_or(InferenceError::NoCompatibleCapacity)
    }

    fn create_capacity_lease(
        &mut self,
        request_id: Uuid,
        worker_id: &str,
    ) -> Result<CapacityLease, InferenceError> {
        let worker = self
            .workers
            .get_mut(worker_id)
            .ok_or(InferenceError::NoCompatibleCapacity)?;
        if worker.leased_slots >= worker.capacity_slots {
            return Err(InferenceError::NoCompatibleCapacity);
        }
        worker.leased_slots += 1;
        let lease = CapacityLease {
            lease_id: Uuid::new_v4(),
            request_id,
            worker_id: worker_id.to_owned(),
            generation: 1,
            state: CapacityLeaseState::Active,
        };
        self.capacity_leases.insert(lease.lease_id, lease.clone());
        Ok(lease)
    }

    fn validate_fence(
        &self,
        admission_id: Uuid,
        fence: &CapacityFence,
    ) -> Result<(), InferenceError> {
        let admission = self
            .admissions
            .get(&admission_id)
            .ok_or(InferenceError::AdmissionNotFound)?;
        let lease = self
            .capacity_leases
            .get(&admission.capacity_lease_id)
            .ok_or(InferenceError::CapacityLeaseNotFound)?;
        if lease.state != CapacityLeaseState::Active {
            return Err(InferenceError::CapacityLeaseNotActive);
        }
        if fence.lease_id != lease.lease_id
            || fence.worker_id != lease.worker_id
            || fence.generation != lease.generation
        {
            return Err(InferenceError::StaleCapacityFence);
        }
        if admission.state.is_terminal() {
            return Err(InferenceError::AdmissionTerminal);
        }
        Ok(())
    }

    fn release_capacity(&mut self, lease_id: Uuid, worker_id: &str, state: CapacityLeaseState) {
        if let Some(lease) = self.capacity_leases.get_mut(&lease_id) {
            lease.state = state;
        }
        if let Some(worker) = self.workers.get_mut(worker_id) {
            worker.leased_slots = worker.leased_slots.saturating_sub(1);
        }
    }
}

fn transition(
    current: &InferenceState,
    current_version: i64,
    expected_version: i64,
    next: InferenceState,
) -> Result<(InferenceState, i64), InferenceError> {
    if current_version != expected_version {
        return Err(InferenceError::StaleStateVersion);
    }
    if *current == next {
        return Ok((current.clone(), current_version));
    }
    if current.is_terminal() {
        return Err(InferenceError::AdmissionTerminal);
    }
    if !current.can_transition_to(&next) {
        return Err(InferenceError::InvalidInferenceState);
    }
    Ok((next, current_version + 1))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use uuid::Uuid;

    use super::super::types::{ModelDescriptor, ModelProvider};
    use super::*;

    fn worker(worker_id: &str, revision: &str, capabilities: &[&str], slots: u32) -> RuntimeWorker {
        RuntimeWorker {
            worker_id: worker_id.to_owned(),
            tenant_id: Some("tenant-1".to_owned()),
            runtime_id: "runtime-1".to_owned(),
            model_revisions: BTreeSet::from([revision.to_owned()]),
            capabilities: capabilities
                .iter()
                .map(|capability| (*capability).to_owned())
                .collect(),
            max_context_tokens: 32_768,
            capacity_slots: slots,
            leased_slots: 0,
            state: WorkerState::Ready,
        }
    }

    fn request(
        request_id: Uuid,
        digest: &str,
        required_capabilities: &[&str],
        context_tokens: u64,
        replayable: bool,
    ) -> InferenceRequest {
        InferenceRequest::new(
            request_id,
            Uuid::new_v4(),
            ModelDescriptor {
                model_id: "qwen3-coder".to_owned(),
                provider: ModelProvider::SelfHosted,
                revision: Some("rev-1".to_owned()),
                ..Default::default()
            },
            digest,
            required_capabilities
                .iter()
                .map(|capability| (*capability).to_owned())
                .collect(),
            context_tokens,
            replayable,
        )
    }

    #[test]
    fn admission_matches_capabilities_and_consumes_capacity() {
        let mut gateway = InferenceGateway::new();
        gateway
            .register_worker(worker("worker-1", "rev-1", &["tool_use"], 1))
            .unwrap();
        assert_eq!(
            gateway.register_worker(worker("worker-1", "rev-1", &["tool_use"], 1)),
            Err(InferenceError::WorkerAlreadyRegistered)
        );
        let request_id = Uuid::new_v4();
        gateway
            .submit(request(request_id, "digest-1", &["tool_use"], 4_000, false))
            .unwrap();

        let admission = gateway.admit(request_id).unwrap();
        assert_eq!(admission.state, InferenceState::Admitted);
        assert_eq!(gateway.worker("worker-1").unwrap().leased_slots, 1);
        assert_eq!(admission.request_digest, "digest-1");
    }

    #[test]
    fn scoped_admission_preserves_session_task_lineage() {
        let mut gateway = InferenceGateway::new();
        gateway
            .register_worker(worker("worker-1", "rev-1", &["tool_use"], 1))
            .unwrap();
        let request_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        let turn_id = Uuid::new_v4();
        let attempt_id = Uuid::new_v4();
        let request = request(request_id, "digest-scoped", &["tool_use"], 4_000, false).with_scope(
            "tenant-1",
            session_id,
            Some(task_id),
            Some(turn_id),
            Some(attempt_id),
        );
        gateway.submit(request).unwrap();

        let admission = gateway.admit(request_id).unwrap();
        assert_eq!(admission.tenant_id.as_deref(), Some("tenant-1"));
        assert_eq!(admission.session_id, Some(session_id));
        assert_eq!(admission.task_id, Some(task_id));
        assert_eq!(admission.turn_id, Some(turn_id));
        assert_eq!(admission.attempt_id, Some(attempt_id));
    }

    #[test]
    fn admission_does_not_cross_tenant_worker_boundaries() {
        let mut gateway = InferenceGateway::new();
        let mut worker = worker("worker-tenant-a", "rev-1", &["tool_use"], 1);
        worker.tenant_id = Some("tenant-a".to_owned());
        gateway.register_worker(worker).unwrap();

        let request = request(Uuid::new_v4(), "digest-tenant", &["tool_use"], 100, true)
            .with_scope("tenant-b", Uuid::new_v4(), None, None, None);
        let request_id = request.request_id;
        gateway.submit(request).unwrap();
        assert_eq!(
            gateway.admit(request_id),
            Err(InferenceError::NoCompatibleCapacity)
        );
    }

    #[test]
    fn pre_admission_submit_is_idempotent_and_can_retry_after_no_capacity() {
        let mut gateway = InferenceGateway::new();
        gateway
            .register_worker(worker("worker-1", "rev-1", &["vision"], 1))
            .unwrap();
        let request_id = Uuid::new_v4();
        let first = gateway
            .submit(request(request_id, "digest-1", &["tool_use"], 4_000, true))
            .unwrap();
        let duplicate = gateway
            .submit(request(request_id, "digest-1", &["tool_use"], 4_000, true))
            .unwrap();
        assert!(!first.duplicate);
        assert!(duplicate.duplicate);
        assert_eq!(
            gateway.submit(request(
                request_id,
                "different-digest",
                &["tool_use"],
                4_000,
                true,
            )),
            Err(InferenceError::RequestDigestConflict)
        );
        assert_eq!(
            gateway.admit(request_id),
            Err(InferenceError::NoCompatibleCapacity)
        );

        gateway
            .register_worker(worker("worker-2", "rev-1", &["tool_use"], 1))
            .unwrap();
        assert_eq!(
            gateway.admit(request_id).unwrap().state,
            InferenceState::Admitted
        );
    }

    #[test]
    fn stale_capacity_fence_cannot_stream_after_reassignment() {
        let mut gateway = InferenceGateway::new();
        gateway
            .register_worker(worker("worker-1", "rev-1", &["tool_use"], 2))
            .unwrap();
        let request_id = Uuid::new_v4();
        gateway
            .submit(request(request_id, "digest-1", &["tool_use"], 4_000, false))
            .unwrap();
        let admission = gateway.admit(request_id).unwrap();
        let stale_fence = admission.fence.clone();
        let current_fence = gateway
            .reassign_capacity(admission.capacity_lease_id)
            .unwrap();

        assert_eq!(
            gateway.start_stream(admission.admission_id, stale_fence),
            Err(InferenceError::StaleCapacityFence)
        );
        gateway
            .start_stream(admission.admission_id, current_fence)
            .unwrap();
    }

    #[test]
    fn stream_final_usage_and_cancellation_are_durable_lifecycle_events() {
        let mut gateway = InferenceGateway::new();
        gateway
            .register_worker(worker("worker-1", "rev-1", &["tool_use"], 2))
            .unwrap();
        let request_id = Uuid::new_v4();
        gateway
            .submit(request(request_id, "digest-1", &["tool_use"], 4_000, false))
            .unwrap();
        let admission = gateway.admit(request_id).unwrap();
        let fence = gateway
            .reassign_capacity(admission.capacity_lease_id)
            .unwrap();
        gateway
            .start_stream(admission.admission_id, fence.clone())
            .unwrap();
        assert_eq!(
            gateway.append_chunk(
                admission.admission_id,
                fence.clone(),
                StreamChunk {
                    sequence: 2,
                    content: "out of order".to_owned(),
                },
            ),
            Err(InferenceError::ChunkSequence {
                expected: 0,
                actual: 2,
            })
        );
        gateway
            .append_chunk(
                admission.admission_id,
                fence.clone(),
                StreamChunk {
                    sequence: 0,
                    content: "hello".to_owned(),
                },
            )
            .unwrap();
        gateway
            .append_chunk(
                admission.admission_id,
                fence.clone(),
                StreamChunk {
                    sequence: 1,
                    content: " world".to_owned(),
                },
            )
            .unwrap();
        gateway
            .complete(
                admission.admission_id,
                fence.clone(),
                FinalResponse {
                    content: "hello world".to_owned(),
                    finish_reason: "stop".to_owned(),
                },
                Usage {
                    input_tokens: 100,
                    output_tokens: 20,
                    cached_tokens: 4,
                },
            )
            .unwrap();
        assert_eq!(
            gateway.admission(admission.admission_id).unwrap().state,
            InferenceState::Completed
        );
        assert_eq!(
            gateway.admission(admission.admission_id).unwrap().usage,
            Some(Usage {
                input_tokens: 100,
                output_tokens: 20,
                cached_tokens: 4,
            })
        );
        assert_eq!(
            gateway.request(request_id).unwrap().state,
            InferenceState::Completed
        );
        assert_eq!(gateway.worker("worker-1").unwrap().leased_slots, 0);

        let cancelled_request = Uuid::new_v4();
        gateway
            .submit(request(
                cancelled_request,
                "digest-2",
                &["tool_use"],
                4_000,
                false,
            ))
            .unwrap();
        let cancelled = gateway.admit(cancelled_request).unwrap();
        gateway
            .cancel(cancelled.admission_id, cancelled.fence)
            .unwrap();
        assert_eq!(
            gateway.admission(cancelled.admission_id).unwrap().state,
            InferenceState::Cancelled
        );
        assert_eq!(
            gateway.request(cancelled_request).unwrap().state,
            InferenceState::Cancelled
        );
    }

    #[test]
    fn post_admission_restart_is_uncertain_until_runtime_proves_safe_replay() {
        let mut gateway = InferenceGateway::new();
        gateway
            .register_worker(worker("worker-1", "rev-1", &["tool_use"], 2))
            .unwrap();
        let request_id = Uuid::new_v4();
        gateway
            .submit(request(request_id, "digest-1", &["tool_use"], 4_000, true))
            .unwrap();
        let admission = gateway.admit(request_id).unwrap();
        gateway
            .recover_after_restart(admission.admission_id)
            .unwrap();
        assert_eq!(
            gateway.admission(admission.admission_id).unwrap().state,
            InferenceState::Uncertain
        );
        assert_eq!(
            gateway.retry_uncertain(admission.admission_id, false),
            Err(InferenceError::UncertainExecution)
        );
        gateway
            .retry_uncertain(admission.admission_id, true)
            .unwrap();
        assert_eq!(
            gateway.admission(admission.admission_id).unwrap().state,
            InferenceState::Admitted
        );

        let non_replayable_id = Uuid::new_v4();
        gateway
            .submit(request(
                non_replayable_id,
                "digest-2",
                &["tool_use"],
                4_000,
                false,
            ))
            .unwrap();
        let non_replayable = gateway.admit(non_replayable_id).unwrap();
        gateway
            .recover_after_restart(non_replayable.admission_id)
            .unwrap();
        assert_eq!(
            gateway.retry_uncertain(non_replayable.admission_id, true),
            Err(InferenceError::UncertainExecution)
        );
        gateway
            .cancel_uncertain(non_replayable.admission_id)
            .unwrap();
        assert_eq!(
            gateway
                .admission(non_replayable.admission_id)
                .unwrap()
                .state,
            InferenceState::Cancelled
        );
    }

    #[test]
    fn capacity_generation_overflow_is_rejected() {
        let mut gateway = InferenceGateway::new();
        gateway
            .register_worker(worker("worker-1", "rev-1", &["tool_use"], 1))
            .unwrap();
        let request_id = Uuid::new_v4();
        gateway
            .submit(request(request_id, "digest-1", &["tool_use"], 4_000, false))
            .unwrap();
        let admission = gateway.admit(request_id).unwrap();
        gateway
            .capacity_leases
            .get_mut(&admission.capacity_lease_id)
            .unwrap()
            .generation = i64::MAX;
        assert_eq!(
            gateway.reassign_capacity(admission.capacity_lease_id),
            Err(InferenceError::GenerationOverflow)
        );
    }
}
