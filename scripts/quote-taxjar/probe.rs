//! PostgreSQL contracts compiling the mounted quote handler and actual TaxJar client.
#![allow(dead_code)]
use axum::{
    Json,
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

pub mod integrations {
    pub use server_integrations_taxjar as taxjar;
}
pub fn is_standalone_runtime() -> bool {
    false
}

#[path = "../../src/server/api/quote_taxjar.rs"]
mod quote_taxjar;

// The build script selects exact original source bytes with the shared Rust
// parser, rejects missing/ambiguous/invalid syntax, and records source hashes.
include!(concat!(env!("OUT_DIR"), "/quote_handler.rs"));

#[cfg(test)]
mod money_test;
#[cfg(test)]
mod provider_test;
