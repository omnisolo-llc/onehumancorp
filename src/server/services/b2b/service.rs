use tonic::{Request, Response, Status};
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::b2b_service_server::B2bService;
use std::sync::RwLock;
use chrono::Utc;
use uuid::Uuid;
use sqlx::PgPool;

pub struct MyB2BService {
    pool: Option<PgPool>,
    approvals: RwLock<Vec<ApprovalRequest>>,
    handoffs: RwLock<Vec<HandoffPackage>>,
    trust_agreements: RwLock<Vec<TrustAgreement>>,
}

impl MyB2BService {
    pub fn new() -> Self {
        MyB2BService {
            pool: None,
            approvals: RwLock::new(Vec::new()),
            handoffs: RwLock::new(Vec::new()),
            trust_agreements: RwLock::new(Vec::new()),
        }
    }

    pub fn new_with_pool(pool: PgPool) -> Self {
        MyB2BService {
            pool: Some(pool),
            approvals: RwLock::new(Vec::new()),
            handoffs: RwLock::new(Vec::new()),
            trust_agreements: RwLock::new(Vec::new()),
        }
    }
}

#[tonic::async_trait]
impl B2bService for MyB2BService {
    async fn get_approvals(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<ApprovalsResponse>, Status> {
        let approvals = self.approvals.read().unwrap();
        Ok(Response::new(ApprovalsResponse {
            approvals: approvals.clone(),
        }))
    }

    async fn create_approval_request(
        &self,
        request: Request<CreateApprovalReq>,
    ) -> Result<Response<ApprovalRequest>, Status> {
        let req = request.into_inner();
        if req.agent_id.is_empty() || req.action.is_empty() {
            return Err(Status::invalid_argument("agentId and action are required"));
        }
        
        let now = Utc::now();
        let risk_level = if req.risk_level.is_empty() {
            if req.estimated_cost_usd > 500.0 {
                "critical".to_string()
            } else if req.estimated_cost_usd > 100.0 {
                "high".to_string()
            } else {
                "medium".to_string()
            }
        } else {
            req.risk_level
        };

        let approval = ApprovalRequest {
            id: format!("approval-{}", now.timestamp()),
            agent_id: req.agent_id,
            action: req.action,
            reason: req.reason,
            estimated_cost_usd: req.estimated_cost_usd,
            risk_level,
            status: "PENDING".to_string(),
            created_at_unix: now.timestamp(),
            decided_at_unix: 0,
            decided_by: String::new(),
        };
        
        let mut approvals = self.approvals.write().unwrap();
        approvals.push(approval.clone());
        
        Ok(Response::new(approval))
    }

    async fn decide_approval(
        &self,
        request: Request<DecideApprovalRequest>,
    ) -> Result<Response<ApprovalsResponse>, Status> {
        let req = request.into_inner();
        if req.approval_id.is_empty() || req.decision.is_empty() {
            return Err(Status::invalid_argument("approvalId and decision are required"));
        }
        
        let new_status = match req.decision.as_str() {
            "approve" => "APPROVED",
            "reject" => "REJECTED",
            _ => return Err(Status::invalid_argument("decision must be 'approve' or 'reject'")),
        };

        let now = Utc::now();
        let mut approvals = self.approvals.write().unwrap();
        let mut found = false;
        
        for a in approvals.iter_mut() {
            if a.id == req.approval_id {
                a.status = new_status.to_string();
                a.decided_at_unix = now.timestamp();
                a.decided_by = req.decided_by.clone();
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(Status::not_found("approval not found"));
        }
        
        Ok(Response::new(ApprovalsResponse {
            approvals: approvals.clone(),
        }))
    }

    async fn get_handoffs(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<HandoffsResponse>, Status> {
        let handoffs = self.handoffs.read().unwrap();
        Ok(Response::new(HandoffsResponse {
            handoffs: handoffs.clone(),
        }))
    }

    async fn create_handoff(
        &self,
        request: Request<CreateHandoffRequest>,
    ) -> Result<Response<HandoffPackage>, Status> {
        let req = request.into_inner();
        if req.from_agent_id.is_empty() || req.intent.is_empty() {
            return Err(Status::invalid_argument("fromAgentId and intent are required"));
        }
        
        let now = Utc::now();
        let handoff = HandoffPackage {
            id: format!("handoff-{}", now.timestamp()),
            from_agent_id: req.from_agent_id,
            to_human_role: req.to_human_role,
            intent: req.intent,
            failed_attempts: req.failed_attempts,
            current_state: req.current_state,
            visual_ground_truth: req.visual_ground_truth,
            status: "pending".to_string(),
            created_at_unix: now.timestamp(),
        };
        
        let mut handoffs = self.handoffs.write().unwrap();
        handoffs.push(handoff.clone());
        
        Ok(Response::new(handoff))
    }

    async fn resolve_handoff(
        &self,
        request: Request<ResolveHandoffRequest>,
    ) -> Result<Response<HandoffsResponse>, Status> {
        let req = request.into_inner();
        if req.handoff_id.is_empty() || req.status.is_empty() {
            return Err(Status::invalid_argument("handoffId and status are required"));
        }
        
        if req.status != "acknowledged" && req.status != "resolved" {
            return Err(Status::invalid_argument("status must be 'acknowledged' or 'resolved'"));
        }

        let mut handoffs = self.handoffs.write().unwrap();
        let mut found = false;
        let mut already_resolved = false;
        
        for h in handoffs.iter_mut() {
            if h.id == req.handoff_id {
                found = true;
                if h.status != "pending" {
                    already_resolved = true;
                } else {
                    h.status = req.status.clone();
                }
                break;
            }
        }
        
        if !found {
            return Err(Status::not_found("handoff not found"));
        }
        
        if already_resolved {
            return Err(Status::failed_precondition("State Changed: This handoff has already been acknowledged or resolved."));
        }
        
        Ok(Response::new(HandoffsResponse {
            handoffs: handoffs.clone(),
        }))
    }

    async fn get_b2b_agreements(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<B2bAgreementsResponse>, Status> {
        let agreements = self.trust_agreements.read().unwrap();
        Ok(Response::new(B2bAgreementsResponse {
            agreements: agreements.clone(),
        }))
    }

    async fn b2b_handshake(
        &self,
                request: Request<B2bHandshakeRequest>,
    ) -> Result<Response<TrustAgreement>, Status> {
        let req = request.into_inner();
        if req.partner_org.is_empty() || req.partner_jwks.is_empty() {
            return Err(Status::invalid_argument("partnerOrg and partnerJwksUrl are required"));
        }
        
        let agreement = TrustAgreement {
            id: format!("ta-{}-{}", req.partner_org.replace(".", "-"), Utc::now().timestamp()),
            partner_org: req.partner_org,
            partner_jwks: req.partner_jwks,
            allowed_roles: req.allowed_roles,
            status: "ACTIVE".to_string(),
            created_at_unix: Utc::now().timestamp(),
        };
        
        let mut agreements = self.trust_agreements.write().unwrap();
        agreements.push(agreement.clone());
        
        Ok(Response::new(agreement))
    }

    async fn b2b_revoke(
        &self,
                request: Request<B2bRevokeRequest>,
    ) -> Result<Response<TrustAgreement>, Status> {
        let req = request.into_inner();
        if req.agreement_id.is_empty() {
            return Err(Status::invalid_argument("agreementId is required"));
        }
        
        let mut agreements = self.trust_agreements.write().unwrap();
        let mut found = false;
        let mut updated = None;
        
        for ag in agreements.iter_mut() {
            if ag.id == req.agreement_id {
                ag.status = "REVOKED".to_string();
                updated = Some(ag.clone());
                found = true;
                break;
            }
        }
        
        if !found {
            return Err(Status::not_found("agreement not found"));
        }
        
        Ok(Response::new(updated.unwrap()))
    }

    async fn create_collective(
        &self,
        request: Request<CreateCollectiveRequest>,
    ) -> Result<Response<Collective>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|m| m.to_str().unwrap_or("")).unwrap_or("").to_string();
        let req = request.into_inner();
        let pool = match &self.pool {
            Some(p) => p,
            None => return Err(Status::internal("Database pool not available")),
        };

        let collective_id = Uuid::new_v4();
        let now = Utc::now().timestamp();

        let collective = Collective {
            id: collective_id.to_string(),
            name: req.name,
            location_center: req.location_center,
            radius_meters: req.radius_meters,
            created_at_unix: now,
            tenant_id: tenant_id.clone(),
        };

        sqlx::query(
            "INSERT INTO collectives (id, name, location_center, radius_meters, created_at_unix, tenant_id) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(collective_id)
        .bind(&collective.name)
        .bind(&collective.location_center)
        .bind(collective.radius_meters as f64)
        .bind(collective.created_at_unix)
        .bind(&collective.tenant_id)
        .execute(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert collective: {}", e)))?;

        // Add creator as active member
        sqlx::query(
            "INSERT INTO collective_members (collective_id, tenant_id, status) VALUES ($1, $2, $3)",
        )
        .bind(collective_id)
        .bind(&collective.tenant_id)
        .bind("ACTIVE")
        .execute(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to insert collective member: {}", e)))?;

        Ok(Response::new(collective))
    }

    async fn join_collective(
        &self,
        request: Request<JoinCollectiveRequest>,
    ) -> Result<Response<CollectiveMember>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|m| m.to_str().unwrap_or("")).unwrap_or("").to_string();
        let req = request.into_inner();
        let pool = match &self.pool {
            Some(p) => p,
            None => return Err(Status::internal("Database pool not available")),
        };

        let collective_uuid = Uuid::parse_str(&req.collective_id).map_err(|_| Status::invalid_argument("Invalid collective ID format"))?;

        sqlx::query(
            "INSERT INTO collective_members (collective_id, tenant_id, status) VALUES ($1, $2, $3) ON CONFLICT (collective_id, tenant_id) DO UPDATE SET status = 'ACTIVE'",
        )
        .bind(collective_uuid)
        .bind(&tenant_id)
        .bind("ACTIVE")
        .execute(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to join collective: {}", e)))?;

        Ok(Response::new(CollectiveMember {
            collective_id: req.collective_id,
            tenant_id,
            status: "ACTIVE".to_string(),
        }))
    }

    async fn get_collectives(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<GetCollectivesResponse>, Status> {
        let tenant_id = request.metadata().get("x-tenant-id").map(|m| m.to_str().unwrap_or("")).unwrap_or("").to_string();
        let pool = match &self.pool {
            Some(p) => p,
            None => return Err(Status::internal("Database pool not available")),
        };

        // Fetch collectives
        use sqlx::Row;
        let collectives_records = sqlx::query(
            "SELECT c.id, c.name, c.location_center, c.radius_meters, c.created_at_unix, c.tenant_id FROM collectives c JOIN collective_members m ON c.id = m.collective_id WHERE m.tenant_id = $1",
        )
        .bind(&tenant_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to fetch collectives: {}", e)))?;

        let mut collectives = Vec::new();
        let mut collective_ids = Vec::new();
        for rec in collectives_records {
            let id_uuid: Uuid = rec.get("id");
            let name: String = rec.get("name");
            let location_center: String = rec.get("location_center");
            let radius_meters: f64 = rec.get("radius_meters");
            let created_at_unix: i64 = rec.get("created_at_unix");
            let row_tenant_id: String = rec.get("tenant_id");
            collective_ids.push(id_uuid);
            collectives.push(Collective {
                id: id_uuid.to_string(),
                name,
                location_center,
                radius_meters: radius_meters as f32,
                created_at_unix,
                tenant_id: row_tenant_id,
            });
        }

        // Fetch members of those collectives
        let mut members = Vec::new();
        if !collective_ids.is_empty() {
            let members_records = sqlx::query(
                "SELECT collective_id, tenant_id, status FROM collective_members WHERE collective_id = ANY($1)",
            )
            .bind(&collective_ids)
            .fetch_all(pool)
            .await
            .map_err(|e| Status::internal(format!("Failed to fetch collective members: {}", e)))?;

            for rec in members_records {
                let c_id: Uuid = rec.get("collective_id");
                let t_id: String = rec.get("tenant_id");
                let stat: String = rec.get("status");
                members.push(CollectiveMember {
                    collective_id: c_id.to_string(),
                    tenant_id: t_id,
                    status: stat,
                });
            }
        }

        Ok(Response::new(GetCollectivesResponse {
            collectives,
            members,
        }))
    }

    async fn suggest_partners(
        &self,
        _request: Request<SuggestPartnersRequest>,
    ) -> Result<Response<SuggestPartnersResponse>, Status> {

        // In a real implementation this would use geohashing and ML matching.
        // For now, return a dummy list.
        let suggested_tenant_ids = vec!["tenant_b_id".to_string(), "tenant_c_id".to_string()];

        Ok(Response::new(SuggestPartnersResponse {
            suggested_tenant_ids,
        }))
    }

    async fn award_loyalty_points(
        &self,
        request: Request<AwardLoyaltyPointsRequest>,
    ) -> Result<Response<AwardLoyaltyPointsResponse>, Status> {
        let req = request.into_inner();
        let pool = match &self.pool {
            Some(p) => p,
            None => return Err(Status::internal("Database pool not available")),
        };

        let collective_uuid = Uuid::parse_str(&req.collective_id).map_err(|_| Status::invalid_argument("Invalid collective ID format"))?;

        let rec = sqlx::query(
            "INSERT INTO collective_loyalty_balances (buyer_id, collective_id, points) VALUES ($1, $2, $3) ON CONFLICT (buyer_id, collective_id) DO UPDATE SET points = collective_loyalty_balances.points + $3 RETURNING points",
        )
        .bind(&req.buyer_id)
        .bind(collective_uuid)
        .bind(req.points_to_add)
        .fetch_one(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to award points: {}", e)))?;

        use sqlx::Row;
        let new_points: i32 = rec.get("points");

        Ok(Response::new(AwardLoyaltyPointsResponse {
            balance: Some(CollectiveLoyaltyBalance {
                buyer_id: req.buyer_id,
                collective_id: req.collective_id,
                points: new_points,
            }),
        }))
    }

    async fn redeem_loyalty_points(
        &self,
        request: Request<RedeemLoyaltyPointsRequest>,
    ) -> Result<Response<RedeemLoyaltyPointsResponse>, Status> {
        let req = request.into_inner();
        let pool = match &self.pool {
            Some(p) => p,
            None => return Err(Status::internal("Database pool not available")),
        };

        let collective_uuid = Uuid::parse_str(&req.collective_id).map_err(|_| Status::invalid_argument("Invalid collective ID format"))?;

        let current_balance_rec = sqlx::query(
            "SELECT points FROM collective_loyalty_balances WHERE buyer_id = $1 AND collective_id = $2",
        )
        .bind(&req.buyer_id)
        .bind(collective_uuid)
        .fetch_optional(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to check points: {}", e)))?;

        use sqlx::Row;
        let current_points: i32 = current_balance_rec.map(|r| r.get("points")).unwrap_or(0);
        if current_points < req.points_to_deduct {
            return Err(Status::failed_precondition("Insufficient points"));
        }

        let new_points = current_points - req.points_to_deduct;

        sqlx::query(
            "UPDATE collective_loyalty_balances SET points = $1 WHERE buyer_id = $2 AND collective_id = $3",
        )
        .bind(new_points)
        .bind(&req.buyer_id)
        .bind(collective_uuid)
        .execute(pool)
        .await
        .map_err(|e| Status::internal(format!("Failed to redeem points: {}", e)))?;

        Ok(Response::new(RedeemLoyaltyPointsResponse {
            balance: Some(CollectiveLoyaltyBalance {
                buyer_id: req.buyer_id,
                collective_id: req.collective_id,
                points: new_points,
            }),
        }))
    }
}
