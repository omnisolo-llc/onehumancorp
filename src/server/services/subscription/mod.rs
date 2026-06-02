use tonic::{Request, Response, Status};
use subscription_proto::ohc::subscription::{
    subscription_service_server::SubscriptionService,
    CreatePlanRequest, CreatePlanResponse,
    SubscribeRequest, SubscribeResponse,
    GetFulfillmentBatchesRequest, GetFulfillmentBatchesResponse,
    SubscriptionPlan, Subscriber, FulfillmentBatch,
};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;
use sqlx::Row;

// Mock Stripe integration to avoid circular dependency
pub struct MockStripeClient {}
impl MockStripeClient {
    pub async fn create_checkout_session(&self, _price_id: &str, _customer_email: &str, _amount_usd: f64) -> Result<String, String> {
        Ok("https://checkout.stripe.com/mock".to_string())
    }
}

pub struct SubscriptionServiceImpl {
    pub pool: PgPool,
    pub stripe_client: MockStripeClient,
}

impl SubscriptionServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            stripe_client: MockStripeClient {},
        }
    }
}

#[tonic::async_trait]
impl SubscriptionService for SubscriptionServiceImpl {
    async fn create_plan(&self, request: Request<CreatePlanRequest>) -> Result<Response<CreatePlanResponse>, Status> {
        let req = request.into_inner();
        let plan_id = Uuid::new_v4();

        let plan = SubscriptionPlan {
            id: plan_id.to_string(),
            organization_id: req.organization_id.clone(),
            name: req.name.clone(),
            description: req.description.clone(),
            price_cents: req.price_cents,
            billing_interval: req.billing_interval.clone(),
            stripe_product_id: format!("prod_mock_{}", plan_id),
            stripe_price_id: format!("price_mock_{}", plan_id),
            created_at_unix: Utc::now().timestamp(),
        };

        sqlx::query(
            "INSERT INTO subscription_plans (id, organization_id, name, description, price_cents, billing_interval, stripe_product_id, stripe_price_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(plan_id)
        .bind(&req.organization_id)
        .bind(&req.name)
        .bind(&req.description)
        .bind(req.price_cents)
        .bind(&req.billing_interval)
        .bind(&plan.stripe_product_id)
        .bind(&plan.stripe_price_id)
        .execute(&self.pool)
        .await
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        Ok(Response::new(CreatePlanResponse { plan: Some(plan) }))
    }

    async fn subscribe(&self, request: Request<SubscribeRequest>) -> Result<Response<SubscribeResponse>, Status> {
        let req = request.into_inner();
        let subscriber_id = Uuid::new_v4();

        let row = sqlx::query(
            "SELECT price_cents FROM subscription_plans WHERE id = $1 AND organization_id = $2"
        )
        .bind(Uuid::parse_str(&req.subscription_plan_id).unwrap_or_default())
        .bind(&req.organization_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?
        .ok_or_else(|| Status::not_found("Plan not found"))?;

        let price_cents: i64 = row.get("price_cents");

        let checkout_url = self.stripe_client.create_checkout_session(
            "mock_price",
            &req.customer_email,
            (price_cents as f64) / 100.0
        ).await.unwrap_or_else(|_| "https://checkout.stripe.com/mock".to_string());

        let subscriber = Subscriber {
            id: subscriber_id.to_string(),
            organization_id: req.organization_id.clone(),
            subscription_plan_id: req.subscription_plan_id.clone(),
            customer_name: req.customer_name.clone(),
            customer_email: req.customer_email.clone(),
            status: "active".to_string(),
            stripe_subscription_id: format!("sub_mock_{}", subscriber_id),
            stripe_customer_id: format!("cus_mock_{}", subscriber_id),
            current_period_end_unix: Utc::now().timestamp() + 30 * 24 * 3600,
            created_at_unix: Utc::now().timestamp(),
        };

        sqlx::query(
            "INSERT INTO subscribers (id, organization_id, subscription_plan_id, customer_name, customer_email, status, stripe_subscription_id, stripe_customer_id, current_period_end)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(subscriber_id)
        .bind(&req.organization_id)
        .bind(Uuid::parse_str(&req.subscription_plan_id).unwrap_or_default())
        .bind(&req.customer_name)
        .bind(&req.customer_email)
        .bind("active")
        .bind(&subscriber.stripe_subscription_id)
        .bind(&subscriber.stripe_customer_id)
        .bind(Utc::now() + chrono::Duration::days(30))
        .execute(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let batch_id = Uuid::new_v4();
        let _ = sqlx::query(
            "INSERT INTO fulfillment_batches (id, organization_id, subscription_plan_id, fulfillment_date, status, total_boxes)
             VALUES ($1, $2, $3, $4, $5, 1)
             ON CONFLICT DO NOTHING"
        )
        .bind(batch_id)
        .bind(&req.organization_id)
        .bind(Uuid::parse_str(&req.subscription_plan_id).unwrap_or_default())
        .bind(chrono::Utc::now().naive_utc().date() + chrono::Duration::days(5))
        .bind("pending")
        .execute(&self.pool).await;

        Ok(Response::new(SubscribeResponse {
            subscriber: Some(subscriber),
            checkout_url,
        }))
    }

    async fn get_fulfillment_batches(&self, request: Request<GetFulfillmentBatchesRequest>) -> Result<Response<GetFulfillmentBatchesResponse>, Status> {
        let req = request.into_inner();

        let records = sqlx::query(
            "SELECT id, organization_id, subscription_plan_id, fulfillment_date, status, total_boxes, created_at FROM fulfillment_batches WHERE organization_id = $1"
        )
        .bind(&req.organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let batches = records.into_iter().map(|r| {
            let id: Uuid = r.get("id");
            let org_id: String = r.get("organization_id");
            let plan_id: Option<Uuid> = r.get("subscription_plan_id");
            let fulfillment_date: chrono::NaiveDate = r.get("fulfillment_date");
            let status: String = r.get("status");
            let total_boxes: i32 = r.get("total_boxes");
            let created_at: chrono::DateTime<Utc> = r.get("created_at");

            FulfillmentBatch {
                id: id.to_string(),
                organization_id: org_id,
                subscription_plan_id: plan_id.map(|u| u.to_string()).unwrap_or_default(),
                fulfillment_date: fulfillment_date.to_string(),
                status,
                total_boxes: total_boxes,
                created_at_unix: created_at.timestamp(),
            }
        }).collect();

        Ok(Response::new(GetFulfillmentBatchesResponse { batches }))
    }
}
pub mod tests;
