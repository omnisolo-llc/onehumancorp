//! Admission and lifecycle for tenant-submitted text analysis. A signed web
//! session is not a workspace, tool, filesystem, or process-execution grant.
use server_common::Claims;
use std::{future::Future, pin::Pin, sync::Arc, time::Duration};

#[path = "workflow_execution/dispatch.rs"]
pub(crate) mod dispatch;
#[path = "workflow_execution/funding.rs"]
pub(crate) mod funding;
#[path = "workflow_execution/receipts.rs"]
pub(crate) mod receipts;
#[cfg(test)]
#[path = "workflow_execution/receipts_contract_test.rs"]
mod receipts_contract_test;

const MAX_TASK_CHARACTERS: usize = 16_000;
const MAX_OUTPUT_BYTES: usize = 64_000;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct AnalysisPolicy {
    pub provider: String,
    pub model: String,
    pub max_output_tokens: i32,
}
impl AnalysisPolicy {
    pub fn new(
        provider: String,
        model: String,
        max_output_tokens: i32,
    ) -> Result<Self, AdmissionError> {
        if !matches!(
            provider.as_str(),
            "openai" | "openai-compatible" | "anthropic" | "minimax" | "ollama"
        ) || model.trim().is_empty()
            || model.len() > 200
            || !(1..=4096).contains(&max_output_tokens)
        {
            return Err(AdmissionError::Unavailable);
        }
        Ok(Self {
            provider,
            model,
            max_output_tokens,
        })
    }
}

#[derive(Clone)]
pub(crate) struct Authority {
    tenant_id: String,
    actor_id: String,
    token_id: String,
    expires_at: i64,
    // Retained as identity context, never passed to a provider or used as a
    // substitute for the Store's actual token revocation and membership checks.
    session_id: Option<String>,
}

pub struct AdmittedAnalysis {
    authority: Authority,
    task: String,
    policy: AnalysisPolicy,
    funding: Option<funding::FundingPolicy>,
    input_token_bound: Option<i64>,
    prepared: Option<omnisolo_builtin_agent::tenant_analysis::PreparedTextAnalysis>,
    usage: Option<funding::UsageTicket>,
    execution_id: String,
    // A non-cloneable admitted request is executed once and owns one bounded
    // in-process slot until completion, cancellation, or failed registration.
    _slot: tokio::sync::OwnedSemaphorePermit,
    registration: Option<Arc<dyn RegistrationLease>>,
}
impl AdmittedAnalysis {
    pub fn with_registration(mut self, registration: Arc<dyn RegistrationLease>) -> Self {
        self.registration = Some(registration);
        self
    }
    pub fn tenant_id(&self) -> &str {
        &self.authority.tenant_id
    }
    pub fn actor_id(&self) -> &str {
        &self.authority.actor_id
    }
    pub fn prepared(
        &self,
    ) -> Option<&omnisolo_builtin_agent::tenant_analysis::PreparedTextAnalysis> {
        self.prepared.as_ref()
    }
    pub fn task(&self) -> &str {
        &self.task
    }
    pub fn session_id(&self) -> Option<&str> {
        self.authority.session_id.as_deref()
    }
    pub fn policy(&self) -> &AnalysisPolicy {
        &self.policy
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmissionError {
    Invalid,
    Forbidden,
    Unavailable,
}
impl AdmissionError {
    pub fn status(&self) -> axum::http::StatusCode {
        match self {
            Self::Invalid => axum::http::StatusCode::BAD_REQUEST,
            Self::Forbidden => axum::http::StatusCode::FORBIDDEN,
            Self::Unavailable => axum::http::StatusCode::SERVICE_UNAVAILABLE,
        }
    }
    pub fn message(&self) -> &'static str {
        match self {
            Self::Invalid => {
                "Only the configured text-analysis model and supported task options are available"
            }
            Self::Forbidden => "Current owner or administrator authority is required",
            Self::Unavailable => "Tenant text analysis is unavailable; no task was started",
        }
    }
}

/// The production adapter performs one configured inference request. Tests may
/// observe this final boundary, but admission/lifecycle always use this module.
pub trait RegistrationLease: Send + Sync {
    fn finished<'a>(&'a self, _status: &'a str) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async {})
    }
    fn is_active(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>>;
}

pub struct InferenceResult {
    pub output: Option<String>,
    pub provider_request_id: Option<String>,
    pub counts: Option<server_harness::middleware::usage_ledger::TokenCounts>,
}

pub trait TextInference: Send + Sync {
    fn prepare(
        &self,
        _task: &str,
        _policy: &AnalysisPolicy,
    ) -> Result<omnisolo_builtin_agent::tenant_analysis::PreparedTextAnalysis, AdmissionError> {
        Err(AdmissionError::Unavailable)
    }
    fn infer<'a>(
        &'a self,
        input: &'a AdmittedAnalysis,
    ) -> Pin<Box<dyn Future<Output = Result<InferenceResult, ()>> + Send + 'a>>;
}

