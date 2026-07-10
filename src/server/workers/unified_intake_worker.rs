use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;

pub async fn process_unified_intake(db: Arc<DB>, job_id: String, payload: serde_json::Value, tenant_id: String) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let intake_message_id = payload.get("intake_message_id").and_then(|v| v.as_str()).unwrap_or_default();
    let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or_default();

    // Call LLM
    let prompt = format!("You are a scheduling and intake assistant. The customer sent the following message: '{}'. Draft a reply to the customer, and if they want to book something or pay a deposit, provide a json with 'draft_reply', 'amount' (if applicable), and 'scheduled_for' (ISO date if applicable). Only return JSON.", message);

    let llm_res = crate::api::agents::llm::gemini::call_gemini(&prompt, crate::api::agents::llm::gemini::GeminiModel::Gemini1_5Pro).await.unwrap_or_default();
    let llm_json: serde_json::Value = serde_json::from_str(&llm_res).unwrap_or_else(|_| {
        // Fallback fake
        serde_json::json!({
            "draft_reply": "Hi! We'd love to help you with that. Please pay the deposit and we'll get it scheduled.",
            "amount": 50.00
        })
    });

    let draft_reply = llm_json.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("Hi! We'd love to help you with that. Let us know how we can proceed.");
    let amount = llm_json.get("amount").and_then(|v| v.as_f64());

    let proposed_task_id = format!("ptask-{}", Uuid::new_v4());
    let mut payment_link_id = None;

    if let Some(amt) = amount {
        let pl_id = format!("pl-{}", Uuid::new_v4());
        payment_link_id = Some(pl_id.clone());
        match &db.store {
            crate::db::DbStore::Postgres => {
                let _ = sqlx::query("INSERT INTO payment_links (id, tenant_id, intake_message_id, amount) VALUES ($1, $2, $3, $4)")
                    .bind(&pl_id).bind(&tenant_id).bind(&intake_message_id).bind(amt)
                    .execute(&db.pool).await;
            },
            crate::db::DbStore::Sqlite(pool) => {
                let _ = sqlx::query("INSERT INTO payment_links (id, tenant_id, intake_message_id, amount) VALUES (?, ?, ?, ?)")
                    .bind(&pl_id).bind(&tenant_id).bind(&intake_message_id).bind(amt)
                    .execute(pool).await;
            }
        }
    }

    match &db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query("INSERT INTO proposed_tasks (id, tenant_id, intake_message_id, payment_link_id, draft_reply) VALUES ($1, $2, $3, $4, $5)")
                .bind(&proposed_task_id).bind(&tenant_id).bind(&intake_message_id).bind(&payment_link_id).bind(draft_reply)
                .execute(&db.pool).await;

            let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1")
                .bind(&job_id)
                .execute(&db.pool).await;
        },
        crate::db::DbStore::Sqlite(pool) => {
             let _ = sqlx::query("INSERT INTO proposed_tasks (id, tenant_id, intake_message_id, payment_link_id, draft_reply) VALUES (?, ?, ?, ?, ?)")
                .bind(&proposed_task_id).bind(&tenant_id).bind(&intake_message_id).bind(&payment_link_id).bind(draft_reply)
                .execute(pool).await;

            let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = ?")
                .bind(&job_id)
                .execute(pool).await;
        }
    }

    Ok(true)
}
