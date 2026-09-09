//! Parallel execution through registered agents, with correlated response streams.
use crate::hub::Hub;
use axum::{
    Json, Router,
    extract::{Extension, State},
    http::StatusCode,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
    routing::post,
};
use futures_util::{StreamExt, stream::FuturesUnordered};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use server_common::Claims;
use server_omnisolo::orchestration::{Agent, Message};
use std::{
    collections::HashSet,
    convert::Infallible,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Deserialize)]
pub struct OrchestrateRequest {
    pub task: String,
    pub experts: Vec<String>,
    #[serde(default = "default_stream")]
    pub stream: bool,
}
fn default_stream() -> bool {
    true
}
impl OrchestrateRequest {
    fn validate(&self) -> Result<(), StatusCode> {
        let unique: HashSet<_> = self.experts.iter().collect();
        if self.task.trim().is_empty()
            || self.task.len() > 32768
            || self.experts.is_empty()
            || self.experts.len() > 16
            || unique.len() != self.experts.len()
            || self
                .experts
                .iter()
                .any(|id| id.is_empty() || id.len() > 128)
        {
            return Err(StatusCode::BAD_REQUEST);
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct OrchestrationEvent {
    event: &'static str,
    data: Value,
}
async fn emit(tx: &mpsc::Sender<OrchestrationEvent>, event: &'static str, data: Value) {
    let _ = tx.send(OrchestrationEvent { event, data }).await;
}
fn is_correlated_response(message: &Message, expert: &str, reply: &str, plan: &str) -> bool {
    message.from_agent == expert && message.to_agent == reply && message.meeting_id == plan
}
pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/api/v1/agents/orchestrate", post(orchestrate))
        .with_state(hub)
}
async fn orchestrate(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Json(request): Json<OrchestrateRequest>,
) -> Result<axum::response::Response, StatusCode> {
    request.validate()?;
    let org = claims
        .organization_id
        .filter(|org| !org.is_empty())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let mut experts = Vec::new();
    for id in &request.experts {
        let agent = hub
            .get_agent(id)
            .await
            .filter(|agent| agent.organization_id == org)
            .ok_or(StatusCode::NOT_FOUND)?;
        experts.push(agent);
    }
    let (tx, rx) = mpsc::channel(64);
    tokio::spawn(run(hub, org, request.task, experts, tx));
    if request.stream {
        let stream = ReceiverStream::new(rx).map(|item| {
            Ok::<_, Infallible>(
                Event::default()
                    .event(item.event)
                    .json_data(item.data)
                    .expect("JSON event serializes"),
            )
        });
        Ok(Sse::new(stream)
            .keep_alive(KeepAlive::default())
            .into_response())
    } else {
        let events: Vec<_> = ReceiverStream::new(rx).collect().await;
        Ok(Json(events).into_response())
    }
}
async fn run(
    hub: Arc<Hub>,
    org: String,
    task: String,
    experts: Vec<Agent>,
    tx: mpsc::Sender<OrchestrationEvent>,
) {
    let plan = uuid::Uuid::new_v4().to_string();
    let reply = format!("orchestration-{plan}");
    hub.register_agent(Agent {
        id: reply.clone(),
        name: "Orchestration coordinator".into(),
        role: "coordinator".into(),
        organization_id: org,
        status: "RUNNING".into(),
        provider_type: "internal".into(),
    })
    .await;
    emit(&tx, "orchestration_start", json!({"plan_id": plan, "steps": experts.iter().enumerate().map(|(i, agent)| json!({"step": i+1, "agent_id": agent.id, "role": agent.role})).collect::<Vec<_>>() })).await;
    let mut pending = FuturesUnordered::new();
    for (index, expert) in experts.iter().cloned().enumerate() {
        let hub = hub.clone();
        let tx = tx.clone();
        let plan = plan.clone();
        let reply = reply.clone();
        let task = task.clone();
        let mut rx = hub.subscribe(reply.clone()).await;
        pending.push(async move {
            let started = Instant::now();
            emit(&tx, "agent_start", json!({"agent_id": expert.id, "step": index+1})).await;
            let request = Message { id: uuid::Uuid::new_v4().to_string(), from_agent: reply.clone(), to_agent: expert.id.clone(), r#type: "TaskDelegation".into(), content: json!({"task": task, "role": expert.role, "plan_id": plan, "reply_to": reply, "response_types": ["task_progress", "task_result", "task_error"]}).to_string(), occurred_at_unix: chrono::Utc::now().timestamp(), meeting_id: plan.clone() };
            let outcome = match hub.clone().delegate_task(reply.clone(), expert.id.clone(), request).await {
                Err(_) => Err("delegation_rejected"),
                Ok(()) => match tokio::time::timeout(Duration::from_secs(120), async {
                    loop {
                        let message = rx.recv().await.map_err(|_| "response_stream_interrupted")?;
                        if !is_correlated_response(&message, &expert.id, &reply, &plan) { continue; }
                        match message.r#type.as_str() {
                            "task_result" => return Ok(message.content),
                            "task_error" => return Err("agent_execution_failed"),
                            "task_progress" => emit(&tx, "agent_progress", json!({"agent_id": expert.id, "status": message.content})).await,
                            _ => {}
                        }
                    }
                }).await { Ok(result) => result, Err(_) => Err("agent_timeout") }
            };
            if matches!(outcome, Err("agent_timeout" | "response_stream_interrupted")) {
                let _ = hub.clone().publish(Message { id: uuid::Uuid::new_v4().to_string(), from_agent: reply.clone(), to_agent: expert.id.clone(), r#type: "task_cancel".into(), content: "orchestration_response_unavailable".into(), occurred_at_unix: chrono::Utc::now().timestamp(), meeting_id: plan.clone() }).await;
            }
            hub.record_agent_execution(&expert.id, started.elapsed(), outcome.is_err()).await;
            let result = match outcome {
                Ok(content) => json!({"agent_id": expert.id, "result": content, "status": "completed"}),
                Err(error) => json!({"agent_id": expert.id, "error": error, "status": "failed"}),
            };
            emit(&tx, if result["status"] == "completed" { "agent_complete" } else { "agent_error" }, result.clone()).await;
            result
        });
    }
    let mut results = Vec::new();
    loop {
        tokio::select! {
            biased;
            _ = tx.closed() => {
                drop(pending);
                for expert in &experts {
                    let _ = hub.clone().publish(Message { id: uuid::Uuid::new_v4().to_string(), from_agent: reply.clone(), to_agent: expert.id.clone(), r#type: "task_cancel".into(), content: "orchestration_client_disconnected".into(), occurred_at_unix: chrono::Utc::now().timestamp(), meeting_id: plan.clone() }).await;
                }
                hub.remove_transient_agent(&reply).await;
                return;
            }
            result = pending.next() => match result { Some(result) => results.push(result), None => break }
        }
    }
    let status = if results.iter().all(|result| result["status"] == "completed") {
        "completed"
    } else {
        "failed"
    };
    emit(
        &tx,
        "orchestration_complete",
        json!({"plan_id": plan, "status": status, "results": results}),
    )
    .await;
    hub.remove_transient_agent(&reply).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hub() -> Arc<Hub> {
        let pool = crate::db::secure_pg_pool_options()
            .connect_lazy("postgres://localhost/test")
            .unwrap();
        let (tx, _) = mpsc::channel(8);
        Arc::new(Hub::new(tx, pool))
    }

    #[tokio::test]
    async fn parallel_execution_reports_only_actual_correlated_results_and_cleans_up() {
        let hub = hub();
        let mut experts = Vec::new();
        for id in ["expert-a", "expert-b"] {
            let agent = Agent {
                id: id.into(),
                organization_id: "org".into(),
                role: "analyst".into(),
                ..Default::default()
            };
            hub.register_agent(agent.clone()).await;
            let mut inbox = hub.subscribe(id.into()).await;
            let worker_hub = hub.clone();
            tokio::spawn(async move {
                let request = inbox.recv().await.unwrap();
                worker_hub
                    .publish(Message {
                        id: uuid::Uuid::new_v4().to_string(),
                        from_agent: id.into(),
                        to_agent: request.from_agent,
                        r#type: "task_result".into(),
                        content: format!("measured result from {id}"),
                        meeting_id: request.meeting_id,
                        ..Default::default()
                    })
                    .await
                    .unwrap();
            });
            experts.push(agent);
        }
        let (tx, rx) = mpsc::channel(64);
        let task = tokio::spawn(run(
            hub.clone(),
            "org".into(),
            "analyze".into(),
            experts,
            tx,
        ));
        let events = tokio::time::timeout(
            Duration::from_secs(2),
            ReceiverStream::new(rx).collect::<Vec<_>>(),
        )
        .await
        .unwrap();
        task.await.unwrap();
        let complete = events
            .iter()
            .filter(|event| event.event == "agent_complete")
            .collect::<Vec<_>>();
        assert_eq!(complete.len(), 2);
        assert!(complete.iter().all(|event| {
            event.data["result"]
                .as_str()
                .unwrap()
                .starts_with("measured result from")
        }));
        assert_eq!(events.last().unwrap().data["status"], "completed");
        assert_eq!(hub.get_agents_by_org("org").await.len(), 2);
        assert_eq!(
            hub.agent_measurements("expert-a")
                .await
                .0
                .messages_processed,
            1
        );
    }

    #[tokio::test]
    async fn browser_disconnect_sends_cancellation_and_removes_coordinator() {
        let hub = hub();
        let expert = Agent {
            id: "expert".into(),
            organization_id: "org".into(),
            ..Default::default()
        };
        hub.register_agent(expert.clone()).await;
        let mut inbox = hub.subscribe(expert.id.clone()).await;
        let (tx, mut rx) = mpsc::channel(64);
        let task = tokio::spawn(run(
            hub.clone(),
            "org".into(),
            "analyze".into(),
            vec![expert],
            tx,
        ));
        while rx.recv().await.unwrap().event != "agent_start" {}
        drop(rx);
        task.await.unwrap();
        let mut cancelled = false;
        while let Ok(message) = inbox.try_recv() {
            cancelled |= message.r#type == "task_cancel";
        }
        assert!(cancelled);
        assert_eq!(hub.get_agents_by_org("org").await.len(), 1);
    }

    #[tokio::test]
    async fn delegation_rejects_cross_organization_and_unavailable_roles() {
        let hub = hub();
        for (id, org) in [("sender", "org-a"), ("recipient", "org-b")] {
            hub.register_agent(Agent {
                id: id.into(),
                organization_id: org.into(),
                ..Default::default()
            })
            .await;
        }
        assert!(
            hub.clone()
                .delegate_task("sender".into(), "recipient".into(), Message::default())
                .await
                .is_err()
        );
        assert!(
            hub.delegate_sub_task("sender", "unregistered-role", "do work", "thread")
                .await
                .is_err()
        );
    }

    #[test]
    fn rejects_empty_duplicate_and_unbounded_work() {
        for request in [
            OrchestrateRequest {
                task: " ".into(),
                experts: vec!["a".into()],
                stream: true,
            },
            OrchestrateRequest {
                task: "work".into(),
                experts: vec![],
                stream: true,
            },
            OrchestrateRequest {
                task: "work".into(),
                experts: vec!["a".into(), "a".into()],
                stream: true,
            },
            OrchestrateRequest {
                task: "work".into(),
                experts: (0..17).map(|n| n.to_string()).collect(),
                stream: true,
            },
        ] {
            assert!(request.validate().is_err());
        }
    }

    #[test]
    fn completion_requires_matching_agent_and_correlation() {
        let message = Message {
            from_agent: "expert".into(),
            to_agent: "reply".into(),
            meeting_id: "plan".into(),
            r#type: "task_result".into(),
            content: "actual result".into(),
            ..Default::default()
        };
        assert!(is_correlated_response(&message, "expert", "reply", "plan"));
        assert!(!is_correlated_response(&message, "other", "reply", "plan"));
        assert!(!is_correlated_response(
            &message,
            "expert",
            "reply",
            "other-plan"
        ));
    }
}
