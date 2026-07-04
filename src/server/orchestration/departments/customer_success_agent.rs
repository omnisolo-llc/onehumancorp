use axum::{Json, routing::post, Router, Extension};
use std::sync::Arc;
use tokio_postgres::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ReengagementRequest {
    tenant_id: String,
}

#[derive(Serialize)]
pub struct ReengagementResponse {
    status: String,
    drafts_created: i32,
}

pub async fn run_dormant_user_check(
    Extension(client): Extension<Arc<Client>>,
    Json(payload): Json<ReengagementRequest>,
) -> Json<ReengagementResponse> {

    // Find customers whose last booking was > 30 days ago and have no future bookings
    let query = "
        WITH customer_last_booking AS (
            SELECT customer_id, MAX(end_time) as last_booking
            FROM bookings
            WHERE tenant_id = $1 AND status IN ('completed', 'confirmed')
            GROUP BY customer_id
        ),
        customer_future_booking AS (
            SELECT customer_id
            FROM bookings
            WHERE tenant_id = $1 AND start_time > CURRENT_TIMESTAMP AND status IN ('scheduled', 'confirmed')
            GROUP BY customer_id
        )
        SELECT clb.customer_id, clb.last_booking
        FROM customer_last_booking clb
        LEFT JOIN customer_future_booking cfb ON clb.customer_id = cfb.customer_id
        WHERE cfb.customer_id IS NULL
        AND clb.last_booking < CURRENT_TIMESTAMP - INTERVAL '30 days'
    ";

    let rows = client.query(query, &[&payload.tenant_id]).await.unwrap_or_else(|_| vec![]);

    let mut drafts_created = 0;

    for row in rows {
        let customer_id: String = row.get("customer_id");

        let insert_draft = "
            INSERT INTO shared_tasks_decomposition (
                id, organization_id, feature_type, task_payload, agent_context, status
            ) VALUES (
                gen_random_uuid()::text,
                $1,
                'proactive_outreach',
                $2::jsonb,
                $3::jsonb,
                'PENDING_APPROVAL'
            )
        ";

        let task_payload = serde_json::json!({
            "feature_type": "proactive_outreach",
            "customer_id": customer_id,
            "draft_reply": format!("Hi! We noticed it's been a while since your last session. Would you like to book a follow-up? https://ohc.page/book?tenant={}&customer={}", payload.tenant_id, customer_id),
            "summary": format!("Customer {} has not booked in over 30 days.", customer_id)
        });

        let agent_context = serde_json::json!({
            "title": "Dormant Customer Follow-up",
            "description": format!("Customer {} is dormant. Action required.", customer_id)
        });

        if client.execute(insert_draft, &[&payload.tenant_id, &task_payload, &agent_context]).await.is_ok() {
            drafts_created += 1;
        }
    }

    Json(ReengagementResponse {
        status: "success".to_string(),
        drafts_created,
    })
}
