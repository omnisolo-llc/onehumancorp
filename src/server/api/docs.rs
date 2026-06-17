use axum::{extract::{Query, Path, Extension}, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use sqlx::{FromRow, Row};

#[derive(Serialize, Clone, FromRow)]
pub struct HelpArticle {
    pub category: String,
    pub title: String,
    pub desc: String,
    pub link: String,
}

#[derive(Serialize, Clone, FromRow)]
pub struct VideoTutorial {
    pub id: i32,
    pub title: String,
    pub duration: String,
    pub video_url: String,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

#[derive(Serialize, Clone, FromRow)]
pub struct WalkthroughStep {
    pub selector: String,
    pub title: String,
    pub text: String,
}

pub async fn get_walkthrough(
    Extension(db): Extension<Arc<DB>>,
    Path(page): Path<String>
) -> Json<Vec<WalkthroughStep>> {
    let steps_result = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_as::<_, WalkthroughStep>(
                "SELECT selector, title, text FROM walkthrough_steps WHERE page_name = $1 ORDER BY step_order"
            ).bind(&page).fetch_all(&db.pool).await.map_err(|e| e.to_string())
        },
        crate::db::DbStore::Sqlite(p) => {
            sqlx::query_as::<_, WalkthroughStep>(
                "SELECT selector, title, text FROM walkthrough_steps WHERE page_name = ? ORDER BY step_order"
            ).bind(&page).fetch_all(p).await.map_err(|e| e.to_string())
        }
    };

    if let Ok(steps) = steps_result {
        if !steps.is_empty() {
            return Json(steps);
        }
    }

    let steps = match page.as_str() {
        "store-setup" => vec![
            WalkthroughStep { selector: "#dashboard-title".to_string(), title: "Set up your store".to_string(), text: "Learn how to easily set up your store and accept your first payment.".to_string() }
        ],
        "dashboard" => vec![
            WalkthroughStep { selector: "#dashboard-title".to_string(), title: "Welcome".to_string(), text: "Welcome to your dashboard! This is your control center.".to_string() },
            WalkthroughStep { selector: "#wrapped-summary".to_string(), title: "AI Savings".to_string(), text: "Here you can see the time and effort your agents have saved you.".to_string() }
        ],
        "pos" => vec![
            WalkthroughStep { selector: "#charge-btn".to_string(), title: "Accept your first payment".to_string(), text: "Enter an amount and tap here to charge.".to_string() }
        ],
        "assistant" => vec![
            WalkthroughStep { selector: "#ohc-help-input-area".to_string(), title: "Activate your AI Support Agent".to_string(), text: "Chat here to activate your AI agent.".to_string() }
        ],
        _ => vec![],
    };
    Json(steps)
}

pub async fn get_tooltips(Extension(db): Extension<Arc<DB>>) -> Json<std::collections::HashMap<String, String>> {
    let mut tooltips = std::collections::HashMap::new();

    let rows_result = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("SELECT tooltip_key, tooltip_text FROM tooltip_registry").fetch_all(&db.pool).await.map(|rows| {
                rows.into_iter().map(|r| (r.get::<String, _>(0), r.get::<String, _>(1))).collect::<Vec<_>>()
            })
        },
        crate::db::DbStore::Sqlite(p) => {
            sqlx::query("SELECT tooltip_key, tooltip_text FROM tooltip_registry").fetch_all(p).await.map(|rows| {
                rows.into_iter().map(|r| (r.get::<String, _>(0), r.get::<String, _>(1))).collect::<Vec<_>>()
            })
        }
    };

    if let Ok(rows) = rows_result {
        for (key, text) in rows {
            tooltips.insert(key, text);
        }
    }

    let registry = crate::get_tooltips_registry().read().unwrap();
    for (k, v) in registry.iter() {
        tooltips.insert(k.clone(), v.clone());
    }
    Json(tooltips)
}

#[derive(Deserialize)]
pub struct TooltipPayload {
    pub id: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub success: bool,
}

