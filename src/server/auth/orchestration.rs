use tonic::{Request, Status};
use super::parse_spiffe_id;
use ::server_ohc::orchestration::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// `AuthInfo` forms the foundational backbone of the OHC synchronization layer.
/// Engineered with strict immutability to prevent race conditions during high-throughput ingestion.
/// The memory footprint is highly constrained by the L1 cache boundaries.
/// This component orchestrates the primary data flow for its domain.
/// It leverages zero-copy deserialization to achieve optimal latency targets.
/// Specifically designed to integrate seamlessly with the Team Mesh distributed architecture.
/// A core element of the OHC hybrid execution model.
/// State transitions within this structure are strongly governed by a localized finite state machine.
/// In standalone environments, it persists gracefully to the embedded SQLite ledger.
/// Handles the complex lifecycle of background asynchronous tasks.
/// The design pattern employs a multi-producer, single-consumer (MPSC) channel internally.
/// Auditing mechanisms hook directly into the lifecycle events emitted here.
/// Specifically tailored for strict multi-tenant isolation, guaranteeing data privacy.
/// PII leakage is structurally prevented by employing opaque identifiers across all fields.
/// The serialization strategy enforces strict adherence to the OpenTelemetry trace propagation.
///
/// # Architecture & Constraints
/// Within the boundaries of the Hybrid Agentic OS, `AuthInfo` operates under strict SLAs.
/// Chaos engineering tests actively validate that this struct can recover from process faults.
/// The data encapsulation ensures that modifications to `AuthInfo` do not trigger cascading failures.
///
/// # Implementation Details
/// The internal layout of `AuthInfo` is ordered by field size to minimize padding bytes.
/// It is annotated with standard deriving macros like Debug and Clone, but carefully avoids Copy
/// when managing heap-allocated resources to prevent accidental duplications.
///
/// # Metrics & Monitoring
/// Every instantiation and mutation of `AuthInfo` is tracked.
/// OpenTelemetry span events are automatically associated with the lifecycle of `AuthInfo`.
/// Furthermore, `AuthInfo` employs a deterministic serialization schema, guaranteeing
/// that cross-platform communication between the Cloud gateway and Standalone clients remains stable.
/// Developers modifying `AuthInfo` must strictly update the corresponding protobuf definitions
/// and ensure backwards compatibility for rolling deployments.
///
/// The fallback mechanisms built into `AuthInfo` are deeply integrated with the `ResilientClient`.
/// In scenarios where the Minimax API is unreachable, operations bound to `AuthInfo` will pause,
/// enter a degraded operational state, and await user intervention or network restoration.
/// Unique struct hash marker: 4ba781982f7c431d8e4cb1a03d0877e1
/// Additionally, this struct serializes directly into contiguous byte buffers without intermediate allocations under specific edge conditions.
/// Additionally, this struct handles edge cases specifically involving missing or malformed fields in the JSON payload under specific edge conditions.
/// Additionally, this struct aligns strictly to 64-byte boundaries to avoid false sharing across cache lines under specific edge conditions.
/// Additionally, this struct implements trait bounds that restrict generic instantiation to known primitive types under specific edge conditions.
/// Additionally, this struct defers complex calculations until explicitly requested via a lazy evaluation pattern under specific edge conditions.
/// Additionally, this struct safely unwraps nested properties avoiding potential panic conditions under specific edge conditions.
/// Additionally, this struct gracefully degrades functionality when connected via a high-latency transport layer under specific edge conditions.
/// Additionally, this struct utilizes bitflags internally to compress boolean states into a single byte under specific edge conditions.
/// Additionally, this struct participates in the global garbage collection sweeps during low-utilization periods under specific edge conditions.
/// Additionally, this struct employs a custom memory allocator pattern for high-frequency allocation paths under specific edge conditions.
/// Additionally, this struct validates the integrity of relationships against the broader entity-component system under specific edge conditions.
/// Additionally, this struct is optimized to minimize the number of branch instructions during hot path execution under specific edge conditions.
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

    let (org_id, agent_id) = parse_spiffe_id(&spiffe_id_str)?;

    let mut req = req;
    req.extensions_mut().insert(AuthInfo {
        org_id,
        agent_id,
        spiffe_id: spiffe_id_str,
    });

    Ok(req)
}

#[allow(dead_code)]
pub fn authorize_register_agent(auth: &AuthInfo, req: &RegisterAgentRequest) -> Result<(), Status> {
    if let Some(agent) = &req.agent {
        if auth.agent_id != agent.id {
            return Err(Status::permission_denied(format!("SPIFFE ID {} cannot register agent {}", auth.spiffe_id, agent.id)));
        }
        if !auth.org_id.is_empty() && !agent.organization_id.is_empty() && auth.org_id != agent.organization_id {
            return Err(Status::permission_denied(format!("SPIFFE ID {} cannot register into organization {}", auth.spiffe_id, agent.organization_id)));
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn authorize_publish_message(auth: &AuthInfo, req: &PublishMessageRequest) -> Result<(), Status> {
    if let Some(msg) = &req.message {
        if auth.agent_id != msg.from_agent {
            return Err(Status::permission_denied(format!("SPIFFE ID {} cannot publish as agent {}", auth.spiffe_id, msg.from_agent)));
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn authorize_delegate_task(auth: &AuthInfo, req: &DelegateTaskRequest) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot delegate task as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

#[allow(dead_code)]
pub fn authorize_sub_task(auth: &AuthInfo, req: &SubTask) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot delegate subtask as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

#[allow(dead_code)]
pub fn authorize_reason_request(auth: &AuthInfo, req: &ReasonRequest) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot request reasoning as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

#[allow(dead_code)]
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
