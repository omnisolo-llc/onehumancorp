use crate::db::DbStore;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, error};

pub async fn start_invoice_followup_worker(db: Arc<crate::db::DB>, orchestrator: Arc<DepartmentOrchestrator>) {
    let mut interval = interval(Duration::from_secs(60 * 60 * 24)); // Run once a day

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            info!("Running invoice followup worker sweep");

            // Find invoices that are overdue (e.g. past due_date, status = 'Draft' or 'Pending')
            // Using different logic for Postgres/Sqlite.
            let mut overdue_invoices: Vec<(String, String, String)> = vec![]; // (id, tenant_id, client_id)

            match &db.store {
                DbStore::Postgres => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, String, Option<String>)>(
                        "SELECT id, tenant_id, client_id FROM invoices WHERE payment_status != 'paid' AND due_date < CURRENT_TIMESTAMP AND (status = 'draft' OR status = 'pending')"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            overdue_invoices.push((row.0, row.1, row.2.unwrap_or_default()));
                        }
                    }
                },
                DbStore::Sqlite(_) => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, String, Option<String>)>(
                        "SELECT id, tenant_id, client_id FROM invoices WHERE payment_status != 'paid' AND due_date < datetime('now') AND (status = 'draft' OR status = 'pending')"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            overdue_invoices.push((row.0, row.1, row.2.unwrap_or_default()));
                        }
                    }
                }
            }

            for (invoice_id, tenant_id, client_id) in overdue_invoices {
                info!("Triggering Finance Agent for overdue invoice {}", invoice_id);

                let event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: tenant_id.clone(),
                    event_type: "invoice.overdue".to_string(),
                    payload: serde_json::json!({
                        "invoice_id": invoice_id,
                        "client_id": client_id
                    }),
                };

                if let Err(e) = orchestrator.dispatch_event(event).await {
                    error!("Failed to dispatch invoice.overdue event: {}", e);
                }
            }
        }
    });
}
