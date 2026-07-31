use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;
use super::models::{message, conversation};

pub struct AppState {
    pub tenant_channels: Arc<Mutex<HashMap<Uuid, broadcast::Sender<message::Model>>>>,
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            tenant_channels: Arc::new(Mutex::new(HashMap::new())),
            db,
        }
    }

    pub async fn get_or_create_channel(&self, tenant_id: Uuid) -> broadcast::Sender<message::Model> {
        let mut channels = self.tenant_channels.lock().await;
        if let Some(sender) = channels.get(&tenant_id) {
            sender.clone()
        } else {
            let (sender, _) = broadcast::channel(100);
            channels.insert(tenant_id, sender.clone());
            sender
        }
    }
}

pub fn chat_ws_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ws/:tenant_id", get(ws_handler))
        .with_state(state)
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, tenant_id))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>, tenant_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let tx = state.get_or_create_channel(tenant_id).await;
    let mut rx = tx.subscribe();

    // Spawn a task to forward messages from the broadcast channel to the websocket
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // Verify message belongs to tenant before sending
            if msg.tenant_id == tenant_id {
                let json_msg = serde_json::to_string(&msg).unwrap();
                if sender.send(Message::Text(json_msg.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Spawn a task to forward messages from the websocket to the broadcast channel and save to DB
    let tx_clone = tx.clone();
    let db = state.db.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if let Ok(mut msg) = serde_json::from_str::<message::Model>(&text) {
                // Enforce tenant_id from connection and ignore provided ID
                msg.tenant_id = tenant_id;
                msg.id = Uuid::new_v4();

                // Verify conversation belongs to tenant
                let convo = conversation::Entity::find_by_id(msg.conversation_id)
                    .filter(conversation::Column::TenantId.eq(tenant_id))
                    .one(&db)
                    .await;

                if let Ok(Some(_)) = convo {
                    // Save to DB
                    use sea_orm::ActiveValue::Set;
                    let active_msg = message::ActiveModel {
                        id: Set(msg.id),
                        tenant_id: Set(msg.tenant_id),
                        conversation_id: Set(msg.conversation_id),
                        sender_id: Set(msg.sender_id),
                        content: Set(msg.content.clone()),
                        created_at: Set(chrono::Utc::now()),
                        updated_at: Set(chrono::Utc::now()),
                        ..Default::default()
                    };

                    if let Ok(inserted) = active_msg.insert(&db).await {
                        let _ = tx_clone.send(inserted);
                    }
                }
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}
