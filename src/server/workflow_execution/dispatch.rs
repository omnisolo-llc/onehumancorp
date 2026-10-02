//! Durable admission and a tracked, single-attempt worker. A request replay is
//! a receipt lookup, never permission to resume an old execution capability.
use super::receipts::{
    CompletionProof, Error, Receipt, ReceiptStore, RequestMetadata, Reservation, StoredPhase,
};
use super::{AdmittedAnalysis, RegistrationLease, WorkflowExecution};
use axum::http::HeaderMap;
use server_common::Claims;
use std::{future::Future, pin::Pin, sync::Arc};
use uuid::Uuid;

pub(crate) fn request_id(headers: &HeaderMap) -> Result<Uuid, Error> {
    let mut values = headers.get_all("idempotency-key").iter();
    let value = values.next().ok_or(Error::RequestIdentity)?;
    if values.next().is_some() {
        return Err(Error::RequestIdentity);
    }
    let value = value.to_str().map_err(|_| Error::RequestIdentity)?;
    let id = Uuid::parse_str(value).map_err(|_| Error::RequestIdentity)?;
    if id.is_nil() || id.to_string() != value {
        return Err(Error::RequestIdentity);
    }
    Ok(id)
}

struct DurableLease {
    receipts: ReceiptStore,
    proof: CompletionProof,
    registration: Option<Arc<dyn RegistrationLease>>,
}
impl RegistrationLease for DurableLease {
    fn is_active(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async move {
            if !self
                .receipts
                .get(&self.proof.authority, &self.proof.receipt.id)
                .await
                .is_ok_and(|r| r.phase == StoredPhase::Dispatching)
            {
                return false;
            }
            match &self.registration {
                Some(registration) => registration.is_active().await,
                None => true,
            }
        })
    }
}

