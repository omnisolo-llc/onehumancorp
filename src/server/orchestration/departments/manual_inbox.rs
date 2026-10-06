//! Explicit signed-user inbox actions. These never fabricate an agent approval.
//! Both manual and department sends share the same committed provider fence.
use super::*;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManualAction {
    pub message_id: String,
    pub approved: bool,
    pub edited_reply: Option<String>,
    pub request_id: Option<String>,
    #[serde(default)]
    pub prepare_only: bool,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct ManualReceipt {
    pub request_id: String,
    pub message_id: String,
    pub state: String,
    pub provider_message_id: Option<String>,
    pub detail: String,
    pub draft_reply: String,
    pub prior_send: Option<ManualSendEvidence>,
}
#[derive(Clone, Debug, Serialize, PartialEq, Eq, sqlx::FromRow)]
pub struct ManualSendEvidence {
    pub request_id: String,
    pub state: String,
    pub provider_message_id: Option<String>,
    pub detail: String,
    pub draft_reply: String,
}
#[derive(sqlx::FromRow)]
struct Request {
    request_id: String, actor_id: String, inbox_message_id: String, dispatch_id: String,
    source: String, recipient: String, body: String, provider_binding: String,
    payload_hash: String, intent: String, state: String, expires_at: i64,
}
fn valid_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 200
        && value.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}
