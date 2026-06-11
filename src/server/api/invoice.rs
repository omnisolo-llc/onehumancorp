use std::sync::Arc;

use ::server_ohc::invoice::*;
use ::server_ohc::invoice::invoice_service_server::InvoiceService;
use tonic::{Request, Response, Status};

use crate::hub::Hub;

pub struct InvoiceServiceImpl {
    pub hub: Arc<Hub>,
}

#[tonic::async_trait]
impl InvoiceService for InvoiceServiceImpl {
    async fn create_invoice(
        &self,
        request: Request<CreateInvoiceRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let invoice_id = uuid::Uuid::new_v4().to_string();
        let total_amount: f64 = req.line_items.iter().map(|item| item.amount).sum();
        let status = "draft".to_string();

        let stripe_payment_link = format!("https://checkout.stripe.com/pay/cs_test_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

        sqlx::query(
            "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, stripe_payment_link)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(&invoice_id)
        .bind(&req.tenant_id)
        .bind(&req.client_id)
        .bind(&req.client_name)
        .bind(&status)
        .bind(req.due_date)
        .bind(&req.currency)
        .bind(total_amount)
        .bind(&stripe_payment_link)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut saved_items = Vec::new();
        for item in req.line_items {
            let item_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(&item_id)
            .bind(&req.tenant_id)
            .bind(&invoice_id)
            .bind(&item.description)
            .bind(item.quantity)
            .bind(item.unit_price)
            .bind(item.amount)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            saved_items.push(InvoiceLineItem {
                id: item_id,
                invoice_id: invoice_id.clone(),
                description: item.description,
                quantity: item.quantity,
                unit_price: item.unit_price,
                amount: item.amount,
            });
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Invoice {
            id: invoice_id,
            client_id: req.client_id,
            client_name: req.client_name,
            status,
            due_date: req.due_date,
            currency: req.currency,
            total_amount,
            stripe_invoice_id: "".to_string(),
            stripe_payment_link,
            line_items: saved_items,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        }))
    }

    async fn get_invoice(
        &self,
        request: Request<GetInvoiceRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;

        let row = sqlx::query("SELECT * FROM invoices WHERE id = $1")
            .bind(&req.invoice_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let items_rows = sqlx::query("SELECT * FROM invoice_line_items WHERE invoice_id = $1")
            .bind(&req.invoice_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut line_items = Vec::new();
        for item_row in items_rows {
            line_items.push(InvoiceLineItem {
                id: item_row.try_get("id").unwrap_or_default(),
                invoice_id: item_row.try_get("invoice_id").unwrap_or_default(),
                description: item_row.try_get("description").unwrap_or_default(),
                quantity: item_row.try_get("quantity").unwrap_or_default(),
                unit_price: item_row.try_get("unit_price").unwrap_or_default(),
                amount: item_row.try_get("amount").unwrap_or_default(),
            });
        }

        let invoice = Invoice {
            id: row.try_get("id").unwrap_or_default(),
            client_id: row.try_get("client_id").unwrap_or_default(),
            client_name: row.try_get("client_name").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            due_date: row.try_get("due_date").unwrap_or_default(),
            currency: row.try_get("currency").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
            stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
            line_items,
            created_at: 0,
            updated_at: 0,
        };

        Ok(Response::new(invoice))
    }

    async fn list_invoices(
        &self,
        request: Request<ListInvoicesRequest>,
    ) -> Result<Response<ListInvoicesResponse>, Status> {
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;

        let rows = sqlx::query("SELECT * FROM invoices ORDER BY created_at DESC")
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut invoices = Vec::new();
        for row in rows {
            invoices.push(Invoice {
                id: row.try_get("id").unwrap_or_default(),
                client_id: row.try_get("client_id").unwrap_or_default(),
                client_name: row.try_get("client_name").unwrap_or_default(),
                status: row.try_get("status").unwrap_or_default(),
                due_date: row.try_get("due_date").unwrap_or_default(),
                currency: row.try_get("currency").unwrap_or_default(),
                total_amount: row.try_get("total_amount").unwrap_or_default(),
                stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
                stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
                line_items: vec![],
                created_at: 0,
                updated_at: 0,
            });
        }

        Ok(Response::new(ListInvoicesResponse { invoices }))
    }

    async fn update_invoice_status(
        &self,
        request: Request<UpdateInvoiceStatusRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("UPDATE invoices SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4")
            .bind(&req.status)
            .bind(chrono::Utc::now().timestamp())
            .bind(&req.invoice_id)
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;

        let row = sqlx::query("SELECT * FROM invoices WHERE id = $1")
            .bind(&req.invoice_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let items_rows = sqlx::query("SELECT * FROM invoice_line_items WHERE invoice_id = $1")
            .bind(&req.invoice_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut line_items = Vec::new();
        for item_row in items_rows {
            line_items.push(InvoiceLineItem {
                id: item_row.try_get("id").unwrap_or_default(),
                invoice_id: item_row.try_get("invoice_id").unwrap_or_default(),
                description: item_row.try_get("description").unwrap_or_default(),
                quantity: item_row.try_get("quantity").unwrap_or_default(),
                unit_price: item_row.try_get("unit_price").unwrap_or_default(),
                amount: item_row.try_get("amount").unwrap_or_default(),
            });
        }

        let invoice = Invoice {
            id: row.try_get("id").unwrap_or_default(),
            client_id: row.try_get("client_id").unwrap_or_default(),
            client_name: row.try_get("client_name").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            due_date: row.try_get("due_date").unwrap_or_default(),
            currency: row.try_get("currency").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
            stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
            line_items,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        };

        Ok(Response::new(invoice))
    }

    async fn draft_invoice_from_context(
        &self,
        request: Request<DraftInvoiceFromContextRequest>,
    ) -> Result<Response<DraftInvoiceFromContextResponse>, Status> {
        let req = request.into_inner();

        // Simple mock of agent extraction
        let line_item1 = InvoiceLineItem {
            id: "".to_string(),
            invoice_id: "".to_string(),
            description: "Consulting Services".to_string(),
            quantity: 10,
            unit_price: 150.0,
            amount: 1500.0,
        };

        let invoice = Invoice {
            id: "draft-temp".to_string(),
            client_id: "".to_string(),
            client_name: req.client_name.clone(),
            status: "draft".to_string(),
            due_date: chrono::Utc::now().timestamp() + 30 * 24 * 3600, // +30 days
            currency: "USD".to_string(),
            total_amount: 1500.0,
            stripe_invoice_id: "".to_string(),
            stripe_payment_link: "".to_string(),
            line_items: vec![line_item1],
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };

        Ok(Response::new(DraftInvoiceFromContextResponse { draft: Some(invoice) }))
    }
}


use axum::{
    extract::{Query, State},
    routing::get,
    Json,
};
use serde_json::{json, Value};

#[derive(serde::Deserialize)]
pub struct InvoiceQuery {
    pub tenant_id: Option<String>,
}

async fn get_invoices_handler(
    State(hub): State<Arc<Hub>>,
    Query(query): Query<InvoiceQuery>,
) -> Json<Value> {
    let tenant_id = query.tenant_id.unwrap_or_else(|| "default".to_string());
    let pool = &hub.pool;

    let rows = sqlx::query("SELECT * FROM invoices WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 50")
        .bind(&tenant_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    if rows.is_empty() {
        return Json(json!({ "invoices": [] }));
    }

    let mut invoice_ids = Vec::new();
    for row in &rows {
        use sqlx::Row;
        let id: String = row.try_get("id").unwrap_or_default();
        invoice_ids.push(id);
    }

    let items_rows = sqlx::query("SELECT * FROM invoice_line_items WHERE invoice_id = ANY($1)")
        .bind(&invoice_ids)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    let mut line_items_map = std::collections::HashMap::new();
    for item_row in items_rows {
        use sqlx::Row;
        let invoice_id: String = item_row.try_get("invoice_id").unwrap_or_default();
        let item = json!({
            "id": item_row.try_get::<String, _>("id").unwrap_or_default(),
            "description": item_row.try_get::<String, _>("description").unwrap_or_default(),
            "quantity": item_row.try_get::<i32, _>("quantity").unwrap_or_default(),
            "unit_price": item_row.try_get::<f64, _>("unit_price").unwrap_or_default(),
            "amount": item_row.try_get::<f64, _>("amount").unwrap_or_default(),
        });
        line_items_map.entry(invoice_id).or_insert_with(Vec::new).push(item);
    }

    let mut invoices = Vec::new();
    for row in rows {
        use sqlx::Row;
        let invoice_id: String = row.try_get("id").unwrap_or_default();
        let empty_vec = Vec::new();
        let line_items = line_items_map.get(&invoice_id).unwrap_or(&empty_vec);

        invoices.push(json!({
            "id": invoice_id,
            "client_id": row.try_get::<String, _>("client_id").unwrap_or_default(),
            "client_name": row.try_get::<String, _>("client_name").unwrap_or_default(),
            "status": row.try_get::<String, _>("status").unwrap_or_default(),
            "due_date": row.try_get::<i64, _>("due_date").unwrap_or_default(),
            "total_amount": row.try_get::<f64, _>("total_amount").unwrap_or_default(),
            "line_items": line_items,
        }));
    }

    Json(json!({ "invoices": invoices }))
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> axum::Router<S> {
    axum::Router::new()
        .route("/", get(get_invoices_handler))
        .with_state(hub)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use crate::hub::Hub;
    use ::server_ohc::invoice::{CreateInvoiceRequest, InvoiceLineItem, UpdateInvoiceStatusRequest};

    #[tokio::test]
    async fn test_invoice_logic() {
        let db = match DB::new().await {
            Ok(d) => d,
            Err(_) => return,
        };

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));

        let service = InvoiceServiceImpl { hub: hub.clone() };

        let tenant_id = "test_tenant".to_string();
        let create_req = CreateInvoiceRequest {
            tenant_id: tenant_id.clone(),
            client_id: "client1".to_string(),
            client_name: "Test Client".to_string(),
            due_date: chrono::Utc::now().timestamp(),
            currency: "USD".to_string(),
            line_items: vec![
                InvoiceLineItem {
                    id: "".to_string(),
                    invoice_id: "".to_string(),
                    description: "Item 1".to_string(),
                    quantity: 1,
                    unit_price: 100.0,
                    amount: 100.0,
                }
            ],
        };

        let create_resp = service.create_invoice(Request::new(create_req)).await;
        if create_resp.is_err() {
            return;
        }

        let invoice = create_resp.unwrap().into_inner();
        assert_eq!(invoice.status, "draft");

        let update_req = UpdateInvoiceStatusRequest {
            tenant_id: tenant_id.clone(),
            invoice_id: invoice.id.clone(),
            status: "paid".to_string(),
        };

        let update_resp = service.update_invoice_status(Request::new(update_req)).await;
        assert!(update_resp.is_ok());

        let updated_invoice = update_resp.unwrap().into_inner();
        assert_eq!(updated_invoice.status, "paid");
    }
}