pub async fn update_tooltip(
    Extension(db): Extension<Arc<DB>>,
    axum::extract::Json(payload): axum::extract::Json<TooltipPayload>
) -> Json<SuccessResponse> {
    match &db.store {
        crate::db::DbStore::Postgres => {
            let _ = sqlx::query("INSERT INTO tooltip_registry (id, tooltip_key, tooltip_text) VALUES ($1, $2, $3) ON CONFLICT (tooltip_key) DO UPDATE SET tooltip_text = EXCLUDED.tooltip_text")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&payload.id)
                .bind(&payload.text)
                .execute(&db.pool).await;
        },
        crate::db::DbStore::Sqlite(p) => {
            let _ = sqlx::query("INSERT OR REPLACE INTO tooltip_registry (id, tooltip_key, tooltip_text) VALUES (?, ?, ?)")
                .bind(uuid::Uuid::new_v4().to_string())
                .bind(&payload.id)
                .bind(&payload.text)
                .execute(p).await;
        }
    };

    let mut registry = crate::get_tooltips_registry().write().unwrap();
    registry.insert(payload.id, payload.text);
    Json(SuccessResponse { success: true })
}

pub fn get_articles_static() -> Vec<HelpArticle> {
    vec![
        HelpArticle { category: "Getting Started".to_string(), title: "Getting Started with Your Store".to_string(), desc: "Welcome to OneHumanCorp! Let's get your business online in under 10 minutes.".to_string(), link: "/help/getting-started-1".to_string() },
        HelpArticle { category: "My Store".to_string(), title: "Adding Products".to_string(), desc: "Add products, track what's in stock, and change how your store looks.".to_string(), link: "/help/add-products".to_string() },
        HelpArticle { category: "Payments".to_string(), title: "Accepting Payments".to_string(), desc: "Learn how to accept credit cards and manage your payouts.".to_string(), link: "/help/accept-payments".to_string() },
        HelpArticle { category: "AI Agents".to_string(), title: "Activate AI Support".to_string(), desc: "Let our AI handle customer inquiries and triage your inbox.".to_string(), link: "/help/ai-support".to_string() },
        HelpArticle { category: "Marketing".to_string(), title: "Grow Your Audience".to_string(), desc: "Use our built-in tools to run promotions and track performance.".to_string(), link: "/help/marketing-tools".to_string() },
        HelpArticle { category: "Account & Billing".to_string(), title: "Manage Billing".to_string(), desc: "Update your subscription and payment methods.".to_string(), link: "/help/billing-settings".to_string() },
        HelpArticle { category: "Advanced".to_string(), title: "API Reference".to_string(), desc: "Use our OpenAPI specs to integrate with OHC.".to_string(), link: "/api-docs".to_string() },
    ]
}

pub fn get_videos_static() -> Vec<VideoTutorial> {
    vec![
        VideoTutorial { id: 1, title: "How to set up your store in 5 minutes".to_string(), duration: "1:15".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 2, title: "Connecting a bank account to accept payments".to_string(), duration: "0:45".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 3, title: "Activating your AI Support Agent".to_string(), duration: "1:25".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 4, title: "Adding a new product to your inventory".to_string(), duration: "0:50".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 5, title: "Managing staff and user permissions".to_string(), duration: "1:10".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 6, title: "Creating a marketing campaign".to_string(), duration: "1:20".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
        VideoTutorial { id: 7, title: "Using the Analytics Dashboard".to_string(), duration: "1:20".to_string(), video_url: "https://www.w3schools.com/html/mov_bbb.mp4".to_string() },
    ]
}

pub async fn list_articles(Extension(db): Extension<Arc<DB>>) -> Json<Vec<HelpArticle>> {
    let db_articles = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_as::<_, HelpArticle>(
                "SELECT category, title, description as \"desc\", link FROM help_articles"
            ).fetch_all(&db.pool).await
        },
        crate::db::DbStore::Sqlite(p) => {
            sqlx::query_as::<_, HelpArticle>(
                "SELECT category, title, description as \"desc\", link FROM help_articles"
            ).fetch_all(p).await
        }
    };

    if let Ok(articles) = db_articles {
        if !articles.is_empty() {
            return Json(articles);
        }
    }
    Json(get_articles_static())
}

