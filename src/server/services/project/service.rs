use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use sqlx::{Pool, Postgres, Row};

use ohc_project_proto::ohc::project::project_service_server::ProjectService;
use ohc_project_proto::ohc::project::{
    CreateProposalRequest, Proposal, GetProposalRequest, UpdateProposalStatusRequest,
    CreateProjectMilestoneRequest, ProjectMilestone, GetProjectMilestonesRequest,
    GetProjectMilestonesResponse, UpdateProjectMilestoneStatusRequest, ProposalLineItem,
};
use server_auth::orchestration::AuthInfo;

pub struct ProjectServiceImpl {
    pub db_pool: Pool<Postgres>,
}

#[tonic::async_trait]
impl ProjectService for ProjectServiceImpl {
    async fn create_proposal(
        &self,
        request: Request<CreateProposalRequest>,
    ) -> Result<Response<Proposal>, Status> {
        let tenant_id = request.extensions().get::<AuthInfo>().map(|ai| ai.org_id.clone()).ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();

        let proposal_id = Uuid::new_v4();
        let customer_id = Uuid::parse_str(&req.customer_id).map_err(|_| Status::invalid_argument("Invalid customer_id"))?;
        let now = Utc::now();

        let mut total_amount_cents = 0;
        for item in &req.line_items {
            total_amount_cents += item.unit_price_cents * item.quantity as i64;
        }

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO interactive_proposals (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, message, created_at, updated_at)
             VALUES ($1, $2, $3, 'Draft', $4, $5, $6, $7, $7)"
        )
        .bind(proposal_id)
        .bind(&tenant_id)
        .bind(customer_id)
        .bind(total_amount_cents)
        .bind(req.required_deposit_cents)
        .bind(&req.message)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut created_line_items = Vec::new();
        for item in req.line_items {
            let item_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO interactive_proposal_line_items (id, proposal_id, description, unit_price_cents, quantity, created_at, updated_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $6)"
            )
            .bind(item_id)
            .bind(proposal_id)
            .bind(&item.description)
            .bind(item.unit_price_cents)
            .bind(item.quantity)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            created_line_items.push(ProposalLineItem {
                id: item_id.to_string(),
                proposal_id: proposal_id.to_string(),
                description: item.description,
                unit_price_cents: item.unit_price_cents,
                quantity: item.quantity,
                created_at: now.timestamp(),
                updated_at: now.timestamp(),
            });
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Proposal {
            id: proposal_id.to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: customer_id.to_string(),
            status: "Draft".to_string(),
            total_amount_cents,
            required_deposit_cents: req.required_deposit_cents,
            checkout_url: "".to_string(),
            message: req.message,
            created_at: now.timestamp(),
            updated_at: now.timestamp(),
            line_items: created_line_items,
        }))
    }

    async fn get_proposal(
        &self,
        request: Request<GetProposalRequest>,
    ) -> Result<Response<Proposal>, Status> {
        let tenant_id = request.extensions().get::<AuthInfo>().map(|ai| ai.org_id.clone()).ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();
        let proposal_id = Uuid::parse_str(&req.proposal_id).map_err(|_| Status::invalid_argument("Invalid proposal_id"))?;

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let row = sqlx::query(
            "SELECT id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, checkout_url, message, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix
             FROM interactive_proposals WHERE id = $1 AND tenant_id = $2"
        )
        .bind(proposal_id)
        .bind(&tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let row = match row {
            Some(r) => r,
            None => return Err(Status::not_found("Proposal not found")),
        };

        let items_rows = sqlx::query(
            "SELECT id, proposal_id, description, unit_price_cents, quantity, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix
             FROM interactive_proposal_line_items WHERE proposal_id = $1"
        )
        .bind(proposal_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut line_items = Vec::new();
        for i_row in items_rows {
            let i_created_at: f64 = i_row.get("created_at_unix");
            let i_updated_at: f64 = i_row.get("updated_at_unix");
            line_items.push(ProposalLineItem {
                id: i_row.get::<Uuid, _>("id").to_string(),
                proposal_id: i_row.get::<Uuid, _>("proposal_id").to_string(),
                description: i_row.get("description"),
                unit_price_cents: i_row.get("unit_price_cents"),
                quantity: i_row.get("quantity"),
                created_at: i_created_at as i64,
                updated_at: i_updated_at as i64,
            });
        }

        let created_at: f64 = row.get("created_at_unix");
        let updated_at: f64 = row.get("updated_at_unix");

        let checkout_url: Option<String> = row.get("checkout_url");
        let message: Option<String> = row.get("message");

        Ok(Response::new(Proposal {
            id: row.get::<Uuid, _>("id").to_string(),
            tenant_id: row.get("tenant_id"),
            customer_id: row.get::<Option<Uuid>, _>("customer_id").map(|u| u.to_string()).unwrap_or_default(),
            status: row.get("status"),
            total_amount_cents: row.get("total_amount_cents"),
            required_deposit_cents: row.get("required_deposit_cents"),
            checkout_url: checkout_url.unwrap_or_default(),
            message: message.unwrap_or_default(),
            created_at: created_at as i64,
            updated_at: updated_at as i64,
            line_items,
        }))
    }

    async fn update_proposal_status(
        &self,
        request: Request<UpdateProposalStatusRequest>,
    ) -> Result<Response<Proposal>, Status> {
        let tenant_id = request.extensions().get::<AuthInfo>().map(|ai| ai.org_id.clone()).ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();
        let proposal_id = Uuid::parse_str(&req.proposal_id).map_err(|_| Status::invalid_argument("Invalid proposal_id"))?;
        let now = Utc::now();

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "UPDATE interactive_proposals SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4"
        )
        .bind(&req.status)
        .bind(now)
        .bind(proposal_id)
        .bind(&tenant_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        // Recursively call get_proposal to fetch updated object
        self.get_proposal(Request::new(GetProposalRequest {
            proposal_id: req.proposal_id,
        })).await
    }

    async fn create_project_milestone(
        &self,
        request: Request<CreateProjectMilestoneRequest>,
    ) -> Result<Response<ProjectMilestone>, Status> {
        let tenant_id = request.extensions().get::<AuthInfo>().map(|ai| ai.org_id.clone()).ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();
        let milestone_id = Uuid::new_v4();
        let proposal_id = Uuid::parse_str(&req.proposal_id).map_err(|_| Status::invalid_argument("Invalid proposal_id"))?;
        let now = Utc::now();

        let due_date = (req.due_date > 0).then(|| chrono::DateTime::from_timestamp(req.due_date, 0).unwrap_or_else(|| now.into()));

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO project_milestones (id, tenant_id, proposal_id, title, description, due_date, status, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, 'Pending', $7, $7)"
        )
        .bind(milestone_id)
        .bind(&tenant_id)
        .bind(proposal_id)
        .bind(&req.title)
        .bind(&req.description)
        .bind(due_date)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ProjectMilestone {
            id: milestone_id.to_string(),
            tenant_id: tenant_id.to_string(),
            proposal_id: proposal_id.to_string(),
            title: req.title,
            description: req.description,
            due_date: due_date.map(|d| d.timestamp()).unwrap_or(0),
            status: "Pending".to_string(),
            invoice_id: "".to_string(),
            created_at: now.timestamp(),
            updated_at: now.timestamp(),
        }))
    }

    async fn get_project_milestones(
        &self,
        request: Request<GetProjectMilestonesRequest>,
    ) -> Result<Response<GetProjectMilestonesResponse>, Status> {
        let tenant_id = request.extensions().get::<AuthInfo>().map(|ai| ai.org_id.clone()).ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();
        let proposal_id = Uuid::parse_str(&req.proposal_id).map_err(|_| Status::invalid_argument("Invalid proposal_id"))?;

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let rows = sqlx::query(
            "SELECT id, tenant_id, proposal_id, title, description, extract(epoch from due_date) as due_date_unix, status, invoice_id, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix
             FROM project_milestones WHERE proposal_id = $1 AND tenant_id = $2 ORDER BY due_date ASC"
        )
        .bind(proposal_id)
        .bind(&tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut milestones = Vec::new();
        for row in rows {
            let due_date: Option<f64> = row.get("due_date_unix");
            let created_at: f64 = row.get("created_at_unix");
            let updated_at: f64 = row.get("updated_at_unix");

            let description: Option<String> = row.get("description");
            let invoice_id: Option<String> = row.get("invoice_id");

            milestones.push(ProjectMilestone {
                id: row.get::<Uuid, _>("id").to_string(),
                tenant_id: row.get("tenant_id"),
                proposal_id: row.get::<Uuid, _>("proposal_id").to_string(),
                title: row.get("title"),
                description: description.unwrap_or_default(),
                due_date: due_date.map(|d| d as i64).unwrap_or(0),
                status: row.get("status"),
                invoice_id: invoice_id.unwrap_or_default(),
                created_at: created_at as i64,
                updated_at: updated_at as i64,
            });
        }

        Ok(Response::new(GetProjectMilestonesResponse { milestones }))
    }

    async fn update_project_milestone_status(
        &self,
        request: Request<UpdateProjectMilestoneStatusRequest>,
    ) -> Result<Response<ProjectMilestone>, Status> {
        let tenant_id = request.extensions().get::<AuthInfo>().map(|ai| ai.org_id.clone()).ok_or_else(|| Status::unauthenticated("Missing AuthInfo"))?;
        let req = request.into_inner();
        let milestone_id = Uuid::parse_str(&req.milestone_id).map_err(|_| Status::invalid_argument("Invalid milestone_id"))?;
        let now = Utc::now();

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let invoice_id_opt = if req.invoice_id.is_empty() { None } else { Some(&req.invoice_id) };

        let row = sqlx::query(
            "UPDATE project_milestones SET status = $1, invoice_id = COALESCE($2, invoice_id), updated_at = $3
             WHERE id = $4 AND tenant_id = $5
             RETURNING id, tenant_id, proposal_id, title, description, extract(epoch from due_date) as due_date_unix, status, invoice_id, extract(epoch from created_at) as created_at_unix, extract(epoch from updated_at) as updated_at_unix"
        )
        .bind(&req.status)
        .bind(invoice_id_opt)
        .bind(now)
        .bind(milestone_id)
        .bind(&tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let row = match row {
            Some(r) => r,
            None => return Err(Status::not_found("Milestone not found")),
        };

        let due_date: Option<f64> = row.get("due_date_unix");
        let created_at: f64 = row.get("created_at_unix");
        let updated_at: f64 = row.get("updated_at_unix");

        let description: Option<String> = row.get("description");
        let invoice_id: Option<String> = row.get("invoice_id");

        Ok(Response::new(ProjectMilestone {
            id: row.get::<Uuid, _>("id").to_string(),
            tenant_id: row.get("tenant_id"),
            proposal_id: row.get::<Uuid, _>("proposal_id").to_string(),
            title: row.get("title"),
            description: description.unwrap_or_default(),
            due_date: due_date.map(|d| d as i64).unwrap_or(0),
            status: row.get("status"),
            invoice_id: invoice_id.unwrap_or_default(),
            created_at: created_at as i64,
            updated_at: updated_at as i64,
        }))
    }
}
