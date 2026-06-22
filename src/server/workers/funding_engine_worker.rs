use sqlx::PgPool;
use tracing::{info, error};
use std::time::Duration;
use uuid::Uuid;
use crate::ohc::agent::service::agent_service_client::AgentServiceClient;
use crate::ohc::agent::service::RunTaskRequest;

pub async fn run_funding_engine_worker(db_pool: PgPool, agent_url: String) {
    info!("Starting Funding Engine Worker");

    let mut interval = tokio::time::interval(Duration::from_secs(60 * 60)); // Run every hour

    loop {
        interval.tick().await;
        info!("Funding Engine: Syncing external grants...");

        // 1. Fetch tenants
        let tenants = match sqlx::query("SELECT id, name FROM tenants").fetch_all(&db_pool).await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to fetch tenants: {}", e);
                continue;
            }
        };

        // 2. Mock external grant DB sync for now (simulate discovering a $10,000 grant)
        let grant_name = "Downtown Revitalization Grant";
        let amount = 10000;
        let deadline = chrono::Utc::now() + chrono::Duration::days(30);

        for tenant_row in tenants {
            use sqlx::Row;
            let tenant_id: String = tenant_row.get("id");
            let tenant_name: String = tenant_row.get("name");

            info!("Funding Engine: Evaluating tenant {} ({})", tenant_id, tenant_name);

            // 3. Connect to Legal Agent for drafting
            let mut client = match AgentServiceClient::connect(agent_url.clone()).await {
                Ok(c) => c,
                Err(e) => {
                    error!("Failed to connect to AgentService: {}", e);
                    continue;
                }
            };

            let prompt = format!(
                "You are the Legal Agent. Draft a 500-word grant proposal for the business '{}'. The grant is '{}' for ${}. Detail how they will use the funds to expand and improve their local presence.",
                tenant_name, grant_name, amount
            );

            let req = tonic::Request::new(RunTaskRequest {
                task_id: Uuid::new_v4().to_string(),
                task: prompt,
                model: "gpt-4o".to_string(),
                llm_provider: "openai".to_string(),
                llm_endpoint: "".to_string(),
                system_prompt: "".to_string(),
                max_tokens: 1000,
                temperature: 0.7,
                max_context_messages: 10,
                injected_context_json: "".to_string(),
                runtime_config: None,
                toolset_config: None,
                department: "legal".to_string(),
                enable_tools_gating: false,
                enable_tao_orchestration_loop: false,
            });

            // This streams events back. Let's collect TEXT_CHUNKS to form the proposal
            let mut stream = match client.run_task(req).await {
                Ok(res) => res.into_inner(),
                Err(e) => {
                    error!("Agent processing failed: {}", e);
                    continue;
                }
            };

            let mut draft_proposal = String::new();
            while let Ok(Some(event)) = stream.message().await {
                if event.r#type == crate::ohc::agent::service::EventType::TextChunk as i32 {
                    draft_proposal.push_str(&event.content);
                }
            }

            // 4. Save to database
            let opportunity_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO funding_opportunities (id, tenant_id, grant_name, amount, draft_proposal_text, deadline, status)
                 VALUES ($1, $2, $3, $4, $5, $6, 'Drafted')"
            )
            .bind(opportunity_id.clone())
            .bind(tenant_id.clone())
            .bind(grant_name)
            .bind(amount)
            .bind(draft_proposal)
            .bind(deadline)
            .execute(&db_pool)
            .await;

            info!("Funding Engine: Saved opportunity {} for tenant {}", opportunity_id, tenant_id);
        }
    }
}