pub async fn search_articles(
    Extension(db): Extension<Arc<DB>>,
    Query(search): Query<SearchQuery>
) -> Json<Vec<HelpArticle>> {
    let q_term = format!("%{}%", search.q.to_lowercase());
    let db_articles = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_as::<_, HelpArticle>(
                "SELECT category, title, description as \"desc\", link FROM help_articles WHERE LOWER(title) LIKE $1 OR LOWER(description) LIKE $1"
            ).bind(&q_term).fetch_all(&db.pool).await
        },
        crate::db::DbStore::Sqlite(p) => {
            sqlx::query_as::<_, HelpArticle>(
                "SELECT category, title, description as \"desc\", link FROM help_articles WHERE LOWER(title) LIKE ? OR LOWER(description) LIKE ?"
            ).bind(&q_term).bind(&q_term).fetch_all(p).await
        }
    };

    if let Ok(articles) = db_articles {
        if !articles.is_empty() {
            return Json(articles);
        }
    }

    let articles = get_articles_static();
    let q = search.q.to_lowercase();
    let filtered: Vec<HelpArticle> = articles.into_iter()
        .filter(|a| a.title.to_lowercase().contains(&q) || a.desc.to_lowercase().contains(&q))
        .collect();
    Json(filtered)
}

pub async fn list_videos(Extension(db): Extension<Arc<DB>>) -> Json<Vec<VideoTutorial>> {
    let db_videos = match &db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query_as::<_, VideoTutorial>(
                "SELECT id as \"id: i32\", title, duration, video_url FROM video_tutorials"
            ).fetch_all(&db.pool).await
        },
        crate::db::DbStore::Sqlite(p) => {
            sqlx::query_as::<_, VideoTutorial>(
                "SELECT id as \"id: i32\", title, duration, video_url FROM video_tutorials"
            ).fetch_all(p).await
        }
    };

    if let Ok(videos) = db_videos {
        if !videos.is_empty() {
            return Json(videos);
        }
    }
    Json(get_videos_static())
}

#[derive(Serialize)]
pub struct ChangelogEntry {
    pub date: String,
    pub title: String,
    pub content: String,
}

pub async fn get_changelog() -> Json<Vec<ChangelogEntry>> {
    Json(vec![
        ChangelogEntry { date: "2024-06-17".to_string(), title: "Scribe Help Center Launched".to_string(), content: "We've launched a new integrated help center, tooltip registry, and interactive walkthroughs.".to_string() },
        ChangelogEntry { date: "2024-06-10".to_string(), title: "AI Swarm Orchestrator Improvements".to_string(), content: "Better coordination and faster task execution for your AI workforce.".to_string() },
    ])
}

pub async fn get_api_docs_spec() -> Json<serde_json::Value> {
    let spec = serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "OneHumanCorp API",
            "version": "1.0.0",
            "description": "API for owners and operators to coordinate people and tools."
        },
        "paths": {
            "/api/help": {
                "get": {
                    "summary": "List help articles",
                    "responses": {
                        "200": {
                            "description": "Success",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": { "$ref": "#/components/schemas/HelpArticle" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        },
        "components": {
            "schemas": {
                "HelpArticle": {
                    "type": "object",
                    "properties": {
                        "category": { "type": "string" },
                        "title": { "type": "string" },
                        "desc": { "type": "string" },
                        "link": { "type": "string" }
                    }
                }
            }
        }
    });
    Json(spec)
}

pub async fn get_article_handler(Path(article_id): Path<String>) -> Json<HelpArticle> {
    let articles = get_articles_static();
    let article = articles.iter().find(|a| a.link.contains(&article_id))
        .cloned()
        .unwrap_or(HelpArticle {
            category: "General".to_string(),
            title: "Article Not Found".to_string(),
            desc: "Sorry, we couldn't find the requested article.".to_string(),
            link: "#".to_string(),
        });
    Json(article)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_db() -> Arc<DB> {
        let pool = crate::db::get_pool();
        Arc::new(DB {
            pool: pool.clone(),
            store: crate::db::DbStore::Postgres,
        })
    }

    #[tokio::test]
    async fn test_list_articles() {
        let db = setup_db().await;
        let res = list_articles(Extension(db)).await;
        assert!(!res.0.is_empty());
    }

    #[tokio::test]
    async fn test_search_articles_found() {
        let db = setup_db().await;
        let res = search_articles(Extension(db), Query(SearchQuery { q: "getting".to_string() })).await;
        assert!(!res.0.is_empty());
    }

    #[tokio::test]
    async fn test_list_videos() {
        let db = setup_db().await;
        let res = list_videos(Extension(db)).await;
        assert!(!res.0.is_empty());
    }
}
