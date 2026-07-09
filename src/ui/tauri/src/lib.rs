use tauri::Manager;

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
fn generate_cloud_bridge_invite() -> String {
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("https://cloud.ohc.network/invite/cb-{}", ts)
}

#[tauri::command]
fn load_ai_provider() -> Result<AiProviderView, String> {
    Ok(to_provider_view(read_ai_provider_config()?))
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StartOnboardingRequest {
    business_type: Option<String>,
    company_name: Option<String>,
    company_description: Option<String>,
    selling_categories: Option<Vec<String>>,
    payment_pref: Option<String>,
    admin_email: Option<String>,
    website_template: Option<String>,
    first_product_name: Option<String>,
    first_product_price: Option<String>,
    domain_choice: Option<String>,
    admin_name: Option<String>,
    admin_password: Option<String>,
    price_type: Option<String>,
    location: Option<String>,
    target_audience: Option<String>,
    ai_agents: Option<Vec<String>>,
    ai_auto_respond: Option<bool>,
    deposit_percentage: Option<i32>,
    lead_time_days: Option<i32>,
}

fn onboarding_state_path() -> std::path::PathBuf {
    std::env::var("OHC_ONBOARDING_STATE_PATH")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(".ohc/onboarding.json"))
}

#[tauri::command]
async fn get_onboarding_state(tenant_id: Option<String>, user_id: Option<String>, _app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let t_id = tenant_id.unwrap_or_else(|| std::env::var("OHC_DEFAULT_TENANT_ID").unwrap_or_else(|_| "default".to_string()));
    let u_id = user_id.unwrap_or_else(|| std::env::var("OHC_DEFAULT_USER_ID").unwrap_or_else(|_| "default".to_string()));

    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/onboarding/draft", backend_url);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|err| err.to_string())?;

    // Priority: Backend API
    if let Ok(response) = client.get(&url)
        .header("X-Tenant-ID", &t_id)
        .header("X-User-ID", &u_id)
        .send().await {
        if response.status().is_success() {
            if let Ok(state) = response.json::<serde_json::Value>().await {
                return Ok(state);
            }
        }
    }

    // Local file fallback ONLY if standalone
    let is_standalone = false;
    if is_standalone {
        let path = onboarding_state_path();
        if path.exists() {
            let content = std::fs::read_to_string(path).map_err(|err| err.to_string())?;
            if let Ok(state) = serde_json::from_str::<serde_json::Value>(&content) {
                return Ok(state);
            }
        }
    }

    Ok(serde_json::json!({}))
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct IntakeRequest {
    input: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct IntakeData {
    business_name: String,
    business_type: String,
    categories: Vec<String>,
    location: Option<String>,
    target_audience: Option<String>,
    initial_products: Vec<serde_json::Value>,
}

#[tauri::command]
async fn process_intake(input: String, image_url: Option<String>, _app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/onboarding/intake", backend_url);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|err| err.to_string())?;

    let response = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "description": input, "image_url": image_url }))
        .send().await
        .map_err(|err| err.to_string())?;

    if response.status().is_success() {
        let text = response.text().await.map_err(|e| e.to_string())?;
        serde_json::from_str(&text).map_err(|e| e.to_string())
    } else {
        Err(format!("Backend error: {}", response.status()))
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct StartOnboardingResponse {
    success: bool,
    message: String,
    organization_id: String,
}

#[tauri::command]
async fn start_onboarding(req: StartOnboardingRequest, _app_handle: tauri::AppHandle) -> Result<StartOnboardingResponse, String> {
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/onboarding/start", backend_url);

    let request = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|err| err.to_string())?
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&req);

    let res = request.send().await.map_err(|err| err.to_string())?;

    if !res.status().is_success() {
        return Err(format!("Failed to start onboarding: {}", res.status()));
    }

    let resp_data = res.json::<StartOnboardingResponse>().await.map_err(|err| err.to_string())?;

    Ok(resp_data)
}

#[tauri::command]
async fn save_onboarding_state(state: serde_json::Value, tenant_id: Option<String>, user_id: Option<String>, _app_handle: tauri::AppHandle) -> Result<(), String> {
    let t_id = tenant_id.unwrap_or_else(|| std::env::var("OHC_DEFAULT_TENANT_ID").unwrap_or_else(|_| "default".to_string()));
    let u_id = user_id.unwrap_or_else(|| std::env::var("OHC_DEFAULT_USER_ID").unwrap_or_else(|_| "default".to_string()));

    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/onboarding/draft", backend_url);

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|err| err.to_string())?;

    // Primary: Backend save
    let _ = client.post(&url)
        .header("X-Tenant-ID", &t_id)
        .header("X-User-ID", &u_id)
        .header("Content-Type", "application/json")
        .json(&state)
        .send().await;

    // Local mirror for standalone persistence
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
    let backend_url = std::env::var("BACKEND_URL").unwrap_or_else(|_| "http://127.0.0.1:18789".to_string());
    let url = format!("{}/api/videos", backend_url);

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[cfg(ohc_bazel_tauri_context)]
    let context = tauri_context();

    #[cfg(not(ohc_bazel_tauri_context))]
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            greet,
            generate_cloud_invite,
            generate_cloud_bridge_invite,
            load_ai_provider,
            save_ai_provider,
            test_ai_provider,
            get_onboarding_state,
            save_onboarding_state,
            start_onboarding,
            process_intake,
            get_help_articles,
            get_help_article,
            get_help_videos,
            get_changelog,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("OHC").unwrap();
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
