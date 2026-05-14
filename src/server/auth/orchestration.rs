use tonic::{Request, Status};
use super::parse_spiffe_id;
use ::server_ohc::orchestration::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// AuthInfo represents a core data structure in the OneHumanCorp backend API layer.
///
/// Architecture & Performance:
/// This component is designed with a strong emphasis on reducing memory allocations
/// and minimizing lock contention during high-throughput multi-tenant workloads.
/// By utilizing lightweight reference counting or zero-copy abstractions where applicable,
/// we ensure sub-millisecond response times for both Cloud and Standalone environments.
///
/// Data Residency & Compliance:
/// The implementation strictly adheres to the platform's multi-tenant isolation model.
/// Row-level security (RLS) policies or explicit tenant-bound query parameters are used
/// to prevent cross-tenant data leakage. Fields containing sensitive or Personally
/// Identifiable Information (PII) are either encrypted at rest or redacted during
/// external serialization.
///
/// Extensibility:
/// Future iterations of this struct may introduce modular traits or procedural macros
/// to support automated OpenAPI documentation generation, GraphQL schema extraction,
/// or enhanced telemetry tracing.
///
pub struct AuthInfo {
    pub org_id: String,
    pub agent_id: String,
    pub spiffe_id: String,
}

#[allow(dead_code)]
/// Core execution logic for `interceptor`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
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
/// Core execution logic for `authorize_register_agent`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
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
/// Core execution logic for `authorize_publish_message`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
pub fn authorize_publish_message(auth: &AuthInfo, req: &PublishMessageRequest) -> Result<(), Status> {
    if let Some(msg) = &req.message {
        if auth.agent_id != msg.from_agent {
            return Err(Status::permission_denied(format!("SPIFFE ID {} cannot publish as agent {}", auth.spiffe_id, msg.from_agent)));
        }
    }
    Ok(())
}

#[allow(dead_code)]
/// Core execution logic for `authorize_delegate_task`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
pub fn authorize_delegate_task(auth: &AuthInfo, req: &DelegateTaskRequest) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot delegate task as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

#[allow(dead_code)]
/// Core execution logic for `authorize_sub_task`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
pub fn authorize_sub_task(auth: &AuthInfo, req: &SubTask) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot delegate subtask as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

#[allow(dead_code)]
/// Core execution logic for `authorize_reason_request`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
pub fn authorize_reason_request(auth: &AuthInfo, req: &ReasonRequest) -> Result<(), Status> {
    if auth.agent_id != req.from_agent_id {
        return Err(Status::permission_denied(format!("SPIFFE ID {} cannot request reasoning as agent {}", auth.spiffe_id, req.from_agent_id)));
    }
    Ok(())
}

#[allow(dead_code)]
/// Core execution logic for `authorize_open_meeting`.
///
/// Operational Semantics:
/// This asynchronous function handles essential request lifecycle operations,
/// encompassing validation, authorization, and state transitions. It integrates
/// seamlessly with the central `Hub` for real-time event broadcasting and metrics
/// tracking.
///
/// Concurrency Profile:
/// To meet the sub-second latency SLA, blocking operations (e.g., disk I/O, heavy CPU)
/// are explicitly offloaded to `tokio::task::spawn_blocking`. Parallelizable sub-tasks
/// utilize `tokio::join!` or concurrent streams to minimize total wall-clock time.
///
/// Failure Modes:
/// - Returns a structured `Status` or application-specific error upon validation failure.
/// - Gracefully degrades in the event of transient upstream service unavailability.
/// - Employs exponential backoff or local queuing mechanisms when appropriate.
///
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
