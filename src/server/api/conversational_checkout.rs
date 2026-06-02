use axum::{
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateCheckoutSessionRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub deposit_amount: f64,
    pub inventory_lock_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateCheckoutSessionResponse {
    pub success: bool,
    pub session_id: String,
    pub payment_url: String,
}

pub async fn create_checkout_session(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    Json(payload): Json<CreateCheckoutSessionRequest>,
) -> impl IntoResponse {
    if payload.tenant_id.is_empty() || payload.customer_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(CreateCheckoutSessionResponse {
                success: false,
                session_id: "".to_string(),
                payment_url: "".to_string(),
            }),
        );
    }

    if payload.deposit_amount <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(CreateCheckoutSessionResponse {
                success: false,
                session_id: "".to_string(),
                payment_url: "".to_string(),
            }),
        );
    }

    let session_id = uuid::Uuid::new_v4().to_string();
    let payment_url = format!("https://checkout.ohc.network/pay/{}", session_id);

    // Persist to DB using query builder to bypass compile-time macro checks
    let result = sqlx::query(
        "INSERT INTO conversational_checkout_session (id, tenant_id, customer_id, status, deposit_amount, inventory_lock_id) VALUES ($1, $2, $3, 'PENDING', $4, $5)"
    )
    .bind(&session_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(&payload.deposit_amount)
    .bind(&payload.inventory_lock_id)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => {
            (
                StatusCode::OK,
                Json(CreateCheckoutSessionResponse {
                    success: true,
                    session_id,
                    payment_url,
                }),
            )
        }
        Err(e) => {
            eprintln!("Error persisting checkout session: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateCheckoutSessionResponse {
                    success: false,
                    session_id: "".to_string(),
                    payment_url: "".to_string(),
                }),
            )
        }
    }
}

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/api/v1/checkout/conversational", post(create_checkout_session))
        .with_state(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;
    use sqlx::PgPool;

    async fn setup_db() -> PgPool {
        let database_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(&database_url)
            .expect("Failed to connect to DB pool lazily")
    }

    #[tokio::test]
    async fn test_create_checkout_session_missing_tenant() {
        let pool = setup_db().await;
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/checkout/conversational")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"tenant_id": "", "customer_id": "cust1", "deposit_amount": 50.0}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_checkout_session_invalid_amount() {
        let pool = setup_db().await;
        let app = router(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/checkout/conversational")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"tenant_id": "tenant1", "customer_id": "cust1", "deposit_amount": 0.0}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
