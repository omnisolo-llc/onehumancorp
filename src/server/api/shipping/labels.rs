use super::super::fulfillment::authentication::ProviderScope;
use super::authority::{AuthorizedOwner, Error};
use crate::integrations::shippo::client::PurchaseLabelResponse;
use sqlx::Row;

/// This records an observed receipt. It does not provide provider-call admission
/// or idempotency; failures after the POST require reconciliation, never retry.
pub async fn record(
    owner: AuthorizedOwner,
    scope: &ProviderScope,
    order_id: &str,
    label: &PurchaseLabelResponse,
) -> Result<(), Error> {
    if label.test != scope.is_test {
        return Err(Error::Conflict(
            "provider receipt mode does not match configured account",
        ));
    }
    if owner.tenant_id() != scope.tenant_id {
        return Err(server_auth::commit_authority::AuthorityError::Forbidden.into());
    }
    match owner {
        AuthorizedOwner::Postgres(owner) => {
            let mut tx = owner.begin().await?;
            let order = sqlx::query_scalar::<_, Option<String>>(
                "SELECT status FROM orders WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
            )
            .bind(&scope.tenant_id)
            .bind(order_id)
            .fetch_optional(tx.connection())
            .await?;
            validate_order(order)?;
            let tasks=sqlx::query("SELECT id,provider,provider_delivery_id,status FROM delivery_tasks WHERE organization_id=$1 AND order_id=$2 FOR UPDATE")
                .bind(&scope.tenant_id).bind(order_id).fetch_all(tx.connection()).await?;
            if tasks.len() > 1 {
                return Err(Error::Conflict(
                    "multiple delivery tasks require reconciliation",
                ));
            }
            let id = if let Some(task) = tasks.first() {
                validate_task(
                    task.try_get("provider")?,
                    task.try_get("provider_delivery_id")?,
                    task.try_get("status")?,
                )?;
                let id: uuid::Uuid = task.try_get("id")?;
                sqlx::query("UPDATE delivery_tasks SET provider='shippo',provider_delivery_id=$3,status='LABEL_CREATED',updated_at=CURRENT_TIMESTAMP WHERE id=$1 AND organization_id=$2")
                    .bind(id).bind(&scope.tenant_id).bind(&label.tracking_number).execute(tx.connection()).await?;
                id
            } else {
                let id = uuid::Uuid::new_v4();
                sqlx::query("INSERT INTO delivery_tasks(id,organization_id,order_id,provider,provider_delivery_id,status) VALUES($1,$2,$3,'shippo',$4,'LABEL_CREATED')")
                    .bind(id).bind(&scope.tenant_id).bind(order_id).bind(&label.tracking_number).execute(tx.connection()).await?;
                id
            };
            sqlx::query("INSERT INTO delivery_provider_bindings(delivery_task_id,organization_id,provider,account_namespace,provider_object_id,label_url,tracking_number,carrier,is_test) VALUES($1,$2,'shippo',$3,$4,$5,$6,$7,$8)")
                .bind(id).bind(&scope.tenant_id).bind(&scope.account_namespace).bind(&label.transaction_id).bind(&label.label_url).bind(&label.tracking_number).bind(&label.carrier).bind(label.test).execute(tx.connection()).await?;
            tx.commit().await?;
        }
        AuthorizedOwner::Sqlite(owner) => {
            let mut tx = owner.begin().await?;
            let order = sqlx::query_scalar::<_, Option<String>>(
                "SELECT status FROM orders WHERE tenant_id=? AND id=?",
            )
            .bind(&scope.tenant_id)
            .bind(order_id)
            .fetch_optional(tx.connection())
            .await?;
            validate_order(order)?;
            let tasks=sqlx::query("SELECT id,provider,provider_delivery_id,status FROM delivery_tasks WHERE organization_id=? AND order_id=?")
                .bind(&scope.tenant_id).bind(order_id).fetch_all(tx.connection()).await?;
            if tasks.len() > 1 {
                return Err(Error::Conflict(
                    "multiple delivery tasks require reconciliation",
                ));
            }
            let id = if let Some(task) = tasks.first() {
                validate_task(
                    task.try_get("provider")?,
                    task.try_get("provider_delivery_id")?,
                    task.try_get("status")?,
                )?;
                let id: String = task.try_get("id")?;
                sqlx::query("UPDATE delivery_tasks SET provider='shippo',provider_delivery_id=?,status='LABEL_CREATED',updated_at=CURRENT_TIMESTAMP WHERE id=? AND organization_id=?")
                    .bind(&label.tracking_number).bind(&id).bind(&scope.tenant_id).execute(tx.connection()).await?;
                id
            } else {
                let id = uuid::Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO delivery_tasks(id,organization_id,order_id,provider,provider_delivery_id,status) VALUES(?,?,?,'shippo',?,'LABEL_CREATED')")
                    .bind(&id).bind(&scope.tenant_id).bind(order_id).bind(&label.tracking_number).execute(tx.connection()).await?;
                id
            };
            sqlx::query("INSERT INTO delivery_provider_bindings(delivery_task_id,organization_id,provider,account_namespace,provider_object_id,label_url,tracking_number,carrier,is_test) VALUES(?,?,'shippo',?,?,?,?,?,?)")
                .bind(&id).bind(&scope.tenant_id).bind(&scope.account_namespace).bind(&label.transaction_id).bind(&label.label_url).bind(&label.tracking_number).bind(&label.carrier).bind(label.test).execute(tx.connection()).await?;
            tx.commit().await?;
        }
    }
    Ok(())
}
fn validate_order(status: Option<Option<String>>) -> Result<(), Error> {
    let Some(status) = status else {
        return Err(Error::Conflict("order is not in the configured tenant"));
    };
    if status.as_deref().is_some_and(|status| {
        matches!(
            status.to_ascii_lowercase().as_str(),
            "canceled" | "cancelled" | "fulfilled" | "returned"
        )
    }) {
        return Err(Error::Conflict(
            "order is no longer eligible; reconcile purchased label",
        ));
    }
    Ok(())
}
fn validate_task(
    provider: Option<String>,
    delivery: Option<String>,
    status: String,
) -> Result<(), Error> {
    if provider
        .as_deref()
        .is_some_and(|provider| provider != "shippo")
        || delivery.is_some()
        || !matches!(
            status.to_ascii_uppercase().as_str(),
            "PENDING" | "PREPARING"
        )
    {
        return Err(Error::Conflict(
            "an existing delivery must be reconciled; its identity cannot be replaced",
        ));
    }
    Ok(())
}

