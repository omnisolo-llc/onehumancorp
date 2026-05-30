use tonic::{Request, Response, Status};
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;
use crate::db::DB;

use ::server_ohc::supply_chain::supply_chain_service_server::SupplyChainService;
use ::server_ohc::supply_chain::{GetLowStockAlertsRequest, GetLowStockAlertsResponse, ApprovePurchaseOrderRequest, ApprovePurchaseOrderResponse, RawMaterial, PurchaseOrder};
use crate::domain::repository::supply_chain_repo::SupplyChainRepo;

pub struct SupplyChainApi {
    pub db: Arc<DB>,
}

#[tonic::async_trait]
impl SupplyChainService for SupplyChainApi {
    async fn get_low_stock_alerts(
        &self,
        request: Request<GetLowStockAlertsRequest>,
    ) -> Result<Response<GetLowStockAlertsResponse>, Status> {
        let req = request.into_inner();
        let repo = SupplyChainRepo::new(self.db.clone());

        match repo.get_low_stock_materials(&req.tenant_id).await {
            Ok(materials) => {
                let proto_materials = materials.into_iter().map(|m| RawMaterial {
                    id: m.id,
                    tenant_id: m.tenant_id,
                    name: m.name,
                    current_quantity: m.current_quantity.unwrap_or(0),
                    reorder_threshold: m.reorder_threshold.unwrap_or(0),
                    created_at_unix: m.created_at.map(|d| d.timestamp()).unwrap_or(0),
                    updated_at_unix: m.updated_at.map(|d| d.timestamp()).unwrap_or(0),
                }).collect();
                Ok(Response::new(GetLowStockAlertsResponse {
                    low_stock_materials: proto_materials,
                }))
            }
            Err(e) => Err(Status::internal(e)),
        }
    }

    async fn approve_purchase_order(
        &self,
        request: Request<ApprovePurchaseOrderRequest>,
    ) -> Result<Response<ApprovePurchaseOrderResponse>, Status> {
        let req = request.into_inner();

        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                let row = sqlx::query(
                    "UPDATE purchase_orders SET status = 'APPROVED', updated_at = $1 WHERE id = $2 AND tenant_id = $3 RETURNING id, tenant_id, vendor_id, status, total_cost, created_at, updated_at"
                )
                .bind(Utc::now())
                .bind(&req.purchase_order_id)
                .bind(&req.tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

                if let Some(r) = row {
                    use sqlx::Row;
                    let po = PurchaseOrder {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        vendor_id: r.get("vendor_id"),
                        status: r.get("status"),
                        total_cost: r.try_get::<f64, _>("total_cost").unwrap_or(0.0),
                        created_at_unix: r.try_get::<chrono::DateTime<Utc>, _>("created_at").map(|d| d.timestamp()).unwrap_or(0),
                        updated_at_unix: r.try_get::<chrono::DateTime<Utc>, _>("updated_at").map(|d| d.timestamp()).unwrap_or(0),
                    };
                    Ok(Response::new(ApprovePurchaseOrderResponse {
                        success: true,
                        purchase_order: Some(po),
                    }))
                } else {
                    Err(Status::not_found("Purchase order not found"))
                }
            },
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
                let row = sqlx::query(
                    "UPDATE purchase_orders SET status = 'APPROVED', updated_at = ? WHERE id = ? AND tenant_id = ? RETURNING id, tenant_id, vendor_id, status, total_cost, created_at, updated_at"
                )
                .bind(Utc::now())
                .bind(&req.purchase_order_id)
                .bind(&req.tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

                tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

                if let Some(r) = row {
                    use sqlx::Row;
                    let po = PurchaseOrder {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        vendor_id: r.get("vendor_id"),
                        status: r.get("status"),
                        total_cost: r.try_get::<f64, _>("total_cost").unwrap_or(0.0),
                        created_at_unix: 0,
                        updated_at_unix: 0,
                    };
                    Ok(Response::new(ApprovePurchaseOrderResponse {
                        success: true,
                        purchase_order: Some(po),
                    }))
                } else {
                    Err(Status::not_found("Purchase order not found"))
                }
            }
        }
    }
}