pub struct WorkflowExecution {
    store: Arc<server_auth::Store>,
    configured: Option<(AnalysisPolicy, Arc<dyn TextInference>)>,
    slots: Arc<tokio::sync::Semaphore>,
    receipts: Option<receipts::ReceiptStore>,
    funding: Option<funding::FundingContext>,
    workers: tokio::sync::Mutex<tokio::task::JoinSet<()>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AnalysisOutcome {
    Completed(String),
    Cancelled,
    BudgetUnavailable,
    OutcomeUnknown,
}

fn authority_admission_error(
    error: server_auth::commit_authority::AuthorityError,
) -> AdmissionError {
    match error {
        server_auth::commit_authority::AuthorityError::Forbidden => AdmissionError::Forbidden,
        _ => AdmissionError::Unavailable,
    }
}

impl WorkflowExecution {
    pub fn unavailable(store: Arc<server_auth::Store>) -> Self {
        Self {
            store,
            configured: None,
            slots: Arc::new(tokio::sync::Semaphore::new(4)),
            receipts: None,
            funding: None,
            workers: tokio::sync::Mutex::new(tokio::task::JoinSet::new()),
        }
    }
    #[cfg(test)]
    pub fn configured(
        store: Arc<server_auth::Store>,
        policy: AnalysisPolicy,
        inference: Arc<dyn TextInference>,
    ) -> Self {
        Self {
            store,
            configured: Some((policy, inference)),
            slots: Arc::new(tokio::sync::Semaphore::new(4)),
            receipts: None,
            funding: None,
            workers: tokio::sync::Mutex::new(tokio::task::JoinSet::new()),
        }
    }
    pub(crate) fn configured_funded(
        store: Arc<server_auth::Store>,
        policy: AnalysisPolicy,
        inference: Arc<dyn TextInference>,
        funding: funding::FundingContext,
    ) -> Self {
        if funding.policy.validate(&policy).is_err() {
            return Self::unavailable(store);
        }
        Self {
            store,
            configured: Some((policy, inference)),
            slots: Arc::new(tokio::sync::Semaphore::new(4)),
            receipts: None,
            funding: Some(funding),
            workers: tokio::sync::Mutex::new(tokio::task::JoinSet::new()),
        }
    }
    pub fn policy(&self) -> Option<&AnalysisPolicy> {
        self.configured.as_ref().map(|(policy, _)| policy)
    }

    pub(crate) async fn available_policy(
        &self,
        claims: &Claims,
        headers: &axum::http::HeaderMap,
    ) -> Result<Option<&AnalysisPolicy>, AdmissionError> {
        let authority = self.authorize(claims, headers).await?;
        if let Some(funding) = &self.funding {
            if funding.policy.operator_tenant != authority.tenant_id {
                return Err(AdmissionError::Forbidden);
            }
            if !tokio::time::timeout(
                Duration::from_secs(3),
                funding.ledger.summary(&authority.tenant_id),
            )
            .await
            .is_ok_and(|value| value.is_ok())
            {
                return Ok(None);
            }
        }
        Ok(self.policy())
    }

