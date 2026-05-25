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
        std::fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }

    let json = serde_json::to_string_pretty(&next).map_err(|err| err.to_string())?;
    std::fs::write(path, format!("{json}\n")).map_err(|err| err.to_string())?;
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
async fn get_tooltips() -> std::collections::HashMap<String, String> {
    let mut m = std::collections::HashMap::new();
    m.insert("bio-input-tooltip".to_string(), "Tell us what you sell and who your customers are. Keep it simple!".to_string());
    m.insert("generate-btn-tooltip".to_string(), "Click here to have our AI build your ready-to-launch store.".to_string());
    m.insert("launch-btn-tooltip".to_string(), "Make your store live on the internet so customers can visit.".to_string());
    m.insert("team-activity-tooltip".to_string(), "See exactly what your AI helpers are doing right now.".to_string());
    m.insert("referral-tooltip".to_string(), "Share this link with friends. You earn credits if they sign up!".to_string());
    m.insert("swarm-online-tooltip".to_string(), "Your AI helpers are working hard on your tasks right now.".to_string());
    m.insert("department-card-tooltip".to_string(), "Click here to see tasks that need your approval.".to_string());
    m.insert("nav-dashboard-tooltip".to_string(), "Check your sales, recent orders, and how your store is doing.".to_string());
    m.insert("nav-agents-tooltip".to_string(), "See your AI team, give them tasks, or hire new helpers.".to_string());
    m.insert("nav-setup-tooltip".to_string(), "Set up your business info, logo, and how you get paid.".to_string());
    m.insert("credit-tooltip".to_string(), "Get free credits for premium tools by inviting a friend.".to_string());
    m.insert("help-btn-tooltip".to_string(), "Need help? Click here for guides, videos, and to ask our AI.".to_string());
    m.insert("changelog-nav-tooltip".to_string(), "See the latest updates and new features we just added.".to_string());
    m
}

#[derive(serde::Serialize)]
struct HelpArticle {
    title: String,
    desc: String,
    link: Option<String>,
}

#[tauri::command]
async fn get_help_articles() -> Vec<HelpArticle> {
    vec![
        HelpArticle { title: "Getting Started".into(), desc: "Learn how to easily set up your store and accept your first payment.".into(), link: Some("/help/getting-started".into()) },
        HelpArticle { title: "My Store".into(), desc: "Add products, track what's in stock, and change how your store looks.".into(), link: Some("/help/my-store".into()) },
        HelpArticle { title: "Getting Paid".into(), desc: "Set up how you get paid, view deposits, and handle simple taxes.".into(), link: Some("/help/payments".into()) },
        HelpArticle { title: "Your AI Helpers".into(), desc: "Learn how to hire AI helpers and give them tasks to do.".into(), link: Some("/help/ai-agents".into()) },
        HelpArticle { title: "Finding Customers".into(), desc: "Send emails to customers and grow your business easily.".into(), link: Some("/help/marketing".into()) },
        HelpArticle { title: "Account & Billing".into(), desc: "View your bills, manage your plan, and invite team members.".into(), link: Some("/help/account-billing".into()) }
    ]
}

#[derive(serde::Serialize)]
struct VideoTutorial {
    id: i32,
    title: String,
    duration: String,
}

#[tauri::command]
async fn get_videos() -> Vec<VideoTutorial> {
    vec![
        VideoTutorial { id: 1, title: "How to set up your first store easily".into(), duration: "1:20".into() },
        VideoTutorial { id: 2, title: "Linking your own website name".into(), duration: "0:45".into() },
        VideoTutorial { id: 3, title: "Getting paid for the first time".into(), duration: "1:10".into() },
        VideoTutorial { id: 4, title: "Hiring your first AI helper".into(), duration: "1:05".into() },
        VideoTutorial { id: 5, title: "Adding and editing your products".into(), duration: "0:55".into() },
        VideoTutorial { id: 6, title: "Sending emails to your customers".into(), duration: "1:15".into() },
        VideoTutorial { id: 7, title: "Seeing how much you sold".into(), duration: "0:50".into() },
        VideoTutorial { id: 8, title: "What to do when you get an order".into(), duration: "1:00".into() },
        VideoTutorial { id: 9, title: "Changing colors and logos".into(), duration: "1:25".into() },
        VideoTutorial { id: 10, title: "Adding staff to your account".into(), duration: "0:40".into() }
    ]
}

#[derive(serde::Serialize)]
struct Link {
    url: String,
    title: String,
}

#[derive(serde::Serialize)]
struct ChatReply {
    reply: String,
    link: Option<Link>,
}

#[tauri::command]
async fn chat_help(message: String) -> ChatReply {
    ChatReply {
        reply: "I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.".into(),
        link: Some(Link { url: "/help".into(), title: "Read the full article →".into() })
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
            load_ai_provider,
            save_ai_provider,
            test_ai_provider,
            get_tooltips,
            get_help_articles,
            get_videos,
            chat_help,
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.set_title("OHC").unwrap();
            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