#[derive(serde::Serialize)]
pub struct Receipt {
    #[serde(flatten)]
    pub label: PurchaseLabelResponse,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "fulfillmentStatus")]
    pub fulfillment_status: String,
}

pub async fn read(
    owner: AuthorizedOwner,
    scope: &ProviderScope,
    transaction: &str,
) -> Result<Option<Receipt>, Error> {
    const QUERY: &str = "SELECT b.provider_object_id,b.label_url,b.tracking_number,b.carrier,b.is_test,d.order_id,d.status FROM delivery_provider_bindings b JOIN delivery_tasks d ON d.id=b.delivery_task_id AND d.organization_id=b.organization_id AND d.provider=b.provider JOIN orders o ON o.id=d.order_id AND o.tenant_id=b.organization_id WHERE b.organization_id=$1 AND b.provider='shippo' AND b.account_namespace=$2 AND b.is_test=$3 AND b.provider_object_id=$4 AND b.label_url IS NOT NULL";
    if owner.tenant_id() != scope.tenant_id {
        return Err(server_auth::commit_authority::AuthorityError::Forbidden.into());
    }
    match owner {
        AuthorizedOwner::Postgres(owner) => {
            let mut tx = owner.begin().await?;
            let row = sqlx::query(QUERY)
                .bind(&scope.tenant_id)
                .bind(&scope.account_namespace)
                .bind(scope.is_test)
                .bind(transaction)
                .fetch_optional(tx.connection())
                .await?;
            let receipt = row
                .map(|row| -> Result<Receipt, sqlx::Error> {
                    Ok(Receipt {
                        label: PurchaseLabelResponse {
                            success: true,
                            transaction_id: row.try_get("provider_object_id")?,
                            label_url: row.try_get("label_url")?,
                            tracking_number: row.try_get("tracking_number")?,
                            carrier: row.try_get("carrier")?,
                            test: row.try_get("is_test")?,
                        },
                        order_id: row.try_get("order_id")?,
                        fulfillment_status: row.try_get("status")?,
                    })
                })
                .transpose()?;
            tx.commit().await?;
            Ok(receipt)
        }
        AuthorizedOwner::Sqlite(owner) => {
            let mut tx = owner.begin().await?;
            let row = sqlx::query(QUERY)
                .bind(&scope.tenant_id)
                .bind(&scope.account_namespace)
                .bind(scope.is_test)
                .bind(transaction)
                .fetch_optional(tx.connection())
                .await?;
            let receipt = row
                .map(|row| -> Result<Receipt, sqlx::Error> {
                    Ok(Receipt {
                        label: PurchaseLabelResponse {
                            success: true,
                            transaction_id: row.try_get("provider_object_id")?,
                            label_url: row.try_get("label_url")?,
                            tracking_number: row.try_get("tracking_number")?,
                            carrier: row.try_get("carrier")?,
                            test: row.try_get("is_test")?,
                        },
                        order_id: row.try_get("order_id")?,
                        fulfillment_status: row.try_get("status")?,
                    })
                })
                .transpose()?;
            tx.commit().await?;
            Ok(receipt)
        }
    }
}
