use tauri::Manager;
use rusqlite::{params, Connection};
use std::sync::Mutex;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AiProviderConfig {
    provider: String,
    model: String,
    base_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    api_key: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AiProviderView {
    provider: String,
    model: String,
    base_url: String,
    api_key_set: bool,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct AiProviderTestResult {
    ok: bool,
    status: u16,
    message: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct LocalProduct {
    id: String,
    title: String,
    inventory_count: i32,
    is_sold_out: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OfflineAction {
    id: i64,
    product_id: String,
    is_sold_out: bool,
    timestamp_ms: i64,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct LocalOrder {
    id: String,
    customer_name: String,
    total_amount: f64,
    status: String,
    created_at: String,
}

struct DbState(Mutex<Connection>);

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn generate_cloud_invite() -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("https://cloud.ohc.network/invite/ref-{}", ts)
}

#[tauri::command]
fn load_ai_provider() -> Result<AiProviderView, String> {
    Ok(to_provider_view(read_ai_provider_config()?))
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OnboardingState {
    business_name: Option<String>,
    assistant_name: Option<String>,
    assistant_tone: Option<String>,
}

fn onboarding_state_path() -> std::path::PathBuf {
    std::env::var("OHC_ONBOARDING_STATE_PATH")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(".ohc/onboarding.json"))
}

#[tauri::command]
async fn get_onboarding_state(_app_handle: tauri::AppHandle) -> Result<OnboardingState, String> {
    // Determine tenant/user dynamically from args or state (here we default or check env for test)
    let tenant_id = std::env::var("OHC_DEFAULT_TENANT_ID").unwrap_or_else(|_| "default".to_string());
    let user_id = std::env::var("OHC_DEFAULT_USER_ID").unwrap_or_else(|_| "default".to_string());

    // Attempt to fetch from backend
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/onboarding/state", backend_url);

    let request = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|err| err.to_string())?
        .get(&url)
        .header("X-Tenant-ID", tenant_id)
        .header("X-User-ID", user_id);

    if let Ok(response) = request.send().await {
        if response.status().is_success() {
            if let Ok(state) = response.json::<OnboardingState>().await {
                // Return successfully from backend
                return Ok(state);
            }
        }
    }

    // Fallback to local file
    let path = onboarding_state_path();
    if !path.exists() {
        return Ok(OnboardingState {
            business_name: None,
            assistant_name: None,
            assistant_tone: None,
        });
    }

    let content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    let state = serde_json::from_str::<OnboardingState>(&content).map_err(|err| err.to_string())?;
    Ok(state)
}

#[tauri::command]
async fn save_onboarding_state(state: OnboardingState, _app_handle: tauri::AppHandle) -> Result<(), String> {
    let tenant_id = std::env::var("OHC_DEFAULT_TENANT_ID").unwrap_or_else(|_| "default".to_string());
    let user_id = std::env::var("OHC_DEFAULT_USER_ID").unwrap_or_else(|_| "default".to_string());

    // Attempt to save to backend
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/onboarding/state", backend_url);

    let request = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|err| err.to_string())?
        .post(&url)
        .header("X-Tenant-ID", tenant_id)
        .header("X-User-ID", user_id)
        .header("Content-Type", "application/json")
        .json(&state);

    // Fire and forget, or wait for success
    let _ = request.send().await;

    // Fallback/mirror to local file
    let path = onboarding_state_path();
    if let Some(parent) = path.parent() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            std::fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(parent)
                .unwrap_or_default();
        }
        #[cfg(not(unix))]
        {
            std::fs::create_dir_all(parent).unwrap_or_default();
        }
    }

    if let Ok(json) = serde_json::to_string_pretty(&state) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            use std::io::Write;
            if let Ok(mut file) = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(path) {
                let _ = file.write_all(format!("{json}\n").as_bytes());
            }
        }
        #[cfg(not(unix))]
        {
            let _ = std::fs::write(path, format!("{json}\n"));
        }
    }

    Ok(())
}

