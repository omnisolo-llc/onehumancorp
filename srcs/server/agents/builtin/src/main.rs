mod agent;
mod hub;
mod llm;
mod tools;

use anyhow::Context;
use std::env;
use tokio_stream::StreamExt;
use tracing::{error, info, warn};

use agent::{run_agent, AgentConfig};
use hub::{Agent, HubMessage, HubServiceClient, PublishMessageRequest, RegisterAgentRequest};
use llm::default_llm_client;
use tools::ToolExecutor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let grpc_endpoint =
        env::var("OHC_GRPC_ENDPOINT").unwrap_or_else(|_| "http://localhost:9090".to_string());
    let agent_name =
        env::var("OHC_AGENT_NAME").unwrap_or_else(|_| "builtin-agent".to_string());
    let agent_role =
        env::var("OHC_AGENT_ROLE").unwrap_or_else(|_| "SOFTWARE_ENGINEER".to_string());
    let agent_id = env::var("OHC_AGENT_ID")
        .unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());

    info!(
        endpoint = %grpc_endpoint,
        name = %agent_name,
        role = %agent_role,
        id = %agent_id,
        "starting OHC builtin agent"
    );

    // Register with hub
    {
        let mut client = HubServiceClient::connect(grpc_endpoint.clone())
            .await
            .context("connect to hub for registration")?;

        let reg = client
            .register_agent(RegisterAgentRequest {
                agent: Some(Agent {
                    id: agent_id.clone(),
                    name: agent_name.clone(),
                    role: agent_role.clone(),
                    organization_id: String::new(),
                    status: "active".to_string(),
                    provider_type: "builtin".to_string(),
                }),
            })
            .await
            .map_err(|s| anyhow::anyhow!("register_agent: {}", s))?;

        if reg.success {
            info!("agent registered successfully");
        } else {
            warn!("agent registration returned success=false");
        }
    }

    // Stream messages with reconnect loop
    let mut backoff_secs: u64 = 1;
    loop {
        info!("connecting to message stream");
        match stream_loop(
            grpc_endpoint.clone(),
            agent_id.clone(),
            agent_name.clone(),
        )
        .await
        {
            Ok(_) => {
                info!("stream ended cleanly, reconnecting in {}s", backoff_secs);
                backoff_secs = 1; // reset on clean exit
            }
            Err(e) => {
                error!(error = %e, "stream error, reconnecting in {}s", backoff_secs);
                backoff_secs = (backoff_secs * 2).min(60); // exponential backoff, cap at 60s
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(backoff_secs)).await;
    }
}

async fn stream_loop(
    grpc_endpoint: String,
    agent_id: String,
    agent_name: String,
) -> anyhow::Result<()> {
    let mut client = HubServiceClient::connect(grpc_endpoint.clone())
        .await
        .context("connect to hub for streaming")?;

    let mut stream = client
        .stream_messages(agent_id.clone())
        .await
        .map_err(|s| anyhow::anyhow!("stream_messages: {}", s))?;

    while let Some(result) = stream.next().await {
        match result {
            Ok(msg) => {
                handle_message(msg, grpc_endpoint.clone(), agent_id.clone(), agent_name.clone())
                    .await;
            }
            Err(e) => {
                return Err(anyhow::anyhow!("stream recv error: {}", e));
            }
        }
    }

    Ok(())
}

async fn handle_message(
    msg: HubMessage,
    grpc_endpoint: String,
    agent_id: String,
    agent_name: String,
) {
    let msg_type = msg.r#type.as_str();

    match msg_type {
        "TaskAssignment" | "TaskDelegation" => {
            let grpc_ep = grpc_endpoint.clone();
            let aid = agent_id.clone();
            let aname = agent_name.clone();
            tokio::spawn(async move {
                process_task(msg, grpc_ep, aid, aname).await;
            });
        }
        _ => {
            info!(msg_type, from = %msg.from_agent, "ignoring message type");
        }
    }
}

async fn process_task(
    msg: HubMessage,
    grpc_endpoint: String,
    agent_id: String,
    agent_name: String,
) {
    info!(
        msg_type = %msg.r#type,
        from = %msg.from_agent,
        "processing task"
    );

    let (task_id, prompt) = parse_task_message(&msg);

    let llm = default_llm_client();
    let mut executor =
        ToolExecutor::new(grpc_endpoint.clone(), agent_id.clone(), agent_name.clone());

    let config = AgentConfig::default();
    let result = run_agent(&prompt, config, llm.as_ref(), &mut executor).await;

    let (status, result_text) = match result {
        Ok(text) => ("completed", text),
        Err(e) => {
            error!(error = %e, "agent task failed");
            ("failed", format!("Error: {}", e))
        }
    };

    let notification = format!(
        "<task-notification>\n<task-id>{}</task-id>\n<status>{}</status>\n<result>{}</result>\n</task-notification>",
        task_id, status, result_text
    );

    publish_result(&grpc_endpoint, &agent_name, &msg.from_agent, &notification).await;
}

fn parse_task_message(msg: &HubMessage) -> (String, String) {
    // Try parsing as JSON
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(&msg.content) {
        // TaskAssignment format
        if let Some(prompt) = v.get("prompt").and_then(|p| p.as_str()) {
            let task_id = v
                .get("issue_id")
                .and_then(|id| id.as_str())
                .unwrap_or(&msg.id)
                .to_string();
            let directive = v
                .get("directive")
                .and_then(|d| d.as_str())
                .unwrap_or("");
            let full_prompt = if directive.is_empty() {
                prompt.to_string()
            } else {
                format!("{}\n\n{}", directive, prompt)
            };
            return (task_id, full_prompt);
        }
        // TaskDelegation format
        if let Some(instruction) = v.get("instruction").and_then(|i| i.as_str()) {
            let task_id = v
                .get("task_id")
                .and_then(|id| id.as_str())
                .unwrap_or(&msg.id)
                .to_string();
            return (task_id, instruction.to_string());
        }
    }

    // Fallback: use raw content as prompt
    (msg.id.clone(), msg.content.clone())
}

async fn publish_result(
    grpc_endpoint: &str,
    from_agent: &str,
    to_agent: &str,
    content: &str,
) {
    match HubServiceClient::connect(grpc_endpoint.to_string()).await {
        Ok(mut client) => {
            let msg = hub::HubMessage {
                id: uuid::Uuid::new_v4().to_string(),
                from_agent: from_agent.to_string(),
                to_agent: to_agent.to_string(),
                r#type: "TaskResult".to_string(),
                content: content.to_string(),
                meeting_id: String::new(),
                occurred_at_unix: chrono::Utc::now().timestamp(),
            };
            match client
                .publish(PublishMessageRequest { message: Some(msg) })
                .await
            {
                Ok(_) => info!("task result published to {}", to_agent),
                Err(e) => error!(error = %e, "failed to publish task result"),
            }
        }
        Err(e) => {
            error!(error = %e, "failed to connect hub for result publish");
        }
    }
}
