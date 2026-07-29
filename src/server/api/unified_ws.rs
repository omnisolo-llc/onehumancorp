use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Extension, Query},
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tokio::sync::broadcast;

#[derive(Deserialize)]
pub struct UnifiedWsQuery {
    pub channels: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "action")]
enum ClientMessage {
    #[serde(rename = "subscribe")]
    Subscribe {
        channel: String,
        topic: String,
    },
    #[serde(rename = "unsubscribe")]
    Unsubscribe {
        channel: String,
        topic: String,
    },
    #[serde(rename = "replay")]
    Replay {
        channel: String,
        from_seq: u64,
        #[serde(default)]
        topic: Option<String>,
    },
    #[serde(rename = "widget_message")]
    WidgetMessage {
        text: String,
        sender_id: String,
    }
}

#[derive(Serialize)]
struct EnvelopeMessage {
    channel: String,
    topic: String,
    data: serde_json::Value,
    seq: u64,
    ts: i64,
}

#[derive(Clone)]
#[allow(dead_code)]
struct ChannelState {
    seq_counters: HashMap<String, u64>,
    subscribed_topics: HashSet<String>,
}

#[allow(dead_code)]
static GLOBAL_SEQ: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

static GLOBAL_BROADCAST: std::sync::OnceLock<broadcast::Sender<String>> = std::sync::OnceLock::new();

fn get_broadcast_tx() -> &'static broadcast::Sender<String> {
    GLOBAL_BROADCAST.get_or_init(|| {
        let (tx, _) = broadcast::channel(4096);
        tx
    })
}

pub async fn unified_ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<server_common::Claims>,
    Query(query): Query<UnifiedWsQuery>,
    axum::extract::State((db, orchestrator)): axum::extract::State<(std::sync::Arc<crate::db::DB>, std::sync::Arc<crate::orchestration::departments::DepartmentOrchestrator>)>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) if !org_id.is_empty() => org_id.to_string(),
        _ => "default".to_string(),
    };

    let initial_channels: Vec<String> = query
        .channels
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    ws.on_upgrade(move |socket| handle_unified_socket(socket, tenant_id, initial_channels, db, orchestrator))
}

