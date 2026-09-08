use axum::{
    Json,
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::common::auth_utils::signed_tenant_id;
use crate::db::{DB, DbStore};

const MAX_TENANT_ID_LENGTH: usize = 128;
const MAX_NEIGHBORS: i64 = 50;

#[derive(Debug, Deserialize)]
pub struct CollectiveQuery {
    pub action: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CollectiveActionRequest {
    pub action: String,
    pub target_tenant_id: Option<String>,
}

fn json_error(status: StatusCode, message: &'static str) -> Response {
    (status, Json(serde_json::json!({ "error": message }))).into_response()
}

pub(crate) fn validate_tenant_id_for_invite(
    owner_tenant_id: &str,
    target_tenant_id: &str,
) -> Result<String, &'static str> {
    let target_tenant_id = target_tenant_id.trim();
    if target_tenant_id.is_empty()
        || target_tenant_id.chars().count() > MAX_TENANT_ID_LENGTH
        || !target_tenant_id.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ':')
        })
    {
        return Err("invalid target tenant id");
    }
    if target_tenant_id.eq_ignore_ascii_case("system") {
        return Err("system tenant cannot be invited");
    }
    if target_tenant_id == owner_tenant_id {
        return Err("cannot invite the current tenant");
    }
    Ok(target_tenant_id.to_string())
}

fn authenticated_tenant(claims: &::server_common::Claims) -> Result<String, Response> {
    signed_tenant_id(claims)
        .ok_or_else(|| json_error(StatusCode::UNAUTHORIZED, "authentication required"))
}

async fn set_collective_bypass_role(
    executor: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    // The query always applies an explicit owner-tenant predicate below. The
    // bypass role is needed only because the member rows are intentionally
    // shared across the tenants that participate in one collective.
    sqlx::query("SET LOCAL ROLE ohc_bypassrls")
        .execute(&mut **executor)
        .await
        .map(|_| ())
}

pub(crate) async fn list_nearby_tenants(
    db: &DB,
    owner_tenant_id: &str,
) -> Result<Vec<String>, sqlx::Error> {
    if let Some(pool) = crate::db::get_mysql_pool_if_exists() {
        return sqlx::query_scalar::<_, String>(
            "SELECT DISTINCT m.tenant_id
             FROM ohc_collective c
             JOIN ohc_collective_member m ON m.collective_id = c.id
             WHERE c.tenant_id = ?
               AND m.tenant_id <> ?
               AND m.status IN ('ACTIVE', 'PENDING')
             LIMIT ?",
        )
        .bind(owner_tenant_id)
        .bind(owner_tenant_id)
        .bind(MAX_NEIGHBORS)
        .fetch_all(&pool)
        .await;
    }

    match &db.store {
        DbStore::Postgres => {
            let mut transaction = db.pool.begin().await?;
            set_collective_bypass_role(&mut transaction).await?;
            let neighbors = sqlx::query_scalar::<_, String>(
                "SELECT DISTINCT m.tenant_id
                 FROM ohc_collective c
                 JOIN ohc_collective_member m ON m.collective_id = c.id
                 WHERE c.tenant_id = $1
                   AND m.tenant_id <> $1
                   AND m.status IN ('ACTIVE', 'PENDING')
                 LIMIT $2",
            )
            .bind(owner_tenant_id)
            .bind(MAX_NEIGHBORS)
            .fetch_all(&mut *transaction)
            .await?;
            transaction.commit().await?;
            Ok(neighbors)
        }
        DbStore::Sqlite(pool) => {
            sqlx::query_scalar::<_, String>(
                "SELECT DISTINCT m.tenant_id
                 FROM ohc_collective c
                 JOIN ohc_collective_member m ON m.collective_id = c.id
                 WHERE c.tenant_id = ?
                   AND m.tenant_id <> ?
                   AND m.status IN ('ACTIVE', 'PENDING')
                 LIMIT ?",
            )
            .bind(owner_tenant_id)
            .bind(owner_tenant_id)
            .bind(MAX_NEIGHBORS)
            .fetch_all(pool)
            .await
        }
    }
}

