use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HelpQuery {
    pub query: Option<String>,
}

#[derive(Deserialize)]
pub struct TooltipQuery {
    pub screen: String,
}

#[derive(Serialize)]
pub struct HelpArticle {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: String,
    pub summary: String,
}

#[derive(Serialize)]
pub struct Tooltip {
    pub id: String,
    pub content: String,
    pub target_element: String,
}

#[derive(Serialize)]
pub struct HelpVideo {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
}

pub fn router<S>() -> Router<S> where S: Clone + Send + Sync + 'static {
    Router::new()
        .route("/articles", get(get_help_articles))
        .route("/tooltips", get(get_tooltips))
        .route("/videos", get(get_videos))
}

async fn get_help_articles(Query(q): Query<HelpQuery>) -> Json<Vec<HelpArticle>> {
    let mut articles = vec![
        HelpArticle {
            id: "1".to_string(),
            title: "Getting Started with OneHuman".to_string(),
            content: "Welcome to OneHuman! We're excited to help you set up your small business. The first step is to configure your store settings and add your logo. Think of this as opening your storefront to the world.".to_string(),
            category: "Getting Started".to_string(),
            summary: "Learn the basics of setting up your account and launching your store.".to_string(),
        },
        HelpArticle {
            id: "2".to_string(),
            title: "Accepting Payments".to_string(),
            content: "Connect your bank account to start accepting credit card payments from your customers. Go to Settings > Payments and follow the prompts. It only takes a few minutes and you're ready to make sales!".to_string(),
            category: "Payments".to_string(),
            summary: "How to connect a bank account and accept payments.".to_string(),
        },
        HelpArticle {
            id: "3".to_string(),
            title: "Adding AI Support Agents".to_string(),
            content: "You can hire AI support agents to answer common customer questions automatically. Head over to the Agents tab and click 'Hire Agent'. They work 24/7 so you can focus on running your business.".to_string(),
            category: "AI Agents".to_string(),
            summary: "Hire an AI agent to handle customer support.".to_string(),
        },
        HelpArticle {
            id: "4".to_string(),
            title: "Your Store Front".to_string(),
            content: "Your store front is what your customers see. Keep it updated with your latest products and services. A beautiful store attracts more customers!".to_string(),
            category: "My Store".to_string(),
            summary: "Managing your public facing store.".to_string(),
        },
    ];

    if let Some(query) = q.query {
        let q_lower = query.to_lowercase();
        articles.retain(|a| {
            a.title.to_lowercase().contains(&q_lower) ||
            a.content.to_lowercase().contains(&q_lower) ||
            a.category.to_lowercase().contains(&q_lower) ||
            a.summary.to_lowercase().contains(&q_lower)
        });
    }

    Json(articles)
}

async fn get_tooltips(Query(q): Query<TooltipQuery>) -> Json<Vec<Tooltip>> {
    let tooltips = vec![
        Tooltip {
            id: "tt_1".to_string(),
            content: "Click here to upload your business logo.".to_string(),
            target_element: "logo-upload".to_string(),
        },
        Tooltip {
            id: "tt_2".to_string(),
            content: "Set the price your customers will pay for this item.".to_string(),
            target_element: "price-input".to_string(),
        },
        Tooltip {
            id: "tt_3".to_string(),
            content: "View your store exactly as your customers see it.".to_string(),
            target_element: "preview-store-btn".to_string(),
        },
        Tooltip {
            id: "tt_4".to_string(),
            content: "Connect your bank account to start getting paid.".to_string(),
            target_element: "connect-bank-btn".to_string(),
        },
    ];

    // Basic filtering (in a real app, you might filter by screen or just return all)
    Json(tooltips)
}

async fn get_videos() -> Json<Vec<HelpVideo>> {
    let videos = vec![
        HelpVideo {
            id: "v_1".to_string(),
            title: "Setting up your store".to_string(),
            url: "https://example.com/videos/setup-store.mp4".to_string(),
            thumbnail: "https://example.com/videos/setup-store-thumb.jpg".to_string(),
        },
        HelpVideo {
            id: "v_2".to_string(),
            title: "Accepting your first payment".to_string(),
            url: "https://example.com/videos/first-payment.mp4".to_string(),
            thumbnail: "https://example.com/videos/first-payment-thumb.jpg".to_string(),
        },
        HelpVideo {
            id: "v_3".to_string(),
            title: "Activating your AI Support Agent".to_string(),
            url: "https://example.com/videos/ai-support.mp4".to_string(),
            thumbnail: "https://example.com/videos/ai-support-thumb.jpg".to_string(),
        },
    ];
    Json(videos)
}
