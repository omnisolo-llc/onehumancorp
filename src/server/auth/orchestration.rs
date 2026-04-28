use tonic::{Request, Status};
use crate::auth::parse_spiffe_id;
use crate::ohc::orchestration::*;

#[derive(Debug, Clone)]
pub struct AuthInfo {
    pub tenant_id: String,
    pub agent_id: String,
    pub spiffe_id: String,
}

pub fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let spiffe_id_str = req.metadata().get("x-spiffe-id")
        .ok_or_else(|| Status::unauthenticated("missing x-spiffe-id header"))?
        .to_str()
        .map_err(|_| Status::unauthenticated("invalid x-spiffe-id header"))?
        .to_string();

    let (tenant_id, agent_id) = parse_spiffe_id(&spiffe_id_str)
        .map_err(|e| Status::permission_denied(e))?;

    let mut req = req;
    req.extensions_mut().insert(AuthInfo {
        tenant_id,
        agent_id,
        spiffe_id: spiffe_id_str,
    });

    Ok(req)
}

pub fn authorize_register_agent(auth: &AuthInfo, req: &RegisterAgentRequest) -> Result<(), Status> {
    if let Some(agent) = &req.agent {
        if auth.agent_id != agent.id {
            return Err(Status::permission_denied(format!("SPIFFE ID {} cannot register agent {}", auth.spiffe_id, agent.id)));
        }
        if !auth.tenant_id.is_empty() && !agent.tenant_id.is_empty() && auth.tenant_id != agent.tenant_id {
            return Err(Status::permission_denied(format!("SPIFFE ID {} cannot register into organization {}", auth.spiffe_id, agent.tenant_id)));
        }
    }
    Ok(())
}

pub fn authorize_publish_message(auth: &AuthInfo, req: &PublishMessageRequest) -> Result<(), Status> {
    if let Some(msg) = &req.message {
        if auth.agent_id != msg.from_agent {
            return Err(Status::permission_denied(format!("SPIFFE ID {} cannot publish as agent {}", auth.spiffe_id, msg.from_agent)));
        }
    }
    Ok(())
}

pub fn authorize_delegate_task(auth: &AuthInfo, req: &DelegateTaskRequest) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot delegate task as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

pub fn authorize_sub_task(auth: &AuthInfo, req: &SubTask) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot delegate subtask as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

pub fn authorize_reason_request(auth: &AuthInfo, req: &ReasonRequest) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot request reasoning as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

pub fn authorize_open_meeting(auth: &AuthInfo, req: &OpenMeetingRequest) -> Result<(), Status> {
    let found = req.participants.iter().any(|p| p == &auth.agent_id);
    if !found {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot open a meeting without being a participant", auth.spiffe_id)));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authorize_register_agent() {
        let auth = AuthInfo {
            tenant_id: "org-1".to_string(),
            agent_id: "agent-1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
        };
        
        let mut req = RegisterAgentRequest::default();
        let mut agent = Agent::default();
        agent.id = "agent-1".to_string();
        agent.tenant_id = "org-1".to_string();
        req.agent = Some(agent);
        
        assert!(authorize_register_agent(&auth, &req).is_ok());
        
        let mut req2 = RegisterAgentRequest::default();
        let mut agent2 = Agent::default();
        agent2.id = "agent-2".to_string();
        agent2.tenant_id = "org-1".to_string();
        req2.agent = Some(agent2);
        
        assert!(authorize_register_agent(&auth, &req2).is_err());
    }
}
