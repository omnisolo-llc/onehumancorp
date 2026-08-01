use axum::{
    extract::{State, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::DB;

#[derive(Clone)]
pub struct ChatState {
    pub db: Arc<DB>,
}

#[derive(Serialize)]
pub struct ChatResponse<T> {
    pub success: bool,
    pub data: Option<T>,
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
}
