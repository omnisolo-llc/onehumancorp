use axum::{
    extract::{Extension, State, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

use crate::db::{DB, DbStore};
use ::server_common::Claims;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Vendor {
    pub id: String,
    pub name: String,
    pub contact_info: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct RawMaterial {
    pub id: String,
    pub name: String,
    pub current_quantity: i32,
    pub reorder_threshold: i32,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct BomItem {
    pub id: String,
    pub finished_good_id: String,
    pub raw_material_id: String,
    pub quantity_required: i32,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/vendors", get(list_vendors).post(create_vendor))
        .route("/raw_materials", get(list_raw_materials).post(create_raw_material))
        .route("/bom_items", get(list_bom_items).post(create_bom_item))
        .with_state(db)
}

async fn list_vendors(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json::<Vec<Vendor>>(vec![])).into_response();
    }

    let rows: Vec<Vendor> = match &db.store {
        DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<Vendor>::new())).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<Vendor>::new())).into_response();
            }
            let res = sqlx::query_as::<_, Vendor>("SELECT id, name, contact_info FROM vendors WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&mut *tx).await.unwrap_or_default();
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<Vendor>::new())).into_response();
            }
            res
        }
        DbStore::Sqlite(pool) => {
            sqlx::query_as::<_, Vendor>("SELECT id, name, contact_info FROM vendors WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_all(pool).await.unwrap_or_default()
        }
    };

    (StatusCode::OK, Json(rows)).into_response()
}

async fn create_vendor(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(mut payload): Json<Vendor>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(payload)).into_response();
    }

    if payload.id.is_empty() {
        payload.id = Uuid::new_v4().to_string();
    }

    let contact_info = payload.contact_info.clone().unwrap_or_default();

    match &db.store {
        DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
            if let Err(e) = sqlx::query("INSERT INTO vendors (id, tenant_id, name, contact_info) VALUES ($1, $2, $3, $4)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.name).bind(&contact_info).execute(&mut *tx).await {
                tracing::error!("Failed to insert vendor: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
        }
        DbStore::Sqlite(pool) => {
            let _ = sqlx::query("INSERT INTO vendors (id, tenant_id, name, contact_info) VALUES (?, ?, ?, ?)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.name).bind(&contact_info).execute(pool).await;
        }
    }

    (StatusCode::OK, Json(payload)).into_response()
}

async fn list_raw_materials(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json::<Vec<RawMaterial>>(vec![])).into_response();
    }

    let rows: Vec<RawMaterial> = match &db.store {
        DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<RawMaterial>::new())).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<RawMaterial>::new())).into_response();
            }
            let res = sqlx::query_as::<_, RawMaterial>("SELECT id, name, current_quantity, reorder_threshold FROM raw_materials WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&mut *tx).await.unwrap_or_default();
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<RawMaterial>::new())).into_response();
            }
            res
        }
        DbStore::Sqlite(pool) => {
            sqlx::query_as::<_, RawMaterial>("SELECT id, name, current_quantity, reorder_threshold FROM raw_materials WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_all(pool).await.unwrap_or_default()
        }
    };

    (StatusCode::OK, Json(rows)).into_response()
}

async fn create_raw_material(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(mut payload): Json<RawMaterial>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(payload)).into_response();
    }

    if payload.id.is_empty() {
        payload.id = Uuid::new_v4().to_string();
    }

    match &db.store {
        DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
            if let Err(e) = sqlx::query("INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold) VALUES ($1, $2, $3, $4, $5)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.name).bind(payload.current_quantity).bind(payload.reorder_threshold).execute(&mut *tx).await {
                tracing::error!("Failed to insert raw material: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
        }
        DbStore::Sqlite(pool) => {
            let _ = sqlx::query("INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold) VALUES (?, ?, ?, ?, ?)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.name).bind(payload.current_quantity).bind(payload.reorder_threshold).execute(pool).await;
        }
    }

    (StatusCode::OK, Json(payload)).into_response()
}

async fn list_bom_items(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json::<Vec<BomItem>>(vec![])).into_response();
    }

    let rows: Vec<BomItem> = match &db.store {
        DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<BomItem>::new())).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<BomItem>::new())).into_response();
            }
            let res = sqlx::query_as::<_, BomItem>("SELECT id, finished_good_id, raw_material_id, quantity_required FROM bom_items WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&mut *tx).await.unwrap_or_default();
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<BomItem>::new())).into_response();
            }
            res
        }
        DbStore::Sqlite(pool) => {
            sqlx::query_as::<_, BomItem>("SELECT id, finished_good_id, raw_material_id, quantity_required FROM bom_items WHERE tenant_id = ?")
                .bind(&tenant_id)
                .fetch_all(pool).await.unwrap_or_default()
        }
    };

    (StatusCode::OK, Json(rows)).into_response()
}

async fn create_bom_item(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(mut payload): Json<BomItem>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(payload)).into_response();
    }

    if payload.id.is_empty() {
        payload.id = Uuid::new_v4().to_string();
    }

    match &db.store {
        DbStore::Postgres => {
            let mut tx = match db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {:?}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
                }
            };
            if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
                tracing::error!("Failed to set org context: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
            if let Err(e) = sqlx::query("INSERT INTO bom_items (id, tenant_id, finished_good_id, raw_material_id, quantity_required) VALUES ($1, $2, $3, $4, $5)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.finished_good_id).bind(&payload.raw_material_id).bind(payload.quantity_required).execute(&mut *tx).await {
                tracing::error!("Failed to insert bom item: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
            if let Err(e) = tx.commit().await {
                tracing::error!("Failed to commit transaction: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(payload)).into_response();
            }
        }
        DbStore::Sqlite(pool) => {
            let _ = sqlx::query("INSERT INTO bom_items (id, tenant_id, finished_good_id, raw_material_id, quantity_required) VALUES (?, ?, ?, ?, ?)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.finished_good_id).bind(&payload.raw_material_id).bind(payload.quantity_required).execute(pool).await;
        }
    }

    (StatusCode::OK, Json(payload)).into_response()
}
