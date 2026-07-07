use crate::db::{LocalDb, OfflineMutation};
use serde_json::json;
use std::time::Duration;
use tauri::AppHandle;
use tauri::Emitter;

pub async fn start_background_sync(app: AppHandle, db: LocalDb) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            let is_online = check_online().await;

            // Just emit online status occasionally so frontend UI can display the indicator
            let pending_count = db.get_pending_mutations().await.map(|m| m.len()).unwrap_or(0);

            app.emit("sync_status", json!({
                "online": is_online,
                "pending": pending_count
            })).unwrap_or_default();

            if is_online {
                if let Ok(mutations) = db.get_pending_mutations().await {
                    if !mutations.is_empty() {
                        app.emit("sync_status", json!({
                            "online": is_online,
                            "pending": mutations.len(),
                            "syncing": true
                        })).unwrap_or_default();

                        if let Ok(success_ids) = push_mutations(mutations).await {
                            let _ = db.remove_mutations(&success_ids).await;
                        }
                    }
                }
            }
        }
    });
}

async fn check_online() -> bool {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/health", backend_url);
    if let Ok(client) = reqwest::Client::builder().timeout(Duration::from_secs(3)).build() {
        return client.get(&url).send().await.is_ok();
    }
    false
}

async fn push_mutations(mutations: Vec<OfflineMutation>) -> Result<Vec<String>, String> {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/v1/sync/offline", backend_url);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let payload = json!({
        "mutations": mutations
    });

    let res = client.post(&url)
        .json(&payload)
        .header("x-spiffe-id", "spiffe://ohc/org/e2e-tenant/agent/tauri-client")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let resp_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        if resp_json.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
            return Ok(mutations.into_iter().map(|m| m.transaction_id).collect());
        }
    }

    Err("Failed to sync".to_string())
}
