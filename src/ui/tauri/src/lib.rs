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

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct StartOnboardingRequest {
    business_type: String,
    company_name: String,
    company_description: String,
    selling_categories: Vec<String>,
    payment_pref: String,
    admin_email: String,
    website_template: String,
    first_product_name: String,
    first_product_price: String,
    domain_choice: String,
    admin_name: String,
    admin_password: String,
    price_type: String,
    location: String,
    target_audience: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StartOnboardingResponse {
    success: bool,
    message: String,
    organization_id: String,
}

#[tauri::command]
async fn submit_onboarding(request: StartOnboardingRequest) -> Result<StartOnboardingResponse, String> {
    // In Tauri standalone, we can connect locally.
    // For a real production app we would read this from app config,
    // but the task asks to hit the real Rust backend to create the onboarding payload.
    // E2E Playwright environments run this at 18789 or dynamic ports, but for Tauri
    // native we proxy it properly.

    let url = match std::env::var("OHC_API_URL") {
        Ok(v) => format!("{}/api/onboarding/start", v),
        Err(_) => "http://127.0.0.1:18789/api/onboarding/start".to_string(),
    };

    let payload = serde_json::json!({
        "business_type": request.business_type,
        "company_name": request.company_name,
        "company_description": request.company_description,
        "selling_categories": request.selling_categories,
        "payment_pref": request.payment_pref,
        "admin_email": request.admin_email,
        "website_template": request.website_template,
        "first_product_name": request.first_product_name,
        "first_product_price": request.first_product_price,
        "domain_choice": request.domain_choice,
        "admin_name": request.admin_name,
        "admin_password": request.admin_password,
        "price_type": request.price_type,
        "location": request.location,
        "target_audience": request.target_audience
    });

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.status().is_success() {
        let result: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
        Ok(StartOnboardingResponse {
            success: result["success"].as_bool().unwrap_or(false),
            message: result["message"].as_str().unwrap_or("").to_string(),
            organization_id: result["organization_id"].as_str().unwrap_or("").to_string(),
        })
    } else {
        Err(format!("Onboarding failed: {}", resp.status()))
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn load_ai_provider() -> Result<AiProviderView, String> {
    Ok(to_provider_view(read_ai_provider_config()?))
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
            load_ai_provider,
            save_ai_provider,
            test_ai_provider,
            submit_onboarding,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("OHC").unwrap();
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
