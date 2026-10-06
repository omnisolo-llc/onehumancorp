//! Assistant text tasks are metadata over canonical admitted workflow receipts.
//! A link commits on the receipt database before dispatch; it is not a queue.
use super::{
    Authority, WorkflowExecution, dispatch,
    receipts::{Error, Receipt, ReceiptStore, RequestMetadata, StoredPhase},
};
use axum::{
    Json, Router,
    extract::{Extension, Path, Query},
    http::{HeaderMap, StatusCode},
    routing::get,
};
use sea_orm::{ConnectionTrait, DatabaseTransaction, QueryResult, Statement, Value};
use serde::Deserialize;
use server_common::Claims;
use std::sync::Arc;

pub(crate) fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/by-request/{id}", get(by_request))
        .route("/tasks/{id}", get(get_task).patch(mutate_task))
}
fn ask() -> String {
    "Ask".into()
}
fn auto() -> String {
    "Auto".into()
}
fn text() -> String {
    "Text".into()
}
fn guarded() -> String {
    "Guarded".into()
}
fn workspace() -> String {
    "Personal OS".into()
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct TaskRequest {
    prompt: String,
    #[serde(default = "workspace")]
    workspace: String,
    #[serde(default)]
    constraints: String,
    #[serde(default = "ask")]
    mode: String,
    #[serde(default = "auto")]
    model: String,
    #[serde(default = "auto")]
    provider: String,
    #[serde(default = "text")]
    output_format: String,
    #[serde(default)]
    work_directory: String,
    #[serde(default = "guarded")]
    permission_profile: String,
}
impl TaskRequest {
    fn validate(&self) -> Result<(String, String), Error> {
        if self.prompt.trim().is_empty()
            || self.prompt.contains('\0')
            || self.workspace.trim().is_empty()
            || self.workspace.chars().count() > 80
            || self.workspace.contains('\0')
            || self.constraints.contains('\0')
            || self.mode != "Ask"
            || self.model != "Auto"
            || self.provider != "Auto"
            || self.output_format != "Text"
            || !self.work_directory.is_empty()
            || self.permission_profile != "Guarded"
        {
            return Err(Error::Invalid);
        }
        let task = if self.constraints.trim().is_empty() {
            self.prompt.clone()
        } else {
            format!(
                "{}\n\nAdditional user constraints:\n{}",
                self.prompt, self.constraints
            )
        };
        if task.chars().count() > super::MAX_TASK_CHARACTERS {
            return Err(Error::Invalid);
        }
        let title = self
            .prompt
            .trim()
            .lines()
            .next()
            .unwrap_or("")
            .chars()
            .take(160)
            .collect();
        Ok((task, title))
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct Mutation {
    action: String,
    #[serde(default)]
    source_receipt_id: Option<String>,
    #[serde(default)]
    title: Option<String>,
}
#[derive(Default, Deserialize)]
pub(crate) struct ListQuery {
    pub before: Option<String>,
}
#[derive(Clone)]
struct TaskLink {
    id: String,
    tenant: String,
    actor: String,
    request: String,
    receipt: String,
    workspace: String,
    title: String,
    archived: bool,
    created: i64,
    updated: i64,
}
fn field<T: sea_orm::TryGetable>(row: &QueryResult, key: &str) -> Result<T, Error> {
    row.try_get("", key).map_err(|_| Error::Corrupt)
}
fn decode(row: QueryResult) -> Result<TaskLink, Error> {
    Ok(TaskLink {
        id: field(&row, "id")?,
        tenant: field(&row, "tenant_id")?,
        actor: field(&row, "actor_id")?,
        request: field(&row, "root_request_id")?,
        receipt: field(&row, "current_receipt_id")?,
        workspace: field(&row, "workspace")?,
        title: field(&row, "title")?,
        archived: field::<i32>(&row, "archived")? != 0,
        created: field(&row, "created_at")?,
        updated: field(&row, "updated_at")?,
    })
}
fn statement(tx: &DatabaseTransaction, sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(tx.get_database_backend(), sql, values)
}
async fn load(
    tx: &DatabaseTransaction,
    authority: &Authority,
    id: &str,
) -> Result<TaskLink, Error> {
    tx.query_one(statement(
        tx,
        "SELECT * FROM assistant_execution_tasks WHERE id=$1 AND tenant_id=$2",
        vec![id.into(), (&authority.tenant_id).into()],
    ))
    .await?
    .ok_or(Error::NotFound)
    .and_then(decode)
}
async fn lookup(
    execution: &WorkflowExecution,
    authority: &Authority,
    id: &str,
) -> Result<TaskLink, Error> {
    let (tx, _) = execution
        .receipt_store()?
        .transaction(authority, false)
        .await?;
    let link = load(&tx, authority, id).await?;
    ReceiptStore::recheck_before_commit(&tx, authority).await?;
    tx.commit().await?;
    Ok(link)
}
fn value(link: &TaskLink, receipt: Receipt) -> Result<serde_json::Value, Error> {
    if receipt.id != link.receipt
        || receipt.tenant_id != link.tenant
        || receipt.actor_id != link.actor
    {
        return Err(Error::Corrupt);
    }
    let (status, step) = match receipt.phase {
        StoredPhase::Queued => ("queued", "Text request admitted; awaiting dispatch"),
        StoredPhase::Dispatching => ("running", "Configured text inference is in progress"),
        StoredPhase::Completed => ("completed", "Text response completed"),
        StoredPhase::Cancelled => (
            "cancelled",
            "No provider effect was dispatched; a new attempt may be requested",
        ),
        StoredPhase::OutcomeUnknown => (
            "outcome_unknown",
            "Execution outcome is unknown; do not retry this task",
        ),
    };
    Ok(
        serde_json::json!({"id":link.id,"workspace_id":link.workspace,"title":link.title,"prompt":receipt.task,
        "status":status,"mode":"Ask","permission_profile":"Text only","model_config_json":{"model":receipt.model,"provider":receipt.provider,"outputFormat":"Text"},
        "current_step":step,"archived":link.archived,"created_at_unix":link.created,"updated_at_unix":link.updated.max(receipt.updated_at),
        "execution":{"id":receipt.id,"request_id":receipt.request_id,"root_request_id":link.request,"tenant_id":receipt.tenant_id,"actor_id":receipt.actor_id,"phase":receipt.phase,"output":receipt.output,"error":receipt.error}}),
    )
}
async fn read(
    execution: &WorkflowExecution,
    claims: &Claims,
    headers: &HeaderMap,
    id: &str,
) -> Result<serde_json::Value, Error> {
    let authority = execution.authorize(claims, headers).await?;
    let link = lookup(execution, &authority, id).await?;
    let receipt = execution
        .get_receipt(claims, headers, &link.receipt)
        .await?;
    value(&link, receipt)
}
async fn associate_root(
    execution: &WorkflowExecution,
    authority: &Authority,
    receipt: &Receipt,
    workspace: &str,
) -> Result<(), Error> {
    let (tx, now) = execution
        .receipt_store()?
        .transaction(authority, true)
        .await?;
    match load(&tx, authority, &receipt.id).await {
        Ok(link) => {
            if link.actor != authority.actor_id || link.request != receipt.request_id {
                return Err(Error::Conflict);
            }
        }
        Err(Error::NotFound) => {
            tx.execute(statement(&tx,"INSERT INTO assistant_execution_tasks(id,tenant_id,actor_id,root_request_id,current_receipt_id,workspace,title,archived,created_at,updated_at) VALUES($1,$2,$3,$4,$1,$5,$6,0,$7,$8)",vec![(&receipt.id).into(),(&authority.tenant_id).into(),(&authority.actor_id).into(),(&receipt.request_id).into(),workspace.into(),(&receipt.name).into(),receipt.created_at.into(),now.into()])).await?;
            tx.execute(statement(&tx,"INSERT INTO assistant_execution_attempts(receipt_id,task_id,tenant_id,actor_id,source_receipt_id) VALUES($1,$1,$2,$3,NULL)",vec![(&receipt.id).into(),(&authority.tenant_id).into(),(&authority.actor_id).into()])).await?;
        }
        Err(error) => return Err(error),
    }
    ReceiptStore::recheck_before_commit(&tx, authority).await?;
    tx.commit().await?;
    Ok(())
}
async fn create(
    execution: &Arc<WorkflowExecution>,
    claims: &Claims,
    headers: &HeaderMap,
    body: TaskRequest,
) -> Result<serde_json::Value, Error> {
    let authority = execution.authorize(claims, headers).await?;
    let (task, title) = body.validate()?;
    let request = RequestMetadata {
        request_id: dispatch::request_id(headers)?,
        name: title,
        workflow: "analysis".into(),
        requested_model: String::new(),
        agent_role: Some(format!("assistant-text:{}", body.workspace)),
    };
    let reservation = execution.prepare(claims, headers, &task, request).await?;
    let id = reservation.receipt().id.clone();
    // Durable association is acknowledged before the single-use capability can dispatch.
    // On failure the queued receipt is retained for same-key recovery/expiry.
    associate_root(
        execution,
        &authority,
        reservation.receipt(),
        &body.workspace,
    )
    .await?;
    execution.dispatch(reservation, None).await?;
    read(execution, claims, headers, &id).await
}
async fn task_for_attempt(
    execution: &WorkflowExecution,
    authority: &Authority,
    receipt: &str,
) -> Result<(String, Option<String>), Error> {
    let (tx, _) = execution
        .receipt_store()?
        .transaction(authority, false)
        .await?;
    let row=tx.query_one(statement(&tx,"SELECT task_id,source_receipt_id FROM assistant_execution_attempts WHERE receipt_id=$1 AND tenant_id=$2 AND actor_id=$3",vec![receipt.into(),(&authority.tenant_id).into(),(&authority.actor_id).into()])).await?.ok_or(Error::NotFound)?;
    let result = (field(&row, "task_id")?, field(&row, "source_receipt_id")?);
    ReceiptStore::recheck_before_commit(&tx, authority).await?;
    tx.commit().await?;
    Ok(result)
}
async fn resume(
    execution: &Arc<WorkflowExecution>,
    claims: &Claims,
    headers: &HeaderMap,
    link: &TaskLink,
    source: &str,
) -> Result<bool, Error> {
    let authority = execution.authorize(claims, headers).await?;
    if authority.actor_id != link.actor {
        return Err(Error::Forbidden);
    }
    let key = dispatch::request_id(headers)?;
    let mut orphan = None;
    match execution
        .receipt_by_request_id(claims, headers, &key.to_string())
        .await
    {
        Ok(receipt) => match task_for_attempt(execution, &authority, &receipt.id).await {
            Ok((task, prior)) if task == link.id && prior.as_deref() == Some(source) => {
                return Ok(true);
            }
            Ok(_) => return Err(Error::Conflict),
            Err(Error::NotFound) => {
                orphan = Some(receipt);
            } // Recovery cannot redispatch an old capability.
            Err(error) => return Err(error),
        },
        Err(Error::NotFound) => {}
        Err(error) => return Err(error),
    }
    if source != link.receipt || link.archived {
        if let Some(orphan) = orphan {
            // Never cancel after a separate association read: another same-key
            // request may now own the target. Expiry/readback resolves a truly
            // unassociated queued receipt without racing a winning dispatch.
            if orphan.phase != StoredPhase::Cancelled
                || orphan.agent_role.as_deref()
                    != Some(format!("assistant-resume:{}:{source}", link.id).as_str())
            {
                return Err(Error::Conflict);
            }
        }
        return Ok(false);
    }
    let previous = execution.get_receipt(claims, headers, source).await?;
    if previous.phase != StoredPhase::Cancelled {
        return Ok(false);
    }
    let reservation = execution
        .prepare(
            claims,
            headers,
            &previous.task,
            RequestMetadata {
                request_id: key,
                name: previous.name,
                workflow: "analysis".into(),
                requested_model: String::new(),
                agent_role: Some(format!("assistant-resume:{}:{source}", link.id)),
            },
        )
        .await?;
    let target = reservation.receipt().id.clone();
    let result=async {
        let (tx,now)=execution.receipt_store()?.transaction(&authority,true).await?;
        let actual=load(&tx,&authority,&link.id).await?;
        if actual.receipt!=source || actual.archived {return Err(Error::Conflict);}
        tx.execute(statement(&tx,"INSERT INTO assistant_execution_attempts(receipt_id,task_id,tenant_id,actor_id,source_receipt_id) VALUES($1,$2,$3,$4,$5)",vec![(&target).into(),(&link.id).into(),(&authority.tenant_id).into(),(&authority.actor_id).into(),source.into()])).await?;
        let changed=tx.execute(statement(&tx,"UPDATE assistant_execution_tasks SET current_receipt_id=$1,updated_at=$2 WHERE id=$3 AND tenant_id=$4 AND actor_id=$5 AND current_receipt_id=$6 AND archived=0",vec![(&target).into(),now.into(),(&link.id).into(),(&authority.tenant_id).into(),(&authority.actor_id).into(),source.into()])).await?.rows_affected();
        if changed!=1 {return Err(Error::Conflict);}
        ReceiptStore::recheck_before_commit(&tx,&authority).await?;tx.commit().await?;Ok::<_,Error>(())
    }.await;
    if matches!(&result, Err(Error::Conflict))
        && let Ok((task, prior)) = task_for_attempt(execution, &authority, &target).await
        && task == link.id
        && prior.as_deref() == Some(source)
    {
        return Ok(true);
    }
    // An unmatched loser remains held until canonical queued expiry. Never
    // cancel a receipt here: association and a winning dispatch may race.
    // A commit error is ambiguous. Never cancel an attempt whose association
    // may have committed, and never retry it with a new request identity.
    result?;
    execution.dispatch(reservation, None).await?;
    Ok(true)
}
async fn mutate(
    execution: &Arc<WorkflowExecution>,
    claims: &Claims,
    headers: &HeaderMap,
    id: &str,
    body: Mutation,
) -> Result<serde_json::Value, Error> {
    let authority = execution.authorize(claims, headers).await?;
    let link = lookup(execution, &authority, id).await?;
    match body.action.as_str() {
        "stop" => {
            if body
                .source_receipt_id
                .as_ref()
                .is_some_and(|id| id != &link.receipt)
            {
                return Err(Error::Conflict);
            }
            execution
                .cancel_receipt(claims, headers, &link.receipt)
                .await?;
        }
        "resume" => {
            if !resume(
                execution,
                claims,
                headers,
                &link,
                body.source_receipt_id.as_deref().ok_or(Error::Invalid)?,
            )
            .await?
            {
                return Ok(
                    serde_json::json!({"error":"No new attempt was dispatched. Refresh this task before requesting another attempt.","effect":"none","rejected_before_dispatch":true,"rejected_request_id":dispatch::request_id(headers)?.to_string()}),
                );
            }
        }
        "archive" | "unarchive" | "rename" => {
            let (tx, now) = execution
                .receipt_store()?
                .transaction(&authority, true)
                .await?;
            let (sql, first) = if body.action == "rename" {
                let title = body
                    .title
                    .as_deref()
                    .map(str::trim)
                    .filter(|title| {
                        !title.is_empty() && title.chars().count() <= 160 && !title.contains('\0')
                    })
                    .ok_or(Error::Invalid)?;
                (
                    "UPDATE assistant_execution_tasks SET title=$1,updated_at=$2 WHERE id=$3 AND tenant_id=$4",
                    Value::from(title),
                )
            } else {
                (
                    "UPDATE assistant_execution_tasks SET archived=$1,updated_at=$2 WHERE id=$3 AND tenant_id=$4",
                    Value::from(i32::from(body.action == "archive")),
                )
            };
            if tx
                .execute(statement(
                    &tx,
                    sql,
                    vec![first, now.into(), id.into(), (&authority.tenant_id).into()],
                ))
                .await?
                .rows_affected()
                != 1
            {
                return Err(Error::NotFound);
            }
            ReceiptStore::recheck_before_commit(&tx, &authority).await?;
            tx.commit().await?;
        }
        _ => return Err(Error::Invalid),
    }
    read(execution, claims, headers, id).await
}
type Reply = (StatusCode, Json<serde_json::Value>);
fn reply(result: Result<serde_json::Value, Error>, status: StatusCode) -> Reply {
    match result {
        Ok(value) if value.get("rejected_before_dispatch").is_some() => {
            (StatusCode::CONFLICT, Json(value))
        }
        Ok(value) => (status, Json(value)),
        Err(error) => (
            error.status(),
            Json(serde_json::json!({"error":error.message()})),
        ),
    }
}
pub(crate) async fn create_task(
    Extension(execution): Extension<Arc<WorkflowExecution>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(body): Json<TaskRequest>,
) -> Reply {
    if body.validate().is_err() {
        return (
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error":"Only bounded text tasks with supported options are accepted", "effect":"none", "rejected_before_admission":true}),
            ),
        );
    }
    reply(
        create(&execution, &claims, &headers, body).await,
        StatusCode::ACCEPTED,
    )
}
pub(crate) async fn get_task(
    Extension(execution): Extension<Arc<WorkflowExecution>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Reply {
    reply(
        read(&execution, &claims, &headers, &id).await,
        StatusCode::OK,
    )
}
pub(crate) async fn mutate_task(
    Extension(execution): Extension<Arc<WorkflowExecution>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(body): Json<Mutation>,
) -> Reply {
    reply(
        mutate(&execution, &claims, &headers, &id, body).await,
        StatusCode::OK,
    )
}
pub(crate) async fn by_request(
    Extension(execution): Extension<Arc<WorkflowExecution>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Reply {
    let result=async {
        let authority=execution.authorize(&claims,&headers).await?;
        let receipt=execution.receipt_by_request_id(&claims,&headers,&id).await?;
        match task_for_attempt(&execution,&authority,&receipt.id).await {
            Ok((task,_))=>{let mut value=read(&execution,&claims,&headers,&task).await?;value["matched_request_id"]=id.into();Ok(value)},
            Err(Error::NotFound) if receipt.phase==StoredPhase::Cancelled && receipt.agent_role.as_deref().is_some_and(|role|role.starts_with("assistant-text:")||role.starts_with("assistant-resume:"))=>Ok(serde_json::json!({"cancelled_request":{"request_id":receipt.request_id,"tenant_id":receipt.tenant_id,"actor_id":receipt.actor_id,"phase":"cancelled"},"effect":"none"})),
            Err(error)=>Err(error),
        }
    }.await;
    reply(result, StatusCode::OK)
}
pub(crate) async fn list_tasks(
    Extension(execution): Extension<Arc<WorkflowExecution>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Query(query): Query<ListQuery>,
) -> Reply {
    let result=async {
        let authority=execution.authorize(&claims,&headers).await?;
        let (tx,_)=execution.receipt_store()?.transaction(&authority,false).await?;
        let before=query.before.unwrap_or_default();
        if !before.is_empty() && uuid::Uuid::parse_str(&before).is_err(){return Err(Error::Invalid);}
        let rows=tx.query_all(statement(&tx,"SELECT * FROM assistant_execution_tasks WHERE tenant_id=$1 AND ($2='' OR id<$2) ORDER BY id DESC LIMIT 4",vec![(&authority.tenant_id).into(),before.into()])).await?;
        let links=rows.into_iter().map(decode).collect::<Result<Vec<_>,_>>()?;
        ReceiptStore::recheck_before_commit(&tx,&authority).await?;tx.commit().await?;
        let mut result=Vec::new();
        for link in links {let receipt=execution.get_receipt(&claims,&headers,&link.receipt).await?;result.push(value(&link,receipt)?);}
        Ok(serde_json::Value::Array(result))
    }.await;
    reply(result, StatusCode::OK)
}
