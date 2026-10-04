//! Stock, orders, and cash receipts share one PostgreSQL transaction. Direct
//! cash sales consume only available stock; online reservations remain intact.
use super::CommitInventoryRequest;
use crate::{
    hub::Hub,
    services::inventory::{InventoryCommitMode, InventoryService},
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, sqlx::FromRow)]
pub struct CashItem {
    pub product_id: String,
    pub quantity: i32,
    pub amount_cents: i64,
    #[serde(default)]
    pub lock_id: String,
}
#[derive(Debug, Serialize)]
pub struct CashReceipt {
    operation_id: String,
    order_id: String,
    tenant_id: String,
    amount_cents: i64,
    customer_id: Option<String>,
    items: Vec<CashItem>,
    // Single-product fields preserve the existing receipt contract.
    #[serde(skip_serializing_if = "Option::is_none")]
    product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lock_id: Option<String>,
}
impl CashReceipt {
    fn new(
        tenant: &str,
        operation: &str,
        order: String,
        amount: i64,
        customer: Option<String>,
        items: Vec<CashItem>,
    ) -> Self {
        let single = if items.len() == 1 {
            items.first()
        } else {
            None
        };
        Self {
            operation_id: operation.into(),
            order_id: order,
            tenant_id: tenant.into(),
            amount_cents: amount,
            customer_id: customer,
            product_id: single.map(|item| item.product_id.clone()),
            quantity: single.map(|item| item.quantity),
            lock_id: single.map(|item| item.lock_id.clone()),
            items,
        }
    }
}
fn no_store(mut response: Response) -> Response {
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        axum::http::HeaderValue::from_static("private, no-store"),
    );
    response
}
fn response(status: StatusCode, state: &str, error: &str) -> Response {
    no_store(
        (
            status,
            Json(serde_json::json!({"success":false,"status":state,"error_message":error})),
        )
            .into_response(),
    )
}
fn completed(receipt: CashReceipt) -> Response {
    no_store(
        Json(serde_json::json!({"success":true,"status":"completed","receipt":receipt}))
            .into_response(),
    )
}
fn unknown(error: impl std::fmt::Display) -> Response {
    tracing::warn!(%error, "Cash completion requires receipt reconciliation");
    response(
        StatusCode::SERVICE_UNAVAILABLE,
        "unknown",
        "Cash sale outcome is unconfirmed. Check the receipt or retry the same operation.",
    )
}
fn valid_id(id: &str) -> bool {
    !id.trim().is_empty() && id.len() <= 512 && !id.chars().any(char::is_control)
}
async fn receipt(
    tx: &mut Transaction<'_, Postgres>,
    tenant: &str,
    operation: &str,
) -> Result<Option<CashReceipt>, sqlx::Error> {
    let header:Option<(String,i64,Option<String>)>=sqlx::query_as("SELECT order_id,amount_cents,customer_id FROM terminal_cash_receipts WHERE tenant_id=$1 AND operation_id=$2")
        .bind(tenant).bind(operation).fetch_optional(&mut **tx).await?;
    let Some((order, amount, customer)) = header else {
        return Ok(None);
    };
    let mut items: Vec<CashItem>=sqlx::query_as("SELECT product_id,quantity,amount_cents,lock_id FROM terminal_cash_receipt_items WHERE tenant_id=$1 AND operation_id=$2 ORDER BY product_id")
        .bind(tenant).bind(operation).fetch_all(&mut **tx).await?;
    // SQL locale ordering can differ from Rust string ordering. Normalize both
    // new requests and stored receipts with one comparator before binding checks.
    items.sort_by(|left, right| left.product_id.cmp(&right.product_id));
    Ok(Some(CashReceipt::new(
        tenant, operation, order, amount, customer, items,
    )))
}

pub async fn read(hub: &Hub, tenant: &str, operation: &str) -> Response {
    let mut tx = match hub.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return unknown(e),
    };
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, tenant).await {
        return unknown(e);
    }
    match receipt(&mut tx, tenant, operation).await {
        Ok(Some(value)) => completed(value),
        Ok(None) => response(
            StatusCode::NOT_FOUND,
            "not_found",
            "No committed receipt found. A pending transaction may still finish; retry only the same operation to reconcile.",
        ),
        Err(e) => unknown(e),
    }
}