pub async fn get_nearby_tenants_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<::server_common::Claims>,
    Query(query): Query<CollectiveQuery>,
) -> Response {
    if query
        .action
        .as_deref()
        .is_some_and(|action| action != "getNearby")
    {
        return json_error(StatusCode::BAD_REQUEST, "unsupported collective action");
    }
    let owner_tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(response) => return response,
    };
    match list_nearby_tenants(&db, &owner_tenant_id).await {
        Ok(neighbors) => Json(serde_json::json!({ "neighbors": neighbors })).into_response(),
        Err(error) => {
            tracing::warn!(tenant_id = %owner_tenant_id, "collective discovery unavailable: {error}");
            json_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "collective discovery unavailable",
            )
        }
    }
}

#[derive(Debug)]
enum InviteStoreError {
    TargetTenantNotFound,
    Database(sqlx::Error),
}

async fn invite_tenant(
    db: &DB,
    owner_tenant_id: &str,
    target_tenant_id: &str,
) -> Result<(), InviteStoreError> {
    if let Some(pool) = crate::db::get_mysql_pool_if_exists() {
        let mut transaction = pool.begin().await.map_err(InviteStoreError::Database)?;
        let target_exists = sqlx::query_scalar::<_, i64>(
            "SELECT EXISTS (SELECT 1 FROM users WHERE tenant_id = ? AND tenant_id <> 'system')",
        )
        .bind(target_tenant_id)
        .fetch_one(&mut *transaction)
        .await
        .map_err(InviteStoreError::Database)?;
        if target_exists == 0 {
            return Err(InviteStoreError::TargetTenantNotFound);
        }

        let collective_id = sqlx::query_scalar::<_, String>(
            "SELECT id FROM ohc_collective WHERE tenant_id = ? ORDER BY created_at ASC LIMIT 1",
        )
        .bind(owner_tenant_id)
        .fetch_optional(&mut *transaction)
        .await
        .map_err(InviteStoreError::Database)?;
        let collective_id = match collective_id {
            Some(id) => id,
            None => {
                let id = uuid::Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO ohc_collective (id, tenant_id, name) VALUES (?, ?, ?)")
                    .bind(&id)
                    .bind(owner_tenant_id)
                    .bind("Main Street Collective")
                    .execute(&mut *transaction)
                    .await
                    .map_err(InviteStoreError::Database)?;
                id
            }
        };

        sqlx::query(
            "INSERT IGNORE INTO ohc_collective_member (collective_id, tenant_id, status)
             VALUES (?, ?, 'ACTIVE')",
        )
        .bind(&collective_id)
        .bind(owner_tenant_id)
        .execute(&mut *transaction)
        .await
        .map_err(InviteStoreError::Database)?;
        sqlx::query(
            "INSERT IGNORE INTO ohc_collective_member (collective_id, tenant_id, status)
             VALUES (?, ?, 'PENDING')",
        )
        .bind(&collective_id)
        .bind(target_tenant_id)
        .execute(&mut *transaction)
        .await
        .map_err(InviteStoreError::Database)?;
        transaction
            .commit()
            .await
            .map_err(InviteStoreError::Database)?;
        return Ok(());
    }

    match &db.store {
        DbStore::Postgres => {
            let mut transaction = db.pool.begin().await.map_err(InviteStoreError::Database)?;
            set_collective_bypass_role(&mut transaction)
                .await
                .map_err(InviteStoreError::Database)?;
            let target_exists = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS (SELECT 1 FROM tenants WHERE id = $1 AND id <> 'system')",
            )
            .bind(target_tenant_id)
            .fetch_one(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            if !target_exists {
                return Err(InviteStoreError::TargetTenantNotFound);
            }

            let collective_id = sqlx::query_scalar::<_, String>(
                "SELECT id FROM ohc_collective WHERE tenant_id = $1 ORDER BY created_at ASC LIMIT 1",
            )
            .bind(owner_tenant_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            let collective_id = match collective_id {
                Some(id) => id,
                None => {
                    let id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO ohc_collective (id, tenant_id, name) VALUES ($1, $2, $3)",
                    )
                    .bind(&id)
                    .bind(owner_tenant_id)
                    .bind("Main Street Collective")
                    .execute(&mut *transaction)
                    .await
                    .map_err(InviteStoreError::Database)?;
                    id
                }
            };

            sqlx::query(
                "INSERT INTO ohc_collective_member (collective_id, tenant_id, status)
                 VALUES ($1, $2, 'ACTIVE') ON CONFLICT DO NOTHING",
            )
            .bind(&collective_id)
            .bind(owner_tenant_id)
            .execute(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            sqlx::query(
                "INSERT INTO ohc_collective_member (collective_id, tenant_id, status)
                 VALUES ($1, $2, 'PENDING') ON CONFLICT DO NOTHING",
            )
            .bind(&collective_id)
            .bind(target_tenant_id)
            .execute(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            transaction
                .commit()
                .await
                .map_err(InviteStoreError::Database)
        }
        DbStore::Sqlite(pool) => {
            let mut transaction = pool.begin().await.map_err(InviteStoreError::Database)?;
            let target_exists = sqlx::query_scalar::<_, i64>(
                "SELECT EXISTS (SELECT 1 FROM tenants WHERE id = ? AND id <> 'system')",
            )
            .bind(target_tenant_id)
            .fetch_one(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            if target_exists == 0 {
                return Err(InviteStoreError::TargetTenantNotFound);
            }
            let collective_id = sqlx::query_scalar::<_, String>(
                "SELECT id FROM ohc_collective WHERE tenant_id = ? ORDER BY created_at ASC LIMIT 1",
            )
            .bind(owner_tenant_id)
            .fetch_optional(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            let collective_id = match collective_id {
                Some(id) => id,
                None => {
                    let id = uuid::Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO ohc_collective (id, tenant_id, name) VALUES (?, ?, ?)",
                    )
                    .bind(&id)
                    .bind(owner_tenant_id)
                    .bind("Main Street Collective")
                    .execute(&mut *transaction)
                    .await
                    .map_err(InviteStoreError::Database)?;
                    id
                }
            };
            sqlx::query(
                "INSERT OR IGNORE INTO ohc_collective_member (collective_id, tenant_id, status)
                 VALUES (?, ?, 'ACTIVE')",
            )
            .bind(&collective_id)
            .bind(owner_tenant_id)
            .execute(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            sqlx::query(
                "INSERT OR IGNORE INTO ohc_collective_member (collective_id, tenant_id, status)
                 VALUES (?, ?, 'PENDING')",
            )
            .bind(&collective_id)
            .bind(target_tenant_id)
            .execute(&mut *transaction)
            .await
            .map_err(InviteStoreError::Database)?;
            transaction
                .commit()
                .await
                .map_err(InviteStoreError::Database)
        }
    }
}

pub async fn invite_tenant_handler(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(request): Json<CollectiveActionRequest>,
) -> Response {
    if request.action != "invite" {
        return json_error(StatusCode::BAD_REQUEST, "unsupported collective action");
    }
    let owner_tenant_id = match authenticated_tenant(&claims) {
        Ok(tenant_id) => tenant_id,
        Err(response) => return response,
    };
    let Some(target_tenant_id) = request.target_tenant_id.as_deref() else {
        return json_error(StatusCode::BAD_REQUEST, "target tenant id is required");
    };
    let target_tenant_id = match validate_tenant_id_for_invite(&owner_tenant_id, target_tenant_id) {
        Ok(target_tenant_id) => target_tenant_id,
        Err(message) => return json_error(StatusCode::BAD_REQUEST, message),
    };
    match invite_tenant(&db, &owner_tenant_id, &target_tenant_id).await {
        Ok(()) => Json(serde_json::json!({ "success": true })).into_response(),
        Err(InviteStoreError::TargetTenantNotFound) => {
            json_error(StatusCode::NOT_FOUND, "target tenant not found")
        }
        Err(InviteStoreError::Database(error)) => {
            tracing::warn!(tenant_id = %owner_tenant_id, "collective invite unavailable: {error}");
            json_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "collective invite unavailable",
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InviteStoreError, invite_tenant, list_nearby_tenants, validate_tenant_id_for_invite,
    };

    #[test]
    fn invite_target_must_be_a_real_non_system_tenant_identifier() {
        assert!(validate_tenant_id_for_invite("org-1", "").is_err());
        assert!(validate_tenant_id_for_invite("org-1", "system").is_err());
        assert!(validate_tenant_id_for_invite("org-1", "org-1").is_err());
        assert!(validate_tenant_id_for_invite("org-1", "other tenant").is_err());
        assert_eq!(
            validate_tenant_id_for_invite("org-1", "tenant_2").unwrap(),
            "tenant_2"
        );
    }

    #[tokio::test]
    async fn nearby_tenants_are_read_from_collective_members() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        for statement in [
            "CREATE TABLE tenants (id TEXT PRIMARY KEY)",
            "CREATE TABLE ohc_collective (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, location_center TEXT, radius_meters REAL, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE ohc_collective_member (collective_id TEXT NOT NULL, tenant_id TEXT NOT NULL, status TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (collective_id, tenant_id))",
            "INSERT INTO tenants (id) VALUES ('tenant-a'), ('tenant-b')",
            "INSERT INTO ohc_collective (id, tenant_id, name) VALUES ('collective-a', 'tenant-a', 'Main Street Collective')",
            "INSERT INTO ohc_collective_member (collective_id, tenant_id, status) VALUES ('collective-a', 'tenant-a', 'ACTIVE'), ('collective-a', 'tenant-b', 'PENDING')",
        ] {
            sqlx::query(statement).execute(&pool).await.unwrap();
        }

        let db = crate::db::DB {
            pool: crate::db::create_dummy_pg_pool().await,
            store: crate::db::DbStore::Sqlite(pool),
        };
        let neighbors = list_nearby_tenants(&db, "tenant-a").await.unwrap();
        assert_eq!(neighbors, vec!["tenant-b".to_string()]);
    }

    #[tokio::test]
    async fn invite_requires_an_existing_target_and_persists_a_pending_member() {
        let pool = crate::db::create_sqlite_pool_for_test().await;
        for statement in [
            "CREATE TABLE tenants (id TEXT PRIMARY KEY)",
            "CREATE TABLE ohc_collective (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, location_center TEXT, radius_meters REAL, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE ohc_collective_member (collective_id TEXT NOT NULL, tenant_id TEXT NOT NULL, status TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP, PRIMARY KEY (collective_id, tenant_id))",
            "INSERT INTO tenants (id) VALUES ('tenant-a'), ('tenant-b')",
        ] {
            sqlx::query(statement).execute(&pool).await.unwrap();
        }
        let db = crate::db::DB {
            pool: crate::db::create_dummy_pg_pool().await,
            store: crate::db::DbStore::Sqlite(pool.clone()),
        };

        invite_tenant(&db, "tenant-a", "tenant-b").await.unwrap();
        let status = sqlx::query_scalar::<_, String>(
            "SELECT status FROM ohc_collective_member WHERE tenant_id = 'tenant-b'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(status, "PENDING");

        assert!(matches!(
            invite_tenant(&db, "tenant-a", "missing-tenant").await,
            Err(InviteStoreError::TargetTenantNotFound)
        ));
    }
}
