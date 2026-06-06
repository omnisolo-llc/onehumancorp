use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::collective::{Collective, CollectiveLoyaltyBalance, CollectiveMember};
use sqlx::PgPool;

#[derive(Clone)]
pub struct CollectiveAppState {
    pub db: PgPool,
}

pub fn router(state: CollectiveAppState) -> Router {
    Router::new()
        .route("/api/v1/collective/nearby", get(get_nearby_collectives))
        .route("/api/v1/collective", post(create_collective))
        .route("/api/v1/collective/:id/join", post(join_collective))
        .route("/api/v1/collective/:id", get(get_collective))
        .route("/api/v1/collective/loyalty/earn", post(earn_loyalty_points))
        .route("/api/v1/collective/loyalty/redeem", post(redeem_loyalty_points))
        .with_state(state)
}

#[derive(Deserialize, Serialize)]
pub struct CreateCollectiveRequest {
    pub name: String,
    pub location_center: Option<String>,
    pub radius_meters: Option<f64>,
    pub initial_members: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct CreateCollectiveResponse {
    pub collective: Collective,
}

async fn get_nearby_collectives(
    State(_state): State<CollectiveAppState>,
) -> Result<Json<Vec<Collective>>, axum::http::StatusCode> {
    // Requires PostGIS or complex location logic, returning empty for safety
    // Note: To fully implement real location search, we'd need Geohash or PostGIS extension enabled
    Ok(Json(vec![]))
}

async fn create_collective(
    State(state): State<CollectiveAppState>,
    Json(payload): Json<CreateCollectiveRequest>,
) -> Result<Json<CreateCollectiveResponse>, axum::http::StatusCode> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let collective = Collective {
        id,
        name: payload.name,
        location_center: payload.location_center,
        radius_meters: payload.radius_meters,
        created_at: now,
        updated_at: now,
    };

    let res = sqlx::query(
        r#"
        INSERT INTO collectives (id, name, location_center, radius_meters, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(collective.id.to_string())
    .bind(&collective.name)
    .bind(&collective.location_center)
    .bind(collective.radius_meters)
    .bind(collective.created_at.naive_utc())
    .bind(collective.updated_at.naive_utc())
    .execute(&state.db)
    .await;

    match res {
        Ok(_) => Ok(Json(CreateCollectiveResponse { collective })),
        Err(e) => {
            eprintln!("Error creating collective: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct JoinCollectiveRequest {
    pub tenant_id: Uuid,
}

async fn join_collective(
    Path(id): Path<Uuid>,
    State(state): State<CollectiveAppState>,
    Json(payload): Json<JoinCollectiveRequest>,
) -> Result<Json<CollectiveMember>, axum::http::StatusCode> {
    let member_id = Uuid::new_v4();
    let now = chrono::Utc::now();
    let member = CollectiveMember {
        id: member_id,
        collective_id: id,
        tenant_id: payload.tenant_id,
        status: "ACTIVE".to_string(),
        joined_at: Some(now),
        created_at: now,
        updated_at: now,
    };

    let res = sqlx::query(
        r#"
        INSERT INTO collective_members (id, collective_id, tenant_id, status, joined_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(member.id.to_string())
    .bind(member.collective_id.to_string())
    .bind(member.tenant_id.to_string())
    .bind(&member.status)
    .bind(member.joined_at.map(|dt| dt.naive_utc()))
    .bind(member.created_at.naive_utc())
    .bind(member.updated_at.naive_utc())
    .execute(&state.db)
    .await;

    match res {
        Ok(_) => Ok(Json(member)),
        Err(e) => {
            eprintln!("Error joining collective: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_collective(
    Path(id): Path<Uuid>,
    State(state): State<CollectiveAppState>,
) -> Result<Json<Option<Collective>>, axum::http::StatusCode> {
    let res = sqlx::query_as::<_, Collective>(
        r#"
        SELECT
            id::uuid as id,
            name,
            location_center,
            radius_meters,
            created_at,
            updated_at
        FROM collectives
        WHERE id = $1
        "#
    )
    .bind(id.to_string())
    .fetch_optional(&state.db)
    .await;

    match res {
        Ok(collective) => Ok(Json(collective)),
        Err(e) => {
            eprintln!("Error getting collective: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct EarnLoyaltyPointsRequest {
    pub collective_id: Uuid,
    pub customer_id: Uuid,
    pub points: i32,
}

async fn earn_loyalty_points(
    State(state): State<CollectiveAppState>,
    Json(payload): Json<EarnLoyaltyPointsRequest>,
) -> Result<Json<CollectiveLoyaltyBalance>, axum::http::StatusCode> {
    let now = chrono::Utc::now();
    let balance_id = Uuid::new_v4();

    let res = sqlx::query_as::<_, CollectiveLoyaltyBalance>(
        r#"
        INSERT INTO collective_loyalty_balances (id, collective_id, customer_id, points_balance, last_updated)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (collective_id, customer_id)
        DO UPDATE SET points_balance = collective_loyalty_balances.points_balance + EXCLUDED.points_balance,
                      last_updated = EXCLUDED.last_updated
        RETURNING
            id::uuid as id,
            collective_id::uuid as collective_id,
            customer_id::uuid as customer_id,
            points_balance,
            last_updated
        "#
    )
    .bind(balance_id.to_string())
    .bind(payload.collective_id.to_string())
    .bind(payload.customer_id.to_string())
    .bind(payload.points)
    .bind(now.naive_utc())
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(balance) => Ok(Json(balance)),
        Err(e) => {
            eprintln!("Error earning loyalty points: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct RedeemLoyaltyPointsRequest {
    pub collective_id: Uuid,
    pub customer_id: Uuid,
    pub points: i32,
}

async fn redeem_loyalty_points(
    State(state): State<CollectiveAppState>,
    Json(payload): Json<RedeemLoyaltyPointsRequest>,
) -> Result<Json<CollectiveLoyaltyBalance>, axum::http::StatusCode> {
    let now = chrono::Utc::now();

    let res = sqlx::query_as::<_, CollectiveLoyaltyBalance>(
        r#"
        UPDATE collective_loyalty_balances
        SET points_balance = points_balance - $1,
            last_updated = $2
        WHERE collective_id = $3 AND customer_id = $4 AND points_balance >= $1
        RETURNING
            id::uuid as id,
            collective_id::uuid as collective_id,
            customer_id::uuid as customer_id,
            points_balance,
            last_updated
        "#
    )
    .bind(payload.points)
    .bind(now.naive_utc())
    .bind(payload.collective_id.to_string())
    .bind(payload.customer_id.to_string())
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(balance) => Ok(Json(balance)),
        Err(sqlx::Error::RowNotFound) => {
            eprintln!("Insufficient points or balance not found");
            Err(axum::http::StatusCode::BAD_REQUEST)
        }
        Err(e) => {
            eprintln!("Error redeeming loyalty points: {:?}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
