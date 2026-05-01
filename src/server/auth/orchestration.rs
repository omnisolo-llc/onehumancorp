use tonic::{Request, Status};
use crate::auth::parse_spiffe_id;
use crate::ohc::orchestration::*;

#[derive(Debug, Clone)]
pub struct AuthInfo {
    pub org_id: String,
    pub agent_id: String,
    pub spiffe_id: String,
}








#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authorize_register_agent() {
        let auth = AuthInfo {
            org_id: "org-1".to_string(),
            agent_id: "agent-1".to_string(),
            spiffe_id: "spiffe://onehumancorp.io/org-1/agent-1".to_string(),
        };
        
        let mut req = RegisterAgentRequest::default();
        let mut agent = Agent::default();
        agent.id = "agent-1".to_string();
        agent.organization_id = "org-1".to_string();
        req.agent = Some(agent);
        
        assert!(authorize_register_agent(&auth, &req).is_ok());
        
        let mut req2 = RegisterAgentRequest::default();
        let mut agent2 = Agent::default();
        agent2.id = "agent-2".to_string();
        agent2.organization_id = "org-1".to_string();
        req2.agent = Some(agent2);
        
        assert!(authorize_register_agent(&auth, &req2).is_err());
    }
}