impl WorkflowExecution {
    pub(crate) fn with_receipts(mut self, database: crate::persistence::AppDatabase) -> Self {
        self.receipts = Some(ReceiptStore::new(database));
        self
    }
    pub(crate) fn receipt_store(&self) -> Result<&ReceiptStore, Error> {
        self.receipts.as_ref().ok_or(Error::Unavailable)
    }
    pub(crate) async fn prepare(
        &self,
        claims: &Claims,
        headers: &HeaderMap,
        task: &str,
        request: RequestMetadata,
    ) -> Result<Reservation, Error> {
        let authority = self.authorize(claims, headers).await?;
        let store = self.receipt_store()?;
        if let Some(prior) = store.find_request(&authority, task, &request).await? {
            return Ok(prior);
        }
        let input = self
            .admit(
                claims,
                headers,
                task,
                &request.requested_model,
                &request.workflow,
            )
            .await?;
        let mut reserved = store.reserve(input, request).await?;
        if !reserved.replayed()
            && let Some(funding) = &self.funding
        {
            match funding.reserve_receipt(reserved.receipt()).await {
                Ok(ticket) => reserved.bind_usage(ticket),
                Err(error) => {
                    // No claim or worker exists. A failed/ambiguous reservation
                    // cannot cause an effect, and this request is never retried.
                    let _ = store.cancel(&authority, &reserved.receipt().id).await;
                    return Err(error);
                }
            }
        }
        Ok(reserved)
    }
    pub(crate) async fn dispatch(
        self: &Arc<Self>,
        reservation: Reservation,
        registration: Option<Arc<dyn RegistrationLease>>,
    ) -> Result<Receipt, Error> {
        let accepted = reservation.receipt().clone();
        let store = self.receipt_store()?.clone();
        let Some(lease) = store.claim(reservation).await? else {
            return Ok(accepted);
        };
        let proof = lease.completion_proof();
        let input: AdmittedAnalysis =
            lease
                .into_admitted()
                .with_registration(Arc::new(DurableLease {
                    receipts: store.clone(),
                    proof: proof.clone(),
                    registration: registration.clone(),
                }));
        let execution = self.clone();
        let mut workers = self.workers.lock().await;
        // JoinSet retains ownership and observes finished workers. A panic or
        // shutdown leaves a durable lease that expires to an unknown outcome.
        while let Some(joined) = workers.try_join_next() {
            if joined.is_err() {
                tracing::warn!(
                    "workflow worker ended without an acknowledged completion; durable lease retained"
                );
            }
        }
        let accepted_id = accepted.id.clone();
        workers.spawn(async move {
            let outcome = execution.run(input).await;
            match store.finish(&proof, &outcome).await {
                Ok(receipt) => {
                    if let Some(registration) = registration {
                        let status = match receipt.phase {
                            StoredPhase::Completed => "COMPLETED",
                            StoredPhase::Cancelled => "CANCELLED",
                            _ => "OUTCOME_UNKNOWN",
                        };
                        registration.finished(status).await;
                    }
                }
                Err(error) => tracing::warn!(receipt_id=%accepted_id, error=%error,
                    database_failure=std::error::Error::source(&error).is_some(),
                    "workflow completion write unconfirmed; durable lease requires reconciliation"),
            }
        });
        Ok(accepted)
    }
    async fn reconcile_receipt(&self, receipt: &Receipt) {
        let Some(policy) = &receipt.funding else {
            return;
        };
        let Ok(store) = self.receipt_store() else {
            return;
        };
        let Ok(context) = super::funding::FundingContext::new(policy.clone(), store.database())
        else {
            return;
        };
        let result = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            match receipt.phase {
                StoredPhase::Cancelled => {
                    context
                        .ledger
                        .cancel_before_dispatch(&receipt.tenant_id, &receipt.id)
                        .await
                }
                StoredPhase::OutcomeUnknown => {
                    context
                        .ledger
                        .require_reconciliation(&receipt.tenant_id, &receipt.id)
                        .await
                }
                _ => Ok(()),
            }
        })
        .await;
        super::funding::observe_write(&receipt.id, "reconcile_receipt", result);
    }

    pub(crate) async fn list_receipts(
        &self,
        claims: &Claims,
        headers: &HeaderMap,
        query: &super::receipts::ReceiptQuery,
    ) -> Result<Vec<Receipt>, Error> {
        let authority = self.authorize(claims, headers).await?;
        let receipts = self.receipt_store()?.list(&authority, query).await?;
        let reconciliation = tokio::time::timeout(std::time::Duration::from_secs(5), async {
            for receipt in &receipts {
                self.reconcile_receipt(receipt).await;
            }
        })
        .await;
        if reconciliation.is_err() {
            tracing::warn!(
                "workflow history accounting reconciliation timed out; durable holds retained"
            );
        }
        Ok(receipts)
    }
    pub(crate) async fn receipt_by_request_id(
        &self,
        claims: &Claims,
        headers: &HeaderMap,
        request_id: &str,
    ) -> Result<Receipt, Error> {
        let authority = self.authorize(claims, headers).await?;
        let receipt = self
            .receipt_store()?
            .by_request_id(&authority, request_id)
            .await?;
        self.reconcile_receipt(&receipt).await;
        Ok(receipt)
    }
    pub(crate) async fn get_receipt(
        &self,
        claims: &Claims,
        headers: &HeaderMap,
        id: &str,
    ) -> Result<Receipt, Error> {
        let authority = self.authorize(claims, headers).await?;
        let receipt = self.receipt_store()?.get(&authority, id).await?;
        self.reconcile_receipt(&receipt).await;
        Ok(receipt)
    }
    pub(crate) async fn cancel_receipt(
        &self,
        claims: &Claims,
        headers: &HeaderMap,
        id: &str,
    ) -> Result<Receipt, Error> {
        let authority = self.authorize(claims, headers).await?;
        let receipt = self.receipt_store()?.cancel(&authority, id).await?;
        self.reconcile_receipt(&receipt).await;
        Ok(receipt)
    }
    #[cfg(test)]
    pub(crate) async fn wait_for_workers(&self) {
        let mut workers = self.workers.lock().await;
        while workers.join_next().await.is_some() {}
    }
    #[cfg(test)]
    pub(crate) async fn stop_workers(&self) {
        let mut workers = self.workers.lock().await;
        workers.abort_all();
        while workers.join_next().await.is_some() {}
    }
}
