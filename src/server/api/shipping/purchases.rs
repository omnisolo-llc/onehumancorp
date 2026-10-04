//! Durable, one-shot admission for the existing order label lifecycle.
//! A committed dispatch fence is never leased, reset, or retried. Provider I/O
//! holds no database lock; GET-only reconciliation may resolve an unknown POST.
use super::{
    authority::{AuthorizedOwner, Error},
    labels,
};
use crate::{
    api::fulfillment::authentication::ProviderScope,
    integrations::shippo::client::{Observation, PurchaseLabelResponse},
};
use server_auth::commit_authority::AuthorityError;
use sqlx::Row;

#[derive(Clone)]
pub struct Intent {
    pub id: String,
    pub order_id: String,
    pub rate_id: String,
    pub transaction_id: Option<String>,
    pub receipt: Option<PurchaseLabelResponse>,
}
impl Intent {
    pub fn metadata(&self) -> String {
        format!("ohc_shipping_{}", self.id)
    }
}
pub enum Admission {
    Dispatch(Intent),
    Existing(Intent),
}
struct Context<'a> {
    scope: &'a ProviderScope,
    actor: &'a str,
    order: &'a str,
}

// Both supported engines run the same decisions and parameterized statements.
// Only row-lock syntax and the existing label recorder differ. The caller owns
// the canonical owner transaction and its final authority check.
macro_rules! repository {
    ($admit:ident, $load:ident, $observe:ident, $connection:ty, $order_query:expr, $intent_query:expr, $record:path) => {
        async fn $load(connection: &mut $connection, context: &Context<'_>) -> Result<Option<Intent>, Error> {
            let Some(row) = sqlx::query($intent_query)
                .bind(&context.scope.tenant_id).bind(context.order)
                .fetch_optional(&mut *connection).await? else { return Ok(None); };
            if row.try_get::<String,_>("actor_id")? != context.actor {
                return Err(AuthorityError::Forbidden.into());
            }
            if row.try_get::<String,_>("account_namespace")? != context.scope.account_namespace
                || row.try_get::<bool,_>("is_test")? != context.scope.is_test {
                return Err(Error::Conflict("purchase account or mode changed; reconcile the original account"));
            }
            let raw: Option<String> = row.try_get("receipt_json")?;
            let receipt: Option<PurchaseLabelResponse> = raw.map(|raw| serde_json::from_str(&raw)
                .map_err(|_| Error::Conflict("stored purchase receipt is invalid; reconciliation required"))).transpose()?;
            let transaction_id: Option<String> = row.try_get("transaction_id")?;
            let recorded = row.try_get::<String,_>("status")? == "recorded";
            if recorded != receipt.is_some() || receipt.as_ref().is_some_and(|label|
                !label.success || label.test != context.scope.is_test || Some(&label.transaction_id) != transaction_id.as_ref()) {
                return Err(Error::Conflict("stored purchase identity is inconsistent; reconciliation required"));
            }
            Ok(Some(Intent { id: row.try_get("id")?, order_id: context.order.to_owned(), rate_id: row.try_get("rate_id")?, transaction_id, receipt }))
        }
        async fn $admit(connection: &mut $connection, context: &Context<'_>, rate: &str) -> Result<Admission, Error> {
            let order = sqlx::query_scalar::<_, Option<String>>($order_query)
                .bind(&context.scope.tenant_id).bind(context.order).fetch_optional(&mut *connection).await?;
            if let Some(intent) = $load(&mut *connection, context).await? {
                if intent.rate_id != rate { return Err(Error::Conflict("an immutable purchase already exists for this order; reconcile before any replacement")); }
                return Ok(Admission::Existing(intent));
            }
            labels::validate_order(order)?;
            let tasks = sqlx::query("SELECT provider,provider_delivery_id,status FROM delivery_tasks WHERE organization_id=$1 AND order_id=$2")
                .bind(&context.scope.tenant_id).bind(context.order).fetch_all(&mut *connection).await?;
            if tasks.len() > 1 { return Err(Error::Conflict("multiple delivery tasks require reconciliation")); }
            for task in tasks {
                labels::validate_task(task.try_get("provider")?, task.try_get("provider_delivery_id")?, task.try_get("status")?)?;
            }
            let bound: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM delivery_provider_bindings b JOIN delivery_tasks d ON d.id=b.delivery_task_id AND d.organization_id=b.organization_id WHERE d.organization_id=$1 AND d.order_id=$2)")
                .bind(&context.scope.tenant_id).bind(context.order).fetch_one(&mut *connection).await?;
            if bound { return Err(Error::Conflict("an existing provider receipt requires reconciliation; its identity cannot be replaced")); }
            let id = uuid::Uuid::new_v4().to_string();
            sqlx::query("INSERT INTO shipping_purchase_intents(id,organization_id,actor_id,order_id,rate_id,account_namespace,is_test,status) VALUES($1,$2,$3,$4,$5,$6,$7,'dispatched')")
                .bind(&id).bind(&context.scope.tenant_id).bind(context.actor).bind(context.order).bind(rate)
                .bind(&context.scope.account_namespace).bind(context.scope.is_test).execute(&mut *connection).await?;
            Ok(Admission::Dispatch(Intent { id, order_id: context.order.to_owned(), rate_id: rate.to_owned(), transaction_id: None, receipt: None }))
        }
        async fn $observe(connection: &mut $connection, context: &Context<'_>, expected: &Intent, observation: &Observation) -> Result<Intent, Error> {
            // Lock order before intent, matching admission and receipt recording.
            sqlx::query($order_query).bind(&context.scope.tenant_id).bind(context.order).fetch_optional(&mut *connection).await?;
            let mut intent = $load(&mut *connection, context).await?.ok_or(Error::Conflict("purchase intent is missing; reconciliation required"))?;
            if intent.id != expected.id || intent.rate_id != expected.rate_id {
                return Err(Error::Conflict("purchase intent changed; reconciliation required"));
            }
            if observation.transaction_id.as_ref().is_some_and(|id| intent.transaction_id.as_ref().is_some_and(|saved| saved != id)) {
                return Err(Error::Conflict("a different provider transaction is already bound to this purchase"));
            }
            if intent.receipt.is_some() { return Ok(intent); }
            if let Some(label) = &observation.label {
                if !label.success || label.test != context.scope.is_test || Some(&label.transaction_id) != observation.transaction_id.as_ref() {
                    return Err(Error::Conflict("provider observation does not contain a matching receipt"));
                }
                $record(&mut *connection, context.scope, context.order, label).await?;
                let receipt = serde_json::to_string(label).map_err(|_| Error::Conflict("provider receipt could not be encoded"))?;
                sqlx::query("UPDATE shipping_purchase_intents SET status='recorded',transaction_id=$3,receipt_json=$4,updated_at=CURRENT_TIMESTAMP WHERE organization_id=$1 AND id=$2")
                    .bind(&context.scope.tenant_id).bind(&intent.id).bind(&label.transaction_id).bind(receipt).execute(&mut *connection).await?;
                intent.transaction_id=Some(label.transaction_id.clone()); intent.receipt=Some(label.clone());
            } else if let Some(id) = &observation.transaction_id {
                sqlx::query("UPDATE shipping_purchase_intents SET transaction_id=$3,updated_at=CURRENT_TIMESTAMP WHERE organization_id=$1 AND id=$2")
                    .bind(&context.scope.tenant_id).bind(&intent.id).bind(id).execute(&mut *connection).await?;
                intent.transaction_id=Some(id.clone());
            }
            Ok(intent)
        }
    }
}
repository!(
    admit_pg,
    load_pg,
    observe_pg,
    sqlx::PgConnection,
    "SELECT status FROM orders WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
    "SELECT * FROM shipping_purchase_intents WHERE organization_id=$1 AND order_id=$2 FOR UPDATE",
    labels::record_pg
);
repository!(
    admit_sqlite,
    load_sqlite,
    observe_sqlite,
    sqlx::SqliteConnection,
    "SELECT status FROM orders WHERE tenant_id=$1 AND id=$2",
    "SELECT * FROM shipping_purchase_intents WHERE organization_id=$1 AND order_id=$2",
    labels::record_sqlite
);

