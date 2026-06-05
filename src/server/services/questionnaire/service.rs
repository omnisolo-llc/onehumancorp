use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::questionnaire_service_server::QuestionnaireService;
use std::sync::Arc;
use crate::hub::Hub;
use uuid::Uuid;
use chrono::Utc;

pub struct MyQuestionnaireService {
    hub: Arc<Hub>,
}

impl MyQuestionnaireService {
    pub fn new(hub: Arc<Hub>) -> Self {
        MyQuestionnaireService { hub }
    }
}

#[tonic::async_trait]
impl QuestionnaireService for MyQuestionnaireService {
    async fn create_template(
        &self,
        request: Request<CreateTemplateRequest>,
    ) -> Result<Response<CreateTemplateResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = request.metadata().get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
        let id = Uuid::new_v4().to_string();

        let template = QuestionnaireTemplate {
            id: id.clone(),
            tenant_id: tenant_id.to_string(),
            title: req.title.clone(),
            status: "draft".to_string(),
        };

        if let crate::db::DbStore::Postgres(pool) = &self.hub.db.store {
            let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            sqlx::query("INSERT INTO questionnaire_templates (id, tenant_id, title, status) VALUES ($1, $2, $3, $4)")
                .bind(&id)
                .bind(tenant_id)
                .bind(&req.title)
                .bind("draft")
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        }

        Ok(Response::new(CreateTemplateResponse {
            template: Some(template),
        }))
    }

    async fn get_template(
        &self,
        request: Request<GetTemplateRequest>,
    ) -> Result<Response<GetTemplateResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = request.metadata().get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");

        let mut tpl = None;
        let mut qs = vec![];

        if let crate::db::DbStore::Postgres(pool) = &self.hub.db.store {
            let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            let row: Result<(String, String, String, String), _> = sqlx::query_as(
                "SELECT id, tenant_id, title, status FROM questionnaire_templates WHERE id = $1"
            )
            .bind(&req.template_id)
            .fetch_one(&mut *tx)
            .await;

            if let Ok(r) = row {
                tpl = Some(QuestionnaireTemplate {
                    id: r.0,
                    tenant_id: r.1,
                    title: r.2,
                    status: r.3,
                });
            }

            let rows: Result<Vec<(String, String, String, String, bool)>, _> = sqlx::query_as(
                "SELECT id, template_id, type, text, is_required FROM questions WHERE template_id = $1"
            )
            .bind(&req.template_id)
            .fetch_all(&mut *tx)
            .await;

            if let Ok(rs) = rows {
                for r in rs {
                    qs.push(Question {
                        id: r.0,
                        template_id: r.1,
                        r#type: r.2,
                        text: r.3,
                        is_required: r.4,
                    });
                }
            }
            tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        }

        Ok(Response::new(GetTemplateResponse {
            template: tpl,
            questions: qs,
        }))
    }

    async fn submit_intake(
        &self,
        request: Request<SubmitIntakeRequest>,
    ) -> Result<Response<SubmitIntakeResponse>, Status> {
        let req = request.into_inner();
        let tenant_id = request.metadata().get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("default_tenant");
        let sub_id = Uuid::new_v4().to_string();

        let submission = IntakeSubmission {
            id: sub_id.clone(),
            customer_id: req.customer_id.clone(),
            status: "submitted".to_string(),
            parsed_entities: "{}".to_string(),
            created_at_unix: Utc::now().timestamp(),
        };

        if let crate::db::DbStore::Postgres(pool) = &self.hub.db.store {
            let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
            sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
                .bind(tenant_id)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            sqlx::query("INSERT INTO intake_submissions (id, tenant_id, customer_id, status) VALUES ($1, $2, $3, $4)")
                .bind(&sub_id)
                .bind(tenant_id)
                .bind(&req.customer_id)
                .bind("submitted")
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            for ans in req.answers {
                sqlx::query("INSERT INTO submission_answers (id, tenant_id, submission_id, question_id, raw_response, media_url) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(Uuid::new_v4().to_string())
                    .bind(tenant_id)
                    .bind(&sub_id)
                    .bind(&ans.question_id)
                    .bind(&ans.raw_response)
                    .bind(&ans.media_url)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| Status::internal(e.to_string()))?;
            }

            // Fire async job
            let job_id = Uuid::new_v4().to_string();
            let payload = serde_json::json!({
                "submission_id": sub_id,
                "customer_id": req.customer_id,
            });
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, $3, $4::jsonb, $5)")
                .bind(&job_id)
                .bind(tenant_id)
                .bind("PARSE_INTAKE_SUBMISSION")
                .bind(serde_json::to_value(&payload).unwrap())
                .bind("PENDING")
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
        }

        Ok(Response::new(SubmitIntakeResponse {
            submission: Some(submission),
        }))
    }
}