#[allow(dead_code)]
fn next_seq() -> u64 {
    GLOBAL_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn channel_topic_prefix(channel: &str, tenant_id: &str) -> String {
    format!("unified:{}:{}", channel, tenant_id)
}

fn build_envelope(channel: &str, topic: &str, data: serde_json::Value, seq: u64) -> String {
    let msg = EnvelopeMessage {
        channel: channel.to_string(),
        topic: topic.to_string(),
        data,
        seq,
        ts: chrono::Utc::now().timestamp_millis(),
    };
    serde_json::to_string(&msg).unwrap_or_default()
}

fn parse_envelope(raw: &str) -> Option<(String, String, serde_json::Value, u64)> {
    let v: serde_json::Value = serde_json::from_str(raw).ok()?;
    let channel = v.get("channel")?.as_str()?.to_string();
    let topic = v.get("topic")?.as_str()?.to_string();
    let data = v.get("data")?.clone();
    let seq = v.get("seq")?.as_u64()?;
    Some((channel, topic, data, seq))
}

async fn replay_from_redis(
    client: &redis::Client,
    channel: &str,
    tenant_id: &str,
    from_seq: u64,
    topic_filter: Option<&str>,
) -> Vec<String> {
    let stream_key = channel_topic_prefix(channel, tenant_id);
    let mut conn = match client.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let min_id = format!("{}-0", from_seq);
    let result: redis::RedisResult<redis::Value> = redis::cmd("XRANGE")
        .arg(&stream_key)
        .arg(&min_id)
        .arg("+")
        .arg("COUNT")
        .arg(200)
        .query_async(&mut conn)
        .await;

    let entries = match result {
        Ok(v) => v,
        Err(_) => return vec![],
    };

    let mut messages = Vec::new();
    if let Ok(sequence) = entries.into_sequence() {
        for entry in sequence {
            if let Ok(entry_parts) = entry.into_sequence() {
                if entry_parts.len() >= 2 {
                    if let Ok(fields) = entry_parts[1].clone().into_sequence() {
                        for field_pair in fields {
                            if let Ok(kv) = field_pair.into_sequence() {
                                if kv.len() == 2 {
                                    if let redis::Value::BulkString(key_bytes) = &kv[0] {
                                        if key_bytes == b"payload" {
                                            if let redis::Value::BulkString(payload_bytes) = &kv[1] {
                                                if let Ok(payload) = String::from_utf8(payload_bytes.clone()) {
                                                    if let Some((ch, topic, data, seq)) = parse_envelope(&payload) {
                                                        if topic_filter.map_or(true, |t| topic == t) {
                                                            messages.push(build_envelope(&ch, &topic, data, seq));
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    messages
}

async fn handle_unified_socket(
    socket: WebSocket,
    tenant_id: String,
    initial_channels: Vec<String>,
    db: std::sync::Arc<crate::db::DB>,
    orchestrator: std::sync::Arc<crate::orchestration::departments::DepartmentOrchestrator>
) {
    let (mut sender, mut receiver) = socket.split();

    let mut state = ChannelState {
        seq_counters: HashMap::new(),
        subscribed_topics: HashSet::new(),
    };

    for ch in &initial_channels {
        for topic_suffix in &["inventory", "orders", "tenant_events", "agent_feed"] {
            let topic = format!("{}:{}", topic_suffix, tenant_id);
            state.subscribed_topics.insert(format!("{}:{}", ch, topic));
        }
    }

    let redis_client_opt = crate::redis_pool::get_redis_client();
    let pubsub_client = redis_client_opt.clone();

    let (ws_tx, mut ws_rx) = tokio::sync::mpsc::channel::<String>(256);

    let tx = get_broadcast_tx();
    let mut broadcast_rx = tx.subscribe();

    let feed_prefix = channel_topic_prefix("feed", &tenant_id);
    let sync_prefix = channel_topic_prefix("sync", &tenant_id);
    let mesh_prefix = channel_topic_prefix("mesh", &tenant_id);

    let send_task = tokio::spawn(async move {
        while let Some(msg) = ws_rx.recv().await {
            if sender.send(WsMessage::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let ws_tx_clone = ws_tx.clone();
    let send_broadcast_task = tokio::spawn(async move {
        loop {
            match broadcast_rx.recv().await {
                Ok(raw) => {
                    if let Some((ch, topic, data, seq)) = parse_envelope(&raw) {
                        let topic_full = format!("{}:{}", ch, topic);
                        if topic_full.starts_with(&feed_prefix)
                            || topic_full.starts_with(&sync_prefix)
                            || topic_full.starts_with(&mesh_prefix)
                        {
                            let envelope = build_envelope(&ch, &topic, data, seq);
                            let _ = ws_tx_clone.send(envelope).await;
                        }
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => {}
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    let pubsub_task = if let Some(client) = pubsub_client {
        let ws_tx_ps = ws_tx.clone();
        let tenant_id_ps = tenant_id.clone();
        tokio::spawn(async move {
            let Ok(mut pubsub_conn) = client.get_async_pubsub().await else {
                return;
            };

            let channels_to_sub: Vec<String> = ["sync", "feed", "mesh"]
                .iter()
                .flat_map(|ch| {
                    let t = tenant_id_ps.clone();
                    ["inventory", "orders", "tenant_events", "agent_feed"]
                        .iter()
                        .map(move |topic| {
                            format!("unified:{}:{}:{}", ch, t, topic)
                        })
                })
                .collect();

            for ch_name in &channels_to_sub {
                let _ = pubsub_conn.subscribe(ch_name.as_str()).await;
            }

            let mut pubsub_stream = pubsub_conn.into_on_message();
            while let Some(msg) = pubsub_stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    let _ = ws_tx_ps.send(payload).await;
                }
            }
        })
    } else {
        tokio::spawn(async {})
    };

    let recv_task = {
        let ws_tx_rt = ws_tx.clone();
        let tenant_id_recv = tenant_id.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    WsMessage::Text(text) => {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                            match client_msg {
                                ClientMessage::Subscribe { channel, topic } => {
                                    let envelope = serde_json::json!({
                                        "action": "subscribed",
                                        "channel": channel,
                                        "topic": topic,
                                        "status": "ok"
                                    });
                                    let _ = ws_tx_rt.send(envelope.to_string()).await;
                                }
                                ClientMessage::Unsubscribe { channel, topic } => {
                                    let envelope = serde_json::json!({
                                        "action": "unsubscribed",
                                        "channel": channel,
                                        "topic": topic,
                                        "status": "ok"
                                    });
                                    let _ = ws_tx_rt.send(envelope.to_string()).await;
                                }
                                ClientMessage::Replay { channel, from_seq, topic } => {
                                    if let Some(client) = redis_client_opt.clone() {
                                        let count = {
                                            let msgs = replay_from_redis(
                                                &client,
                                                &channel,
                                                &tenant_id_recv,
                                                from_seq,
                                                topic.as_deref(),
                                            )
                                            .await;
                                            let count = msgs.len();
                                            for msg_str in msgs {
                                                let _ = ws_tx_rt.send(msg_str).await;
                                            }
                                            count
                                        };
                                        let ack = serde_json::json!({
                                            "action": "replay_done",
                                            "channel": channel,
                                            "from_seq": from_seq,
                                            "count": count
                                        });
                                        let _ = ws_tx_rt.send(ack.to_string()).await;
                                    }
                                }
                                ClientMessage::WidgetMessage { text, sender_id } => {
                                    if let Ok(tid) = uuid::Uuid::parse_str(&tenant_id_recv) {
                                        let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
                                        let source = "widget".to_string();

                                        // Resolve identity if possible, though for a web widget it might just be the sender_id initially
                                        let resolver = crate::orchestration::identity_resolution::IdentityResolver::new(db.clone());
                                        let customer_id_result = resolver.resolve_or_create_customer(&tenant_id_recv, &sender_id, &source).await;

                                        let contact_id = if let Ok(cid) = customer_id_result.clone() {
                                            if let Ok(c_uuid) = uuid::Uuid::parse_str(&cid) {
                                                if let Ok(c) = sqlx::query_as::<_, crate::services::chat::models::ChatContact>("SELECT * FROM chat_contacts WHERE id = $1 AND tenant_id = $2").bind(&c_uuid).bind(&tid).fetch_one(&db.pool).await {
                                                    c.id
                                                } else {
                                                    let c = chat_service.create_contact(tid, Some(sender_id.clone()), None, None).await.unwrap_or_else(|_| crate::services::chat::models::ChatContact {
                                                         id: c_uuid, tenant_id: tid, name: Some(sender_id.clone()), email: None, phone: None, created_at: chrono::Utc::now(), updated_at: chrono::Utc::now()
                                                    });
                                                    c.id
                                                }
                                            } else {
                                                let c = chat_service.create_contact(tid, Some(sender_id.clone()), None, None).await.unwrap_or_else(|_| crate::services::chat::models::ChatContact {
                                                     id: uuid::Uuid::new_v4(), tenant_id: tid, name: Some(sender_id.clone()), email: None, phone: None, created_at: chrono::Utc::now(), updated_at: chrono::Utc::now()
                                                });
                                                c.id
                                            }
                                        } else {
                                            let c = chat_service.create_contact(tid, Some(sender_id.clone()), None, None).await.unwrap_or_else(|_| crate::services::chat::models::ChatContact {
                                                 id: uuid::Uuid::new_v4(), tenant_id: tid, name: Some(sender_id.clone()), email: None, phone: None, created_at: chrono::Utc::now(), updated_at: chrono::Utc::now()
                                            });
                                            c.id
                                        };

                                        // Find or create inbox
                                        let inbox = if let Ok(inbox) = sqlx::query_as::<_, crate::services::chat::models::ChatInbox>("SELECT * FROM chat_inboxes WHERE tenant_id = $1 LIMIT 1").bind(&tid).fetch_one(&db.pool).await {
                                            inbox
                                        } else {
                                            chat_service.create_inbox(tid, "Web Widget Inbox".to_string()).await.unwrap_or_else(|_| crate::services::chat::models::ChatInbox {
                                                id: uuid::Uuid::new_v4(), tenant_id: tid, name: "Fallback".into(), created_at: chrono::Utc::now(), updated_at: chrono::Utc::now()
                                            })
                                        };

                                        // Find or create channel
                                        let _ = if let Ok(chan) = sqlx::query_as::<_, crate::services::chat::models::ChatChannel>("SELECT * FROM chat_channels WHERE tenant_id = $1 AND channel_type = 'widget' LIMIT 1").bind(&tid).fetch_one(&db.pool).await {
                                            chan
                                        } else {
                                            chat_service.create_channel(tid, inbox.id, "widget".to_string(), serde_json::json!({})).await.unwrap_or_else(|_| crate::services::chat::models::ChatChannel {
                                                id: uuid::Uuid::new_v4(), tenant_id: tid, inbox_id: inbox.id, channel_type: "widget".into(), config: serde_json::json!({}), created_at: chrono::Utc::now(), updated_at: chrono::Utc::now()
                                            })
                                        };

                                        // Find or create conversation
                                        let conversation = if let Ok(conv) = sqlx::query_as::<_, crate::services::chat::models::ChatConversation>("SELECT * FROM chat_conversations WHERE tenant_id = $1 AND contact_id = $2 LIMIT 1").bind(&tid).bind(&contact_id).fetch_one(&db.pool).await {
                                            conv
                                        } else {
                                            chat_service.start_conversation(tid, inbox.id, contact_id, None).await.unwrap_or_else(|_| crate::services::chat::models::ChatConversation {
                                                id: uuid::Uuid::new_v4(), tenant_id: tid, inbox_id: inbox.id, contact_id: contact_id, assignee_id: None, status: "open".into(), created_at: chrono::Utc::now(), updated_at: chrono::Utc::now()
                                            })
                                        };

                                        // Insert message
                                        let _ = chat_service.send_message(tid, conversation.id, "contact".to_string(), Some(contact_id), text.clone()).await;

                                        let inbox_message_id = uuid::Uuid::new_v4().to_string();

                                        // Dispatch event to Orchestrator
                                        let mut payload = serde_json::json!({
                                            "message_id": inbox_message_id, // we use a dummy id here for backwards compatibility if needed
                                            "inbox_message_id": inbox_message_id,
                                            "source": "widget",
                                            "content": text.clone(),
                                            "sender_id": sender_id.clone()
                                        });
                                        if let Ok(c_id) = customer_id_result {
                                            payload["customer_id"] = serde_json::json!(c_id);
                                        }

                                        let event = crate::orchestration::departments::types::DepartmentEvent {
                                            id: uuid::Uuid::new_v4().to_string(),
                                            tenant_id: tenant_id_recv.clone(),
                                            event_type: "tenant.omnichannel.message.received".to_string(),
                                            payload,
                                        };

                                        let _ = orchestrator.dispatch_event(event).await;

                                        let ack = serde_json::json!({
                                            "action": "widget_message_received",
                                            "status": "ok"
                                        });
                                        let _ = ws_tx_rt.send(ack.to_string()).await;
                                    }
                                }
                            }
                        }
                    }
                    WsMessage::Close(_) => break,
                    _ => {}
                }
            }
        })
    };

    tokio::select! {
        _ = send_task => {}
        _ = send_broadcast_task => {}
        _ = pubsub_task => {}
        _ = recv_task => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_serialization() {
        let envelope = build_envelope(
            "sync",
            "inventory:tenant-123",
            serde_json::json!({"event": "updated"}),
            42,
        );
        let v: serde_json::Value = serde_json::from_str(&envelope).unwrap();
        assert_eq!(v["channel"], "sync");
        assert_eq!(v["topic"], "inventory:tenant-123");
        assert_eq!(v["seq"], 42);
        assert_eq!(v["data"]["event"], "updated");
        assert!(v["ts"].as_i64().unwrap() > 0);
    }

    #[test]
    fn test_parse_envelope() {
        let envelope = build_envelope(
            "feed",
            "agent_feed:tenant-456",
            serde_json::json!({"action": "new_item"}),
            99,
        );
        let result = parse_envelope(&envelope);
        assert!(result.is_some());
        let (ch, topic, data, seq) = result.unwrap();
        assert_eq!(ch, "feed");
        assert_eq!(topic, "agent_feed:tenant-456");
        assert_eq!(data["action"], "new_item");
        assert_eq!(seq, 99);
    }

    #[test]
    fn test_parse_envelope_invalid() {
        assert!(parse_envelope("not json").is_none());
        assert!(parse_envelope("{}").is_none());
        assert!(parse_envelope(r#"{"channel":"x"}"#).is_none());
    }

    #[test]
    fn test_next_seq_is_monotonic() {
        let s1 = next_seq();
        let s2 = next_seq();
        let s3 = next_seq();
        assert!(s1 < s2);
        assert!(s2 < s3);
    }

    #[test]
    fn test_channel_topic_prefix() {
        assert_eq!(
            channel_topic_prefix("sync", "tenant-1"),
            "unified:sync:tenant-1"
        );
        assert_eq!(
            channel_topic_prefix("mesh", "tenant-99"),
            "unified:mesh:tenant-99"
        );
    }

    #[test]
    fn test_client_message_deserialization_subscribe() {
        let json = r#"{"action": "subscribe", "channel": "sync", "topic": "inventory:tenant-123"}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        match msg {
            ClientMessage::Subscribe { channel, topic } => {
                assert_eq!(channel, "sync");
                assert_eq!(topic, "inventory:tenant-123");
            }
            _ => panic!("expected Subscribe"),
        }
    }

    #[test]
    fn test_client_message_deserialization_unsubscribe() {
        let json = r#"{"action": "unsubscribe", "channel": "mesh", "topic": "direct:agent-1"}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        match msg {
            ClientMessage::Unsubscribe { channel, topic } => {
                assert_eq!(channel, "mesh");
                assert_eq!(topic, "direct:agent-1");
            }
            _ => panic!("expected Unsubscribe"),
        }
    }

    #[test]
    fn test_client_message_deserialization_replay() {
        let json = r#"{"action": "replay", "channel": "sync", "from_seq": 12340, "topic": "inventory:tenant-123"}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        match msg {
            ClientMessage::Replay { channel, from_seq, topic } => {
                assert_eq!(channel, "sync");
                assert_eq!(from_seq, 12340);
                assert_eq!(topic, Some("inventory:tenant-123".to_string()));
            }
            _ => panic!("expected Replay"),
        }
    }

    #[test]
    fn test_client_message_deserialization_replay_no_topic() {
        let json = r#"{"action": "replay", "channel": "feed", "from_seq": 50}"#;
        let msg: ClientMessage = serde_json::from_str(json).unwrap();
        match msg {
            ClientMessage::Replay { channel, from_seq, topic } => {
                assert_eq!(channel, "feed");
                assert_eq!(from_seq, 50);
                assert!(topic.is_none());
            }
            _ => panic!("expected Replay"),
        }
    }

    #[test]
    fn test_channel_state_initial_topics() {
        let mut state = ChannelState {
            seq_counters: HashMap::new(),
            subscribed_topics: HashSet::new(),
        };
        for topic_suffix in &["inventory", "orders", "tenant_events"] {
            state
                .subscribed_topics
                .insert(format!("sync:{}:tenant-1", topic_suffix));
        }
        assert!(state.subscribed_topics.contains("sync:inventory:tenant-1"));
        assert!(state.subscribed_topics.contains("sync:orders:tenant-1"));
        assert!(!state.subscribed_topics.contains("mesh:inventory:tenant-1"));
    }

    #[test]
    fn test_envelope_roundtrip() {
        let envelope = build_envelope(
            "mesh",
            "direct:agent-7",
            serde_json::json!({"hello": "world", "nested": {"a": 1}}),
            1000,
        );
        let parsed = parse_envelope(&envelope).unwrap();
        assert_eq!(parsed.0, "mesh");
        assert_eq!(parsed.1, "direct:agent-7");
        assert_eq!(parsed.2["hello"], "world");
        assert_eq!(parsed.2["nested"]["a"], 1);
        assert_eq!(parsed.3, 1000);
    }

    #[test]
    fn test_get_broadcast_tx_returns_same_instance() {
        let tx1 = get_broadcast_tx();
        let tx2 = get_broadcast_tx();
        assert!(std::ptr::eq(tx1, tx2));
    }

    #[test]
    fn test_envelope_with_complex_data() {
        let envelope = build_envelope(
            "sync",
            "inventory:tenant-abc",
            serde_json::json!({
                "items": [{"id": 1, "name": "Widget"}],
                "meta": {"count": 1, "tenant": "tenant-abc"}
            }),
            500,
        );
        let parsed = parse_envelope(&envelope).unwrap();
        assert_eq!(parsed.2["items"][0]["name"], "Widget");
        assert_eq!(parsed.2["meta"]["count"], 1);
    }

    #[test]
    fn test_subscribe_unsubscribe_roundtrip() {
        let sub = r#"{"action": "subscribe", "channel": "sync", "topic": "orders:tenant-1"}"#;
        let unsub = r#"{"action": "unsubscribe", "channel": "sync", "topic": "orders:tenant-1"}"#;
        let sub_msg: ClientMessage = serde_json::from_str(sub).unwrap();
        let unsub_msg: ClientMessage = serde_json::from_str(unsub).unwrap();
        match (sub_msg, unsub_msg) {
            (ClientMessage::Subscribe { channel: c1, topic: t1 }, ClientMessage::Unsubscribe { channel: c2, topic: t2 }) => {
                assert_eq!(c1, c2);
                assert_eq!(t1, t2);
            }
            _ => panic!("expected matching subscribe/unsubscribe pair"),
        }
    }
}