fn context<'a>(
    owner: &AuthorizedOwner,
    scope: &'a ProviderScope,
    actor: &'a str,
    order: &'a str,
) -> Result<Context<'a>, Error> {
    if owner.tenant_id() != scope.tenant_id {
        return Err(AuthorityError::Forbidden.into());
    }
    Ok(Context {
        scope,
        actor,
        order,
    })
}
pub async fn admit(
    owner: AuthorizedOwner,
    scope: &ProviderScope,
    order: &str,
    rate: &str,
) -> Result<Admission, Error> {
    let actor = owner.actor_id().to_owned();
    let context = context(&owner, scope, &actor, order)?;
    match owner {
        AuthorizedOwner::Postgres(owner) => {
            let mut tx = owner.begin().await?;
            let result = admit_pg(tx.connection(), &context, rate).await?;
            tx.commit().await?;
            Ok(result)
        }
        AuthorizedOwner::Sqlite(owner) => {
            let mut tx = owner.begin().await?;
            let result = admit_sqlite(tx.connection(), &context, rate).await?;
            tx.commit().await?;
            Ok(result)
        }
    }
}
pub async fn load(
    owner: AuthorizedOwner,
    scope: &ProviderScope,
    order: &str,
) -> Result<Option<Intent>, Error> {
    let actor = owner.actor_id().to_owned();
    let context = context(&owner, scope, &actor, order)?;
    match owner {
        AuthorizedOwner::Postgres(owner) => {
            let mut tx = owner.begin().await?;
            let result = load_pg(tx.connection(), &context).await?;
            tx.commit().await?;
            Ok(result)
        }
        AuthorizedOwner::Sqlite(owner) => {
            let mut tx = owner.begin().await?;
            let result = load_sqlite(tx.connection(), &context).await?;
            tx.commit().await?;
            Ok(result)
        }
    }
}
pub async fn observe(
    owner: AuthorizedOwner,
    scope: &ProviderScope,
    intent: &Intent,
    observation: &Observation,
) -> Result<Intent, Error> {
    let actor = owner.actor_id().to_owned();
    let context = context(&owner, scope, &actor, &intent.order_id)?;
    match owner {
        AuthorizedOwner::Postgres(owner) => {
            let mut tx = owner.begin().await?;
            let result = observe_pg(tx.connection(), &context, intent, observation).await?;
            tx.commit().await?;
            Ok(result)
        }
        AuthorizedOwner::Sqlite(owner) => {
            let mut tx = owner.begin().await?;
            let result = observe_sqlite(tx.connection(), &context, intent, observation).await?;
            tx.commit().await?;
            Ok(result)
        }
    }
}
