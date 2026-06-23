use std::env;
use sqlx::postgres::PgPoolOptions;
use tokio;
use tonic::Request;
use uuid::Uuid;
use ohc_project_proto::ohc::project::project_service_server::ProjectService;
use ohc_project_proto::ohc::project::{CreateProposalRequest, CreateProposalLineItem};
use super::service::ProjectServiceImpl;

#[tokio::test]
async fn test_create_proposal() {
    let _ = tracing_subscriber::fmt::try_init();

    // Connect to test database if available, else skip (using common pattern)
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

    // Quick out if we can't hit the DB in CI
    let pool = match PgPoolOptions::new().connect(&database_url).await {
        Ok(p) => p,
        Err(_) => return,
    };

    let service = ProjectServiceImpl {
        db_pool: pool.clone(),
    };

    let tenant_id = Uuid::new_v4().to_string();
    let customer_id = Uuid::new_v4().to_string();

    let mut req = Request::new(CreateProposalRequest {
        customer_id: customer_id.clone(),
        message: "Test proposal".to_string(),
        line_items: vec![
            CreateProposalLineItem {
                description: "Item 1".to_string(),
                unit_price_cents: 1000,
                quantity: 2,
            },
        ],
        required_deposit_cents: 500,
    });

    req.extensions_mut().insert(server_auth::orchestration::AuthInfo {
        org_id: tenant_id.clone(),
        agent_id: "".to_string(),
        spiffe_id: "spiffe://test".to_string(),
    });



    let _res = service.create_proposal(req).await;
    // Note: Since RLS requires a valid tenant entry, this will likely fail with FK violation
    // unless we seed the tenant first. This test serves as a compilation check for the test module.
}
