//! Atomic, tenant-bound local onboarding. Preparation is not publication.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use server_omnisolo::orchestration::StartOnboardingRequest;
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, PgPool};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductIdentity {
    pub product_id: Option<String>,
    #[serde(default)]
    pub variant_ids: Vec<Option<String>>,
}
#[derive(Clone, Debug)]
pub struct CatalogProduct {
    pub product_id: Option<String>,
    pub name: String,
    pub description: String,
    pub cents: i64,
    pub item_type: String,
    pub metadata: Value,
    pub variants: Vec<CatalogVariant>,
}
#[derive(Clone, Debug)]
pub struct CatalogVariant {
    pub variant_id: Option<String>,
    pub name: String,
    pub cents: i64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewedVariant {
    pub variant_id: String,
    pub name: String,
    pub price_modifier: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReviewedProduct {
    pub product_id: String,
    pub item_type: String,
    pub name: String,
    pub price: String,
    pub description: String,
    pub variants: Vec<ReviewedVariant>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub channel: String,
    pub action: String,
    pub msg_id: String,
    pub payload: Value,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Preparation {
    pub preparation_id: String,
    pub status: String,
    pub organization_id: String,
    pub user_id: String,
    pub primary_product_id: String,
    pub reviewed_request: Value,
    pub catalog: Vec<ReviewedProduct>,
    pub notification_status: String,
    // Stored only in a protected database column. Public projection excludes these.
    pub fingerprint: String,
    pub source_identity: Option<String>,
    pub snapshots: Value,
    pub notifications: Vec<Notification>,
    pub seeded_agent_ids: Vec<String>,
    pub seeded_agent_snapshots: Value,
    pub tenant_snapshot: Value,
}
impl Preparation {
    pub fn public(&self) -> Value {
        json!({"preparation_id":self.preparation_id,"status":self.status,"organization_id":self.organization_id,"user_id":self.user_id,"primary_product_id":self.primary_product_id,"reviewed_request":self.reviewed_request,"catalog":self.catalog,"notification_status":self.notification_status})
    }
    pub fn response(&self) -> Value {
        json!({"success":true,"message":"Local business setup saved; publication is a separate outcome.","status":self.status,"organization_id":self.organization_id,"user_id":self.user_id,"preparation_id":self.preparation_id,"product_ids":self.catalog.iter().map(|p|&p.product_id).collect::<Vec<_>>(),"preparation":self.public()})
    }
}
#[derive(Debug)]
pub enum Error {
    Invalid(&'static str),
    Conflict(&'static str),
    Unauthorized,
    Database(sqlx::Error),
    Commit(sqlx::Error),
}
impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        Self::Database(e)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(s) | Self::Conflict(s) => f.write_str(s),
            Self::Unauthorized => f.write_str("active_authenticated_owner_required"),
            Self::Database(e) => write!(f, "preparation_transaction_failed: {e}"),
            Self::Commit(e) => write!(f, "preparation_commit_outcome_unknown: {e}"),
        }
    }
}
impl Error {
    pub fn response(self) -> axum::response::Response {
        use axum::response::IntoResponse;
        let (status, reason) = match &self {
            Self::Invalid(s) => (axum::http::StatusCode::BAD_REQUEST, *s),
            Self::Conflict(s) => (axum::http::StatusCode::CONFLICT, *s),
            Self::Unauthorized => (
                axum::http::StatusCode::UNAUTHORIZED,
                "active_authenticated_owner_required",
            ),
            Self::Database(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "preparation_transaction_failed",
            ),
            Self::Commit(_) => (
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                "commit_outcome_unknown",
            ),
        };
        tracing::warn!(reason, "Onboarding operation rejected");
        (status,axum::Json(json!({"success":false,"error":reason,"reconciliation_required":matches!(self,Self::Commit(_)|Self::Conflict(_))}))).into_response()
    }
}
/// Strict base-10 minor units. Fractional cents, exponent notation and NaN are invalid.
pub fn money(value: &str, signed: bool) -> Result<i64, Error> {
    let (negative, value) = match value.strip_prefix('-') {
        Some(v) if signed => (true, v),
        _ => (false, value),
    };
    let (whole, fraction) = value.split_once('.').unwrap_or((value, ""));
    if whole.is_empty()
        || !whole.bytes().all(|c| c.is_ascii_digit())
        || fraction.len() > 2
        || !fraction.bytes().all(|c| c.is_ascii_digit())
        || value.ends_with('.')
    {
        return Err(Error::Invalid("invalid_money"));
    }
    let whole = whole
        .parse::<i64>()
        .map_err(|_| Error::Invalid("invalid_money"))?;
    let fraction = if fraction.is_empty() {
        0
    } else {
        fraction
            .parse::<i64>()
            .map_err(|_| Error::Invalid("invalid_money"))?
            * if fraction.len() == 1 { 10 } else { 1 }
    };
    let cents = whole
        .checked_mul(100)
        .and_then(|n| n.checked_add(fraction))
        .filter(|n| *n <= 1_000_000_000)
        .ok_or(Error::Invalid("invalid_money"))?;
    Ok(if negative { -cents } else { cents })
}
fn display_money(cents: i64) -> String {
    format!(
        "{}{}.{:02}",
        if cents < 0 { "-" } else { "" },
        cents.abs() / 100,
        cents.abs() % 100
    )
}
fn valid_identity(value: &str) -> bool {
    !value.is_empty() && value.trim() == value && !value.eq_ignore_ascii_case("system")
}
pub fn validate_catalog(products: &[CatalogProduct]) -> Result<(), Error> {
    if products.is_empty() || products.len() > 50 {
        return Err(Error::Invalid("reviewed_catalog_required"));
    }
    let mut product_ids = BTreeSet::new();
    let mut variant_ids = BTreeSet::new();
    for p in products {
        if p.name.trim().is_empty()
            || p.name.chars().count() > 200
            || p.description.chars().count() > 10_000
            || !(0..=1_000_000_000).contains(&p.cents)
            || p.variants.len() > 50
        {
            return Err(Error::Invalid("invalid_catalog_entry"));
        }
        if let Some(id) = &p.product_id
            && (!valid_identity(id) || !product_ids.insert(id))
        {
            return Err(Error::Invalid("duplicate_product_identity"));
        }
        for v in &p.variants {
            if v.name.trim().is_empty()
                || v.name.chars().count() > 200
                || !(-1_000_000_000..=1_000_000_000).contains(&v.cents)
                || p.cents.checked_add(v.cents).is_none_or(|n| n < 0)
            {
                return Err(Error::Invalid("invalid_variant"));
            }
            if let Some(id) = &v.variant_id
                && (!valid_identity(id) || !variant_ids.insert(id))
            {
                return Err(Error::Invalid("duplicate_variant_identity"));
            }
        }
    }
    Ok(())
}
fn notification(channel: &str, action: &str, payload: Value) -> Notification {
    Notification {
        channel: channel.into(),
        action: action.into(),
        payload,
        msg_id: uuid::Uuid::new_v4().to_string(),
    }
}
/// Reused by singleton, persona defaults and reviewed multi-product preparation.
pub async fn save_catalog(
    tx: &mut PgConnection,
    tenant: &str,
    products: &[CatalogProduct],
) -> Result<Vec<ReviewedProduct>, Error> {
    validate_catalog(products)?;
    let mut saved = vec![];
    for p in products {
        let id = p
            .product_id
            .clone()
            .unwrap_or_else(|| format!("prod-{}", uuid::Uuid::new_v4()));
        let saved_type = if p.product_id.is_some() {
            sqlx::query_scalar::<_,String>("UPDATE products SET title=$1,description=$2,price_cents=$3,price=$3::bigint::numeric/100,metadata=(COALESCE(metadata,'{}'::jsonb)-'price_type'-'deposit_percentage'-'deposit_payment_status'-'lead_time_days') || $4,updated_at=clock_timestamp() WHERE id=$5 AND tenant_id=$6 RETURNING COALESCE(type,'physical')")
                .bind(&p.name).bind(&p.description).bind(p.cents).bind(&p.metadata).bind(&id).bind(tenant).fetch_optional(&mut *tx).await?.ok_or(Error::Conflict("prepared_product_missing"))?
        } else {
            sqlx::query("INSERT INTO products (id,tenant_id,title,description,price_cents,price,type,metadata) VALUES($1,$2,$3,$4,$5,$5::bigint::numeric/100,$6,$7)").bind(&id).bind(tenant).bind(&p.name).bind(&p.description).bind(p.cents).bind(&p.item_type).bind(&p.metadata).execute(&mut *tx).await?;
            p.item_type.clone()
        };
        let mut variants = vec![];
        for v in &p.variants {
            let vid = v
                .variant_id
                .clone()
                .unwrap_or_else(|| format!("var-{}", uuid::Uuid::new_v4()));
            if v.variant_id.is_some() {
                let n=sqlx::query("UPDATE product_variants SET name=$1,price_modifier=$2 WHERE id=$3 AND tenant_id=$4 AND product_id=$5").bind(&v.name).bind(v.cents).bind(&vid).bind(tenant).bind(&id).execute(&mut *tx).await?.rows_affected();
                if n != 1 {
                    return Err(Error::Conflict("prepared_variant_missing"));
                }
            } else {
                sqlx::query("INSERT INTO product_variants(id,tenant_id,product_id,name,sku,price_modifier,inventory_count) VALUES($1,$2,$3,$4,'',$5,0)").bind(&vid).bind(tenant).bind(&id).bind(&v.name).bind(v.cents).execute(&mut *tx).await?;
            }
            variants.push(ReviewedVariant {
                variant_id: vid,
                name: v.name.clone(),
                price_modifier: display_money(v.cents),
            });
        }
        saved.push(ReviewedProduct {
            product_id: id,
            item_type: saved_type,
            name: p.name.clone(),
            description: p.description.clone(),
            price: display_money(p.cents),
            variants,
        });
    }
    Ok(saved)
}
async fn snapshots(
    tx: &mut PgConnection,
    tenant: &str,
    products: &[ReviewedProduct],
) -> Result<Value, Error> {
    let mut result = serde_json::Map::new();
    for p in products {
        let row = sqlx::query_scalar::<_, Value>(
            "SELECT to_jsonb(p) FROM products p WHERE id=$1 AND tenant_id=$2 FOR UPDATE",
        )
        .bind(&p.product_id)
        .bind(tenant)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or(Error::Conflict("prepared_product_missing"))?;
        let variants=sqlx::query_scalar::<_,Value>("SELECT to_jsonb(v) FROM product_variants v WHERE product_id=$1 AND tenant_id=$2 ORDER BY id FOR UPDATE").bind(&p.product_id).bind(tenant).fetch_all(&mut *tx).await?;
        result.insert(
            p.product_id.clone(),
            json!({"product":row,"variants":variants}),
        );
    }
    Ok(Value::Object(result))
}
async fn lock_identity(tx: &mut PgConnection, tenant: &str, user: &str) -> Result<(), Error> {
    if !valid_identity(tenant) || !valid_identity(user) {
        return Err(Error::Unauthorized);
    }
    server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
    if sqlx::query_scalar::<_, String>("SELECT id FROM tenants WHERE id=$1 FOR UPDATE")
        .bind(tenant)
        .fetch_optional(&mut *tx)
        .await?
        .is_none()
    {
        return Err(Error::Unauthorized);
    }
    if sqlx::query_scalar::<_,String>("SELECT id FROM users WHERE id=$1 AND tenant_id=$2 AND active=TRUE AND EXISTS(SELECT 1 FROM unnest(roles) AS role WHERE lower(role) IN ('admin','owner')) FOR SHARE").bind(user).bind(tenant).fetch_optional(&mut *tx).await?.is_none(){return Err(Error::Unauthorized);}
    Ok(())
}
async fn receipt(tx: &mut PgConnection, tenant: &str) -> Result<Option<Preparation>, Error> {
    sqlx::query_scalar::<_,Value>("SELECT preparation_receipt FROM onboarding_state WHERE tenant_id=$1 AND preparation_receipt IS NOT NULL FOR UPDATE").bind(tenant).fetch_optional(tx).await?.map(|v|serde_json::from_value(v).map_err(|_|Error::Conflict("legacy_preparation_requires_reconciliation"))).transpose()
}
fn validate_revision(old: &Preparation, products: &[CatalogProduct]) -> Result<(), Error> {
    // Never infer row identity from array position, name or an AI-generated label.
    for p in &old.catalog {
        let new = products
            .iter()
            .find(|n| n.product_id.as_deref() == Some(&p.product_id))
            .ok_or(Error::Conflict(
                "destructive_catalog_revision_not_supported",
            ))?;
        for v in &p.variants {
            if !new
                .variants
                .iter()
                .any(|n| n.variant_id.as_deref() == Some(&v.variant_id))
            {
                return Err(Error::Conflict(
                    "destructive_variant_revision_not_supported",
                ));
            }
        }
    }
    for p in products {
        if let Some(id) = &p.product_id {
            let oldp = old
                .catalog
                .iter()
                .find(|p| &p.product_id == id)
                .ok_or(Error::Conflict("unknown_prepared_product"))?;
            if p.variants.iter().any(|v| {
                v.variant_id
                    .as_ref()
                    .is_some_and(|id| !oldp.variants.iter().any(|v| &v.variant_id == id))
            }) {
                return Err(Error::Conflict("unknown_prepared_variant"));
            }
        } else if p.variants.iter().any(|v| v.variant_id.is_some()) {
            return Err(Error::Conflict("unknown_prepared_variant"));
        }
    }
    Ok(())
}
#[allow(clippy::too_many_arguments)]
pub async fn prepare(
    pool: &PgPool,
    tenant: &str,
    user: &str,
    req: &StartOnboardingRequest,
    products: &[CatalogProduct],
    previous: Option<&str>,
    source_identity: Option<&str>,
    flags: Value,
) -> Result<Preparation, Error> {
    validate_catalog(products)?;
    if req.company_name.trim().is_empty()
        || req.company_name.chars().count() > 4000
        || req
            .deposit_percentage
            .is_some_and(|n| !(0..=100).contains(&n))
        || req.lead_time_days.is_some_and(|n| !(0..=3650).contains(&n))
    {
        return Err(Error::Invalid("invalid_preparation"));
    }
    let request = serde_json::to_value(req).map_err(|_| Error::Invalid("invalid_preparation"))?;
    let identities:Vec<_>=products.iter().map(|p|json!({"product_id":p.product_id,"variant_ids":p.variants.iter().map(|v|&v.variant_id).collect::<Vec<_>>()})).collect();
    let fingerprint=format!("{:x}",Sha256::digest(serde_json::to_vec(&json!({"request":request,"identities":identities,"previous":previous,"source_identity":source_identity})).map_err(|_|Error::Invalid("invalid_preparation"))?));
    let mut tx = pool.begin().await?;
    lock_identity(&mut tx, tenant, user).await?;
    let old = receipt(&mut tx, tenant).await?;
    if let Some(old) = &old {
        if old.user_id != user {
            return Err(Error::Conflict("preparation_owned_by_another_user"));
        }
        if old.fingerprint == fingerprint
            || (source_identity.is_some() && old.source_identity.as_deref() == source_identity)
        {
            return Ok(old.clone());
        }
        if old.status == "launched" {
            return Err(Error::Conflict("setup_already_launched_use_catalog_editor"));
        }
        if previous != Some(old.preparation_id.as_str()) {
            return Err(Error::Conflict("preparation_revision_required"));
        }
        validate_revision(old, products)?;
        let tenant_snapshot: Value = sqlx::query_scalar(
            "SELECT jsonb_build_object('name',name,'subdomain',subdomain) FROM tenants WHERE id=$1",
        )
        .bind(tenant)
        .fetch_one(&mut *tx)
        .await?;
        if tenant_snapshot != old.tenant_snapshot {
            return Err(Error::Conflict("prepared_business_identity_changed"));
        }
        if snapshots(&mut tx, tenant, &old.catalog).await? != old.snapshots {
            return Err(Error::Conflict("prepared_catalog_changed"));
        }
    } else {
        if previous.is_some()
            || products.iter().any(|p| {
                p.product_id.is_some() || p.variants.iter().any(|v| v.variant_id.is_some())
            })
        {
            return Err(Error::Conflict("unknown_preparation"));
        }
        let launched:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM onboarding_state WHERE tenant_id=$1 AND state_json->>'status'='launched')").bind(tenant).fetch_one(&mut *tx).await?;
        if launched {
            return Err(Error::Conflict("legacy_launch_requires_reconciliation"));
        }
    }
    let catalog = save_catalog(&mut tx, tenant, products).await?;
    let seeded_agent_ids = seed_agents(&mut tx, tenant, &req.ai_agents).await?;
    sqlx::query("UPDATE tenants SET name=$1,subdomain=$2 WHERE id=$3")
        .bind(&req.company_name)
        .bind(&req.domain_choice)
        .bind(tenant)
        .execute(&mut *tx)
        .await?;
    let mut notifications = vec![];
    for (p, source) in catalog.iter().zip(products) {
        notifications.push(notification("products_inbox","ProductCreated",json!({"product_id":p.product_id,"name":p.name,"description":p.description,"price_cents":source.cents,"price":source.cents as f64/100.0,"item_type":p.item_type,"organization_id":tenant,"tenant_id":tenant})));
    }
    notifications.push(notification("promoter_inbox","GenerateStorefront",json!({"organization_id":tenant,"tenant_id":tenant,"company_name":req.company_name,"business_type":req.business_type})));
    notifications.push(notification(
        "protector_inbox",
        "GeneratePolicies",
        json!({"organization_id":tenant,"tenant_id":tenant,"company_name":req.company_name}),
    ));
    let mut prepared = Preparation {
        preparation_id: uuid::Uuid::new_v4().to_string(),
        status: "prepared".into(),
        organization_id: tenant.into(),
        user_id: user.into(),
        primary_product_id: old
            .as_ref()
            .map(|o| o.primary_product_id.clone())
            .unwrap_or_else(|| catalog[0].product_id.clone()),
        reviewed_request: request,
        catalog,
        notification_status: "awaiting_launch".into(),
        fingerprint,
        source_identity: source_identity.map(str::to_owned),
        snapshots: Value::Null,
        notifications,
        seeded_agent_ids: old
            .as_ref()
            .map(|o| o.seeded_agent_ids.clone())
            .unwrap_or_default(),
        seeded_agent_snapshots: old
            .as_ref()
            .map(|o| o.seeded_agent_snapshots.clone())
            .unwrap_or_else(|| json!({})),
        tenant_snapshot: Value::Null,
    };
    for id in &seeded_agent_ids {
        let snapshot: Value =
            sqlx::query_scalar("SELECT to_jsonb(a) FROM agents a WHERE id=$1 AND tenant_id=$2")
                .bind(id)
                .bind(tenant)
                .fetch_one(&mut *tx)
                .await?;
        prepared.seeded_agent_snapshots[id] = snapshot;
    }
    prepared.tenant_snapshot = sqlx::query_scalar(
        "SELECT jsonb_build_object('name',name,'subdomain',subdomain) FROM tenants WHERE id=$1",
    )
    .bind(tenant)
    .fetch_one(&mut *tx)
    .await?;
    prepared.seeded_agent_ids.extend(seeded_agent_ids);
    prepared.seeded_agent_ids.sort();
    prepared.seeded_agent_ids.dedup();
    prepared.snapshots = snapshots(&mut tx, tenant, &prepared.catalog).await?;
    sqlx::query("INSERT INTO onboarding_state(tenant_id,user_id,current_step,state_json,preparation_receipt,updated_at) VALUES($1,$2,1,$3,$4,clock_timestamp()) ON CONFLICT(tenant_id,user_id) DO UPDATE SET current_step=GREATEST(onboarding_state.current_step,1),state_json=onboarding_state.state_json || EXCLUDED.state_json,preparation_receipt=EXCLUDED.preparation_receipt,updated_at=clock_timestamp()")
        .bind(tenant).bind(user).bind(flags).bind(serde_json::to_value(&prepared).map_err(|_|Error::Invalid("invalid_preparation"))?).execute(&mut *tx).await?;
    tx.commit().await.map_err(Error::Commit)?;
    Ok(prepared)
}
async fn seed_agents(
    tx: &mut PgConnection,
    tenant: &str,
    selected: &[String],
) -> Result<Vec<String>, Error> {
    let defaults = [
        ("Operations", "The Manager", "Operations"),
        ("Marketing & Advertising", "The Promoter", "Marketing"),
        ("Sales & Acquisition", "The Salesperson", "Sales"),
        ("Customer Success", "The Ambassador", "CustomerSuccess"),
        ("Finance & Payments", "The Accountant", "Finance"),
        ("Legal & Compliance", "The Protector", "Legal"),
        ("Business Advisory", "The Advisor", "Advisory"),
        ("Discovery & SEO", "The Scout", "Discovery"),
    ];
    let mut ids = vec![];
    for (name, role, key) in defaults {
        if !selected.is_empty() && !selected.iter().any(|n| n == name) {
            continue;
        }
        let id = format!("{tenant}-{}", key.to_lowercase());
        let saved=sqlx::query("INSERT INTO agents(id,tenant_id,name,role,status,provider_type) VALUES($1,$2,$3,$4,'IDLE','builtin') ON CONFLICT(id) DO NOTHING").bind(&id).bind(tenant).bind(name).bind(role).execute(&mut *tx).await?;
        if saved.rows_affected() == 1 {
            ids.push(id);
        } else if sqlx::query_scalar::<_, String>("SELECT tenant_id FROM agents WHERE id=$1")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await?
            .as_deref()
            != Some(tenant)
        {
            return Err(Error::Conflict("agent_identity_owned_by_another_tenant"));
        }
    }
    Ok(ids)
}
pub async fn read_state(
    pool: &PgPool,
    tenant: &str,
    user: &str,
) -> Result<
    (
        Option<Preparation>,
        Value,
        i32,
        Option<chrono::DateTime<chrono::Utc>>,
    ),
    Error,
> {
    let mut tx = pool.begin().await?;
    lock_identity(&mut tx, tenant, user).await?;
    let prepared = receipt(&mut tx, tenant)
        .await?
        .filter(|p| p.user_id == user);
    let (state, step, updated_at) = sqlx::query_as::<_, (Value, i32, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT state_json,current_step,updated_at FROM onboarding_state WHERE tenant_id=$1 AND user_id=$2",
    )
    .bind(tenant)
    .bind(user)
    .fetch_optional(&mut *tx)
    .await?
    .unwrap_or_else(|| (json!({}), 0, None));
    tx.commit().await.map_err(Error::Commit)?;
    Ok((prepared, state, step, updated_at))
}
pub async fn read(pool: &PgPool, tenant: &str, user: &str) -> Result<Option<Preparation>, Error> {
    Ok(read_state(pool, tenant, user).await?.0)
}
pub async fn launch(
    pool: &PgPool,
    tenant: &str,
    user: &str,
    id: &str,
) -> Result<Preparation, Error> {
    let mut tx = pool.begin().await?;
    lock_identity(&mut tx, tenant, user).await?;
    let mut p = receipt(&mut tx, tenant)
        .await?
        .ok_or(Error::Conflict("committed_preparation_required"))?;
    if p.user_id != user || p.preparation_id != id {
        return Err(Error::Conflict("preparation_identity_mismatch"));
    }
    if p.status == "launched" {
        return Ok(p);
    }
    let tenant_snapshot: Value = sqlx::query_scalar(
        "SELECT jsonb_build_object('name',name,'subdomain',subdomain) FROM tenants WHERE id=$1",
    )
    .bind(tenant)
    .fetch_one(&mut *tx)
    .await?;
    if tenant_snapshot != p.tenant_snapshot {
        return Err(Error::Conflict("prepared_business_identity_changed"));
    }
    if snapshots(&mut tx, tenant, &p.catalog).await? != p.snapshots {
        return Err(Error::Conflict("prepared_catalog_changed"));
    }
    for (role, topic) in [
        ("The Manager", "tenant.booking.created"),
        ("The Manager", "tenant.order.placed"),
        ("The Promoter", "tenant.product.created"),
        ("The Salesperson", "tenant.lead.created"),
        ("The Ambassador", "tenant.message.received"),
        ("The Accountant", "tenant.payment.success"),
        ("The Protector", "tenant.contract.signed"),
        ("The Advisor", "tenant.report.generated"),
        ("The Scout", "tenant.seo.optimized"),
    ] {
        sqlx::query("INSERT INTO agent_event_subscriptions(tenant_id,agent_role,topic) VALUES($1,$2,$3) ON CONFLICT DO NOTHING").bind(tenant).bind(role).bind(topic).execute(&mut *tx).await?;
    }
    let auto = p.reviewed_request["ai_auto_respond"]
        .as_bool()
        .unwrap_or(false);
    if auto {
        let selected = p.reviewed_request["ai_agents"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        for id in &p.seeded_agent_ids {
            let expected = &p.seeded_agent_snapshots[id];
            if !selected.is_empty() && !selected.iter().any(|name| name == &expected["name"]) {
                continue;
            }
            let current: Option<Value> = sqlx::query_scalar(
                "SELECT to_jsonb(a) FROM agents a WHERE id=$1 AND tenant_id=$2 FOR UPDATE",
            )
            .bind(id)
            .bind(tenant)
            .fetch_optional(&mut *tx)
            .await?;
            if current.as_ref() != Some(expected) {
                return Err(Error::Conflict("prepared_agent_changed"));
            }
            sqlx::query(
                "UPDATE agents SET status='ACTIVE' WHERE id=$1 AND tenant_id=$2 AND status='IDLE'",
            )
            .bind(id)
            .bind(tenant)
            .execute(&mut *tx)
            .await?;
        }
    }
    sqlx::query("INSERT INTO sub_agent_queue(id,tenant_id,parent_task_id,payload,status,scheduled_at,created_at,updated_at) VALUES($1,$2,NULL,$3,'QUEUED',clock_timestamp()+INTERVAL '7 days',clock_timestamp(),clock_timestamp())")
        .bind(format!("onboarding-health-{}",p.preparation_id)).bind(tenant).bind(json!({"agent_role":"The Advisor","task":"weekly_health_report","tenant_id":tenant})).execute(&mut *tx).await?;
    sqlx::query("INSERT INTO agent_feed_items(id,tenant_id,event_source,context_payload,proposed_action,lifecycle_state) VALUES($1,$2,'system',$3,$4,'PENDING_APPROVAL')")
        .bind(format!("onboarding-welcome-{}",p.preparation_id)).bind(tenant).bind(json!({"description":"Your local business setup is saved. Review storefront and provider readiness before publishing.","feature_type":"onboarding_welcome","company_name":p.reviewed_request["company_name"]})).bind(json!({"action_type":"review_storefront"})).execute(&mut *tx).await?;
    p.status = "launched".into();
    p.notification_status = "pending".into();
    sqlx::query("UPDATE onboarding_state SET current_step=GREATEST(current_step,5),state_json=state_json||'{\"status\":\"launched\"}'::jsonb,preparation_receipt=$1,updated_at=clock_timestamp() WHERE tenant_id=$2 AND user_id=$3")
        .bind(serde_json::to_value(&p).map_err(|_|Error::Invalid("invalid_preparation"))?).bind(tenant).bind(user).execute(&mut *tx).await?;
    tx.commit().await.map_err(Error::Commit)?;
    Ok(p)
}
/// Claim before any in-memory publication. An interrupted claim is reconciled,
/// never automatically resent because consumers are not proven idempotent.
pub async fn claim_notifications(
    pool: &PgPool,
    tenant: &str,
    user: &str,
    id: &str,
) -> Result<Vec<Notification>, Error> {
    let mut tx = pool.begin().await?;
    lock_identity(&mut tx, tenant, user).await?;
    let mut p = receipt(&mut tx, tenant)
        .await?
        .ok_or(Error::Conflict("committed_preparation_required"))?;
    if p.user_id != user || p.preparation_id != id || p.status != "launched" {
        return Err(Error::Conflict("preparation_identity_mismatch"));
    }
    if p.notification_status != "pending" {
        return Ok(vec![]);
    }
    // Hub's broadcast API cannot confirm receipt or downstream completion.
    p.notification_status = "delivery_unconfirmed".into();
    sqlx::query(
        "UPDATE onboarding_state SET preparation_receipt=$1 WHERE tenant_id=$2 AND user_id=$3",
    )
    .bind(serde_json::to_value(&p).map_err(|_| Error::Invalid("invalid_preparation"))?)
    .bind(tenant)
    .bind(user)
    .execute(&mut *tx)
    .await?;
    tx.commit().await.map_err(Error::Commit)?;
    Ok(p.notifications)
}
