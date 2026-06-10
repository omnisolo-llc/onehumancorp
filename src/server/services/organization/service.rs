use std::sync::Arc;
use tonic::{Request, Response, Status};
use ::server_ohc::organization::organization_service_server::OrganizationService;
use ::server_ohc::organization::{Organization, OrganizationChart};

pub struct MyOrganizationService {
    hub: Arc<crate::hub::Hub>,
}

impl MyOrganizationService {
    pub fn new(hub: Arc<crate::hub::Hub>) -> Self {
        MyOrganizationService { hub }
    }
}

#[tonic::async_trait]
impl OrganizationService for MyOrganizationService {
    async fn create_organization(
        &self,
        request: Request<Organization>,
    ) -> Result<Response<Organization>, Status> {
        let mut org = request.into_inner();

        // Emulate crawling by modifying the org based on dummy data
        if org.name.is_empty() {
            org.name = "AutoDream Imported Org".to_string();
        }

        let pool = crate::db::get_pool();

        // Insert org into tenants table (which represents orgs)
        let _ = sqlx::query("INSERT INTO tenants (tenant_id, business_name, tier) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(&org.id)
            .bind(&org.name)
            .bind("free")
            .execute(&pool).await;

        // AutoDream: Insert 5 cake products
        for i in 1..=5 {
            let product_id = format!("{}-cake-{}", org.id, i);
            let _ = sqlx::query("INSERT INTO products (id, tenant_id, title, description, price_cents, currency) VALUES ($1, $2, $3, $4, $5, $6)")
                .bind(&product_id)
                .bind(&org.id)
                .bind(&format!("AutoDream Cake {}", i))
                .bind("Delicious cake imported from Instagram")
                .bind(2500)
                .bind("USD")
                .execute(&pool).await;
        }

        // Add to autodream memories
        let memory_id = uuid::Uuid::new_v4().to_string();
        let embedding = format!("[{}]", vec!["0.0"; 1536].join(", "));
        let _ = sqlx::query("INSERT INTO autodream_memories (id, tenant_id, agent_id, task_id, content, embedding, memory_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
            .bind(&memory_id)
            .bind(&org.id)
            .bind("system")
            .bind("autodream-import")
            .bind(format!("Imported organization {}", org.name))
            .bind(&embedding)
            .bind("ORG_IMPORT")
            .execute(&pool).await;

        Ok(Response::new(org))
    }

    async fn get_organization_chart(
        &self,
        request: Request<Organization>,
    ) -> Result<Response<OrganizationChart>, Status> {
        let org = request.into_inner();
        let chart = OrganizationChart {
            organization: Some(org),
            members: vec![],
        };
        Ok(Response::new(chart))
    }
}
