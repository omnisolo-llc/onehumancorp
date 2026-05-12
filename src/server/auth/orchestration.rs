use tonic::{Request, Status};
use crate::auth::parse_spiffe_id;
use crate::ohc::orchestration::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AuthInfo {
    pub org_id: String,
    pub agent_id: String,
    pub spiffe_id: String,
}

#[allow(dead_code)]
pub fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let spiffe_id_str = req.metadata().get("x-spiffe-id")
        .ok_or_else(|| Status::unauthenticated("missing x-spiffe-id header"))?
        .to_str()
        .map_err(|_| Status::unauthenticated("invalid x-spiffe-id header"))?
        .to_string();

    let (org_id, agent_id) = parse_spiffe_id(&spiffe_id_str)
        .map_err(|e| Status::permission_denied(e))?;

    let mut req = req;
    req.extensions_mut().insert(AuthInfo {
        org_id,
        agent_id,
        spiffe_id: spiffe_id_str,
    });

    Ok(req)
}