#[tauri::command]
fn save_ai_provider(config: AiProviderConfig) -> Result<AiProviderView, String> {
    let mut current = read_ai_provider_config()?;
    let mut next = normalize_ai_provider_config(config);
    if next.api_key.as_deref().unwrap_or("").trim().is_empty() {
        next.api_key = current.api_key.take();
    }

    validate_ai_provider_config(&next)?;
    let path = ai_provider_config_path();
    if let Some(parent) = path.parent() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            std::fs::DirBuilder::new()
                .recursive(true)
                .mode(0o700)
                .create(parent)
                .map_err(|err| err.to_string())?;
        }
        #[cfg(not(unix))]
        {
            std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }
    }

    let json = serde_json::to_string_pretty(&next).map_err(|err| err.to_string())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)
            .map_err(|err| err.to_string())?;
        file.write_all(format!("{json}\n").as_bytes())
            .map_err(|err| err.to_string())?;
    }
    #[cfg(not(unix))]
    {
        std::fs::write(path, format!("{json}\n")).map_err(|err| err.to_string())?;
    }

    Ok(to_provider_view(next))
}

#[tauri::command]
async fn test_ai_provider(config: AiProviderConfig) -> Result<AiProviderTestResult, String> {
    let current = read_ai_provider_config()?;
    let mut next = normalize_ai_provider_config(config);
    if next.api_key.is_none() && current.provider == next.provider {
        next.api_key = current.api_key;
    }
    validate_ai_provider_config(&next)?;

    let url = endpoint_url(&next.base_url, "chat/completions");
    let body = serde_json::json!({
        "model": next.model,
        "messages": [
            { "role": "user", "content": "Reply with only: ok" }
        ],
        "max_tokens": 8,
        "stream": false
    });

    let mut request = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|err| err.to_string())?
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body);

    if let Some(api_key) = next.api_key.as_deref().filter(|key| !key.trim().is_empty()) {
        request = request.header("Authorization", format!("Bearer {}", api_key.trim()));
    }

    let response = request.send().await.map_err(|err| err.to_string())?;
    let status = response.status();
    if status.is_success() {
        Ok(AiProviderTestResult {
            ok: true,
            status: status.as_u16(),
            message: format!("Connected to {}", url),
        })
    } else {
        let text = response.text().await.unwrap_or_default();
        Ok(AiProviderTestResult {
            ok: false,
            status: status.as_u16(),
            message: if text.trim().is_empty() {
                format!("{} returned HTTP {}", url, status.as_u16())
            } else {
                text
            },
        })
    }
}

fn ai_provider_config_path() -> std::path::PathBuf {
    std::env::var("OHC_LLM_CONFIG_PATH")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(".ohc/ai-provider.json"))
}

fn read_ai_provider_config() -> Result<AiProviderConfig, String> {
    let path = ai_provider_config_path();
    if !path.exists() {
        return Ok(default_ai_provider_config());
    }

    let content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
    let config =
        serde_json::from_str::<AiProviderConfig>(&content).map_err(|err| err.to_string())?;
    Ok(normalize_ai_provider_config(config))
}

fn default_ai_provider_config() -> AiProviderConfig {
    AiProviderConfig {
        provider: "openai".to_string(),
        model: "gpt-4.1-mini".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        api_key: None,
    }
}

fn normalize_ai_provider_config(mut config: AiProviderConfig) -> AiProviderConfig {
    config.provider = config.provider.trim().to_string();
    config.model = if config.model.trim().is_empty() {
        default_model_for_provider(&config.provider)
    } else {
        config.model.trim().to_string()
    };
    config.base_url = if config.base_url.trim().is_empty() {
        default_base_url_for_provider(&config.provider)
    } else {
        normalize_api_base_url(&config.base_url)
    };
    config.api_key = config
        .api_key
        .map(|key| key.trim().to_string())
        .filter(|key| !key.is_empty());
    config
}

fn validate_ai_provider_config(config: &AiProviderConfig) -> Result<(), String> {
    if config.provider.is_empty() {
        return Err("Provider is required".to_string());
    }
    if config.model.is_empty() {
        return Err("Model is required".to_string());
    }
    if config.base_url.is_empty() {
        return Err("Base URL is required".to_string());
    }
    Ok(())
}

fn to_provider_view(config: AiProviderConfig) -> AiProviderView {
    AiProviderView {
        provider: config.provider,
        model: config.model,
        base_url: config.base_url,
        api_key_set: config.api_key.is_some(),
    }
}

fn default_model_for_provider(provider: &str) -> String {
    match provider {
        "minimax" => "MiniMax-M2.7".to_string(),
        "openai" => "gpt-4.1-mini".to_string(),
        _ => String::new(),
    }
}

fn default_base_url_for_provider(provider: &str) -> String {
    match provider {
        "minimax" => "https://api.minimax.chat/v1".to_string(),
        "openai" => "https://api.openai.com/v1".to_string(),
        _ => String::new(),
    }
}

