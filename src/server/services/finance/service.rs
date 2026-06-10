use ::server_finance::finance_service_server::FinanceService;
use ::server_finance::{
    CreateInvoiceRequest, CreateInvoiceResponse, Expense, GetInvoiceRequest, GetInvoiceResponse,
    Invoice, InvoiceLineItem, ProcessExpenseRequest, ProcessExpenseResponse, UploadReceiptRequest,
    UploadReceiptResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MyFinanceService {}

impl MyFinanceService {
    pub fn new(_db: Arc<crate::db::DB>) -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl FinanceService for MyFinanceService {
    async fn create_invoice(
        &self,
        request: Request<CreateInvoiceRequest>,
    ) -> Result<Response<CreateInvoiceResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let invoice_id = Uuid::new_v4().to_string();
        let pool = crate::db::get_pool();

        let mut db_tx = pool.begin().await.map_err(|e| Status::internal(format!("db error: {}", e)))?;

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
            return Err(Status::internal(format!("Failed to set org context: {}", e)));
        }

        let mut total_amount_cents = 0;
        for item in &req.line_items {
            total_amount_cents += item.quantity as i64 * item.unit_price_cents;
        }

        let due_date = if req.due_date.is_empty() {
            None
        } else {
            chrono::NaiveDate::parse_from_str(&req.due_date, "%Y-%m-%d").ok()
        };

        let insert_invoice_res = sqlx::query(
            "INSERT INTO invoices (id, tenant_id, client_id, currency, total_amount_cents, due_date)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&invoice_id)
        .bind(&tenant_id)
        .bind(&req.client_id)
        .bind(&req.currency)
        .bind(total_amount_cents)
        .bind(due_date)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = insert_invoice_res {
            return Ok(Response::new(CreateInvoiceResponse {
                invoice: None,
                success: false,
                error_message: e.to_string(),
            }));
        }

        let mut inserted_line_items = Vec::new();
        for item in req.line_items {
            let item_id = Uuid::new_v4().to_string();
            let insert_item_res = sqlx::query(
                "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price_cents)
                 VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&item_id)
            .bind(&tenant_id)
            .bind(&invoice_id)
            .bind(&item.description)
            .bind(item.quantity)
            .bind(item.unit_price_cents)
            .execute(&mut *db_tx)
            .await;

            if let Err(e) = insert_item_res {
                return Ok(Response::new(CreateInvoiceResponse {
                    invoice: None,
                    success: false,
                    error_message: e.to_string(),
                }));
            }
            inserted_line_items.push(InvoiceLineItem {
                id: item_id,
                description: item.description,
                quantity: item.quantity,
                unit_price_cents: item.unit_price_cents,
            });
        }

        let ledger_state_change = serde_json::json!({
            "invoice_id": invoice_id,
            "total_amount_cents": total_amount_cents,
            "currency": req.currency
        });

        let insert_ledger_res = sqlx::query(
            "INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change)
             VALUES ($1, $2, 'FINANCE', 'INVOICE_CREATED', $3::jsonb)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .bind(ledger_state_change)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = insert_ledger_res {
             return Ok(Response::new(CreateInvoiceResponse {
                invoice: None,
                success: false,
                error_message: format!("Ledger error: {}", e),
            }));
        }

        db_tx.commit().await.map_err(|e| Status::internal(format!("commit error: {}", e)))?;

        let invoice = Invoice {
            id: invoice_id,
            tenant_id,
            client_id: req.client_id,
            status: "DRAFT".to_string(),
            currency: req.currency,
            total_amount_cents,
            due_date: req.due_date,
            stripe_payment_link: "".to_string(),
            line_items: inserted_line_items,
        };

        Ok(Response::new(CreateInvoiceResponse {
            invoice: Some(invoice),
            success: true,
            error_message: "".to_string(),
        }))
    }

    async fn get_invoice(
        &self,
        request: Request<GetInvoiceRequest>,
    ) -> Result<Response<GetInvoiceResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let pool = crate::db::get_pool();

        let invoice_row = sqlx::query(
            "SELECT * FROM invoices WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&req.invoice_id)
        .bind(&tenant_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| Status::internal(format!("db error: {}", e)))?;

        if let Some(row) = invoice_row {
            let due_date_opt: Option<chrono::NaiveDate> = sqlx::Row::get(&row, "due_date");
            let mut invoice = Invoice {
                id: sqlx::Row::get(&row, "id"),
                tenant_id: sqlx::Row::get(&row, "tenant_id"),
                client_id: sqlx::Row::get(&row, "client_id"),
                status: sqlx::Row::get(&row, "status"),
                currency: sqlx::Row::get(&row, "currency"),
                total_amount_cents: sqlx::Row::get(&row, "total_amount_cents"),
                due_date: due_date_opt.map(|d| d.to_string()).unwrap_or_default(),
                stripe_payment_link: sqlx::Row::get::<Option<String>, _>(&row, "stripe_payment_link").unwrap_or_default(),
                line_items: vec![],
            };

            let items_rows = sqlx::query(
                "SELECT * FROM invoice_line_items WHERE invoice_id = $1 AND tenant_id = $2"
            )
            .bind(&req.invoice_id)
            .bind(&tenant_id)
            .fetch_all(&pool)
            .await
            .map_err(|e| Status::internal(format!("db error: {}", e)))?;

            for item_row in items_rows {
                invoice.line_items.push(InvoiceLineItem {
                    id: sqlx::Row::get(&item_row, "id"),
                    description: sqlx::Row::get(&item_row, "description"),
                    quantity: sqlx::Row::get(&item_row, "quantity"),
                    unit_price_cents: sqlx::Row::get(&item_row, "unit_price_cents"),
                });
            }

            Ok(Response::new(GetInvoiceResponse {
                invoice: Some(invoice),
                success: true,
                error_message: "".to_string(),
            }))
        } else {
            Ok(Response::new(GetInvoiceResponse {
                invoice: None,
                success: false,
                error_message: "Invoice not found".to_string(),
            }))
        }
    }

    async fn upload_receipt(
        &self,
        request: Request<UploadReceiptRequest>,
    ) -> Result<Response<UploadReceiptResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let expense_id = Uuid::new_v4().to_string();

        let pool = crate::db::get_pool();
        let mut db_tx = pool.begin().await.map_err(|e| Status::internal(format!("db error: {}", e)))?;
        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
            return Err(Status::internal(format!("Failed to set org context: {}", e)));
        }

        use base64::{Engine as _, engine::general_purpose};
        let encoded_image = general_purpose::STANDARD.encode(&req.image_data);

        let job_payload = serde_json::json!({
            "expense_id": expense_id,
            "mime_type": req.mime_type,
            "image_data": encoded_image
        });

        let insert_job_res = sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload)
             VALUES ($1, $2, 'FINANCE_PROCESS_RECEIPT', $3::jsonb)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&tenant_id)
        .bind(&job_payload)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = insert_job_res {
            return Ok(Response::new(UploadReceiptResponse {
                expense: None,
                success: false,
                error_message: format!("Failed to queue processing job: {}", e),
            }));
        }

        let insert_exp_res = sqlx::query(
            "INSERT INTO expenses (id, tenant_id, amount_cents, currency, merchant, category, expense_date, status)
             VALUES ($1, $2, 0, 'USD', 'Pending OCR', 'Uncategorized', CURRENT_DATE, 'PENDING_REVIEW')"
        )
        .bind(&expense_id)
        .bind(&tenant_id)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = insert_exp_res {
             return Ok(Response::new(UploadReceiptResponse {
                expense: None,
                success: false,
                error_message: format!("Failed to create expense record: {}", e),
            }));
        }

        db_tx.commit().await.map_err(|e| Status::internal(format!("commit error: {}", e)))?;

        let expense = Expense {
            id: expense_id,
            tenant_id,
            amount_cents: 0,
            currency: "USD".to_string(),
            merchant: "Pending OCR".to_string(),
            category: "Uncategorized".to_string(),
            expense_date: "".to_string(),
            receipt_image_url: "".to_string(),
            project_id: "".to_string(),
            status: "PENDING_REVIEW".to_string(),
        };

        Ok(Response::new(UploadReceiptResponse {
            expense: Some(expense),
            success: true,
            error_message: "".to_string(),
        }))
    }

    async fn process_expense(
        &self,
        request: Request<ProcessExpenseRequest>,
    ) -> Result<Response<ProcessExpenseResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let req = request.into_inner();
        let pool = crate::db::get_pool();
        let mut db_tx = pool.begin().await.map_err(|e| Status::internal(format!("db error: {}", e)))?;

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *db_tx, &tenant_id).await {
            return Err(Status::internal(format!("Failed to set org context: {}", e)));
        }

        let new_status = if req.approved { "APPROVED" } else { "REJECTED" };

        let update_res = sqlx::query(
            "UPDATE expenses SET status = $1, project_id = $2 WHERE id = $3 AND tenant_id = $4"
        )
        .bind(new_status)
        .bind(if req.project_id.is_empty() { None } else { Some(&req.project_id) })
        .bind(&req.expense_id)
        .bind(&tenant_id)
        .execute(&mut *db_tx)
        .await;

        if let Err(e) = update_res {
             return Ok(Response::new(ProcessExpenseResponse {
                success: false,
                error_message: e.to_string(),
            }));
        }

        if req.approved {
            let expense_row = sqlx::query("SELECT amount_cents, currency, merchant FROM expenses WHERE id = $1")
                .bind(&req.expense_id)
                .fetch_one(&mut *db_tx)
                .await;

            if let Ok(row) = expense_row {
                let amount_cents: i64 = sqlx::Row::get(&row, "amount_cents");
                let currency: String = sqlx::Row::get(&row, "currency");

                let ledger_state_change = serde_json::json!({
                    "expense_id": req.expense_id,
                    "amount_cents": amount_cents,
                    "currency": currency,
                    "project_id": req.project_id
                });

                let insert_ledger_res = sqlx::query(
                    "INSERT INTO ohc_universal_ledger (id, tenant_id, department, action_type, state_change)
                     VALUES ($1, $2, 'FINANCE', 'EXPENSE_APPROVED', $3::jsonb)"
                )
                .bind(Uuid::new_v4().to_string())
                .bind(&tenant_id)
                .bind(ledger_state_change)
                .execute(&mut *db_tx)
                .await;

                if let Err(e) = insert_ledger_res {
                     return Ok(Response::new(ProcessExpenseResponse {
                        success: false,
                        error_message: format!("Ledger error: {}", e),
                    }));
                }
            }
        }

        db_tx.commit().await.map_err(|e| Status::internal(format!("commit error: {}", e)))?;

        Ok(Response::new(ProcessExpenseResponse {
            success: true,
            error_message: "".to_string(),
        }))
    }
}
