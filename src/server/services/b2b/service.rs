use tonic::{Request, Response, Status};
use crate::ohc::orchestration::*;
use crate::ohc::orchestration::b2b_service_server::B2bService;
use std::sync::RwLock;
use chrono::Utc;

pub struct MyB2BService {
    approvals: RwLock<Vec<ApprovalRequest>>,
    handoffs: RwLock<Vec<HandoffPackage>>,
    trust_agreements: RwLock<Vec<TrustAgreement>>,
}

impl MyB2BService {
    pub fn new() -> Self {
        MyB2BService {
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
}
