use axum::{routing::{get, post}, Json, Router};
use serde_json::{json, Value};

async fn get_articles() -> Json<Value> {
    Json(json!([
        {
            "id": "getting-started",
            "title": "Getting Started",
            "summary": "Learn the basics of setting up your business on OHC.",
            "content": "Welcome! The first step is to configure your store name and pick a theme. Navigate to Setup Wizard from your dashboard."
        },
        {
            "id": "my-store",
            "title": "My Store",
            "summary": "Manage your products and orders.",
            "content": "To add a product, click 'Add Product' from the quick actions menu."
        },
        {
            "id": "payments",
            "title": "Payments",
            "summary": "How to accept your first payment.",
            "content": "Link your bank account in Settings to start receiving money from your customers."
        },
        {
            "id": "ai-agents",
            "title": "AI Agents",
            "summary": "Let agents work for you.",
            "content": "You can hire Sales or Marketing agents from the 'My Agents' screen."
        },
        {
            "id": "marketing",
            "title": "Marketing",
            "summary": "Grow your business.",
            "content": "Use the Integrations button to connect to Facebook."
        },
        {
            "id": "account-billing",
            "title": "Account & Billing",
            "summary": "Manage your subscription plan.",
            "content": "Visit the Billing page from your quick actions menu."
        }
    ]))
}

async fn get_videos() -> Json<Value> {
    Json(json!([
        {
            "id": "vid-1",
            "title": "Set up your store in 90 seconds",
            "url": "https://example.com/vid-1.mp4",
            "thumbnail": "https://example.com/thumb-1.jpg"
        },
        {
            "id": "vid-2",
            "title": "Accept your first payment",
            "url": "https://example.com/vid-2.mp4",
            "thumbnail": "https://example.com/thumb-2.jpg"
        },
        {
            "id": "vid-3",
            "title": "Activate your AI Support Agent",
            "url": "https://example.com/vid-3.mp4",
            "thumbnail": "https://example.com/thumb-3.jpg"
        }
    ]))
}

async fn get_changelog() -> Json<Value> {
    Json(json!([
        {
            "version": "v1.2.0",
            "date": "October 12, 2023",
            "title": "New Sales Agent Features",
            "description": "Your Sales Agent can now automatically respond to Facebook messages. No setup required!"
        },
        {
            "version": "v1.1.0",
            "date": "September 28, 2023",
            "title": "Faster Payouts",
            "description": "Money now reaches your bank account in 24 hours instead of 48 hours."
        }
    ]))
}

async fn chat_handler(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    let _question = payload.get("message").and_then(|m| m.as_str()).unwrap_or("");
    Json(json!({
        "reply": "I'm your Help Agent! To learn more about setting up your store, please read our 'Getting Started' article.",
        "article_link": "getting-started"
    }))
}

async fn get_tooltips() -> Json<Value> {
    Json(json!({
        "#integrations-btn": "Connect external apps like Facebook here.",
        ".nav-item:nth-child(4)": "Click here to add a new product to your store catalog.",
        "#main-nav a:nth-child(2)": "View and manage your AI employees."
    }))
}

async fn get_walkthroughs() -> Json<Value> {
    Json(json!({
        "setup-store": [
            { "selector": "#dashboard-screen h1", "text": "Welcome to your Dashboard! This is your home base." },
            { "selector": "button[onclick=\"showScreen('setup-screen')\"]", "text": "Click here to start the setup wizard and build your website." }
        ],
        "accept-payment": [
            { "selector": "button[onclick=\"showScreen('settings-screen')\"]", "text": "Go to Settings to link your bank account." }
        ],
        "activate-agent": [
            { "selector": "button[onclick=\"showScreen('agents-screen')\"]", "text": "Visit the Agents page to hire your first AI worker." }
        ]
    }))
}

pub fn router<S>() -> Router<S> where S: Clone + Send + Sync + 'static {
    Router::new()
        .route("/articles", get(get_articles))
        .route("/videos", get(get_videos))
        .route("/changelog", get(get_changelog))
        .route("/chat", post(chat_handler))
        .route("/tooltips", get(get_tooltips))
        .route("/walkthroughs", get(get_walkthroughs))
}
