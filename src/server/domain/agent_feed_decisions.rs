//! Atomic owner decision, edited content, legacy synchronization and durable
//! PostgreSQL queue admission. A queue receipt never proves a business outcome.
use crate::domain::repository::agent_feed_repo::AgentFeedItem;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use server_auth::commit_authority::{AuthorityError, AuthorizedPgOwner};
use sqlx::{FromRow, PgConnection, types::Json};

#[derive(Deserialize)]
pub struct DecisionInput {
    pub state: String,
    pub proposed_action: Option<Value>,
    pub context_payload: Option<Value>,
    #[serde(default)]
    pub edited_payload: Option<String>,
    #[serde(default)]
    pub modified_content: Option<String>,
}
#[derive(Debug)]
pub enum Error {
    Authority(AuthorityError),
    Database(sqlx::Error),
    NotFound,
    Invalid(&'static str),
    Conflict(&'static str),
}
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e)
    }
}
impl From<AuthorityError> for Error {
    fn from(e: AuthorityError) -> Self {
        Self::Authority(e)
    }
}
#[derive(Clone, FromRow)]
pub struct DispatchRecord {
    pub tenant_id: String,
    pub action_id: String,
    pub origin: String,
    pub decision_state: String,
    pub actor_id: String,
    pub token_id: String,
    pub job_id: Option<String>,
    pub dispatch_payload: Option<Json<Value>>,
    pub dispatch_status: String,
    pub attempted_at: Option<DateTime<Utc>>,
    pub dispatch_returned_at: Option<DateTime<Utc>>,
    pub detail: Option<String>,
}
#[derive(Serialize)]
pub struct DispatchStatus {
    pub status: String,
    pub job_id: Option<String>,
    pub attempted_at: Option<DateTime<Utc>>,
    pub dispatch_returned_at: Option<DateTime<Utc>>,
    pub detail: Option<String>,
}
impl From<DispatchRecord> for DispatchStatus {
    fn from(r: DispatchRecord) -> Self {
        Self {
            status: r.dispatch_status,
            job_id: r.job_id,
            attempted_at: r.attempted_at,
            dispatch_returned_at: r.dispatch_returned_at,
            detail: r.detail,
        }
    }
}
#[derive(Serialize)]
pub struct DecisionResponse {
    #[serde(flatten)]
    pub item: AgentFeedItem,
    pub decision_recorded: bool,
    pub dispatch: Option<DispatchStatus>,
}
pub async fn lock_action(
    conn: &mut PgConnection,
    tenant: &str,
    id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-feed-decision-v1',$1::text,$2::text)::text,0))").bind(tenant).bind(id).execute(conn).await?;
    Ok(())
}
pub async fn receipt(
    conn: &mut PgConnection,
    tenant: &str,
    id: &str,
) -> Result<Option<DispatchRecord>, sqlx::Error> {
    let mut record: Option<DispatchRecord> = sqlx::query_as(
        "SELECT * FROM agent_feed_decisions WHERE tenant_id=$1 AND action_id=$2 FOR UPDATE",
    )
    .bind(tenant)
    .bind(id)
    .fetch_optional(&mut *conn)
    .await?;
    if let Some(record) = record.as_mut()
        && record.dispatch_status == "PENDING"
        && record.attempted_at.is_none()
    {
        let queued: Option<(String, String, Option<Json<Value>>)> = sqlx::query_as("SELECT status,job_type,payload FROM ohc_job_queue WHERE tenant_id=$1 AND id=$2 FOR UPDATE")
            .bind(tenant).bind(&record.job_id).fetch_optional(&mut *conn).await?;
        let runnable = queued.is_some_and(|(state, kind, payload)| {
            matches!(state.as_str(), "PENDING" | "PROCESSING")
                && kind == "agent_feed_action"
                && payload == record.dispatch_payload
        });
        if !runnable {
            record.dispatch_status = "RECONCILIATION_REQUIRED".into();
            record.detail = Some("Queue admission ended or became unavailable without a matching durable attempt; reconcile before retrying".into());
            sqlx::query("UPDATE agent_feed_decisions SET dispatch_status=$1,detail=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND action_id=$4")
                .bind(&record.dispatch_status).bind(&record.detail).bind(tenant).bind(id).execute(&mut *conn).await?;
            sqlx::query("UPDATE ohc_job_queue SET status='RECONCILIATION_REQUIRED',updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND id=$2 AND job_type='agent_feed_action' AND status IN ('PENDING','PROCESSING')")
                .bind(tenant).bind(&record.job_id).execute(conn).await?;
        }
    }
    Ok(record)
}
fn normalized_state(state: &str) -> Result<&str, Error> {
    match state {
        "APPROVED" | "DISMISSED" | "PAUSED" | "PENDING_APPROVAL" => Ok(state),
        "REJECTED" => Ok("DISMISSED"),
        _ => Err(Error::Invalid("Unsupported decision state")),
    }
}
fn edit_action(
    original: Option<Json<Value>>,
    requested: Option<Value>,
    edited: Option<String>,
) -> Result<Option<Json<Value>>, Error> {
    let mut result = requested.map(Json).or(original);
    if let Some(text) = edited {
        let value = result.get_or_insert_with(|| Json(json!({})));
        let object = value
            .as_object_mut()
            .ok_or(Error::Invalid("Edited proposed action must be an object"))?;
        let mut found = false;
        for field in [
            "draft_reply",
            "generated_response",
            "summary",
            "message",
            "draft_message",
            "draft_action",
        ] {
            if object.contains_key(field) {
                object.insert(field.into(), Value::String(text.clone()));
                found = true;
            }
        }
        if !found {
            object.insert("message".into(), Value::String(text));
        }
    }
    Ok(result)
}
async fn load_item(
    conn: &mut PgConnection,
    tenant: &str,
    id: &str,
) -> Result<(AgentFeedItem, &'static str), Error> {
    if let Some(item) =
        sqlx::query_as("SELECT * FROM agent_feed_items WHERE tenant_id=$1 AND id=$2 FOR UPDATE")
            .bind(tenant)
            .bind(id)
            .fetch_optional(&mut *conn)
            .await?
    {
        return Ok((item, "canonical"));
    }
    // Preserve legacy approvals instead of silently dropping their controls.
    for (origin, query) in [
        (
            "approval",
            "SELECT id,tenant_id,department AS event_source,jsonb_build_object('description',description) AS context_payload,payload AS proposed_action,CASE WHEN status IN ('DRAFT','PENDING','PENDING_APPROVAL') THEN 'PENDING_APPROVAL' WHEN status='REJECTED' THEN 'DISMISSED' ELSE status END AS lifecycle_state,created_at,updated_at FROM agent_approvals WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
        ),
        (
            "request",
            "SELECT id,tenant_id,COALESCE(agent_type,department_type,'operations') AS event_source,jsonb_build_object('description',COALESCE(description,'Action Request: '||action_type)) AS context_payload,payload AS proposed_action,CASE WHEN UPPER(status) IN ('PENDING','DRAFT') THEN 'PENDING_APPROVAL' WHEN UPPER(status)='REJECTED' THEN 'DISMISSED' ELSE UPPER(status) END AS lifecycle_state,created_at,updated_at FROM agent_action_requests WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
        ),
        (
            "inbox",
            "SELECT id,tenant_id,COALESCE(source,'omni_inbox') AS event_source,jsonb_build_object('customer_message',original_content) AS context_payload,jsonb_build_object('draft_reply',COALESCE(draft_reply,''),'action_type','Draft Reply','feature_type',CASE WHEN source='Instagram DM' THEN 'instagram_dm' ELSE 'omni_inbox' END,'inbox_message_id',id) AS proposed_action,'PENDING_APPROVAL' AS lifecycle_state,created_at,updated_at FROM omni_inbox_messages WHERE tenant_id=$1 AND id=$2 AND status NOT IN ('resolved','dismissed','sent','processed') FOR UPDATE",
        ),
        (
            "order",
            "SELECT id,tenant_id,'orders' AS event_source,jsonb_build_object('description','Pending Order') AS context_payload,jsonb_build_object('message','Process Order') AS proposed_action,'PENDING_APPROVAL' AS lifecycle_state,created_at,updated_at FROM orders WHERE tenant_id=$1 AND id=$2 AND status='pending' FOR UPDATE",
        ),
        (
            "invoice",
            "SELECT id,tenant_id,'invoices' AS event_source,jsonb_build_object('description','Action Required: Overdue Invoice') AS context_payload,jsonb_build_object('message','Send Reminder') AS proposed_action,'PENDING_APPROVAL' AS lifecycle_state,created_at,updated_at FROM invoices WHERE tenant_id=$1 AND id=$2 AND LOWER(status) IN ('draft','overdue') FOR UPDATE",
        ),
    ] {
        if let Some(item) = sqlx::query_as(query)
            .bind(tenant)
            .bind(id)
            .fetch_optional(&mut *conn)
            .await?
        {
            return Ok((item, origin));
        }
    }
    Err(Error::NotFound)
}
async fn sync_legacy(
    conn: &mut PgConnection,
    item: &AgentFeedItem,
    origin: &str,
) -> Result<(), sqlx::Error> {
    let status = match item.lifecycle_state.as_str() {
        "DISMISSED" => "REJECTED",
        "PENDING_APPROVAL" => "DRAFT",
        state => state,
    };
    let rows=sqlx::query("UPDATE agent_approvals SET status=$1,payload=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4").bind(status).bind(&item.proposed_action).bind(&item.tenant_id).bind(&item.id).execute(&mut *conn).await?.rows_affected();
    if rows == 0 {
        let status = match status {
            "APPROVED" => "Approved",
            "REJECTED" => "Rejected",
            "PAUSED" => "Paused",
            _ => "Pending",
        };
        sqlx::query("UPDATE agent_action_requests SET status=$1,payload=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4").bind(status).bind(&item.proposed_action).bind(&item.tenant_id).bind(&item.id).execute(&mut *conn).await?;
    }
    if origin == "inbox" {
        // Approval is not a receipt for message delivery.
        let status = match item.lifecycle_state.as_str() {
            "APPROVED" => "approved",
            "DISMISSED" => "dismissed",
            "PAUSED" => "paused",
            _ => "unread",
        };
        let draft = item
            .proposed_action
            .as_ref()
            .and_then(|v| v.get("draft_reply"))
            .and_then(Value::as_str);
        sqlx::query("UPDATE omni_inbox_messages SET status=$1,draft_reply=COALESCE($2,draft_reply),updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4").bind(status).bind(draft).bind(&item.tenant_id).bind(&item.id).execute(&mut *conn).await?;
    } else if origin == "order" {
        let status = match item.lifecycle_state.as_str() {
            "APPROVED" => "processing",
            "DISMISSED" => "cancelled",
            _ => "pending",
        };
        sqlx::query(
            "UPDATE orders SET status=$1,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$2 AND id=$3",
        )
        .bind(status)
        .bind(&item.tenant_id)
        .bind(&item.id)
        .execute(&mut *conn)
        .await?;
    } else if origin == "invoice" && item.lifecycle_state == "DISMISSED" {
        sqlx::query("UPDATE invoices SET status='cancelled',updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND id=$2").bind(&item.tenant_id).bind(&item.id).execute(&mut *conn).await?;
    }
    Ok(())
}
fn dispatch_payload(item: &AgentFeedItem) -> Option<Json<Value>> {
    let incident = item.event_source == "incident_resolution";
    let payload = if incident {
        item.context_payload.as_ref()
    } else {
        item.proposed_action
            .as_ref()
            .or(item.context_payload.as_ref())
    }?;
    let feature = if incident {
        None
    } else {
        Some(payload.get("feature_type")?.as_str()?)
    };
    Some(Json(
        json!({"action_id":item.id,"tenant_id":item.tenant_id,"is_incident":incident,"feature_type":feature,"payload":payload.0,"event_source":item.event_source}),
    ))
}
pub async fn record(
    owner: AuthorizedPgOwner,
    token_id: &str,
    id: &str,
    input: DecisionInput,
) -> Result<DecisionResponse, Error> {
    let state = normalized_state(&input.state)?.to_string();
    if id.trim().is_empty() || id.len() > 128 {
        return Err(Error::Invalid("Invalid feed item identity"));
    }
    if input.edited_payload.is_some()
        && input.modified_content.is_some()
        && input.edited_payload != input.modified_content
    {
        return Err(Error::Invalid("Conflicting edited content"));
    }
    let tenant = owner.tenant_id().to_owned();
    let actor = owner.actor_id().to_owned();
    let mut tx = owner.begin().await?;
    let conn = tx.connection();
    lock_action(conn, &tenant, id).await?;
    let (mut item, origin) = load_item(conn, &tenant, id).await?;
    let prior = receipt(conn, &tenant, id).await?;
    let action = edit_action(
        item.proposed_action.clone(),
        input.proposed_action,
        input.edited_payload.or(input.modified_content),
    )?;
    let context = input
        .context_payload
        .map(Json)
        .or(item.context_payload.clone());
    let changed = action != item.proposed_action || context != item.context_payload;
    if changed && item.lifecycle_state == "APPROVED" {
        return Err(Error::Conflict(
            "An approved action cannot be edited by replay",
        ));
    }
    if let Some(ref previous) = prior {
        let fresh_authority = state == "APPROVED"
            && previous.dispatch_status == "CANCELLED"
            && previous.attempted_at.is_none()
            && (previous.actor_id != actor || previous.token_id != token_id);
        if state == "APPROVED"
            && item.lifecycle_state == "APPROVED"
            && previous.dispatch_status == "CANCELLED"
            && !fresh_authority
        {
            return Err(Error::Conflict(
                "The cancelled dispatch needs an explicit approval under fresh current authority",
            ));
        }
        if changed && (item.lifecycle_state == "APPROVED" || previous.attempted_at.is_some()) {
            return Err(Error::Conflict(
                "An approved or attempted action cannot be edited by replay",
            ));
        }
        if previous.attempted_at.is_some()
            && state == "APPROVED"
            && previous.decision_state != "APPROVED"
        {
            return Err(Error::Conflict(
                "An attempted action requires reconciliation before another approval",
            ));
        }
        if state == item.lifecycle_state
            && state == previous.decision_state
            && !changed
            && !fresh_authority
        {
            let response = DecisionResponse {
                item,
                decision_recorded: true,
                dispatch: Some(previous.clone().into()),
            };
            tx.commit().await?;
            return Ok(response);
        }
    }
    let uncertain_history = prior.is_none()
        && !matches!(
            item.lifecycle_state.as_str(),
            "PENDING_APPROVAL" | "PENDING" | "DRAFT"
        );
    item.lifecycle_state = state.clone();
    item.proposed_action = action;
    item.context_payload = context;
    sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,COALESCE($7,CURRENT_TIMESTAMP),CURRENT_TIMESTAMP) ON CONFLICT(id) DO UPDATE SET context_payload=EXCLUDED.context_payload,proposed_action=EXCLUDED.proposed_action,lifecycle_state=EXCLUDED.lifecycle_state,updated_at=CURRENT_TIMESTAMP WHERE agent_feed_items.tenant_id=EXCLUDED.tenant_id")
      .bind(id).bind(&tenant).bind(&item.event_source).bind(&item.context_payload).bind(&item.proposed_action).bind(&state).bind(item.created_at).execute(&mut *conn).await?;
    // PostgreSQL's ON CONFLICT UPDATE triggers participate in this transaction,
    // including the older agent_feed mirror. Re-read its committed representation.
    item = sqlx::query_as("SELECT * FROM agent_feed_items WHERE tenant_id=$1 AND id=$2")
        .bind(&tenant)
        .bind(id)
        .fetch_optional(&mut *conn)
        .await?
        .ok_or(Error::NotFound)?;
    let origin = prior
        .as_ref()
        .map(|r| r.origin.as_str())
        .unwrap_or(origin)
        .to_owned();
    sync_legacy(conn, &item, &origin).await?;
    let mut saved = prior.clone().unwrap_or(DispatchRecord {
        tenant_id: tenant.clone(),
        action_id: id.into(),
        origin,
        decision_state: state.clone(),
        actor_id: actor.clone(),
        token_id: token_id.into(),
        job_id: None,
        dispatch_payload: None,
        dispatch_status: "NOT_REQUESTED".into(),
        attempted_at: None,
        dispatch_returned_at: None,
        detail: None,
    });
    saved.decision_state = state.clone();
    if saved.attempted_at.is_none() {
        if uncertain_history || saved.dispatch_status == "RECONCILIATION_REQUIRED" {
            saved.dispatch_status = "RECONCILIATION_REQUIRED".into();
            saved.detail = Some(
                "Historical dispatch outcome is unconfirmed; reconcile before resubmitting".into(),
            );
        } else if state == "APPROVED" {
            saved.actor_id = actor;
            saved.token_id = token_id.into();
            if let Some(payload) = dispatch_payload(&item) {
                if let Some(ref job) = saved.job_id {
                    let count=sqlx::query("UPDATE ohc_job_queue SET payload=$1,status='PENDING',next_retry_at=CURRENT_TIMESTAMP,updated_at=CURRENT_TIMESTAMP WHERE id=$2 AND tenant_id=$3 AND job_type='agent_feed_action' AND status IN ('PENDING','PROCESSING','CANCELLED')").bind(&payload).bind(job).bind(&tenant).execute(&mut *conn).await?.rows_affected();
                    if count != 1 {
                        return Err(Error::Conflict(
                            "Existing queue admission requires reconciliation",
                        ));
                    }
                } else {
                    let job = uuid::Uuid::new_v4().to_string();
                    sqlx::query("INSERT INTO ohc_job_queue(id,tenant_id,job_type,payload,status,next_retry_at)VALUES($1,$2,'agent_feed_action',$3,'PENDING',CURRENT_TIMESTAMP)").bind(&job).bind(&tenant).bind(&payload).execute(&mut *conn).await?;
                    saved.job_id = Some(job);
                }
                saved.dispatch_payload = Some(payload);
                saved.dispatch_status = "PENDING".into();
                saved.detail = None;
            }
        } else if saved.job_id.is_some() {
            saved.dispatch_status = "CANCELLED".into();
            saved.detail = Some("Decision revoked before dispatch".into());
            sqlx::query("UPDATE ohc_job_queue SET status='CANCELLED',updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND tenant_id=$2 AND status IN ('PENDING','PROCESSING')").bind(&saved.job_id).bind(&tenant).execute(&mut *conn).await?;
        }
    }
    sqlx::query("INSERT INTO agent_feed_decisions(tenant_id,action_id,origin,decision_state,actor_id,token_id,job_id,dispatch_payload,dispatch_status,attempted_at,dispatch_returned_at,detail)VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12) ON CONFLICT(tenant_id,action_id) DO UPDATE SET decision_state=EXCLUDED.decision_state,actor_id=EXCLUDED.actor_id,token_id=EXCLUDED.token_id,job_id=EXCLUDED.job_id,dispatch_payload=EXCLUDED.dispatch_payload,dispatch_status=EXCLUDED.dispatch_status,detail=EXCLUDED.detail,updated_at=CURRENT_TIMESTAMP")
        .bind(&tenant).bind(id).bind(&saved.origin).bind(&saved.decision_state).bind(&saved.actor_id).bind(&saved.token_id).bind(&saved.job_id).bind(&saved.dispatch_payload).bind(&saved.dispatch_status).bind(saved.attempted_at).bind(saved.dispatch_returned_at).bind(&saved.detail).execute(conn).await?;
    tx.commit().await?;
    Ok(DecisionResponse {
        item,
        decision_recorded: true,
        dispatch: Some(saved.into()),
    })
}
pub async fn read(owner: AuthorizedPgOwner, id: &str) -> Result<DecisionResponse, Error> {
    let tenant = owner.tenant_id().to_owned();
    let mut tx = owner.begin().await?;
    lock_action(tx.connection(), &tenant, id).await?;
    let (item, _) = load_item(tx.connection(), &tenant, id).await?;
    let receipt = receipt(tx.connection(), &tenant, id).await?;
    let response = DecisionResponse {
        item,
        decision_recorded: receipt.is_some(),
        dispatch: receipt.map(Into::into),
    };
    tx.commit().await?;
    Ok(response)
}