    async fn current_authority(&self, authority: &Authority) -> Result<(), AdmissionError> {
        server_auth::commit_authority::require_current_owner(
            &self.store,
            &authority.tenant_id,
            &authority.actor_id,
            &authority.token_id,
            authority.expires_at,
        )
        .await
        .map_err(authority_admission_error)
    }

    pub(crate) fn canonical_pg_authority(
        &self,
        pool: &sqlx::PgPool,
    ) -> Result<server_auth::commit_authority::CanonicalPgAuthority, AdmissionError> {
        server_auth::commit_authority::CanonicalPgAuthority::bind(self.store.clone(), pool)
            .map_err(authority_admission_error)
    }

    // Receipt reconciliation needs current identity even when no provider is available.
    pub(crate) async fn authorize(
        &self,
        claims: &Claims,
        headers: &axum::http::HeaderMap,
    ) -> Result<Authority, AdmissionError> {
        let owner = server_auth::commit_authority::verify_owner(&self.store, claims, headers)
            .await
            .map_err(authority_admission_error)?;
        Ok(Authority {
            tenant_id: owner.tenant_id().to_owned(),
            actor_id: owner.actor_id().to_owned(),
            token_id: owner.token_id().to_owned(),
            expires_at: owner.expires_at(),
            session_id: owner.session_id().map(str::to_owned),
        })
    }

    pub async fn admit(
        &self,
        claims: &Claims,
        headers: &axum::http::HeaderMap,
        task: &str,
        requested_model: &str,
        workflow: &str,
    ) -> Result<AdmittedAnalysis, AdmissionError> {
        let authority = self.authorize(claims, headers).await?;
        if let Some(funding) = &self.funding
            && funding.policy.operator_tenant != authority.tenant_id
        {
            return Err(AdmissionError::Forbidden);
        }
        if task.trim().is_empty()
            || task.chars().count() > MAX_TASK_CHARACTERS
            || !matches!(workflow, "" | "expert_task" | "analysis")
        {
            return Err(AdmissionError::Invalid);
        }
        let (policy, inference) = self
            .configured
            .as_ref()
            .ok_or(AdmissionError::Unavailable)?;
        if !requested_model.trim().is_empty()
            && requested_model != "Auto"
            && requested_model != policy.model
        {
            return Err(AdmissionError::Invalid);
        }
        let prepared = match inference.prepare(task, policy) {
            Ok(request) => Some(request),
            Err(error) if self.funding.is_some() => return Err(error),
            Err(_) => None, // Only the test-only unmetered fixture constructor.
        };
        let slot = self
            .slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| AdmissionError::Unavailable)?;
        Ok(AdmittedAnalysis {
            authority,
            task: task.to_owned(),
            policy: policy.clone(),
            funding: self.funding.as_ref().map(|funding| funding.policy.clone()),
            input_token_bound: prepared.as_ref().map(|request| request.input_token_bound()),
            prepared,
            usage: None,
            execution_id: uuid::Uuid::new_v4().to_string(),
            _slot: slot,
            registration: None,
        })
    }

    async fn current_execution_authority(
        &self,
        input: &AdmittedAnalysis,
    ) -> Result<(), AdmissionError> {
        self.current_authority(&input.authority).await?;
        if let Some(registration) = &input.registration {
            let active = tokio::time::timeout(Duration::from_secs(3), registration.is_active())
                .await
                .map_err(|_| AdmissionError::Unavailable)?;
            if !active {
                return Err(AdmissionError::Forbidden);
            }
        }
        Ok(())
    }