pub async fn commit(hub: &Hub, tenant: &str, request: &CommitInventoryRequest) -> Response {
    let Some(operation) = request
        .operation_id
        .as_deref()
        .filter(|value| valid_id(value))
    else {
        return response(
            StatusCode::BAD_REQUEST,
            "rejected",
            "A stable operation_id is required.",
        );
    };
    let Some(amount) = request.amount_cents.filter(|amount| *amount >= 0) else {
        return response(
            StatusCode::BAD_REQUEST,
            "rejected",
            "A nonnegative amount_cents is required.",
        );
    };
    let mut items = request.items.clone().unwrap_or_else(|| {
        vec![CashItem {
            product_id: request.product_id.clone(),
            quantity: request.quantity,
            amount_cents: amount,
            lock_id: request.lock_id.clone(),
        }]
    });
    items.sort_by(|a, b| a.product_id.cmp(&b.product_id));
    if items.is_empty()
        || items.len() > 100
        || items.iter().any(|item| {
            !valid_id(&item.product_id)
                || item.quantity <= 0
                || item.amount_cents < 0
                || (!item.lock_id.is_empty() && !valid_id(&item.lock_id))
        })
        || items
            .windows(2)
            .any(|pair| pair[0].product_id == pair[1].product_id)
    {
        return response(
            StatusCode::BAD_REQUEST,
            "rejected",
            "One to 100 distinct products with positive quantities and nonnegative amounts are required.",
        );
    }
    if items
        .iter()
        .try_fold(0_i64, |sum, item| sum.checked_add(item.amount_cents))
        != Some(amount)
    {
        return response(
            StatusCode::CONFLICT,
            "rejected",
            "Sale total does not match its items.",
        );
    }
    if request.tenant_id != tenant {
        return response(
            StatusCode::FORBIDDEN,
            "rejected",
            "Tenant does not match authenticated authority.",
        );
    }
    let mut tx = match hub.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return unknown(e),
    };
    if let Err(e) = crate::common::auth_utils::set_org_context(&mut *tx, tenant).await {
        return unknown(e);
    }
    if let Err(e) = sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,0))")
        .bind(format!("cash-operation:{tenant}:{operation}"))
        .execute(&mut *tx)
        .await
    {
        return unknown(e);
    }
    // Replay is resolved before price/stock/Redis validation. Lost responses and
    // later price changes cannot turn one committed operation into another sale.
    match receipt(&mut tx, tenant, operation).await {
        Ok(Some(value)) => {
            if value.items != items
                || value.amount_cents != amount
                || value.customer_id != request.customer_id
            {
                return response(
                    StatusCode::CONFLICT,
                    "rejected",
                    "Operation already belongs to different sale details.",
                );
            }
            return completed(value);
        }
        Ok(None) => {}
        Err(e) => return unknown(e),
    }
    if let Some(customer) = &request.customer_id {
        let owned: bool = match sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM customers WHERE tenant_id=$1 AND id=$2)",
        )
        .bind(tenant)
        .bind(customer)
        .fetch_one(&mut *tx)
        .await
        {
            Ok(value) => value,
            Err(e) => return unknown(e),
        };
        if !owned {
            return response(
                StatusCode::CONFLICT,
                "rejected",
                "Customer is not in this tenant.",
            );
        }
    }
    let service = InventoryService::new(hub.redis_client());
    // Every caller locks product rows in canonical order, preventing opposing
    // cart orders from deadlocking. PostgreSQL serializes competing allocations.
    for item in &items {
        let price: Option<i64> = match sqlx::query_scalar(
            "SELECT price_cents FROM products WHERE tenant_id=$1 AND id=$2 FOR UPDATE",
        )
        .bind(tenant)
        .bind(&item.product_id)
        .fetch_optional(&mut *tx)
        .await
        {
            Ok(value) => value,
            Err(e) => return unknown(e),
        };
        if price.and_then(|price| price.checked_mul(i64::from(item.quantity)))
            != Some(item.amount_cents)
        {
            return response(
                StatusCode::CONFLICT,
                "rejected",
                "Product price or quantity changed. Review the sale amount.",
            );
        }
        if !item.lock_id.is_empty() {
            let used:bool=match sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM terminal_cash_receipt_items WHERE tenant_id=$1 AND product_id=$2 AND lock_id=$3)").bind(tenant).bind(&item.product_id).bind(&item.lock_id).fetch_one(&mut *tx).await {Ok(value)=>value,Err(e)=>return unknown(e)};
            if used {
                return response(
                    StatusCode::CONFLICT,
                    "rejected",
                    "Reservation already has a completed sale.",
                );
            }
        }
        match service
            .commit_inventory_in_transaction(
                &mut tx,
                tenant,
                &item.product_id,
                item.quantity,
                &item.lock_id,
                if item.lock_id.is_empty() {
                    InventoryCommitMode::AvailableOnly
                } else {
                    InventoryCommitMode::ReservedOnly
                },
            )
            .await
        {
            Ok(result) if !result.success => {
                return response(StatusCode::CONFLICT, "rejected", &result.error_message);
            }
            Ok(_) => {}
            Err(e) => return unknown(e),
        }
    }
    let value = CashReceipt::new(
        tenant,
        operation,
        uuid::Uuid::new_v4().to_string(),
        amount,
        request.customer_id.clone(),
        items,
    );
    if let Err(e)=sqlx::query("INSERT INTO orders(id,tenant_id,customer_id,total_amount,status) VALUES($1,$2,$3,$4::bigint::numeric/100,'completed')").bind(&value.order_id).bind(tenant).bind(&value.customer_id).bind(amount).execute(&mut *tx).await {return unknown(e);}
    if let Err(e)=sqlx::query("INSERT INTO terminal_cash_receipts(tenant_id,operation_id,order_id,amount_cents,customer_id) VALUES($1,$2,$3,$4,$5)").bind(tenant).bind(operation).bind(&value.order_id).bind(amount).bind(&value.customer_id).execute(&mut *tx).await {return unknown(e);}
    for item in &value.items {
        if let Err(e)=sqlx::query("INSERT INTO order_items(id,tenant_id,order_id,product_id,quantity,price) VALUES($1,$2,$3,$4,$5,$6::bigint::numeric/100/$5)").bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(&value.order_id).bind(&item.product_id).bind(item.quantity).bind(item.amount_cents).execute(&mut *tx).await {return unknown(e);}
        if let Err(e)=sqlx::query("INSERT INTO terminal_cash_receipt_items(tenant_id,operation_id,product_id,quantity,amount_cents,lock_id) VALUES($1,$2,$3,$4,$5,$6)").bind(tenant).bind(operation).bind(&item.product_id).bind(item.quantity).bind(item.amount_cents).bind(&item.lock_id).execute(&mut *tx).await {return unknown(e);}
        let check = serde_json::json!({"source":"pos","order_id":value.order_id,"quantity":item.quantity,"reason":"in_person_sale_inventory_check"});
        if let Err(e)=sqlx::query("INSERT INTO agent_action_requests(id,tenant_id,action_type,status,product_id,payload) VALUES($1,$2,'InventoryCheck','Pending',$3,$4)").bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(&item.product_id).bind(check).execute(&mut *tx).await {return unknown(e);}
    }
    if let Err(e) = tx.commit().await {
        return unknown(e);
    }
    for item in &value.items {
        service
            .finish_inventory_commit(tenant, &item.product_id, &item.lock_id)
            .await;
    }
    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: value.order_id.clone(),
        tenant_id: tenant.to_string(),
        event_type: "POS_SALE_COMPLETED".to_string(),
        payload: serde_json::json!({"order_id":value.order_id,"operation_id":operation,"tenant_id":tenant,"customer_id":value.customer_id,"amount":amount as f64 / 100.0,"amount_cents":amount,"source":"in_person_pos"}),
    };
    // A notification failure cannot undo this already-committed sale. Exact
    // replays return the receipt above and never republish the completion.
    if let Err(error) = hub
        .publish_mesh_event(::server_omnisolo::orchestration::MeshEvent {
            event_id: value.order_id.clone(),
            topic: "pos_sales".to_string(),
            payload: serde_json::to_vec(&event).unwrap_or_default(),
            timestamp: chrono::Utc::now().timestamp(),
        })
        .await
    {
        tracing::warn!(%error, order_id=%value.order_id, "Cash sale committed; completion notification failed");
    }
    completed(value)
}
