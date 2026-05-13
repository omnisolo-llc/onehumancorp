use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;

// Simple internal representation for DocsRegistry methods
#[derive(Debug, Clone)]
pub struct Session {
    pub tenant_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
pub struct Article {
    pub id: String,
    pub tenant_id: String,
    pub category: String,
    pub title: String,
    pub content: String,
    pub read_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow, PartialEq)]
pub struct VideoTutorial {
    pub id: String,
    pub tenant_id: String,
    pub title: String,
    pub url: String,
    pub duration: i32,
}

pub struct DocsRegistry {
    pub pool: PgPool,
}

impl DocsRegistry {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_article(&self, session: &Session, id: String, category: String, title: String, content: String, read_time: String) -> Result<Article, String> {
        let tenant_id = session.tenant_id.clone();

        let article = sqlx::query_as::<_, Article>(
            "INSERT INTO articles (id, tenant_id, category, title, content, read_time)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, tenant_id, category, title, content, read_time"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(category)
        .bind(title)
        .bind(content)
        .bind(read_time)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(article)
    }

    pub async fn get_articles(&self, session: &Session) -> Result<Vec<Article>, String> {
        let tenant_id = session.tenant_id.clone();

        let articles = sqlx::query_as::<_, Article>(
            "SELECT id, tenant_id, category, title, content, read_time FROM articles WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(articles)
    }

    pub async fn create_video_tutorial(&self, session: &Session, id: String, title: String, url: String, duration: i32) -> Result<VideoTutorial, String> {
        let tenant_id = session.tenant_id.clone();

        if duration > 90 {
            return Err("Duration exceeds 90 seconds limit".to_string());
        }

        let video = sqlx::query_as::<_, VideoTutorial>(
            "INSERT INTO video_tutorials (id, tenant_id, title, url, duration)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, tenant_id, title, url, duration"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(title)
        .bind(url)
        .bind(duration)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(video)
    }

    pub async fn get_videos(&self, session: &Session) -> Result<Vec<VideoTutorial>, String> {
        let tenant_id = session.tenant_id.clone();

        let videos = sqlx::query_as::<_, VideoTutorial>(
            "SELECT id, tenant_id, title, url, duration FROM video_tutorials WHERE tenant_id = $1"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(videos)
    }
}

// Axum Handlers

async fn list_articles(
    State(transport): State<Arc<dyn MeshTransport>>,
) -> Json<Vec<Article>> {
    // For now we return dummy data as the true Session extraction requires full middleware setup
    Json(vec![
        Article {
            id: "getting-started".to_string(),
            tenant_id: "default".to_string(),
            category: "Getting Started".to_string(),
            title: "Welcome to OneHumanCorp".to_string(),
            content: "Setting up your store is quick and easy.".to_string(),
            read_time: "2 min".to_string(),
        }
    ])
}

async fn list_videos(
    State(transport): State<Arc<dyn MeshTransport>>,
) -> Json<Vec<VideoTutorial>> {
    Json(vec![
        VideoTutorial {
            id: "vid1".to_string(),
            tenant_id: "default".to_string(),
            title: "Set up your store in 60 seconds".to_string(),
            url: "https://example.com/setup-store.mp4".to_string(),
            duration: 60,
        }
    ])
}

pub fn router() -> Router<Arc<dyn MeshTransport>> {
    Router::new()
        .route("/articles", get(list_articles))
        .route("/videos", get(list_videos))
}
