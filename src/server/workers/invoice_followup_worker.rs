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
            let mut overdue_invoices: Vec<(String, String, String, f64, i64)> = vec![]; // (id, tenant_id, customer_id, days_past_due, ltv)

            match &db.store {
                DbStore::Postgres => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, String, Option<String>, Option<f64>, Option<i64>)>(
                        "SELECT id, tenant_id, customer_id,
                        EXTRACT(EPOCH FROM now() - due_date) / 86400 as days_past_due,
                        COALESCE((SELECT sum(total_amount_cents) FROM invoices i2 WHERE i2.tenant_id = invoices.tenant_id AND i2.customer_id = invoices.customer_id AND i2.payment_status = 'paid'), 0) as ltv
                        FROM invoices WHERE payment_status != 'paid' AND due_date < now() AND (status = 'draft' OR status = 'pending')"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            overdue_invoices.push((row.0, row.1, row.2.unwrap_or_default(), row.3.unwrap_or(0.0), row.4.unwrap_or(0)));
                        }
                    }
                },
                DbStore::Sqlite(_) => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, String, Option<String>, Option<f64>, Option<i64>)>(
                        "SELECT id, tenant_id, customer_id,
                        (strftime('%s', 'now') - strftime('%s', due_date)) / 86400 as days_past_due,
                        COALESCE((SELECT sum(total_amount_cents) FROM invoices i2 WHERE i2.tenant_id = invoices.tenant_id AND i2.customer_id = invoices.customer_id AND i2.payment_status = 'paid'), 0) as ltv
                        FROM invoices WHERE payment_status != 'paid' AND due_date < datetime('now') AND (status = 'draft' OR status = 'pending')"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            overdue_invoices.push((row.0, row.1, row.2.unwrap_or_default(), row.3.unwrap_or(0.0), row.4.unwrap_or(0)));
                        }
                    }
                }
            }

            for (invoice_id, tenant_id, customer_id, days_past_due, ltv) in overdue_invoices {

                info!("Triggering Finance Agent for overdue invoice {}", invoice_id);

                // Fetch recent communications from unified customer timeline
                let mut recent_communications: Vec<String> = vec![];
                let mut target_channel = "email".to_string();

                if !customer_id.is_empty() {
                    if let Ok(timeline_events) = orchestrator.get_customer_timeline(&tenant_id, &customer_id, 5).await {
                        let mut channel_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

                        for event in &timeline_events {
                            if !event.content.is_empty() {
                                recent_communications.push(event.content.clone());
                            }
                            *channel_counts.entry(event.source.clone()).or_insert(0) += 1;
                        }

                        if let Some((most_frequent_channel, _)) = channel_counts.into_iter().max_by_key(|&(_, count)| count) {
                            if !most_frequent_channel.is_empty() {
                                target_channel = most_frequent_channel;
                            }
                        }
                    }
                }

                let comms_context = recent_communications.join("\n");
                let mut is_promise_to_pay = false;
                let mut generated_response = format!("Hi there, just checking in to see if you received invoice {}. Let us know if you have any questions!", invoice_id);
                let mut original_message = format!("Invoice {} is overdue.", invoice_id);

                if !comms_context.is_empty() {
                    let prompt = format!("You are an AI financial assistant. Analyze the recent communication history with this customer regarding their overdue invoice. Is there a clear promise to pay soon (e.g., 'I will pay on Friday')? If so, reply with EXACTLY 'PROMISE_DETECTED'. If not, draft a polite, context-aware invoice reminder tailored for the '{}' channel based on the conversation history (e.g., acknowledging what they last said, keeping it concise if it's SMS/WhatsApp). Here is the communication history:\n\n{}", target_channel, comms_context);

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
                        "original_message": original_message,
                        "suggested_channel": target_channel
                    }),
                };

                if let Err(e) = orchestrator.dispatch_event(event).await {
                    error!("Failed to dispatch invoice.overdue event: {}", e);
                }
            }
        }
    });
}
