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
        .route("/approve_po", post(approve_po))
        .route("/pending_pos", get(list_pending_pos))
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
            sqlx::query_as::<_, Vendor>("SELECT id, name, contact_info FROM vendors WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&db.pool).await.unwrap_or_default()
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
            let _ = sqlx::query("INSERT INTO vendors (id, tenant_id, name, contact_info) VALUES ($1, $2, $3, $4)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.name).bind(&contact_info).execute(&db.pool).await;
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
            sqlx::query_as::<_, RawMaterial>("SELECT id, name, current_quantity, reorder_threshold FROM raw_materials WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&db.pool).await.unwrap_or_default()
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
            let _ = sqlx::query("INSERT INTO raw_materials (id, tenant_id, name, current_quantity, reorder_threshold) VALUES ($1, $2, $3, $4, $5)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.name).bind(payload.current_quantity).bind(payload.reorder_threshold).execute(&db.pool).await;
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
            sqlx::query_as::<_, BomItem>("SELECT id, finished_good_id, raw_material_id, quantity_required FROM bom_items WHERE tenant_id = $1")
                .bind(&tenant_id)
                .fetch_all(&db.pool).await.unwrap_or_default()
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
            let _ = sqlx::query("INSERT INTO bom_items (id, tenant_id, finished_good_id, raw_material_id, quantity_required) VALUES ($1, $2, $3, $4, $5)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.finished_good_id).bind(&payload.raw_material_id).bind(payload.quantity_required).execute(&db.pool).await;
        }
        DbStore::Sqlite(pool) => {
            let _ = sqlx::query("INSERT INTO bom_items (id, tenant_id, finished_good_id, raw_material_id, quantity_required) VALUES (?, ?, ?, ?, ?)")
                .bind(&payload.id).bind(&tenant_id).bind(&payload.finished_good_id).bind(&payload.raw_material_id).bind(payload.quantity_required).execute(pool).await;
        }
    }

    (StatusCode::OK, Json(payload)).into_response()
}

#[derive(Deserialize)]
pub struct ApprovePoRequest {
    pub purchase_order_id: String,
}

async fn approve_po(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ApprovePoRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    match &db.store {
        DbStore::Postgres => {
            let res = sqlx::query("UPDATE purchase_orders SET status = 'APPROVED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
                .bind(&payload.purchase_order_id)
                .bind(&tenant_id)
                .execute(&db.pool).await;
            if res.is_ok() {
                StatusCode::OK.into_response()
            } else {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
        DbStore::Sqlite(pool) => {
            let res = sqlx::query("UPDATE purchase_orders SET status = 'APPROVED', updated_at = CURRENT_TIMESTAMP WHERE id = ? AND tenant_id = ?")
                .bind(&payload.purchase_order_id)
                .bind(&tenant_id)
                .execute(pool).await;
            if res.is_ok() {
                StatusCode::OK.into_response()
            } else {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[derive(Serialize)]
pub struct PendingPoDto {
    pub id: String,
    pub vendor_id: Option<String>,
    pub product_name: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub total_cost: f64,
    pub predicted_depletion_date: Option<chrono::DateTime<chrono::Utc>>,
    pub days_until_empty: i32,
}

async fn list_pending_pos(
    State(db): State<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_default();
    if tenant_id.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json::<Vec<PendingPoDto>>(vec![])).into_response();
    }

    // Join purchase_orders, po_line_items, products, and inventory_predictions
    // To return DRAFT POs along with AI prediction context.
    let query_postgres = r#"
        SELECT
            po.id, po.vendor_id, po.total_cost,
            pli.quantity, pli.unit_price,
            p.name as product_name,
            ip.predicted_depletion_date
        FROM purchase_orders po
        JOIN po_line_items pli ON po.id = pli.purchase_order_id
        JOIN products p ON p.id = pli.raw_material_id
        LEFT JOIN inventory_predictions ip ON ip.product_id = p.id
        WHERE po.tenant_id = $1 AND po.status = 'DRAFT'
    "#;

    let query_sqlite = r#"
        SELECT
            po.id, po.vendor_id, po.total_cost,
            pli.quantity, pli.unit_price,
            p.name as product_name,
            ip.predicted_depletion_date
        FROM purchase_orders po
        JOIN po_line_items pli ON po.id = pli.purchase_order_id
        JOIN products p ON p.id = pli.raw_material_id
        LEFT JOIN inventory_predictions ip ON ip.product_id = p.id
        WHERE po.tenant_id = ? AND po.status = 'DRAFT'
    "#;

    // A bit manual mapping to calculate days_until_empty
    match &db.store {
        DbStore::Postgres => {
            if let Ok(rows) = sqlx::query(query_postgres).bind(&tenant_id).fetch_all(&db.pool).await {
                let mut dtos = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let dep_date: Option<chrono::DateTime<chrono::Utc>> = r.get("predicted_depletion_date");
                    let days_until_empty = if let Some(d) = dep_date {
                        (d - chrono::Utc::now()).num_days() as i32
                    } else {
                        999
                    };
                    dtos.push(PendingPoDto {
                        id: r.get("id"),
                        vendor_id: r.get("vendor_id"),
                        product_name: r.get("product_name"),
                        quantity: r.get("quantity"),
                        unit_price: r.try_get("unit_price").unwrap_or(0.0),
                        total_cost: r.try_get("total_cost").unwrap_or(0.0),
                        predicted_depletion_date: dep_date,
                        days_until_empty,
                    });
                }
                (StatusCode::OK, Json(dtos)).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<PendingPoDto>>(vec![])).into_response()
            }
        }
        DbStore::Sqlite(pool) => {
            if let Ok(rows) = sqlx::query(query_sqlite).bind(&tenant_id).fetch_all(pool).await {
                let mut dtos = Vec::new();
                for r in rows {
                    use sqlx::Row;
                    let dep_date: Option<chrono::DateTime<chrono::Utc>> = r.get("predicted_depletion_date");
                    let days_until_empty = if let Some(d) = dep_date {
                        (d - chrono::Utc::now()).num_days() as i32
                    } else {
                        999
                    };
                    dtos.push(PendingPoDto {
                        id: r.get("id"),
                        vendor_id: r.get("vendor_id"),
                        product_name: r.get("product_name"),
                        quantity: r.get("quantity"),
                        unit_price: r.try_get("unit_price").unwrap_or(0.0),
                        total_cost: r.try_get("total_cost").unwrap_or(0.0),
                        predicted_depletion_date: dep_date,
                        days_until_empty,
                    });
                }
                (StatusCode::OK, Json(dtos)).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<PendingPoDto>>(vec![])).into_response()
            }
        }
    }
}