fn validate(tenant: &str, actor: &str, action: &ManualAction) -> Result<(), Error> {
    if tenant.trim().is_empty() || tenant.eq_ignore_ascii_case("system") || actor.trim().is_empty() {
        return Err(Error::Invalid("Signed user and tenant identity required"));
    }
    if !valid_id(&action.message_id) || action.request_id.as_deref().is_some_and(|id| !valid_id(id))
        || action.edited_reply.as_deref().is_some_and(|body| body.chars().count() > 16_000)
        || (action.prepare_only && (!action.approved || action.request_id.is_none() || action.edited_reply.is_none())) {
        return Err(Error::Invalid("Invalid manual inbox action"));
    }
    if action.approved && action.edited_reply.as_deref().is_some_and(|body| body.trim().is_empty()) {
        return Err(Error::Invalid("A non-empty explicit reply is required"));
    }
    Ok(())
}
fn digest(value: &Value) -> String { format!("{:x}", Sha256::digest(value.to_string().as_bytes())) }
async fn request(mut tx: &mut Transaction, tenant: &str, id: &str) -> Result<Option<Request>, Error> {
    Ok(database!(tx,c,{
        sqlx::query_as::<_,Request>("SELECT request_id,actor_id,inbox_message_id,dispatch_id,source,recipient,body,provider_binding,payload_hash,intent,state,expires_at FROM manual_inbox_requests WHERE tenant_id=$1 AND request_id=$2")
            .bind(tenant).bind(id).fetch_optional(c).await?
    }))
}
async fn receipt(mut tx: &mut Transaction, tenant: &str, row: &Request) -> Result<ManualReceipt, Error> {
    let saved: Option<DeliveryReceipt> = database!(tx,c,{
        sqlx::query_as("SELECT state,provider_message_id,detail FROM department_message_dispatches WHERE tenant_id=$1 AND action_id=$2")
            .bind(tenant).bind(&row.dispatch_id).fetch_optional(c).await?
    });
    let (state, provider_message_id, detail) = match saved {
        Some(saved) => (saved.state, saved.provider_message_id, saved.detail),
        None if matches!(row.state.as_str(), "dismissed" | "resolved") => (row.state.clone(),None,"Message closed; pending replies retired. A previously claimed send cannot be recalled; check its saved receipt.".into()),
        None if row.state == "pending" && row.expires_at > chrono::Utc::now().timestamp() =>
            ("pending".into(), None, "Reply prepared; no provider send attempted".into()),
        None => ("retired".into(), None, "This pending reply was replaced, expired or dismissed; no send attempted".into()),
    };
    let prior_send: Option<ManualSendEvidence> = if matches!(row.intent.as_str(), "dismissed" | "resolved") {
        database!(tx,c,{
            sqlx::query_as("SELECT r.request_id,d.state,d.provider_message_id,d.detail,r.body AS draft_reply FROM manual_inbox_requests r JOIN department_message_dispatches d ON d.tenant_id=r.tenant_id AND d.action_id=r.dispatch_id WHERE r.tenant_id=$1 AND r.actor_id=$2 AND r.inbox_message_id=$3 AND r.intent='send' ORDER BY r.intent_revision DESC LIMIT 1")
                .bind(tenant).bind(&row.actor_id).bind(&row.inbox_message_id).fetch_optional(c).await?
        })
    } else { None };
    Ok(ManualReceipt { request_id: row.request_id.clone(), message_id: row.inbox_message_id.clone(), state,
        provider_message_id, detail, draft_reply: row.body.clone(), prior_send })
}
pub async fn read(store: &Store, tenant: &str, actor: &str, message: &str, id: Option<&str>) -> Result<Option<ManualReceipt>, Error> {
    if actor.trim().is_empty() || !valid_id(message) || id.is_some_and(|id| !valid_id(id)) {
        return Err(Error::Invalid("Invalid manual request identity"));
    }
    let mut tx = store.begin(tenant).await?;
    let found: Option<Request> = database!(tx,c,{
        sqlx::query_as("SELECT request_id,actor_id,inbox_message_id,dispatch_id,source,recipient,body,provider_binding,payload_hash,intent,state,expires_at FROM manual_inbox_requests WHERE tenant_id=$1 AND actor_id=$2 AND inbox_message_id=$3 AND (CAST($4 AS TEXT) IS NULL OR request_id=$4) ORDER BY intent_revision DESC LIMIT 1")
            .bind(tenant).bind(actor).bind(message).bind(id).fetch_optional(c).await?
    });
    let result = if let Some(row) = found { Some(receipt(&mut tx, tenant, &row).await?) } else { None };
    tx.commit().await?;
    Ok(result)
}
pub async fn apply(store: &Store, tenant: &str, actor: &str, action: &ManualAction) -> Result<ManualReceipt, Error> {
    apply_with(store, tenant, actor, action, &LiveProvider).await
}
async fn retire_pending(mut tx: &mut Transaction, tenant: &str, message: &str) -> Result<(), Error> {
    database!(tx,c,{
        sqlx::query("UPDATE manual_inbox_requests SET state='retired' WHERE tenant_id=$1 AND inbox_message_id=$2 AND state='pending'")
            .bind(tenant).bind(message).execute(c).await?;
    });
    Ok(())
}
async fn apply_with(store: &Store, tenant: &str, actor: &str, action: &ManualAction, provider: &dyn DeliveryProvider) -> Result<ManualReceipt, Error> {
    validate(tenant, actor, action)?;
    let body = action.edited_reply.as_deref().unwrap_or("");
    // No clock, generated row, status or mutable provider field enters the legacy
    // key. Recipient/account are immutably frozen in the first recorded request.
    let id = action.request_id.clone().unwrap_or_else(|| format!("legacy-{}", digest(&serde_json::json!([tenant,actor,action.message_id,body,action.approved]))));
    let mut tx = store.begin(tenant).await?;
    // Same canonical inbox row locks as department dispatch, for every writer.
    let canonical_inbox = inbox(&mut tx, tenant, &action.message_id).await?;
    let intent = if !action.approved {"dismissed"} else if action.edited_reply.is_none() {"resolved"} else {"send"};
    let existing = request(&mut tx, tenant, &id).await?;
    let row = if let Some(row) = existing {
        if row.actor_id != actor || row.inbox_message_id != action.message_id || row.body != body || row.intent != intent {
            return Err(Error::Invalid("Request identity belongs to different content or author"));
        }
        if row.state != "pending" || row.expires_at <= chrono::Utc::now().timestamp() {
            if row.state == "pending" {
                database!(tx,c,{sqlx::query("UPDATE manual_inbox_requests SET state='retired' WHERE tenant_id=$1 AND request_id=$2 AND state='pending'").bind(tenant).bind(&id).execute(c).await?;});
            }
            let result = receipt(&mut tx, tenant, &row).await?;
            tx.commit().await?;
            return Ok(result);
        }
        row
    } else {
        if intent == "send" {
            // A newly invented request ID never clears an unknown/accepted claim.
            let fenced: Option<(String,)> = database!(tx,c,{
                sqlx::query_as("SELECT action_id FROM department_message_dispatches WHERE tenant_id=$1 AND inbox_message_id=$2 AND state IN ('unknown','accepted')")
                    .bind(tenant).bind(&action.message_id).fetch_optional(c).await?
            });
            if fenced.is_some() || canonical_inbox.status.as_deref().is_some_and(|s| historical_state(s) || terminal_state(s)) {
                return Err(Error::Invalid("Inbox is closed or has an existing send outcome; check the saved receipt before another action"));
            }
        }
        let source = canonical_inbox.source.clone().unwrap_or_default();
        let recipient = canonical_inbox.sender_id.clone().unwrap_or_default();
        let binding = if intent == "send" { credential(&mut tx,tenant,&source,None).await?.map(|credential| credential.binding(&source)) } else { None };
        let provider_binding = serde_json::to_string(&binding).map_err(|_|Error::Invalid("Invalid provider binding"))?;
        let dispatch_id = format!("manual-{}",digest(&serde_json::json!([tenant,id])));
        let now = chrono::Utc::now().timestamp();
        // Admission, owner retirement and revision advance share the canonical
        // inbox lock and one transaction. Replays return above without advance.
        let revision: i64 = database!(tx,c,{
            sqlx::query_scalar("INSERT INTO manual_inbox_intents(tenant_id,inbox_message_id,revision) VALUES($1,$2,1) ON CONFLICT(tenant_id,inbox_message_id) DO UPDATE SET revision=manual_inbox_intents.revision+1 RETURNING revision")
                .bind(tenant).bind(&action.message_id).fetch_one(c).await?
        });
        let hash = digest(&serde_json::json!([tenant,actor,id,action.message_id,source,recipient,body,binding,intent,revision]));
        retire_pending(&mut tx,tenant,&action.message_id).await?;
        let state = if intent == "send" {"pending"} else {intent};
        database!(tx,c,{
            sqlx::query("INSERT INTO manual_inbox_requests(tenant_id,request_id,actor_id,inbox_message_id,dispatch_id,source,recipient,body,provider_binding,payload_hash,intent,intent_revision,state,created_at,expires_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)")
                .bind(tenant).bind(&id).bind(actor).bind(&action.message_id).bind(&dispatch_id).bind(&source).bind(&recipient).bind(body).bind(&provider_binding).bind(&hash).bind(intent).bind(revision).bind(state).bind(now).bind(now+300).execute(c).await?;
        });
        Request {request_id:id.clone(),actor_id:actor.into(),inbox_message_id:action.message_id.clone(),dispatch_id,source,recipient,body:body.into(),provider_binding,payload_hash:hash,intent:intent.into(),state:state.into(),expires_at:now+300}
    };
    if intent != "send" {
        for table in ["inbox_messages", "omni_inbox_messages"] {
            let query = format!("UPDATE {table} SET status=$1 WHERE tenant_id=$2 AND id=$3");
            database!(tx,c,{sqlx::query(&query).bind(intent).bind(tenant).bind(&action.message_id).execute(c).await?;});
        }
        let result=receipt(&mut tx,tenant,&row).await?;
        tx.commit().await?;
        return Ok(result);
    }
    if action.prepare_only {
        let result = receipt(&mut tx,tenant,&row).await?;
        tx.commit().await?;
        return Ok(result);
    }
    let binding: Option<Binding> = serde_json::from_str(&row.provider_binding).map_err(|_|Error::Invalid("Invalid frozen provider binding"))?;
    let creds = if let Some(binding) = &binding { credential(&mut tx,tenant,&row.source,Some(&binding.credential_id)).await? } else { None };
    let canonical = canonical_inbox.source.as_deref() == Some(row.source.as_str()) && canonical_inbox.sender_id.as_deref() == Some(row.recipient.as_str());
    let terminal = canonical_inbox.terminal;
    let historical = canonical_inbox.status.as_deref().is_some_and(historical_state);
    let ready = canonical && !terminal && !historical && match (&binding,&creds) {
        (Some(binding),Some(creds)) => creds.binding(&row.source)==*binding && creds.configured(binding)
            && (valid_recipient(&row.source,&row.recipient) || (binding.integration_id=="whatsapp_cloud_api" && row.source=="whatsapp" && crate::integrations::meta::client::is_business_scoped_recipient(&row.recipient))),
        _ => false,
    };
    let state = if ready || historical {"unknown"} else {"blocked"};
    let detail = if historical {"Prior send outcome is unverified; reconcile before another attempt"} else if ready {"Provider outcome not yet confirmed; reconcile before retrying"} else {"Frozen recipient, provider account or inbox state changed or is unavailable; no send attempted"};
    let changed = database!(tx,c,{
        sqlx::query("UPDATE manual_inbox_requests SET state='claimed' WHERE tenant_id=$1 AND request_id=$2 AND state='pending' AND expires_at>$3")
            .bind(tenant).bind(&id).bind(chrono::Utc::now().timestamp()).execute(c).await?.rows_affected()
    });
    if changed != 1 { return Err(Error::Invalid("Pending reply ownership expired or was retired")); }
    let now = chrono::Utc::now().timestamp();
    let inserted = database!(tx,c,{
        sqlx::query("INSERT INTO department_message_dispatches(tenant_id,action_id,inbox_message_id,inbox_kind,payload_hash,provider_binding,state,detail,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$9) ON CONFLICT DO NOTHING")
            .bind(tenant).bind(&row.dispatch_id).bind(&row.inbox_message_id).bind(&canonical_inbox.kind).bind(&row.payload_hash).bind(&row.provider_binding).bind(state).bind(detail).bind(now).execute(c).await?.rows_affected()
    });
    if inserted != 1 { return Err(Error::Invalid("This inbox already has a fenced reply; reconcile before another send")); }
    if !terminal { set_inbox_state(&mut tx,tenant,&row.inbox_message_id,if ready || historical {"delivery_unknown"} else {"delivery_blocked"},&row.body).await?; }
    let initial = receipt(&mut tx,tenant,&row).await?;
    tx.commit().await?; // Durable manual claim precedes all provider HTTP.
    if !ready { return Ok(initial); }
    let result = provider.send(binding.as_ref().ok_or(Error::Invalid("Missing binding"))?,&row.recipient,&row.body,creds.as_ref().ok_or(Error::Invalid("Missing credentials"))?).await;
    let (state,provider_message_id,detail,inbox_state) = match result {
        Ok(id) if !id.trim().is_empty() && id.len()<=1024 => ("accepted",Some(id),"Provider accepted the message; delivery is unconfirmed","provider_accepted"),
        Err(SendFailure::Rejected) => ("rejected",None,"Provider rejected the message; no acceptance confirmed","delivery_failed"),
        Err(SendFailure::Blocked) => ("blocked",None,"Provider configuration invalid; no send attempted","delivery_blocked"),
        _ => ("unknown",None,"Provider outcome unknown; reconcile before retrying","delivery_unknown"),
    };
    let mut tx = store.begin(tenant).await?;
    let current = inbox(&mut tx,tenant,&row.inbox_message_id).await?;
    let changed = database!(tx,c,{
        sqlx::query("UPDATE department_message_dispatches SET state=$1,provider_message_id=$2,detail=$3,updated_at=$4 WHERE tenant_id=$5 AND action_id=$6 AND state='unknown' AND payload_hash=$7")
            .bind(state).bind(&provider_message_id).bind(detail).bind(chrono::Utc::now().timestamp()).bind(tenant).bind(&row.dispatch_id).bind(&row.payload_hash).execute(c).await?.rows_affected()
    });
    if changed != 1 { return Err(Error::Invalid("Provider result could not be correlated to its durable claim")); }
    // Dismissal after the claim cannot recall HTTP, but a late receipt must not
    // reopen the dismissed conversation or overwrite a newer draft.
    if !current.terminal { set_inbox_state(&mut tx,tenant,&row.inbox_message_id,inbox_state,&row.body).await?; }
    let final_receipt = receipt(&mut tx,tenant,&row).await?;
    tx.commit().await?;
    Ok(final_receipt)
}
