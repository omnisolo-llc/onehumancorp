
use axum::{routing::{get, post}, Router, Json};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelpArticle {
    pub id: String,
    pub title: String,
    pub content: String,
    pub topic: String,
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tooltip {
    pub id: String,
    pub element_selector: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WalkthroughStep {
    pub id: String,
    pub target: String,
    pub message: String,
    pub position: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VideoMetadata {
    pub id: String,
    pub title: String,
    pub url: String,
    pub duration_seconds: u32,
    pub thumbnail_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReleaseNote {
    pub version: String,
    pub date: String,
    pub content: String,
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatResponse {
    pub reply: String,
    pub article_link: Option<String>,
}

static CACHED_ARTICLES: OnceLock<Vec<HelpArticle>> = OnceLock::new();

fn init_articles() -> Vec<HelpArticle> {
    let mut articles = Vec::new();
    let docs_dir = "docs/business/public/app/help_center_content";

    if let Ok(entries) = std::fs::read_dir(docs_dir) {
        let mut paths: Vec<_> = entries.filter_map(|e| e.ok()).map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    let title = path.file_stem().and_then(|s| s.to_str()).unwrap_or("Untitled").to_string();
                    articles.push(HelpArticle {
                        id: title.clone(),
                        title: title.replace("_", " ").to_uppercase(),
                        content,
                        topic: "Guides".into(),
                        keywords: vec!["guide".into()],
                    });
                }
            }
        }
    }

    if articles.is_empty() {
        articles.push(HelpArticle {
            id: "getting-started".into(),
            title: "Getting Started with OHC".into(),
            content: "Welcome to OHC! To begin, navigate to the Dashboard and click on 'Setup Wizard'. This will guide you through adding your business details, connecting your bank, and inviting your first team members.".into(),
            topic: "Onboarding".into(),
            keywords: vec!["start".into(), "setup".into(), "onboarding".into()],
        });
        articles.push(HelpArticle {
            id: "managing-agents".into(),
            title: "How to Manage AI Agents".into(),
            content: "Your AI Agents act as autonomous employees. Go to the 'Agents' tab to view their current tasks. You can assign new missions by typing in plain English. Remember: they learn from your feedback, so correct them if they make mistakes.".into(),
            topic: "Agents".into(),
            keywords: vec!["agents".into(), "ai".into(), "tasks".into(), "automation".into()],
        });
        articles.push(HelpArticle {
            id: "accepting-payments".into(),
            title: "Accepting Your First Payment".into(),
            content: "Before accepting payments, ensure your bank account is linked under 'Settings > Integrations'. Once linked, you can generate an invoice or send a direct payment link to your customers from the 'Payments' dashboard.".into(),
            topic: "Finance".into(),
            keywords: vec!["payments".into(), "money".into(), "invoices".into(), "billing".into()],
        });
    }

    articles
}

pub fn help_routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/articles", get(get_articles))
        .route("/tooltips", get(get_tooltips))
        .route("/walkthroughs", get(get_walkthroughs))
        .route("/videos", get(get_videos))
        .route("/changelog", get(get_changelog))
        .route("/chat", post(chat))
}

async fn get_articles() -> Json<Vec<HelpArticle>> {
    let articles = CACHED_ARTICLES.get_or_init(init_articles);
    Json(articles.clone())
}

async fn get_tooltips() -> Json<Vec<Tooltip>> {
    Json(vec![
        Tooltip {
            id: "t1".into(),
            element_selector: "#help-btn".into(),
            text: "Click here to open the Help Center.".into(),
        },
        Tooltip {
            id: "t2".into(),
            element_selector: ".store-setup-btn".into(),
            text: "Start configuring your storefront here.".into(),
        }
    ])
}

async fn get_walkthroughs() -> Json<Vec<WalkthroughStep>> {
    Json(vec![
        WalkthroughStep {
            id: "w1".into(),
            target: "#setup-store".into(),
            message: "Welcome! Click here to begin setting up your store profile and connecting your bank account.".into(),
            position: "bottom".into(),
        }
    ])
}

async fn get_videos() -> Json<Vec<VideoMetadata>> {
    Json(vec![
        VideoMetadata {
            id: "v1".into(),
            title: "Quickstart: First 5 Minutes".into(),
            url: "https://example.com/videos/quickstart.mp4".into(),
            duration_seconds: 300,
            thumbnail_url: Some("https://example.com/thumbs/quickstart.png".into()),
        }
    ])
}

async fn get_changelog() -> Json<Vec<ReleaseNote>> {
    Json(vec![
        ReleaseNote {
            version: "1.2.0".into(),
            date: "2023-11-15".into(),
            content: "Added new interactive Walkthroughs and contextual Tooltips to help new users get started faster.".into(),
            image_url: None,
        }
    ])
}

async fn chat(Json(payload): Json<ChatMessage>) -> Json<ChatResponse> {
    let lower = payload.message.to_lowercase();
    let (reply, best_link) = if lower.contains("store") {
        ("To set up your store, navigate to the My Store section and click the 'Setup Wizard'. It will guide you through the process.".to_string(), "/help/articles/article_1".to_string())
    } else if lower.contains("payment") || lower.contains("money") || lower.contains("invoice") {
        ("You can accept payments by going to the 'Settings > Integrations' tab and linking your bank account. After that, use the Payments tab to generate invoices.".to_string(), "/help/articles/article_2".to_string())
    } else if lower.contains("agent") || lower.contains("ai") {
        ("Go to the 'Agents' tab to manage your AI employees. Give them clear, plain-language instructions to automate your tasks.".to_string(), "/help/articles/article_3".to_string())
    } else if lower.contains("sync") || lower.contains("offline") {
        ("If you see the 'Offline' indicator, check your internet. OHC stores data locally and syncs automatically when reconnected.".to_string(), "/help/articles/article_4".to_string())
    } else {
        (format!("AI Assistant: I received your query '{}'. Try searching the Help Center for more detailed guides.", payload.message), "/help/articles/article_1".to_string())
    };

    Json(ChatResponse {
        reply,
        article_link: Some(best_link),
    })
}
