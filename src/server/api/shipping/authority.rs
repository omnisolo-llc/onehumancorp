//! Private shipping uses the existing canonical owner transaction boundary.
//! Provider webhooks have a separate, explicitly configured account authority.
use crate::db::{DB, DbStore};
use axum::{
    Json,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use server_auth::commit_authority::{
    AuthorityError, AuthorizedPgOwner, AuthorizedSqliteOwner, CanonicalPgAuthority,
    CanonicalSqliteAuthority,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ShippingAccess {
    store: Arc<server_auth::Store>,
    postgres: Option<CanonicalPgAuthority>,
    sqlite: Option<CanonicalSqliteAuthority>,
}
impl ShippingAccess {
    /// Prove the configured business objects are the canonical identity store's
    /// objects before replacing its pool with that canonical pool. Never copy or
    /// infer identity from matching tenant IDs, rows, credentials, or filenames.
    pub async fn configured(db: &DB, store: Arc<server_auth::Store>) -> Self {
        let mut access = Self {
            store: store.clone(),
            postgres: None,
            sqlite: None,
        };
        match &db.store {
            DbStore::Postgres => {
                if let Some(repository) = store.portable_repo() {
                    match server_auth::commit_authority::canonical_pg_data_pool(
                        repository.connection(),
                        &db.pool,
                        &[
                            "orders",
                            "delivery_tasks",
                            "delivery_provider_bindings",
                            "shipping_purchase_intents",
                            "customers",
                            "order_items",
                            "products",
                        ],
                    )
                    .await
                    {
                        Ok(pool) => access.postgres = CanonicalPgAuthority::bind(store, &pool).ok(),
                        Err(error) => {
                            tracing::warn!(%error,"Canonical shipping business storage binding unavailable")
                        }
                    }
                }
            }
            DbStore::Sqlite(pool) => {
                // The shared helper proves exact canonical pool identity. An
                // independently opened SQLite pool is unsupported, even when a
                // filename or copied user row happens to match.
                access.sqlite = CanonicalSqliteAuthority::bind(store, pool).ok();
            }
        }
        access
    }
    pub async fn authorize(
        &self,
        claims: &server_common::Claims,
        headers: &HeaderMap,
    ) -> Result<AuthorizedOwner, Error> {
        server_auth::commit_authority::verify_owner(&self.store, claims, headers).await?;
        if let Some(authority) = &self.postgres {
            return Ok(AuthorizedOwner::Postgres(
                authority.authorize(claims, headers).await?,
            ));
        }
        if let Some(authority) = &self.sqlite {
            return Ok(AuthorizedOwner::Sqlite(
                authority.authorize(claims, headers).await?,
            ));
        }
        Err(AuthorityError::Unavailable.into())
    }
    pub async fn authorize_postgres(
        &self,
        claims: &server_common::Claims,
        headers: &HeaderMap,
    ) -> Result<AuthorizedPgOwner, Error> {
        match self.authorize(claims, headers).await? {
            AuthorizedOwner::Postgres(owner) => Ok(owner),
            AuthorizedOwner::Sqlite(_) => Err(AuthorityError::Unavailable.into()),
        }
    }
}
pub enum AuthorizedOwner {
    Postgres(AuthorizedPgOwner),
    Sqlite(AuthorizedSqliteOwner),
}
impl AuthorizedOwner {
    pub fn tenant_id(&self) -> &str {
        match self {
            Self::Postgres(owner) => owner.tenant_id(),
            Self::Sqlite(owner) => owner.tenant_id(),
        }
    }
    pub fn actor_id(&self) -> &str {
        match self {
            Self::Postgres(owner) => owner.actor_id(),
            Self::Sqlite(owner) => owner.actor_id(),
        }
    }
    pub async fn confirm(self) -> Result<(), Error> {
        match self {
            Self::Postgres(owner) => owner.begin().await?.commit().await?,
            Self::Sqlite(owner) => owner.begin().await?.commit().await?,
        }
        Ok(())
    }
}
#[derive(Debug)]
pub enum Error {
    Authority(AuthorityError),
    Database(sqlx::Error),
    Conflict(&'static str),
}
impl From<AuthorityError> for Error {
    fn from(error: AuthorityError) -> Self {
        Self::Authority(error)
    }
}
impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => write!(f, "{error}"),
            Self::Database(error) => write!(f, "{error}"),
            Self::Conflict(reason) => write!(f, "{reason}"),
        }
    }
}
pub fn response(error: Error) -> Response {
    let (status, message) = match error {
        Error::Authority(AuthorityError::Forbidden) => (
            StatusCode::FORBIDDEN,
            "Current canonical business owner authority is required",
        ),
        Error::Conflict(message) => (StatusCode::CONFLICT, message),
        error => {
            tracing::warn!(%error,"Private shipping operation is unconfirmed");
            (
                StatusCode::SERVICE_UNAVAILABLE,
                "Canonical shipping storage is unavailable; the operation is unconfirmed",
            )
        }
    };
    (
        status,
        Json(serde_json::json!({"success":false,"error":message})),
    )
        .into_response()
}
