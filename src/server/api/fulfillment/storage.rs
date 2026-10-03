use super::{
    authentication::ProviderScope,
    tracking::{TrackingUpdate, transition_allowed},
};
use sqlx::Row;

#[derive(Debug)]
pub enum Error {
    Database(sqlx::Error),
    Conflict(&'static str),
}
impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(f, "{error}"),
            Self::Conflict(reason) => write!(f, "{reason}"),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Outcome {
    Applied,
    Ignored,
}

pub async fn ensure_sqlite_schema(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    let rows = sqlx::query("PRAGMA table_info(delivery_tasks)")
        .fetch_all(&mut *tx)
        .await?;
    for (name, kind) in [
        ("provider", "TEXT"),
        ("provider_delivery_id", "TEXT"),
        ("delivery_location_lat", "REAL"),
        ("delivery_location_lng", "REAL"),
    ] {
        if !rows.iter().any(|row| row.get::<String, _>("name") == name) {
            sqlx::query(&format!(
                "ALTER TABLE delivery_tasks ADD COLUMN {name} {kind}"
            ))
            .execute(&mut *tx)
            .await?;
        }
    }
    sqlx::raw_sql(include_str!("bindings_sqlite.sql"))
        .execute(&mut *tx)
        .await?;
    tx.commit().await
}

pub async fn apply_tracking(
    pool: &sqlx::PgPool,
    scope: &ProviderScope,
    update: &TrackingUpdate,
    digest: &str,
) -> Result<Outcome, Error> {
    if update.is_test.is_some_and(|mode| mode != scope.is_test) {
        return Err(Error::Conflict(
            "provider mode does not match configured account",
        ));
    }
    let mut tx = pool.begin().await?;
    server_common::auth_utils::set_org_context(&mut *tx, &scope.tenant_id).await?;
    // Lock the canonical order together with its task and immutable account binding.
    // A provider ID is never resolved through an OHC order ID or a caller's tenant hint.
    let rows=sqlx::query(
        "SELECT b.delivery_task_id, b.last_event_at_ms, b.last_event_digest, d.status, o.status AS order_status, o.id AS order_id
         FROM delivery_provider_bindings b
         JOIN delivery_tasks d ON d.id=b.delivery_task_id AND d.organization_id=b.organization_id AND d.provider=b.provider
         JOIN orders o ON o.id=d.order_id AND o.tenant_id=b.organization_id
         WHERE b.organization_id=$1 AND b.provider=$2 AND b.account_namespace=$3 AND b.is_test=$4
           AND (($5::text IS NOT NULL AND b.provider_object_id=$5
                 AND ($2='doordash' OR ((b.tracking_number IS NULL OR b.tracking_number=$6) AND (b.carrier IS NULL OR b.carrier=$7))))
             OR ($5::text IS NULL AND $2='shippo' AND b.carrier=$7 AND b.tracking_number=$6))
         LIMIT 2 FOR UPDATE OF b,d,o")
        .bind(&scope.tenant_id).bind(update.provider).bind(&scope.account_namespace).bind(scope.is_test)
        .bind(&update.provider_object_id).bind(&update.tracking_number).bind(&update.carrier)
        .fetch_all(&mut *tx).await?;
    if rows.len() != 1 {
        return Err(Error::Conflict(
            "provider identity is unbound or ambiguous; reconcile the delivery binding",
        ));
    }
    let row = &rows[0];
    let order_status = row
        .try_get::<Option<String>, _>("order_status")?
        .unwrap_or_default();
    let last_digest = row.try_get::<Option<String>, _>("last_event_digest")?;
    if matches!(
        order_status.to_ascii_lowercase().as_str(),
        "canceled" | "cancelled" | "fulfilled" | "returned"
    ) || last_digest.as_deref() == Some(digest)
        || !transition_allowed(
            &row.try_get::<String, _>("status")?,
            row.try_get("last_event_at_ms")?,
            update,
        )
    {
        tx.commit().await?;
        return Ok(Outcome::Ignored);
    }
    let id: uuid::Uuid = row.try_get("delivery_task_id")?;
    sqlx::query("UPDATE delivery_tasks SET status=$3,driver_id=COALESCE($4,driver_id),delivery_location_lat=COALESCE($5,delivery_location_lat),delivery_location_lng=COALESCE($6,delivery_location_lng),provider_delivery_id=COALESCE(provider_delivery_id,$7),updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND organization_id=$2")
        .bind(id).bind(&scope.tenant_id).bind(&update.status).bind(&update.driver_id).bind(update.latitude).bind(update.longitude).bind(&update.tracking_number).execute(&mut *tx).await?;
    sqlx::query("UPDATE delivery_provider_bindings SET last_event_at_ms=$3,last_event_digest=$4,carrier=COALESCE(carrier,$5),tracking_number=COALESCE(tracking_number,$6) WHERE delivery_task_id=$1 AND organization_id=$2")
        .bind(id).bind(&scope.tenant_id).bind(update.status_at_ms).bind(digest).bind(&update.carrier).bind(&update.tracking_number).execute(&mut *tx).await?;
    let projected = match update.status.as_str() {
        "TRANSIT" => Some("shipped"),
        "DELIVERED" => Some("fulfilled"),
        "RETURNED" => Some("returned"),
        _ => None,
    };
    if let Some(status) = projected {
        sqlx::query("UPDATE orders SET status=$3,updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND tenant_id=$2 AND lower(COALESCE(status,'')) NOT IN ('canceled','cancelled','fulfilled','returned')")
            .bind(row.try_get::<String,_>("order_id")?).bind(&scope.tenant_id).bind(status).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(Outcome::Applied)
}