fn normalize_api_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/').to_string();
    for suffix in ["/chat/completions", "/embeddings"] {
        if let Some(root) = trimmed.strip_suffix(suffix) {
            return root.trim_end_matches('/').to_string();
        }
    }
    trimmed
}

fn endpoint_url(base_url: &str, endpoint: &str) -> String {
    format!("{}/{}", base_url.trim_end_matches('/'), endpoint)
}

#[tauri::command]
async fn get_help_articles() -> Result<serde_json::Value, String> {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/help", backend_url);

    let request = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|err| err.to_string())?
        .get(&url);

    let response = request.send().await.map_err(|err| err.to_string())?;
    if response.status().is_success() {
        let json: serde_json::Value = response.json().await.map_err(|err| err.to_string())?;
        Ok(json)
    } else {
        Err(format!("Backend returned {}", response.status()))
    }
}

#[tauri::command]
async fn get_help_article(id: String) -> Result<serde_json::Value, String> {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/help/{}", backend_url, id);

    let request = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|err| err.to_string())?
        .get(&url);

    let response = request.send().await.map_err(|err| err.to_string())?;
    if response.status().is_success() {
        let json: serde_json::Value = response.json().await.map_err(|err| err.to_string())?;
        Ok(json)
    } else {
        Err(format!("Backend returned {}", response.status()))
    }
}

#[tauri::command]
async fn get_help_videos() -> Result<serde_json::Value, String> {
    // For now, mock the videos as the endpoint is partially mocked anyway
    Ok(serde_json::json!([
        { "id": 1, "title": "How to set up your first store easily", "duration": "1:20", "video_url": "https://www.w3schools.com/html/mov_bbb.mp4" },
        { "id": 2, "title": "Accept your first payment", "duration": "1:15", "video_url": "https://www.w3schools.com/html/mov_bbb.mp4" }
    ]))
}


#[tauri::command]
async fn get_changelog() -> Result<serde_json::Value, String> {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/changelog", backend_url);

    let request = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|err| err.to_string())?
        .get(&url);

    let response = request.send().await.map_err(|err| err.to_string())?;
    if response.status().is_success() {
        let json: serde_json::Value = response.json().await.map_err(|err| err.to_string())?;
        Ok(json)
    } else {
        Err(format!("Backend returned {}", response.status()))
    }
}

#[cfg(ohc_bazel_tauri_context)]
macro_rules! tauri_build_context {
    () => {
        include!("../tauri-build-context.rs");
    };
}

#[cfg(ohc_bazel_tauri_context)]
tauri_build_context!();

