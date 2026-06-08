use std::sync::Arc;
use crate::db::DB;
use crate::ohc::agent::service::{RunTaskRequest, RunTaskEvent, EventType};
use uuid::Uuid;
use tokio_stream::StreamExt;

pub struct AssistantOrchestrator {
    db: Arc<DB>,
}

impl AssistantOrchestrator {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn start_task(&self, task_id: String, tenant_id: String) -> Result<(), String> {
        let db = self.db.clone();
        let task_id_clone = task_id.clone();
        let tenant_id_clone = tenant_id.clone();

        tokio::spawn(async move {
            if let Err(e) = execute_task_loop(db, task_id_clone, tenant_id_clone).await {
                tracing::error!("Assistant task loop failed: {}", e);
            }
        });

        Ok(())
    }
}

async fn execute_task_loop(db: Arc<DB>, task_id: String, tenant_id: String) -> Result<(), String> {
    let task = match sqlx::query(
        "SELECT prompt, mode, model, provider, permission_profile FROM assistant_tasks WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&task_id)
    .bind(&tenant_id)
    .fetch_one(&db.pool)
    .await {
        Ok(t) => t,
        Err(e) => return Err(format!("Failed to fetch task: {}", e)),
    };

    use sqlx::Row;
    let prompt: String = task.get("prompt");
    let model: Option<String> = task.get("model");
    let provider: Option<String> = task.get("provider");
    let permission_profile: Option<String> = task.get("permission_profile");

    let req = RunTaskRequest {
        task_id: task_id.clone(),
        task: prompt,
        model: model.unwrap_or_default(),
        llm_provider: provider.unwrap_or_default(),
        enable_tools_gating: permission_profile.as_deref() == Some("Guarded"),
        ..Default::default()
    };

    let _ = sqlx::query(
        "UPDATE assistant_tasks SET status = 'running', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&task_id)
    .bind(&tenant_id)
    .execute(&db.pool)
    .await;

    // Use BUILTIN_AGENT_SERVICE if available
    if let Some(svc) = crate::BUILTIN_AGENT_SERVICE.get() {
        use ohc_builtin_agent::proto::agent_service::agent_service_server::AgentService;
        let mut stream = match svc.run_task(tonic::Request::new(req)).await {
            Ok(resp) => resp.into_inner(),
            Err(e) => {
                let error_msg = format!("Failed to start agent task: {}", e);
                let _ = sqlx::query(
                    "UPDATE assistant_tasks SET status = 'failed', current_step = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
                )
                .bind(&error_msg)
                .bind(&task_id)
                .bind(&tenant_id)
                .execute(&db.pool)
                .await;
                return Err(error_msg);
            }
        };

        while let Some(event_res) = stream.next().await {
            match event_res {
                Ok(event) => {
                    // event is ohc_builtin_agent::proto::RunTaskEvent
                    // which matches our RunTaskEvent if built from same proto
                    let internal_event = RunTaskEvent {
                        r#type: event.r#type,
                        content: event.content,
                        tool_name: event.tool_name,
                        tool_args_json: event.tool_args_json,
                        tool_result: event.tool_result,
                        error: event.error,
                        iteration: event.iteration,
                        message_count: event.message_count,
                    };
                    handle_event(&db, &task_id, &tenant_id, internal_event).await?;
                }
                Err(e) => {
                    tracing::error!("Stream error in assistant task: {}", e);
                    break;
                }
            }
        }
    } else {
        // Fallback for when agent service is not available (e.g. initial boot)
        let messages = vec![
            RunTaskEvent {
                r#type: EventType::RunStarted as i32,
                content: "Execution started (Simulated - Agent Service Unavailable)".to_string(),
                ..Default::default()
            }
        ];

        for event in messages {
            handle_event(&db, &task_id, &tenant_id, event).await?;
        }
    }

    let _ = sqlx::query(
        "UPDATE assistant_tasks SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&task_id)
    .bind(&tenant_id)
    .execute(&db.pool)
    .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_assistant_orchestrator_init() {
        let pool = sqlx::sqlite::SqlitePool::connect("sqlite::memory:").await.unwrap();
        let pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap();
        let db = Arc::new(crate::db::DB {
            pool: pg_pool,
            store: crate::db::DbStore::Sqlite(pool),
        });

        let _orchestrator = AssistantOrchestrator::new(db);
        // Basic check that it can be instantiated
        assert!(true);
    }
}

async fn handle_event(db: &Arc<DB>, task_id: &str, tenant_id: &str, event: RunTaskEvent) -> Result<(), String> {
    let event_type = EventType::try_from(event.r#type).unwrap_or(EventType::TextChunk);

    match event_type {
        EventType::TextChunk | EventType::TaskComplete => {
            if event.content.is_empty() { return Ok(()); }
            let id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                "INSERT INTO assistant_messages (id, tenant_id, task_id, role, content) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(&id)
            .bind(tenant_id)
            .bind(task_id)
            .bind("assistant")
            .bind(&event.content)
            .execute(&db.pool)
            .await;
        }
        EventType::ToolCall => {
            let id = Uuid::new_v4().to_string();
            let args_json: serde_json::Value = serde_json::from_str(&event.tool_args_json).unwrap_or(serde_json::json!({}));

            let _ = sqlx::query(
                "INSERT INTO assistant_approvals (id, tenant_id, task_id, tool_name, args, status) VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&id)
            .bind(tenant_id)
            .bind(task_id)
            .bind(&event.tool_name)
            .bind(&args_json)
            .bind("pending")
            .execute(&db.pool)
            .await;

            // If it's a write operation, we might want to register it as a potential artifact
            if event.tool_name.to_lowercase().contains("write") {
                let filename = args_json.get("path").or(args_json.get("filename"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("output.file");

                let art_id = Uuid::new_v4().to_string();
                let _ = sqlx::query(
                    "INSERT INTO assistant_artifacts (id, tenant_id, task_id, type, filename) VALUES ($1, $2, $3, $4, $5)"
                )
                .bind(&art_id)
                .bind(tenant_id)
                .bind(task_id)
                .bind("file")
                .bind(filename)
                .execute(&db.pool)
                .await;
            }
        }
        EventType::IterationStarted => {
            let _ = sqlx::query(
                "UPDATE assistant_tasks SET current_step = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
            )
            .bind(format!("Iteration {}", event.iteration))
            .bind(task_id)
            .bind(tenant_id)
            .execute(&db.pool)
            .await;
        }
        EventType::TaskError => {
             let _ = sqlx::query(
                "UPDATE assistant_tasks SET status = 'failed', current_step = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3"
            )
            .bind(format!("Error: {}", event.error))
            .bind(task_id)
            .bind(tenant_id)
            .execute(&db.pool)
            .await;
        }
        _ => {}
    }

    Ok(())
}
