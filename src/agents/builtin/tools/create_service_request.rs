use ohc_builtin_agent_core::types::ToolError;
use serde_json::json;
use std::sync::Arc;
use super::{Tool, pydantic::{PydanticToolExecutor, PydanticAdapter}};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateServiceRequestArgs {
    pub tenant_id: String,
    pub customer_id: String,
    pub description: String,
    pub job_type: Option<String>,
    pub location: Option<String>,
    pub urgency: Option<String>,
}

pub struct CreateServiceRequestExecutor;

#[async_trait::async_trait]
impl PydanticToolExecutor<CreateServiceRequestArgs> for CreateServiceRequestExecutor {
    async fn execute_typed(&self, args: CreateServiceRequestArgs) -> Result<String, ToolError> {
        let request_id = Uuid::new_v4();

        let db_url = std::env::var("OHC_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());

        let pool = sqlx::PgPool::connect(&db_url).await
            .map_err(|e| ToolError::LlmRecoverable(format!("DB connection failed: {}", e)))?;

        sqlx::query(
            "INSERT INTO service_requests (id, tenant_id, customer_id, description, job_type, location, urgency, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 'PENDING', NOW(), NOW())"
        )
        .bind(request_id)
        .bind(&args.tenant_id)
        .bind(match Uuid::parse_str(&args.customer_id) {
            Ok(u) => u.to_string(),
            Err(_) => return Err(ToolError::LlmRecoverable("Invalid customer_id format".to_string()))
        })
        .bind(&args.description)
        .bind(&args.job_type)
        .bind(&args.location)
        .bind(&args.urgency)
        .execute(&pool)
        .await
        .map_err(|e| ToolError::LlmRecoverable(format!("DB insert service_request failed: {}", e)))?;

        Ok(json!({
            "status": "success",
            "message": "ServiceRequest created successfully.",
            "service_request_id": request_id.to_string()
        }).to_string())
    }
}

pub fn create_service_request_tool() -> Tool {
    Tool {
        name: "create_service_request".to_string(),
        description: "Create a structured service request for a new customer inquiry.".to_string(),
        is_read_only: false,
        parameters: json!({
            "type": "object",
            "properties": {
                "tenant_id": { "type": "string" },
                "customer_id": { "type": "string", "description": "UUID of the customer" },
                "description": { "type": "string", "description": "Description of the requested service" },
                "job_type": { "type": "string", "description": "Type of job (e.g., Plumbing, Electrical)" },
                "location": { "type": "string", "description": "Service location/address" },
                "urgency": { "type": "string", "description": "Urgency of the request (e.g., HIGH, LOW)" }
            },
            "required": ["tenant_id", "customer_id", "description"]
        }),
        execute: Arc::new(PydanticAdapter::new(CreateServiceRequestExecutor)),
    }
}