#[tauri::command]
fn get_local_menu(state: tauri::State<DbState>) -> Result<Vec<LocalProduct>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, inventory_count, is_sold_out FROM products ORDER BY title ASC")
        .map_err(|e| e.to_string())?;
    let product_iter = stmt
        .query_map([], |row| {
            Ok(LocalProduct {
                id: row.get(0)?,
                title: row.get(1)?,
                inventory_count: row.get(2)?,
                is_sold_out: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut products = Vec::new();
    for product in product_iter {
        products.push(product.map_err(|e| e.to_string())?);
    }
    Ok(products)
}

#[tauri::command]
fn toggle_sold_out(
    id: String,
    is_sold_out: bool,
    state: tauri::State<DbState>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE products SET is_sold_out = ? WHERE id = ?",
        params![is_sold_out, id],
    )
    .map_err(|e| e.to_string())?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    conn.execute(
        "INSERT INTO offline_actions (product_id, is_sold_out, timestamp_ms) VALUES (?, ?, ?)",
        params![id, is_sold_out, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_local_orders(state: tauri::State<DbState>) -> Result<Vec<LocalOrder>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, customer_name, total_amount, status, created_at FROM orders ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;
    let order_iter = stmt
        .query_map([], |row| {
            Ok(LocalOrder {
                id: row.get(0)?,
                customer_name: row.get(1)?,
                total_amount: row.get(2)?,
                status: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut orders = Vec::new();
    for order in order_iter {
        orders.push(order.map_err(|e| e.to_string())?);
    }
    Ok(orders)
}

#[tauri::command]
async fn sync_offline_actions(state: tauri::State<'_, DbState>) -> Result<bool, String> {
    let actions = {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn
            .prepare("SELECT id, product_id, is_sold_out, timestamp_ms FROM offline_actions ORDER BY timestamp_ms ASC")
            .map_err(|e| e.to_string())?;
        let action_iter = stmt
            .query_map([], |row| {
                Ok(OfflineAction {
                    id: row.get(0)?,
                    product_id: row.get(1)?,
                    is_sold_out: row.get(2)?,
                    timestamp_ms: row.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?;

        let mut actions = Vec::new();
        for action in action_iter {
            actions.push(action.map_err(|e| e.to_string())?);
        }
        actions
    };

    if actions.is_empty() {
        return Ok(true);
    }

    let mutations: Vec<serde_json::Value> = actions
        .iter()
        .map(|a| {
            serde_json::json!({
                "transaction_id": format!("off-{}", a.id),
                "product_id": a.product_id,
                "quantity_deducted": 0,
                "is_sold_out": a.is_sold_out,
            })
        })
        .collect();

    let backend_url =
        std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/v1/sync/offline", backend_url);
    let tenant_id =
        std::env::var("OHC_DEFAULT_TENANT_ID").unwrap_or_else(|_| "default".to_string());

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .header(
            "x-spiffe-id",
            format!("spiffe://ohc/org/{}/agent/mobile", tenant_id),
        )
        .json(&serde_json::json!({ "mutations": mutations }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let conn = state.0.lock().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM offline_actions", [])
            .map_err(|e| e.to_string())?;

        // Also refresh local products and orders from backend
        let menu_url = format!("{}/api/v1/catalog/products?tenant_id={}", backend_url, tenant_id);
        if let Ok(menu_res) = client.get(menu_url).send().await {
            if menu_res.status().is_success() {
                if let Ok(products) = menu_res.json::<Vec<serde_json::Value>>().await {
                    let _ = conn.execute("DELETE FROM products", []);
                    for p in products {
                        let _ = conn.execute(
                            "INSERT INTO products (id, title, inventory_count, is_sold_out) VALUES (?, ?, ?, ?)",
                            params![
                                p["id"].as_str().unwrap_or(""),
                                p["title"].as_str().unwrap_or(""),
                                p["inventory_count"].as_i64().unwrap_or(0),
                                p["is_sold_out"].as_bool().unwrap_or(false),
                            ],
                        );
                    }
                }
            }
        }

        let orders_url = format!("{}/api/ui/orders?tenant_id={}", backend_url, tenant_id);
        if let Ok(orders_res) = client.get(orders_url).send().await {
            if orders_res.status().is_success() {
                if let Ok(orders) = orders_res.json::<Vec<serde_json::Value>>().await {
                    let _ = conn.execute("DELETE FROM orders", []);
                    for o in orders {
                        let _ = conn.execute(
                            "INSERT INTO orders (id, customer_name, total_amount, status, created_at) VALUES (?, ?, ?, ?, ?)",
                            params![
                                o["id"].as_str().unwrap_or(""),
                                o["customer_name"].as_str().unwrap_or("Guest"),
                                o["total_amount"].as_f64().unwrap_or(0.0),
                                o["status"].as_str().unwrap_or(""),
                                o["created_at"].as_str().unwrap_or(""),
                            ],
                        );
                    }
                }
            }
        }

        Ok(true)
    } else {
        Err(format!("Backend sync failed: {}", res.status()))
    }
}

#[tauri::command]
fn get_backend_url() -> String {
    std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(ohc_bazel_tauri_context)]
    let context = tauri_context();

    #[cfg(not(ohc_bazel_tauri_context))]
    let context = tauri::generate_context!();

    let db_path = ".ohc/local_pos.db";
    if let Some(parent) = std::path::Path::new(db_path).parent() {
        std::fs::create_dir_all(parent).unwrap_or_default();
    }
    let conn = Connection::open(db_path).expect("failed to open local sqlite db");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            inventory_count INTEGER DEFAULT 0,
            is_sold_out BOOLEAN DEFAULT 0
        )",
        [],
    )
    .expect("failed to create local products table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            customer_name TEXT,
            total_amount REAL,
            status TEXT,
            created_at TEXT
        )",
        [],
    )
    .expect("failed to create local orders table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS offline_actions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id TEXT NOT NULL,
            is_sold_out BOOLEAN NOT NULL,
            timestamp_ms INTEGER NOT NULL
        )",
        [],
    )
    .expect("failed to create local actions table");

    tauri::Builder::default()
        .manage(DbState(Mutex::new(conn)))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_cloud_invite,
            load_ai_provider,
            save_ai_provider,
            test_ai_provider,
            get_onboarding_state,
            save_onboarding_state,
            get_help_articles,
            get_help_article,
            get_help_videos,
            get_changelog,
            get_local_menu,
            get_local_orders,
            toggle_sold_out,
            sync_offline_actions,
            get_backend_url,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("OHC").unwrap();
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
