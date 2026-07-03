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
            let mut overdue_invoices: Vec<(String, String, String)> = vec![]; // (id, tenant_id, customer_id)

            match &db.store {
                DbStore::Postgres => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, String, Option<String>)>(
                        "SELECT id, tenant_id, customer_id FROM invoices WHERE payment_status != 'paid' AND due_date < extract(epoch from now()) AND (status = 'draft' OR status = 'pending')"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            overdue_invoices.push((row.0, row.1, row.2.unwrap_or_default()));
                        }
                    }
                },
                DbStore::Sqlite(_) => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, String, Option<String>)>(
                        "SELECT id, tenant_id, customer_id FROM invoices WHERE payment_status != 'paid' AND due_date < strftime('%s', 'now') AND (status = 'draft' OR status = 'pending')"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            overdue_invoices.push((row.0, row.1, row.2.unwrap_or_default()));
                        }
                    }
                }
            }

            for (invoice_id, tenant_id, customer_id) in overdue_invoices {
                info!("Triggering Finance Agent for overdue invoice {}", invoice_id);

                // Fetch recent communications
                let mut recent_communications: Vec<String> = vec![];
                if !customer_id.is_empty() {
                    match &db.store {
                        DbStore::Postgres => {
                            if let Ok(rows) = sqlx::query_as::<_, (String,)>("SELECT original_content FROM omni_inbox_messages WHERE tenant_id = $1 AND customer_id = $2 ORDER BY created_at DESC LIMIT 5")
                                .bind(&tenant_id).bind(&customer_id).fetch_all(&db.pool).await {
                                for row in rows {
                                    recent_communications.push(row.0);
                                }
                            }
                        },
                        DbStore::Sqlite(_) => {
                            if let Ok(rows) = sqlx::query_as::<_, (String,)>("SELECT original_content FROM omni_inbox_messages WHERE tenant_id = ? AND customer_id = ? ORDER BY created_at DESC LIMIT 5")
                                .bind(&tenant_id).bind(&customer_id).fetch_all(&db.pool).await {
                                for row in rows {
                                    recent_communications.push(row.0);
                                }
                            }
                        }
                    }
                }

                let comms_context = recent_communications.join("\n");
                let mut is_promise_to_pay = false;
                let mut generated_response = format!("Hi there, just checking in to see if you received invoice {}. Let us know if you have any questions!", invoice_id);
                let mut original_message = format!("Invoice {} is overdue.", invoice_id);

                if !comms_context.is_empty() {
                    let prompt = format!("You are an AI financial assistant. Analyze the recent communication history with this customer regarding their overdue invoice. Is there a clear promise to pay soon (e.g., 'I will pay on Friday')? If so, reply with EXACTLY 'PROMISE_DETECTED'. If not, draft a polite, context-aware invoice reminder based on the conversation history (e.g., acknowledging what they last said). Here is the communication history:\n\n{}", comms_context);

                    let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
                        Ok("minimax") => {
                            if let Ok(api_key) = std::env::var("MINIMAX_API_KEY") {
                                crate::minimax::MinimaxClient::new(api_key).reason(&prompt).await
                            } else {
                                crate::minimax::LocalLLMClient::new().reason(&prompt).await
                            }
                        },
                        _ => crate::minimax::LocalLLMClient::new().reason(&prompt).await,
                    };

                    if let Ok(res) = llm_res {
                        if res.contains("PROMISE_DETECTED") {
                            is_promise_to_pay = true;
                        } else {
                            generated_response = res.trim().to_string();
                            original_message = format!("Invoice {} is overdue. Recent contact: {}", invoice_id, recent_communications.first().unwrap_or(&"".to_string()));
                        }
                    }
                }

                if is_promise_to_pay {
                    info!("Promise to pay detected for invoice {}. Updating cash flow prediction and pausing reminder.", invoice_id);
                    continue;
                }

                let event = DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: tenant_id.clone(),
                    event_type: "invoice.overdue".to_string(),
                    payload: serde_json::json!({
                        "invoice_id": invoice_id,
                        "customer_id": customer_id,
                        "generated_response": generated_response,
                        "original_message": original_message
                    }),
                };

                if let Err(e) = orchestrator.dispatch_event(event).await {
                    error!("Failed to dispatch invoice.overdue event: {}", e);
                }
            }
        }
    });
}
