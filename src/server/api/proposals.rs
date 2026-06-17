use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", post(create_proposal))
        .route("/{id}", get(get_proposal))
        .route("/{id}", put(update_proposal))
        .route("/{id}/accept", post(accept_proposal))
}

#[derive(Serialize)]
pub struct ProposalResponse {
    pub proposal: Proposal,
    pub line_items: Vec<ProposalLineItem>,
}

#[derive(Deserialize)]
pub struct ProposalQuery {
    pub mobile_optimized: Option<bool>,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct Proposal {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<Uuid>,
    pub status: String,
    pub total_amount_cents: i64,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, sqlx::FromRow, Clone)]
pub struct ProposalLineItem {
    pub id: String,
    pub proposal_id: String,
    pub description: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct CreateProposalRequest {
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub total_amount_cents: i64,
    pub line_items: Vec<ProposalLineItemRequest>,
}

#[derive(Deserialize)]
pub struct UpdateProposalRequest {
    pub total_amount_cents: Option<i64>,
    pub status: Option<String>,
    pub line_items: Vec<ProposalLineItemRequest>,
}

#[derive(Deserialize)]
pub struct ProposalLineItemRequest {
    pub description: String,
    pub quantity: i32,
    pub unit_price_cents: i64,
}

async fn create_proposal(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProposalRequest>,
) -> impl IntoResponse {
    let proposal_id = Uuid::new_v4().to_string();
    let customer_uuid = payload.customer_id.as_ref().and_then(|id| Uuid::parse_str(id).ok());

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let res = sqlx::query(
        "INSERT INTO proposals (id, tenant_id, customer_id, status, total_amount_cents) VALUES ($1, $2, $3, 'draft', $4)"
    )
    .bind(&proposal_id)
    .bind(&payload.tenant_id)
    .bind(customer_uuid)
    .bind(payload.total_amount_cents)
    .execute(&mut *tx)
    .await;

    if let Err(e) = res {
        tracing::error!("Failed to insert proposal: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4().to_string();
        let res = sqlx::query(
            "INSERT INTO proposal_line_items (id, proposal_id, description, quantity, unit_price_cents) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&item_id)
        .bind(&proposal_id)
        .bind(&item.description)
        .bind(item.quantity)
        .bind(item.unit_price_cents)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert proposal line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::CREATED, Json(serde_json::json!({"id": proposal_id}))).into_response()
}

async fn update_proposal(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateProposalRequest>,
) -> impl IntoResponse {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let update_res = sqlx::query(
        "UPDATE proposals SET updated_at = NOW(), total_amount_cents = COALESCE($1, total_amount_cents), status = COALESCE($2, status) WHERE id = $3"
    )
    .bind(payload.total_amount_cents)
    .bind(&payload.status)
    .bind(&id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = update_res {
        tracing::error!("Failed to update proposal: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let delete_res = sqlx::query("DELETE FROM proposal_line_items WHERE proposal_id = $1")
        .bind(&id)
        .execute(&mut *tx)
        .await;

    if let Err(e) = delete_res {
        tracing::error!("Failed to delete old proposal line items: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4().to_string();
        let res = sqlx::query(
            "INSERT INTO proposal_line_items (id, proposal_id, description, quantity, unit_price_cents) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&item_id)
        .bind(&id)
        .bind(&item.description)
        .bind(item.quantity)
        .bind(item.unit_price_cents)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert new proposal line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

async fn get_proposal(
    State(pool): State<PgPool>,
    Query(query): Query<ProposalQuery>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let (proposal_res, items_res) = tokio::join!(
        sqlx::query_as::<_, Proposal>("SELECT * FROM proposals WHERE id = $1")
            .bind(&id)
            .fetch_optional(&pool),
        sqlx::query_as::<_, ProposalLineItem>("SELECT * FROM proposal_line_items WHERE proposal_id = $1")
            .bind(&id)
            .fetch_all(&pool)
    );

    let proposal = match proposal_res {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch proposal: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut line_items = match items_res {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch proposal line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if mobile_optimized {
        let mut q = proposal;
        q.created_at = None;
        q.updated_at = None;
        q.expires_at = None;

        for item in &mut line_items {
            item.created_at = None;
            item.updated_at = None;
        }

        (StatusCode::OK, Json(ProposalResponse { proposal: q, line_items })).into_response()
    } else {
        (StatusCode::OK, Json(ProposalResponse { proposal, line_items })).into_response()
    }
}

async fn accept_proposal(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match sqlx::query("UPDATE proposals SET status = 'accepted', updated_at = NOW() WHERE id = $1")
        .bind(&id)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to accept proposal: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response()
        }
    }
}
