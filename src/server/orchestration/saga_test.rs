use uuid::Uuid;
use super::saga::*;
use sqlx::PgPool;
use std::env;


#[tokio::test]
async fn test_saga_orchestration() {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
    // Since the CI might not have a postgres instance we should gracefully skip or mock if possible.
    // Here we'll try to connect but gracefully return if it times out
    let pool = match PgPool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(_) => {
            println!("Skipping saga_test as no DB is available");
            return;
        }
    };

    let orchestrator = SagaOrchestrator::new(pool);
    let tenant_id = "test_tenant";

    // Start saga
    let context = serde_json::json!({"project_name": "Test Project"});
    let saga_id = orchestrator.start_saga(tenant_id, "ProjectIntake", context).await.unwrap();

    // Add steps
    let step1_id = orchestrator.add_step(tenant_id, saga_id, "Draft Proposal", "SalesAgent").await.unwrap();
    let step2_id = orchestrator.add_step(tenant_id, saga_id, "Create Tasks", "OperationsAgent").await.unwrap();

    // Complete step 1
    orchestrator.complete_step(tenant_id, step1_id).await.unwrap();

    // Fail step 2
    orchestrator.fail_step(tenant_id, step2_id).await.unwrap();
}
