//! Legacy owner decisions and local effects share one canonical transaction.
//! The durable receipt is also the replay fence; this route never calls providers.
use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde_json::{Value, json};
use server_auth::commit_authority::{
    AuthorityError, CanonicalPgAuthority, CanonicalSqliteAuthority, OwnerPgTransaction,
    OwnerSqliteTransaction, canonical_pg_data_pool,
};
use server_common::Claims;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct TriageActionPayload {
    pub triage_item_id: String,
    pub approved: bool,
    #[serde(default)]
    pub edited_payload: Option<String>,
}

pub fn router() -> Router<Arc<crate::db::DB>> {
    Router::new()
        .route("/api/v1/ui/triage/decisions/{id}", get(read_decision))
        .route("/api/v1/ui/triage/action", post(action))
        .route("/api/v1/triage/action", post(action))
}

#[derive(Debug)]
enum Error {
    Authority(AuthorityError),
    Database(sqlx::Error),
    Invalid(&'static str),
    Conflict(&'static str),
    Unsupported(&'static str),
    NotFound,
}
impl From<AuthorityError> for Error {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}
impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
fn failure(error: Error) -> Response {
    let (status, message) = match error {
        Error::Authority(AuthorityError::Forbidden) => (
            StatusCode::FORBIDDEN,
            "Current canonical owner authority is required",
        ),
        Error::Invalid(message) => (StatusCode::BAD_REQUEST, message),
        Error::Conflict(message) => (StatusCode::CONFLICT, message),
        Error::Unsupported(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        Error::NotFound => (StatusCode::NOT_FOUND, "Triage item not found"),
        error => {
            // Do not echo SQL, credentials, or customer content in API errors.
            match error {
                Error::Database(ref database) => {
                    tracing::warn!(error=%database, "Legacy triage persistence unconfirmed")
                }
                _ => tracing::warn!(?error, "Legacy triage persistence unconfirmed"),
            }
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Decision outcome is unconfirmed. Read the recorded decision before retrying; keep the original request held and do not resubmit changed content.",
            )
        }
    };
    // Losing the COMMIT acknowledgment does not prove that no decision exists.
    // Only a durable receipt (including an identical retry) can confirm it.
    let decision_recorded = if status == StatusCode::SERVICE_UNAVAILABLE {
        Value::Null
    } else {
        json!(false)
    };
    (
        status,
        [("cache-control", "no-store")],
        Json(json!({"success":false,"decision_recorded":decision_recorded,"error":message})),
    )
        .into_response()
}

// Both engines use the same decision/effect logic. Only typed JSON and timestamp
// SQL differs; SQLite's BEGIN IMMEDIATE and PostgreSQL's advisory lock serialize
// the receipt check through commit, including concurrent requests after restart.
enum OwnerTransaction {
    Pg(OwnerPgTransaction),
    Sqlite(OwnerSqliteTransaction),
}
impl OwnerTransaction {
    fn sql<'a>(&self, postgres: &'a str, sqlite: &'a str) -> &'a str {
        match self {
            Self::Pg(_) => postgres,
            Self::Sqlite(_) => sqlite,
        }
    }
    async fn commit(self) -> Result<(), Error> {
        match self {
            Self::Pg(tx) => tx.commit().await?,
            Self::Sqlite(tx) => tx.commit().await?,
        }
        Ok(())
    }
}
macro_rules! fetch {
    ($tx:expr, $row:ty, $sql:expr $(, $value:expr)* $(,)?) => {{
        let query = $sql;
        match &mut *$tx {
            OwnerTransaction::Pg(tx) => sqlx::query_as::<_, $row>(query)$(.bind($value))*.fetch_optional(tx.connection()).await?,
            OwnerTransaction::Sqlite(tx) => sqlx::query_as::<_, $row>(query)$(.bind($value))*.fetch_optional(tx.connection()).await?,
        }
    }};
}
macro_rules! execute {
    ($tx:expr, $sql:expr $(, $value:expr)* $(,)?) => {{
        let query = $sql;
        match &mut *$tx {
            OwnerTransaction::Pg(tx) => sqlx::query(query)$(.bind($value))*.execute(tx.connection()).await?.rows_affected(),
            OwnerTransaction::Sqlite(tx) => sqlx::query(query)$(.bind($value))*.execute(tx.connection()).await?.rows_affected(),
        }
    }};
}
fn exactly_one(count: u64) -> Result<(), Error> {
    if count != 1 {
        return Err(Error::Conflict(
            "The intended row was not changed; refresh before retrying",
        ));
    }
    Ok(())
}

pub async fn action(
    State(db): State<Arc<crate::db::DB>>,
    store: Option<Extension<Arc<server_auth::Store>>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Json(payload): Json<TriageActionPayload>,
) -> Response {
    match record(&db, store.map(|s| s.0), &claims, &headers, payload).await {
        Ok(receipt) => {
            if let Some(tenant) = receipt["item"]["tenant_id"].as_str() {
                crate::invalidate_legacy_triage_caches(tenant).await;
            }
            (
                StatusCode::OK,
                [("cache-control", "no-store")],
                Json(receipt),
            )
                .into_response()
        }
        Err(error) => failure(error),
    }
}
async fn begin_owner_transaction(
    db: &crate::db::DB,
    store: Option<Arc<server_auth::Store>>,
    claims: &Claims,
    headers: &HeaderMap,
) -> Result<(String, String, OwnerTransaction), Error> {
    let store = store.ok_or(AuthorityError::Unavailable)?;
    let expected_user = headers
        .get_all("x-ohc-expected-user")
        .iter()
        .collect::<Vec<_>>();
    let expected_tenant = headers
        .get_all("x-ohc-expected-tenant")
        .iter()
        .collect::<Vec<_>>();
    if (!expected_user.is_empty() || !expected_tenant.is_empty())
        && (expected_user.len() != 1
            || expected_tenant.len() != 1
            || expected_user[0].to_str().ok() != Some(claims.sub.as_str())
            || expected_tenant[0].to_str().ok() != claims.organization_id.as_deref())
    {
        return Err(AuthorityError::Forbidden.into());
    }
    let transaction = match &db.store {
        crate::db::DbStore::Postgres => {
            let repository = store.portable_repo().ok_or(AuthorityError::Unavailable)?;
            let pool = canonical_pg_data_pool(
                repository.connection(),
                &db.pool,
                &[
                    "legacy_triage_decisions",
                    "daily_work_items",
                    "triage_items",
                    "triage_proposed_actions",
                    "unified_triage_actions",
                    "unified_threads",
                    "agent_feed_items",
                    "agent_feed_decisions",
                    "task_envelopes",
                    "omni_inbox_messages",
                    "orders",
                    "customers",
                    "quotes",
                    "quote_line_items",
                    "bookings",
                    "services",
                    "products",
                    "shifts",
                    "ohc_staff_member",
                ],
            )
            .await?;
            let owner = CanonicalPgAuthority::bind(store, &pool)?
                .authorize(claims, headers)
                .await?;
            (
                owner.tenant_id().to_owned(),
                owner.actor_id().to_owned(),
                OwnerTransaction::Pg(owner.begin().await?),
            )
        }
        crate::db::DbStore::Sqlite(pool) => {
            let owner = CanonicalSqliteAuthority::bind(store, pool)?
                .authorize(claims, headers)
                .await?;
            (
                owner.tenant_id().to_owned(),
                owner.actor_id().to_owned(),
                OwnerTransaction::Sqlite(owner.begin().await?),
            )
        }
    };
    Ok(transaction)
}

/// Read the already-committed receipt without replaying a decision or local effect.
/// A missing receipt is information only, never permission to retry a mutation.
pub async fn read_decision(
    State(db): State<Arc<crate::db::DB>>,
    store: Option<Extension<Arc<server_auth::Store>>>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    async fn read(
        db: &crate::db::DB,
        store: Option<Arc<server_auth::Store>>,
        claims: &Claims,
        headers: &HeaderMap,
        id: &str,
    ) -> Result<Value, Error> {
        if id.trim().is_empty() || id.len() > 255 {
            return Err(Error::Invalid("Invalid triage identity"));
        }
        let (tenant, _, mut tx) = begin_owner_transaction(db, store, claims, headers).await?;
        if let OwnerTransaction::Pg(pg) = &mut tx {
            sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-legacy-triage-v1',$1::text,$2::text)::text,0))")
                .bind(&tenant).bind(id).execute(pg.connection()).await?;
        }
        let result = match stored_receipt(&mut tx, &tenant, id).await? {
            Some(receipt) => receipt,
            None => {
                // Do not disclose foreign/nonexistent identities or fabricate a terminal state.
                load_item(&mut tx, &tenant, id).await?;
                json!({"success":true,"decision_recorded":false,"item":{"id":id,"tenant_id":tenant},
                    "message":"No committed receipt was found. Keep the original request held; no mutation was retried."})
            }
        };
        tx.commit().await?;
        Ok(result)
    }
    match read(&db, store.map(|value| value.0), &claims, &headers, &id).await {
        Ok(receipt) => (
            StatusCode::OK,
            [("cache-control", "no-store")],
            Json(receipt),
        )
            .into_response(),
        Err(error) => failure(error),
    }
}

async fn record(
    db: &crate::db::DB,
    store: Option<Arc<server_auth::Store>>,
    claims: &Claims,
    headers: &HeaderMap,
    payload: TriageActionPayload,
) -> Result<Value, Error> {
    if payload.triage_item_id.trim().is_empty()
        || payload.triage_item_id.len() > 255
        || payload
            .edited_payload
            .as_ref()
            .is_some_and(|v| v.len() > 256 * 1024)
    {
        return Err(Error::Invalid("Invalid triage identity or edited content"));
    }
    let (tenant, actor, mut tx) = begin_owner_transaction(db, store, claims, headers).await?;
    if let OwnerTransaction::Pg(pg) = &mut tx {
        sqlx::query("SELECT pg_catalog.pg_advisory_xact_lock(pg_catalog.hashtextextended(pg_catalog.jsonb_build_array('ohc-legacy-triage-v1',$1::text,$2::text)::text,0))")
            .bind(&tenant).bind(&payload.triage_item_id).execute(pg.connection()).await?;
    }
    let result = record_in_transaction(&mut tx, &tenant, &actor, &payload).await?;
    tx.commit().await?;
    Ok(result)
}

#[derive(Clone, Copy, PartialEq)]
enum Origin {
    Daily,
    Triage,
    Unified,
    Feed,
    Inbox,
    Task,
    Order,
}
struct Item {
    origin: Origin,
    status: String,
    action_type: Option<String>,
    action: Option<Value>,
    customer: Option<String>,
    action_id: Option<String>,
    thread_id: Option<String>,
}
fn parse_json(text: Option<String>) -> Result<Option<Value>, Error> {
    text.map(|s| {
        serde_json::from_str(&s).map_err(|_| Error::Invalid("Stored action JSON is invalid"))
    })
    .transpose()
}
fn text_action(text: Option<String>) -> Option<Value> {
    text.map(|s| serde_json::from_str(&s).unwrap_or(Value::String(s)))
}
async fn load_item(tx: &mut OwnerTransaction, tenant: &str, id: &str) -> Result<Item, Error> {
    // Lock source rows too, so other routes cannot replace drafts mid-decision.
    let query = tx.sql("SELECT status,CAST(suggested_actions AS TEXT) FROM daily_work_items WHERE tenant_id=$1 AND id=$2 FOR UPDATE", "SELECT status,CAST(suggested_actions AS TEXT) FROM daily_work_items WHERE tenant_id=$1 AND id=$2");
    if let Some((status, action)) = fetch!(tx, (String, Option<String>), query, tenant, id) {
        return Ok(Item {
            origin: Origin::Daily,
            status,
            action: parse_json(action)?,
            action_type: None,
            customer: None,
            action_id: None,
            thread_id: None,
        });
    }
    let query = tx.sql("SELECT status,CAST(customer_id AS TEXT) FROM triage_items WHERE tenant_id=$1 AND id=$2 FOR UPDATE", "SELECT status,CAST(customer_id AS TEXT) FROM triage_items WHERE tenant_id=$1 AND id=$2");
    if let Some((status, customer)) = fetch!(tx, (String, Option<String>), query, tenant, id) {
        let count = fetch!(
            tx,
            (i64,),
            "SELECT COUNT(*) FROM triage_proposed_actions WHERE tenant_id=$1 AND triage_item_id=$2",
            tenant,
            id
        )
        .ok_or(Error::NotFound)?
        .0;
        if count > 1 {
            return Err(Error::Conflict(
                "This item has multiple actions; choose an explicit action before approving",
            ));
        }
        let query = tx.sql("SELECT id,action_type,CAST(payload AS TEXT) FROM triage_proposed_actions WHERE tenant_id=$1 AND triage_item_id=$2 FOR UPDATE", "SELECT id,action_type,CAST(payload AS TEXT) FROM triage_proposed_actions WHERE tenant_id=$1 AND triage_item_id=$2");
        let action = fetch!(
            tx,
            (String, Option<String>, Option<String>),
            query,
            tenant,
            id
        );
        let (action_id, action_type, action) = match action {
            Some((id, kind, content)) => (Some(id), kind, text_action(content)),
            None => (None, None, None),
        };
        return Ok(Item {
            origin: Origin::Triage,
            status,
            action,
            action_type,
            customer,
            action_id,
            thread_id: None,
        });
    }
    let query = tx.sql("SELECT status,action_type,action_payload,thread_id FROM unified_triage_actions WHERE tenant_id=$1 AND id=$2 FOR UPDATE", "SELECT status,action_type,action_payload,thread_id FROM unified_triage_actions WHERE tenant_id=$1 AND id=$2");
    if let Some((status, action_type, action, thread_id)) = fetch!(
        tx,
        (String, String, Option<String>, String),
        query,
        tenant,
        id
    ) {
        let customer = fetch!(
            tx,
            (Option<String>,),
            "SELECT CAST(customer_id AS TEXT) FROM unified_threads WHERE tenant_id=$1 AND id=$2",
            tenant,
            &thread_id
        )
        .ok_or(Error::NotFound)?
        .0;
        return Ok(Item {
            origin: Origin::Unified,
            status,
            action: text_action(action),
            action_type: Some(action_type),
            customer,
            action_id: None,
            thread_id: Some(thread_id),
        });
    }
    let query = tx.sql("SELECT lifecycle_state,CAST(proposed_action AS TEXT) FROM agent_feed_items WHERE tenant_id=$1 AND id=$2 FOR UPDATE", "SELECT lifecycle_state,CAST(proposed_action AS TEXT) FROM agent_feed_items WHERE tenant_id=$1 AND id=$2");
    if let Some((status, action)) = fetch!(tx, (String, Option<String>), query, tenant, id) {
        if let OwnerTransaction::Pg(pg) = tx {
            let prior: Option<(String,)> = sqlx::query_as(
                "SELECT action_id FROM agent_feed_decisions WHERE tenant_id=$1 AND action_id=$2",
            )
            .bind(tenant)
            .bind(id)
            .fetch_optional(pg.connection())
            .await?;
            if prior.is_some() {
                return Err(Error::Conflict(
                    "Use the agent feed decision route to read or change this action",
                ));
            }
        }
        let action = parse_json(action)?;
        if action
            .as_ref()
            .is_some_and(|a| a.get("feature_type").is_some())
        {
            return Err(Error::Unsupported(
                "Use the agent feed decision route for this dispatched action",
            ));
        }
        let action_type = action
            .as_ref()
            .and_then(|a| a.get("action_type"))
            .and_then(Value::as_str)
            .map(str::to_owned);
        return Ok(Item {
            origin: Origin::Feed,
            status,
            action,
            action_type,
            customer: None,
            action_id: None,
            thread_id: None,
        });
    }
    let query = tx.sql("SELECT status,draft_reply FROM omni_inbox_messages WHERE tenant_id=$1 AND id=$2 FOR UPDATE", "SELECT status,draft_reply FROM omni_inbox_messages WHERE tenant_id=$1 AND id=$2");
    if let Some((status, action)) = fetch!(tx, (String, Option<String>), query, tenant, id) {
        return Ok(Item {
            origin: Origin::Inbox,
            status,
            action: action.map(Value::String),
            action_type: Some("Draft Reply".into()),
            customer: None,
            action_id: None,
            thread_id: None,
        });
    }
    // task_envelopes is PostgreSQL-only in the standalone schema. An absent
    // optional source is not a failed write or a fabricated successful decision.
    let has_tasks =
        match tx {
            OwnerTransaction::Pg(_) => true,
            OwnerTransaction::Sqlite(sqlite) => sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='task_envelopes'",
            )
            .fetch_one(sqlite.connection())
            .await?
                == 1,
        };
    if has_tasks {
        let query = tx.sql("SELECT status,CAST(payload AS TEXT) FROM task_envelopes WHERE tenant_id=$1 AND id=$2 FOR UPDATE", "SELECT status,CAST(payload AS TEXT) FROM task_envelopes WHERE tenant_id=$1 AND id=$2");
        if let Some((status, action)) = fetch!(tx, (String, Option<String>), query, tenant, id) {
            return Ok(Item {
                origin: Origin::Task,
                status,
                action: parse_json(action)?,
                action_type: None,
                customer: None,
                action_id: None,
                thread_id: None,
            });
        }
    }
    let query = tx.sql(
        "SELECT status FROM orders WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
        "SELECT status FROM orders WHERE tenant_id=$1 AND id=$2",
    );
    if let Some((status,)) = fetch!(tx, (String,), query, tenant, id) {
        return Ok(Item {
            origin: Origin::Order,
            status,
            action: None,
            action_type: None,
            customer: None,
            action_id: None,
            thread_id: None,
        });
    }
    Err(Error::NotFound)
}

async fn stored_receipt(
    tx: &mut OwnerTransaction,
    tenant: &str,
    id: &str,
) -> Result<Option<Value>, Error> {
    if let Some((approved, edited, receipt)) = fetch!(
        tx,
        (bool, Option<String>, String),
        "SELECT approved,edited_payload,receipt FROM legacy_triage_decisions WHERE tenant_id=$1 AND action_id=$2",
        tenant,
        id
    ) {
        let receipt: Value = serde_json::from_str(&receipt)
            .map_err(|_| Error::Conflict("Recorded receipt requires reconciliation"))?;
        if receipt["success"] != true
            || receipt["decision_recorded"] != true
            || receipt["item"]["id"] != id
            || receipt["item"]["tenant_id"] != tenant
            || receipt["item"]["lifecycle_state"] != if approved { "APPROVED" } else { "DISMISSED" }
            || receipt["item"]["edited_payload"] != json!(edited)
        {
            return Err(Error::Conflict(
                "Recorded receipt does not match its decision identity",
            ));
        }
        return Ok(Some(receipt));
    }
    Ok(None)
}

async fn record_in_transaction(
    tx: &mut OwnerTransaction,
    tenant: &str,
    actor: &str,
    payload: &TriageActionPayload,
) -> Result<Value, Error> {
    let id = &payload.triage_item_id;
    if let Some(receipt) = stored_receipt(tx, tenant, id).await? {
        let state = if payload.approved {
            "APPROVED"
        } else {
            "DISMISSED"
        };
        if receipt["item"]["lifecycle_state"] != state
            || payload
                .edited_payload
                .as_ref()
                .is_some_and(|text| receipt["item"]["edited_payload"] != json!(text))
        {
            return Err(Error::Conflict(
                "A recorded decision cannot be replaced by a different decision or edit",
            ));
        }
        return Ok(receipt);
    }
    let mut item = load_item(tx, tenant, id).await?;
    if !matches!(
        item.status.to_ascii_uppercase().as_str(),
        "PENDING" | "PENDING_APPROVAL" | "DRAFT" | "UNREAD" | "OPEN"
    ) {
        return Err(Error::Conflict(
            "Historical decision has no durable receipt; reconcile before retrying",
        ));
    }
    if let Some(edited) = payload.edited_payload.as_deref() {
        if item.origin == Origin::Daily {
            item.action =
                edit_daily_work_actions(item.action, Some(edited)).map_err(Error::Invalid)?;
        } else if item.origin == Origin::Feed || item.origin == Origin::Task {
            let action = item.action.get_or_insert_with(|| json!({}));
            let object = action
                .as_object_mut()
                .ok_or(Error::Invalid("Stored action has no editable object"))?;
            let mut found = false;
            for key in [
                "draft_reply",
                "generated_response",
                "summary",
                "message",
                "draft_message",
                "draft_action",
            ] {
                if object.contains_key(key) {
                    object.insert(key.into(), json!(edited));
                    found = true;
                }
            }
            if !found {
                object.insert("message".into(), json!(edited));
            }
        } else if item.origin == Origin::Order
            || (item.origin == Origin::Triage && item.action_id.is_none())
        {
            return Err(Error::Invalid("This item has no editable draft"));
        } else if item.origin == Origin::Inbox {
            item.action = Some(Value::String(edited.to_owned()));
        } else {
            item.action = text_action(Some(edited.to_owned()));
        }
    }
    let (execution, detail, effect_id) = if payload.approved && item.origin != Origin::Daily {
        local_effect(tx, tenant, &item).await?
    } else {
        (
            "NOT_REQUESTED",
            "Decision and draft saved; no execution was requested",
            None,
        )
    };
    save_item(tx, tenant, id, &item, payload.approved).await?;
    let stored_item = load_item(tx, tenant, id).await?;
    let expected_status = match item.origin {
        Origin::Daily | Origin::Feed | Origin::Task => {
            if payload.approved {
                "APPROVED"
            } else {
                "DISMISSED"
            }
        }
        Origin::Triage | Origin::Unified | Origin::Inbox => {
            if payload.approved {
                "resolved"
            } else {
                "dismissed"
            }
        }
        Origin::Order => {
            if payload.approved {
                "processing"
            } else {
                "cancelled"
            }
        }
    };
    if stored_item.origin != item.origin
        || stored_item.status != expected_status
        || stored_item.action != item.action
    {
        return Err(Error::Conflict(
            "Stored decision or draft does not match the requested change",
        ));
    }
    let receipt = json!({"status":"success","success":true,"decision_recorded":true,
        "item":{"id":id,"tenant_id":tenant,"lifecycle_state":if payload.approved {"APPROVED"} else {"DISMISSED"},"edited_payload":payload.edited_payload,"proposed_action":item.action},
        "dispatch":{"status":execution,"detail":detail,"receipt_id":effect_id}});
    exactly_one(execute!(
        tx,
        "INSERT INTO legacy_triage_decisions(tenant_id,action_id,approved,edited_payload,receipt,actor_id) VALUES($1,$2,$3,$4,$5,$6)",
        tenant,
        id,
        payload.approved,
        &payload.edited_payload,
        receipt.to_string(),
        actor
    ))?;
    // Read back the stored representation before commit (including triggers).
    let (stored,) = fetch!(
        tx,
        (String,),
        "SELECT receipt FROM legacy_triage_decisions WHERE tenant_id=$1 AND action_id=$2",
        tenant,
        id
    )
    .ok_or(Error::NotFound)?;
    let stored: Value = serde_json::from_str(&stored)
        .map_err(|_| Error::Conflict("Recorded receipt requires reconciliation"))?;
    if stored != receipt {
        return Err(Error::Conflict(
            "Recorded receipt does not match this decision",
        ));
    }
    Ok(stored)
}

fn action_text(action: &Option<Value>) -> Option<String> {
    action.as_ref().map(|value| {
        value
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| value.to_string())
    })
}
async fn save_item(
    tx: &mut OwnerTransaction,
    tenant: &str,
    id: &str,
    item: &Item,
    approved: bool,
) -> Result<(), Error> {
    let state = if approved { "APPROVED" } else { "DISMISSED" };
    let legacy = if approved { "resolved" } else { "dismissed" };
    let json_action = item.action.as_ref().map(Value::to_string);
    let text = action_text(&item.action);
    let count = match item.origin {
        Origin::Daily => {
            let query = tx.sql("UPDATE daily_work_items SET status=$1,suggested_actions=CAST($2 AS JSONB),updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4", "UPDATE daily_work_items SET status=$1,suggested_actions=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4");
            execute!(tx, query, state, &json_action, tenant, id)
        }
        Origin::Triage => {
            if let Some(action_id) = &item.action_id {
                exactly_one(execute!(
                    tx,
                    "UPDATE triage_proposed_actions SET payload=$1 WHERE tenant_id=$2 AND id=$3 AND triage_item_id=$4",
                    &text,
                    tenant,
                    action_id,
                    id
                ))?;
            }
            execute!(
                tx,
                "UPDATE triage_items SET status=$1 WHERE tenant_id=$2 AND id=$3",
                legacy,
                tenant,
                id
            )
        }
        Origin::Unified => {
            exactly_one(execute!(
                tx,
                "UPDATE unified_threads SET status='resolved' WHERE tenant_id=$1 AND id=$2",
                tenant,
                &item.thread_id
            ))?;
            execute!(
                tx,
                "UPDATE unified_triage_actions SET status=$1,action_payload=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4",
                legacy,
                &text,
                tenant,
                id
            )
        }
        Origin::Feed => {
            let query = tx.sql("UPDATE agent_feed_items SET lifecycle_state=$1,proposed_action=CAST($2 AS JSONB),updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4", "UPDATE agent_feed_items SET lifecycle_state=$1,proposed_action=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4");
            execute!(tx, query, state, &json_action, tenant, id)
        }
        Origin::Inbox => execute!(
            tx,
            "UPDATE omni_inbox_messages SET status=$1,draft_reply=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4",
            if approved { "resolved" } else { "dismissed" },
            &text,
            tenant,
            id
        ),
        Origin::Task => {
            let query = tx.sql("UPDATE task_envelopes SET status=$1,payload=CAST($2 AS JSONB),updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4", "UPDATE task_envelopes SET status=$1,payload=$2,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$3 AND id=$4");
            execute!(tx, query, state, &json_action, tenant, id)
        }
        Origin::Order => execute!(
            tx,
            "UPDATE orders SET status=$1,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$2 AND id=$3",
            if approved { "processing" } else { "cancelled" },
            tenant,
            id
        ),
    };
    exactly_one(count)
}

async fn local_effect(
    tx: &mut OwnerTransaction,
    tenant: &str,
    item: &Item,
) -> Result<(&'static str, &'static str, Option<String>), Error> {
    let Some(kind) = item.action_type.as_deref() else {
        return Ok((
            "NOT_REQUESTED",
            "Decision and draft saved; no execution was requested",
            None,
        ));
    };
    if kind == "SocialPostDraft" {
        return Ok((
            "NOT_REQUESTED",
            "Social draft approved; publishing was not requested",
            None,
        ));
    }
    if kind == "Draft Reply" {
        if action_text(&item.action).is_some_and(|text| text.contains("{{payment_link")) {
            return Err(Error::Unsupported(
                "Create and verify the payment link before approving this reply",
            ));
        }
        return Ok((
            "NOT_REQUESTED",
            "Reply draft approved and saved; no message delivery was requested",
            None,
        ));
    }
    // Legacy feed cards may wrap a local quote/booking payload in draft_reply.
    // Keep that alias while preserving the outer action metadata in the receipt.
    let wrapped_action = if item.origin == Origin::Feed {
        item.action
            .as_ref()
            .and_then(|action| action.get("draft_reply"))
            .and_then(Value::as_str)
            .map(|text| {
                serde_json::from_str::<Value>(text)
                    .map_err(|_| Error::Invalid("This local action requires a JSON object"))
            })
            .transpose()?
    } else {
        None
    };
    let action = wrapped_action
        .as_ref()
        .or(item.action.as_ref())
        .filter(|v| v.is_object())
        .ok_or(Error::Invalid("This local action requires a JSON object"))?;
    if matches!(
        kind,
        "Approve Draft" | "Draft Quote" | "ProposedInvoice" | "Accept Booking" | "Reorder"
    ) {
        let customer = item
            .customer
            .as_deref()
            .or_else(|| action.get("client_id").and_then(Value::as_str))
            .ok_or(Error::Invalid("The quote requires a customer"))?;
        require_reference(tx, "customers", tenant, customer).await?;
        let amount = action
            .get("total_amount_cents")
            .and_then(Value::as_i64)
            .ok_or(Error::Invalid("The quote requires an amount in cents"))?;
        let deposit = action
            .get("required_deposit_cents")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        if amount < 0 || deposit < 0 || deposit > amount {
            return Err(Error::Invalid("Invalid quote amount or deposit"));
        }
        if deposit > 0 {
            return Err(Error::Unsupported(
                "A payment link must be created through the payment workflow before this quote can be approved",
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        // This is a saved quote, never a claim that it was sent or paid.
        exactly_one(execute!(
            tx,
            "INSERT INTO quotes(id,tenant_id,customer_id,status,total_amount_cents,updated_at) VALUES($1,$2,$3,'DRAFT',$4,CURRENT_TIMESTAMP)",
            &id,
            tenant,
            customer,
            amount
        ))?;
        if let Some(items) = action.get("line_items") {
            for line in items
                .as_array()
                .ok_or(Error::Invalid("Quote lines must be an array"))?
            {
                let description = line
                    .get("description")
                    .and_then(Value::as_str)
                    .ok_or(Error::Invalid("A quote line requires a description"))?;
                let quantity = line.get("quantity").and_then(Value::as_i64).unwrap_or(1);
                let price = line
                    .get("unit_price_cents")
                    .and_then(Value::as_i64)
                    .ok_or(Error::Invalid("A quote line requires a price"))?;
                if !(1..=i32::MAX as i64).contains(&quantity) || price < 0 {
                    return Err(Error::Invalid("Invalid quote line quantity or price"));
                }
                let optional = line
                    .get("is_optional")
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let line_id = uuid::Uuid::new_v4().to_string();
                let query = tx.sql("INSERT INTO quote_line_items(id,quote_id,description,unit_price_cents,quantity,is_optional,tenant_id) VALUES($1,$2,$3,$4,$5,$6,$7)", "INSERT INTO quote_line_items(id,quote_id,description,unit_price_cents,quantity,is_optional) VALUES($1,$2,$3,$4,$5,$6)");
                match tx {
                    OwnerTransaction::Pg(pg) => {
                        exactly_one(
                            sqlx::query(query)
                                .bind(&line_id)
                                .bind(&id)
                                .bind(description)
                                .bind(price)
                                .bind(quantity as i32)
                                .bind(optional)
                                .bind(tenant)
                                .execute(pg.connection())
                                .await?
                                .rows_affected(),
                        )?;
                    }
                    OwnerTransaction::Sqlite(sqlite) => {
                        exactly_one(
                            sqlx::query(query)
                                .bind(&line_id)
                                .bind(&id)
                                .bind(description)
                                .bind(price)
                                .bind(quantity as i32)
                                .bind(optional)
                                .execute(sqlite.connection())
                                .await?
                                .rows_affected(),
                        )?;
                    }
                }
            }
        }
        return Ok((
            "LOCAL_COMMITTED",
            "Quote saved locally; delivery and payment were not requested",
            Some(id),
        ));
    }
    if kind == "Reassign Shift" {
        let shift = action
            .get("shift_id")
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .ok_or(Error::Invalid("A shift is required"))?;
        let staff = action
            .get("new_staff_id")
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .ok_or(Error::Invalid("A staff member is required"))?;
        let staff_table = tx.sql("ohc_staff_member", "staff_profiles");
        require_reference(tx, staff_table, tenant, staff).await?;
        exactly_one(execute!(
            tx,
            "UPDATE shifts SET staff_id=$1 WHERE tenant_id=$2 AND id=$3",
            staff,
            tenant,
            shift
        ))?;
        return Ok((
            "LOCAL_COMMITTED",
            "Shift reassigned locally; no SMS notification was sent",
            Some(shift.to_owned()),
        ));
    }
    if matches!(kind, "Draft Booking" | "SuggestedCalendarSlot") {
        let customer = item
            .customer
            .as_deref()
            .or_else(|| action.get("customer_id").and_then(Value::as_str))
            .ok_or(Error::Invalid("The booking requires a customer"))?;
        require_reference(tx, "customers", tenant, customer).await?;
        let (table, service) =
            if let Some(service) = action.get("service_id").and_then(Value::as_str) {
                ("services", service)
            } else {
                (
                    "products",
                    action
                        .get("product_id")
                        .and_then(Value::as_str)
                        .ok_or(Error::Invalid("The booking requires a service or product"))?,
                )
            };
        require_reference(tx, table, tenant, service).await?;
        let start = action
            .get("start_time")
            .and_then(Value::as_str)
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .ok_or(Error::Invalid("A valid booking start time is required"))?
            .with_timezone(&chrono::Utc);
        let end = action
            .get("end_time")
            .and_then(Value::as_str)
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .ok_or(Error::Invalid("A valid booking end time is required"))?
            .with_timezone(&chrono::Utc);
        if end <= start {
            return Err(Error::Invalid("Booking end must follow its start"));
        }
        let id = uuid::Uuid::new_v4().to_string();
        exactly_one(execute!(
            tx,
            "INSERT INTO bookings(id,tenant_id,customer_id,service_id,start_time,end_time,status) VALUES($1,$2,$3,$4,$5,$6,'scheduled')",
            &id,
            tenant,
            customer,
            service,
            start,
            end
        ))?;
        return Ok((
            "LOCAL_COMMITTED",
            "Booking saved locally; no external calendar confirmation was requested",
            Some(id),
        ));
    }
    Err(Error::Unsupported(
        "This action has no supported legacy executor; no decision or effect was saved",
    ))
}
async fn require_reference(
    tx: &mut OwnerTransaction,
    table: &str,
    tenant: &str,
    id: &str,
) -> Result<(), Error> {
    // All table names are internal constants. A same-tenant reference is required
    // even on legacy tables without composite tenant foreign keys.
    let query = format!(
        "SELECT CAST(id AS TEXT) FROM {table} WHERE tenant_id=$1 AND CAST(id AS TEXT)=$2{}",
        tx.sql(" FOR SHARE", "")
    );
    if fetch!(tx, (String,), &query, tenant, id).is_none() {
        return Err(Error::Invalid(
            "The referenced record does not belong to this tenant",
        ));
    }
    Ok(())
}

pub(crate) fn edit_daily_work_actions(
    actions: Option<serde_json::Value>,
    edited_payload: Option<&str>,
) -> Result<Option<serde_json::Value>, &'static str> {
    let Some(edited) = edited_payload else {
        return Ok(actions);
    };
    let mut actions = actions.unwrap_or(serde_json::Value::Null);
    if actions.is_null() {
        actions = serde_json::json!({});
    }
    let (action, array_action) = match &mut actions {
        serde_json::Value::Object(action) => (action, false),
        serde_json::Value::Array(actions) => {
            // DailyWorkCard displays the first action. Keep its metadata and
            // every later action intact when applying the owner's text edit.
            let Some(serde_json::Value::Object(action)) = actions.first_mut() else {
                return Err("daily-work draft has no editable action");
            };
            (action, true)
        }
        _ => return Err("daily-work draft has an unsupported shape"),
    };
    let has_draft_reply = action.contains_key("draft_reply");
    let has_message = action.contains_key("message");
    if has_draft_reply || (!array_action && !has_message) {
        action.insert("draft_reply".to_string(), serde_json::json!(edited));
    }
    if has_message || (array_action && !has_draft_reply) {
        action.insert("message".to_string(), serde_json::json!(edited));
    }
    Ok(Some(actions))
}