    pub async fn run(&self, mut input: AdmittedAnalysis) -> AnalysisOutcome {
        if self.current_execution_authority(&input).await.is_err() {
            if let (Some(funding), Some(ticket)) = (&self.funding, &input.usage) {
                funding.cancel_reserved(ticket).await;
            }
            return AnalysisOutcome::Cancelled;
        }
        let Some((policy, inference)) = &self.configured else {
            return AnalysisOutcome::Cancelled;
        };
        if policy != &input.policy {
            return AnalysisOutcome::Cancelled;
        }
        if let Some(funding) = &self.funding {
            if input.funding.as_ref() != Some(&funding.policy) {
                return AnalysisOutcome::Cancelled;
            }
            if input.usage.is_none() {
                match funding.reserve_direct(&input).await {
                    Ok(ticket) => input.usage = Some(ticket),
                    Err(receipts::Error::Budget) => return AnalysisOutcome::BudgetUnavailable,
                    Err(_) => return AnalysisOutcome::OutcomeUnknown,
                }
            }
            let ticket = input
                .usage
                .as_ref()
                .expect("funded admission has a reservation");
            if funding.dispatched(ticket).await.is_err() {
                return AnalysisOutcome::OutcomeUnknown;
            }
            // A committed in-flight reservation alone is never permission to
            // ignore a cancellation or a changed owner immediately before I/O.
            if self.current_execution_authority(&input).await.is_err() {
                funding.unknown(ticket).await;
                return AnalysisOutcome::OutcomeUnknown;
            }
        }
        let future = inference.infer(&input);
        tokio::pin!(future);
        let deadline = tokio::time::sleep(Duration::from_secs(90));
        tokio::pin!(deadline);
        let mut recheck = tokio::time::interval(Duration::from_millis(250));
        recheck.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        recheck.tick().await;
        let outcome = loop {
            tokio::select! {
                biased;
                _=&mut deadline=>break AnalysisOutcome::OutcomeUnknown,
                _=recheck.tick()=>{
                    if self.current_execution_authority(&input).await.is_err() {break AnalysisOutcome::OutcomeUnknown;}
                }
                result=&mut future=>{
                    if let Ok(result)=&result
                        && let (Some(funding),Some(ticket))=(&self.funding,&input.usage) {
                        let acknowledged = funding.settle(ticket,&server_harness::middleware::usage_ledger::UsageReceipt {
                            provider_request_id:result.provider_request_id.clone().filter(|id|!id.is_empty()).unwrap_or_else(||format!("unknown:{}",ticket.event_id)),
                            counts:result.counts.clone(),
                        }).await;
                        // Provider text alone cannot acknowledge a funded
                        // effect. Preserve uncertainty and the durable hold
                        // until accounting confirms the observed receipt.
                        if !acknowledged { break AnalysisOutcome::OutcomeUnknown; }
                    }
                    if self.current_execution_authority(&input).await.is_err() {break AnalysisOutcome::OutcomeUnknown;}
                    break match result {
                        Ok(InferenceResult{output:Some(output),..}) if !output.trim().is_empty() && output.len()<=MAX_OUTPUT_BYTES=>AnalysisOutcome::Completed(output),
                        _=>AnalysisOutcome::OutcomeUnknown,
                    };
                }
            }
        };
        if outcome == AnalysisOutcome::OutcomeUnknown
            && let (Some(funding), Some(ticket)) = (&self.funding, &input.usage)
        {
            funding.unknown(ticket).await;
        }
        outcome
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{
        Mutex,
        atomic::{AtomicBool, Ordering},
    };

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Observed {
        tenant: String,
        actor: String,
        task: String,
        model: String,
    }
    struct Inference {
        seen: Mutex<Vec<Observed>>,
        fail: bool,
        hold: bool,
        started: tokio::sync::Notify,
        release: tokio::sync::Notify,
        dropped: AtomicBool,
    }
    impl Inference {
        fn new(fail: bool, hold: bool) -> Arc<Self> {
            Arc::new(Self {
                seen: Mutex::new(vec![]),
                fail,
                hold,
                started: tokio::sync::Notify::new(),
                release: tokio::sync::Notify::new(),
                dropped: AtomicBool::new(false),
            })
        }
    }
    struct CompletionGuard<'a>(&'a AtomicBool);
    impl Drop for CompletionGuard<'_> {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }
    impl TextInference for Inference {
        fn infer<'a>(
            &'a self,
            input: &'a AdmittedAnalysis,
        ) -> Pin<Box<dyn Future<Output = Result<InferenceResult, ()>> + Send + 'a>> {
            Box::pin(async move {
                let _guard = CompletionGuard(&self.dropped);
                self.seen.lock().unwrap().push(Observed {
                    tenant: input.tenant_id().into(),
                    actor: input.actor_id().into(),
                    task: input.task().into(),
                    model: input.policy().model.clone(),
                });
                self.started.notify_one();
                if self.hold {
                    self.release.notified().await;
                }
                if self.fail {
                    Err(())
                } else {
                    Ok(InferenceResult {
                        output: Some(format!("analysis of {}", input.task())),
                        provider_request_id: None,
                        counts: None,
                    })
                }
            })
        }
    }
    struct Fixture {
        store: Arc<server_auth::Store>,
        execution: Arc<WorkflowExecution>,
        inference: Arc<Inference>,
        identities: Vec<(Claims, axum::http::HeaderMap)>,
    }
    impl Fixture {
        async fn new(fail: bool, hold: bool) -> Self {
            let store = Arc::new(server_auth::Store::new());
            let inference = Inference::new(fail, hold);
            let execution = Arc::new(WorkflowExecution::configured(
                store.clone(),
                AnalysisPolicy::new("ollama".into(), "configured-model".into(), 512).unwrap(),
                inference.clone(),
            ));
            let mut identities = vec![];
            for tenant in ["dispatch-a", "dispatch-b"] {
                let user = store
                    .create_user(
                        format!("owner-{tenant}"),
                        format!("owner-{tenant}@example.test"),
                        "public-local-dispatch-fixture-password".into(),
                        vec![server_auth::ROLE_ADMIN.into()],
                        tenant.into(),
                    )
                    .await
                    .unwrap();
                let token = store.issue_token(&user).unwrap();
                let claims = store.validate_token(&token).await.unwrap();
                let mut headers = axum::http::HeaderMap::new();
                headers.insert(
                    axum::http::header::AUTHORIZATION,
                    format!("Bearer {token}").parse().unwrap(),
                );
                identities.push((claims, headers));
            }
            Self {
                store,
                execution,
                inference,
                identities,
            }
        }
        async fn admit(&self, index: usize) -> AdmittedAnalysis {
            let (claims, headers) = &self.identities[index];
            self.execution
                .admit(
                    claims,
                    headers,
                    &format!(
                        "private text for {}",
                        claims.organization_id.as_deref().unwrap()
                    ),
                    "Auto",
                    "expert_task",
                )
                .await
                .unwrap()
        }
    }

    #[tokio::test]
    async fn two_signed_tenants_reach_only_their_own_immutable_execution_snapshot() {
        let f = Fixture::new(false, false).await;
        for index in 0..2 {
            let input = f.admit(index).await;
            let task = input.task().to_owned();
            assert_eq!(
                input.session_id(),
                f.identities[index].0.session_id.as_deref()
            );
            assert_eq!(
                f.execution.run(input).await,
                AnalysisOutcome::Completed(format!("analysis of {task}"))
            );
        }
        let seen = f.inference.seen.lock().unwrap();
        assert_eq!(seen.len(), 2);
        for (index, call) in seen.iter().enumerate() {
            assert_eq!(
                call.tenant,
                f.identities[index].0.organization_id.as_deref().unwrap()
            );
            assert_eq!(call.actor, f.identities[index].0.sub);
            assert_eq!(call.model, "configured-model");
            assert!(call.task.contains(&call.tenant));
        }
    }

    #[tokio::test]
    async fn forged_claims_or_duplicate_bearers_cannot_authorize_dispatch() {
        let f = Fixture::new(false, false).await;
        let (claims, headers) = &f.identities[0];
        let mut forged = claims.clone();
        forged.sub = f.identities[1].0.sub.clone();
        assert!(matches!(
            f.execution
                .admit(&forged, headers, "private text", "Auto", "expert_task")
                .await,
            Err(AdmissionError::Forbidden)
        ));
        let mut duplicated = headers.clone();
        duplicated.append(
            axum::http::header::AUTHORIZATION,
            headers[axum::http::header::AUTHORIZATION].clone(),
        );
        assert!(matches!(
            f.execution
                .admit(claims, &duplicated, "private text", "Auto", "expert_task")
                .await,
            Err(AdmissionError::Forbidden)
        ));
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn unsupported_model_or_workspace_workflow_creates_no_execution() {
        let f = Fixture::new(false, false).await;
        let (claims, headers) = &f.identities[0];
        for (model, workflow) in [
            ("unconfigured-model", "expert_task"),
            ("Auto", "ohc_review_branch"),
            ("Auto", "ohc_business_swarm"),
        ] {
            assert!(matches!(
                f.execution
                    .admit(claims, headers, "must not dispatch", model, workflow)
                    .await,
                Err(AdmissionError::Invalid)
            ));
        }
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn missing_execution_configuration_does_not_accept_a_task() {
        let f = Fixture::new(false, false).await;
        let execution = WorkflowExecution::unavailable(f.store.clone());
        let (claims, headers) = &f.identities[0];
        assert!(matches!(
            execution
                .admit(claims, headers, "must not dispatch", "Auto", "expert_task")
                .await,
            Err(AdmissionError::Unavailable)
        ));
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn receipt_reconciliation_authentication_does_not_require_an_available_provider() {
        let f = Fixture::new(false, false).await;
        let execution = WorkflowExecution::unavailable(f.store.clone());
        let (claims, headers) = &f.identities[0];
        let authority = execution.authorize(claims, headers).await.unwrap();
        assert_eq!(
            authority.tenant_id,
            claims.organization_id.as_deref().unwrap()
        );
        assert_eq!(authority.actor_id, claims.sub);
        let mut forged = claims.clone();
        forged.sub = f.identities[1].0.sub.clone();
        assert!(matches!(
            execution.authorize(&forged, headers).await,
            Err(AdmissionError::Forbidden)
        ));
        f.store
            .update_user(
                &claims.sub,
                None,
                Some(vec!["staff".into()]),
                None,
                claims.organization_id.as_deref().unwrap(),
            )
            .await
            .unwrap();
        assert!(matches!(
            execution.authorize(claims, headers).await,
            Err(AdmissionError::Forbidden)
        ));
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn role_revocation_after_admission_prevents_the_first_effect() {
        let f = Fixture::new(false, false).await;
        let input = f.admit(0).await;
        let claims = &f.identities[0].0;
        f.store
            .update_user(
                &claims.sub,
                None,
                Some(vec!["staff".into()]),
                None,
                claims.organization_id.as_deref().unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(f.execution.run(input).await, AnalysisOutcome::Cancelled);
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn actual_token_revocation_before_launch_prevents_dispatch() {
        let f = Fixture::new(false, false).await;
        let input = f.admit(0).await;
        let claims = &f.identities[0].0;
        f.store
            .revoke_token(
                claims.jti.clone(),
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                claims.organization_id.as_deref().unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(f.execution.run(input).await, AnalysisOutcome::Cancelled);
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn an_inactive_user_cannot_run_previously_admitted_work() {
        let f = Fixture::new(false, false).await;
        let input = f.admit(0).await;
        let claims = &f.identities[0].0;
        f.store
            .update_user(
                &claims.sub,
                None,
                None,
                Some(false),
                claims.organization_id.as_deref().unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(f.execution.run(input).await, AnalysisOutcome::Cancelled);
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn revocation_during_inference_cancels_local_work_and_keeps_unknown_outcome() {
        let f = Fixture::new(false, true).await;
        let input = f.admit(0).await;
        let execution = f.execution.clone();
        let run = tokio::spawn(async move { execution.run(input).await });
        tokio::time::timeout(Duration::from_secs(2), f.inference.started.notified())
            .await
            .unwrap();
        let claims = &f.identities[0].0;
        f.store
            .revoke_token(
                claims.jti.clone(),
                chrono::DateTime::from_timestamp(claims.exp, 0).unwrap(),
                claims.organization_id.as_deref().unwrap(),
            )
            .await
            .unwrap();
        let outcome = tokio::time::timeout(Duration::from_secs(2), run)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(outcome, AnalysisOutcome::OutcomeUnknown);
        assert!(f.inference.dropped.load(Ordering::SeqCst));
        f.inference.release.notify_one();
        tokio::task::yield_now().await;
        assert_eq!(
            f.inference.seen.lock().unwrap().len(),
            1,
            "no retry or late second effect"
        );
    }

    #[tokio::test]
    async fn an_inference_error_is_unknown_and_is_never_automatically_retried() {
        let f = Fixture::new(true, false).await;
        assert_eq!(
            f.execution.run(f.admit(0).await).await,
            AnalysisOutcome::OutcomeUnknown
        );
        assert_eq!(f.inference.seen.lock().unwrap().len(), 1);
    }
    struct Registration(AtomicBool);
    impl RegistrationLease for Registration {
        fn is_active(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
            Box::pin(async move { self.0.load(Ordering::SeqCst) })
        }
    }

    #[tokio::test]
    async fn removed_registration_prevents_the_first_inference() {
        let f = Fixture::new(false, false).await;
        let input = f
            .admit(0)
            .await
            .with_registration(Arc::new(Registration(AtomicBool::new(false))));
        assert_eq!(f.execution.run(input).await, AnalysisOutcome::Cancelled);
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn removed_registration_cancels_pending_work_without_accepting_late_completion() {
        let f = Fixture::new(false, true).await;
        let registration = Arc::new(Registration(AtomicBool::new(true)));
        let input = f.admit(0).await.with_registration(registration.clone());
        let execution = f.execution.clone();
        let run = tokio::spawn(async move { execution.run(input).await });
        tokio::time::timeout(Duration::from_secs(2), f.inference.started.notified())
            .await
            .unwrap();
        registration.0.store(false, Ordering::SeqCst);
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(2), run)
                .await
                .unwrap()
                .unwrap(),
            AnalysisOutcome::OutcomeUnknown
        );
        assert!(f.inference.dropped.load(Ordering::SeqCst));
        f.inference.release.notify_one();
        tokio::task::yield_now().await;
        assert_eq!(f.inference.seen.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn admission_is_bounded_and_releases_slots_when_uncommitted_work_is_dropped() {
        let f = Fixture::new(false, false).await;
        let mut admitted = Vec::new();
        for _ in 0..4 {
            admitted.push(f.admit(0).await);
        }
        let (claims, headers) = &f.identities[0];
        assert!(matches!(
            f.execution
                .admit(claims, headers, "excess work", "Auto", "expert_task")
                .await,
            Err(AdmissionError::Unavailable)
        ));
        drop(admitted.pop());
        let replacement = f.admit(0).await;
        drop(replacement);
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn an_expired_admitted_snapshot_cannot_start_inference() {
        let f = Fixture::new(false, false).await;
        let mut input = f.admit(0).await;
        // Advance only this private test snapshot's deadline; no production
        // caller can edit an admitted identity or extend its signed expiry.
        input.authority.expires_at = chrono::Utc::now().timestamp() - 1;
        assert_eq!(f.execution.run(input).await, AnalysisOutcome::Cancelled);
        assert!(f.inference.seen.lock().unwrap().is_empty());
    }
}
