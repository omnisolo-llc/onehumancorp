use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>,
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
    pub mutation_type: Option<String>,
    pub payload: Option<String>,
}

pub struct CRDTOfflineSynchronizer;

impl CRDTOfflineSynchronizer {
    pub async fn process_batch(
        pool: &PgPool,
        tenant_id: &str,
        mutations: &[OfflineMutation],
    ) -> Result<(usize, Vec<String>), String> {
        let mut failed_count = 0;
        let mut failed_transactions = Vec::new();

        for mutation in mutations {
            if mutation.mutation_type.as_deref() == Some("draft_quote") {
                continue;
            }
            let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await {
                tracing::error!("Failed to set org context: {}", e);
                failed_count += 1;
                failed_transactions.push(mutation.transaction_id.clone());
                continue;
            }

            let current_stock_res = sqlx::query("SELECT inventory_count FROM products WHERE id = $1 AND tenant_id = $2 FOR UPDATE")
                .bind(&mutation.product_id)
                .bind(tenant_id)
                .fetch_optional(&mut *tx)
                .await;

            match current_stock_res {
                Ok(Some(row)) => {
                    let stock: i32 = sqlx::Row::get(&row, "inventory_count");
                    let is_conflict = stock < mutation.quantity_deducted;

                    // CRDT Operation: decrement inventory idempotently/commutatively
                    let query = "
                        UPDATE products
                        SET inventory_count = GREATEST(0, inventory_count - $1)
                        WHERE id = $2 AND tenant_id = $3
                    ";

                    let _ = sqlx::query(query)
                        .bind(mutation.quantity_deducted)
                        .bind(&mutation.product_id)
                        .bind(tenant_id)
                        .execute(&mut *tx)
                        .await;

                    // Record successful pos offline transaction
                    let res = sqlx::query(
                        "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, status, amount_cents, currency, payload)
                         VALUES ($1, $2, $3, 'RESOLVED', $4, $5, $6::jsonb)
                         ON CONFLICT DO NOTHING"
                    )
                    .bind(Uuid::new_v4().to_string())
                    .bind(tenant_id)
                    .bind(&mutation.transaction_id)
                    .bind(mutation.amount.unwrap_or(0))
                    .bind(mutation.currency.as_deref().unwrap_or("USD"))
                    .bind(serde_json::to_value(&mutation).unwrap())
                    .execute(&mut *tx)
                    .await;

                    let amount_cents = mutation.amount.unwrap_or(0);
                    let total_amount = (amount_cents as f64) / 100.0;

                    let order_id = uuid::Uuid::new_v4().to_string();
                    let customer_id = None::<String>; // offline mutation doesn't specify customer_id currently

                    let _ = sqlx::query("INSERT INTO orders (id, tenant_id, customer_id, total_amount, status) VALUES ($1, $2, $3, $4, 'completed')")
                        .bind(&order_id).bind(tenant_id).bind(&customer_id).bind(total_amount).execute(&mut *tx).await;

                    let item_id = uuid::Uuid::new_v4().to_string();
                    let _ = sqlx::query("INSERT INTO order_items (id, tenant_id, order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4, $5, $6)")
                        .bind(&item_id).bind(tenant_id).bind(&order_id).bind(&mutation.product_id).bind(mutation.quantity_deducted).bind(total_amount).execute(&mut *tx).await;


                    let new_stock = std::cmp::max(0, stock - mutation.quantity_deducted);

                    if new_stock <= 5 && !is_conflict {
                        let job_id = uuid::Uuid::new_v4().to_string();
                        let job_payload = serde_json::json!({
                            "product_id": mutation.product_id,
                            "remaining_stock": new_stock,
                            "threshold": 5,
                            "message": format!("Stock for product {} has dropped to {}.", mutation.product_id, new_stock)
                        }).to_string();

                        let _ = sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ($1, $2, 'operations', 'LowStockAlert', $3::jsonb, 'PENDING')")
                            .bind(job_id)
                            .bind(tenant_id)
                            .bind(&job_payload)
                            .execute(&mut *tx)
                            .await;

                        let action_request_id = uuid::Uuid::new_v4().to_string();
                        let action_payload = serde_json::json!({
                            "product_id": mutation.product_id,
                            "remaining_stock": new_stock,
                            "suggested_action": "Restock Item"
                        }).to_string();

                        let _ = sqlx::query("INSERT INTO agent_action_requests (id, tenant_id, action_type, status, confidence_score, product_id, payload, created_at, updated_at) VALUES ($1, $2, 'Reorder', 'Pending', 0.95, $3, $4::jsonb, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)")
                            .bind(&action_request_id).bind(tenant_id).bind(&mutation.product_id).bind(&action_payload).execute(&mut *tx).await;
                    }

                    if is_conflict {
                        let ai_task_id = uuid::Uuid::new_v4().to_string();
                        let ai_payload = serde_json::json!({
                            "transaction_id": mutation.transaction_id,
                            "product_id": mutation.product_id,
                            "expected_stock": mutation.quantity_deducted,
                            "actual_stock": stock,
                            "message": format!("Heads up! A pop-up sale overlapped with an online order for {}. Operations has drafted an email to the online customer.", mutation.product_id)
                        }).to_string();

                        let _ = sqlx::query(
                            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
                             VALUES ($1, $2, 'POS_INVENTORY_CONFLICT_RESOLUTION', $3::jsonb, 'PENDING')"
                        )
                        .bind(&ai_task_id)
                        .bind(tenant_id)
                        .bind(&ai_payload)
                        .execute(&mut *tx)
                        .await;
                    }

                    if res.is_ok() {
                        let _ = tx.commit().await;
                        continue;
                    } else {
                        tracing::error!("Failed to record pos transaction: {:?}", res.err());
                    }
                }
                Ok(None) => {
                    tracing::warn!("Product {} not found or unauthorized for tenant {}", mutation.product_id, tenant_id);
                }
                Err(e) => {
                    tracing::error!("Failed to deduct inventory for product {}: {}", mutation.product_id, e);
                }
            }

            // If we reach here, it's a failure
            failed_count += 1;
            failed_transactions.push(mutation.transaction_id.clone());

            let _ = tx.rollback().await;

            // Record failure in pos_offline_transactions
            let _ = sqlx::query(
                "INSERT INTO pos_offline_transactions (id, tenant_id, client_id, status, amount_cents, currency, payload)
                 VALUES ($1, $2, $3, 'FAILED', $4, $5, $6::jsonb)
                 ON CONFLICT DO NOTHING"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(tenant_id)
            .bind(&mutation.transaction_id)
            .bind(mutation.amount.unwrap_or(0))
            .bind(mutation.currency.as_deref().unwrap_or("USD"))
            .bind(serde_json::to_value(&mutation).unwrap())
            .execute(pool)
            .await;

            // Enqueue Operations AI Agent job for graceful failure handling
            let agent_payload = serde_json::json!({
                "workflow": "ohc_business_swarm",
                "task": "Handle offline POS sync failure",
                "context": format!("Transaction {} failed to sync offline due to inventory discrepancy or decline.", mutation.transaction_id),
                "action": "OperationsAgent: generate a plain-language alert for the business owner and draft a follow-up message to the customer regarding the declined offline transaction."
            }).to_string();

            let _ = sqlx::query(
                "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload) VALUES ($1, $2, 'agent_task', $3::jsonb)"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(tenant_id)
            .bind(agent_payload)
            .execute(pool)
            .await;
        }

        Ok((failed_count, failed_transactions))
    }
}
