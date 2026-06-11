#[cfg(test)]
mod tests {
    use crate::mesh::protocol::{Intent, TeammateMessage};
    use crate::mesh::memory_transport::MemoryMeshTransport;
    use crate::mesh::transport::MeshTransport;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use serde_json::json;

    #[tokio::test]
    async fn test_mesh_triage_ops_sales_negotiation() {
        let transport = Arc::new(MemoryMeshTransport::new());
        let tenant_id = "tenant_maya_bakery";

        let responses = Arc::new(Mutex::new(Vec::new()));
        let responses_clone = responses.clone();

        // Subscribe Work Triage Agent
        let _cancel_triage = transport.subscribe(tenant_id, Box::new(move |msg| {
            if msg.intent == Intent::Response && msg.target_agent_id == Some("triage_agent".to_string()) {

                let responses_clone2 = responses_clone.clone();
                let msg_clone = msg.clone();
                tokio::spawn(async move {
                    let mut res = responses_clone2.lock().await;
                    res.push(msg_clone);
                });
            }
        })).await.unwrap();

        let transport_ops = transport.clone();
        let _cancel_ops = transport.subscribe(tenant_id, Box::new(move |msg| {
            if msg.intent == Intent::Query && msg.target_department == Some("operations".to_string()) {
                let response = TeammateMessage {
                    message_id: "msg_ops_res_1".to_string(),
                    tenant_id: msg.tenant_id.clone(),
                    sender_agent_id: "ops_agent".to_string(),
                    target_department: None,
                    target_agent_id: Some(msg.sender_agent_id.clone()),
                    intent: Intent::Response,
                    payload: json!({ "available": true, "slots": 2 }),
                    context_id: msg.context_id.clone(),
                };
                let t = transport_ops.clone();
                tokio::spawn(async move {
                    let _ = t.publish(response).await;
                });
            }
        })).await.unwrap();

        let transport_sales = transport.clone();
        let _cancel_sales = transport.subscribe(tenant_id, Box::new(move |msg| {
            if msg.intent == Intent::Query && msg.target_department == Some("sales".to_string()) {
                let response = TeammateMessage {
                    message_id: "msg_sales_res_1".to_string(),
                    tenant_id: msg.tenant_id.clone(),
                    sender_agent_id: "sales_agent".to_string(),
                    target_department: None,
                    target_agent_id: Some(msg.sender_agent_id.clone()),
                    intent: Intent::Response,
                    payload: json!({ "available": true, "base_price": 50, "deposit": 25 }),
                    context_id: msg.context_id.clone(),
                };
                let t = transport_sales.clone();
                tokio::spawn(async move {
                    let _ = t.publish(response).await;
                });
            }
        })).await.unwrap();

        // Work Triage sends requests
        let req_ops = TeammateMessage {
            message_id: "msg_req_ops_1".to_string(),
            tenant_id: tenant_id.to_string(),
            sender_agent_id: "triage_agent".to_string(),
            target_department: Some("operations".to_string()),
            target_agent_id: None,
            intent: Intent::Query,
            payload: json!({ "check": "delivery_capacity", "date": "next Tuesday" }),
            context_id: Some("ctx_customer_dm_123".to_string()),
        };

        let req_sales = TeammateMessage {
            message_id: "msg_req_sales_1".to_string(),
            tenant_id: tenant_id.to_string(),
            sender_agent_id: "triage_agent".to_string(),
            target_department: Some("sales".to_string()),
            target_agent_id: None,
            intent: Intent::Query,
            payload: json!({ "check": "inventory_price", "item": "vegan cake" }),
            context_id: Some("ctx_customer_dm_123".to_string()),
        };

        transport.publish(req_ops).await.unwrap();
        transport.publish(req_sales).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let final_responses = responses.lock().await;
        assert_eq!(final_responses.len(), 2);

        let has_ops = final_responses.iter().any(|m| m.sender_agent_id == "ops_agent");
        let has_sales = final_responses.iter().any(|m| m.sender_agent_id == "sales_agent");

        assert!(has_ops);
        assert!(has_sales);
    }
}
