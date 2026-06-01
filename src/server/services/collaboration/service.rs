use tonic::{Request, Response, Status};
use uuid::Uuid;
use ::server_ohc::collaboration::*;
use ::server_ohc::collaboration::collaboration_service_server::CollaborationService;

use sqlx::{PgPool, Executor, Row};
use std::sync::Arc;

pub struct MyCollaborationService {
    db_pool: PgPool,
}

impl MyCollaborationService {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl CollaborationService for MyCollaborationService {
    async fn create_bundle(
        &self,
        request: Request<BundleProduct>,
    ) -> Result<Response<BundleProduct>, Status> {
        let mut bundle = request.into_inner();
        bundle.id = Uuid::new_v4().to_string();

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO bundle_products (id, name, price_cents) VALUES ($1, $2, $3)"
        )
        .bind(&bundle.id)
        .bind(&bundle.name)
        .bind(bundle.price_cents)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        for item in &bundle.items {
            sqlx::query(
                "INSERT INTO bundle_items (bundle_id, product_id, tenant_id, split_cents) VALUES ($1, $2, $3, $4)"
            )
            .bind(&bundle.id)
            .bind(&item.product_id)
            .bind(&item.tenant_id)
            .bind(item.split_cents)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(bundle))
    }

    async fn get_bundle(
        &self,
        request: Request<GetBundleRequest>,
    ) -> Result<Response<BundleProduct>, Status> {
        let req = request.into_inner();

        let bundle_row = sqlx::query(
            "SELECT id, name, price_cents FROM bundle_products WHERE id = $1"
        )
        .bind(&req.bundle_id)
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = bundle_row {
            let mut bundle = BundleProduct {
                id: row.get("id"),
                name: row.get("name"),
                price_cents: row.get("price_cents"),
                items: vec![],
            };

            let item_rows = sqlx::query(
                "SELECT product_id, tenant_id, split_cents FROM bundle_items WHERE bundle_id = $1"
            )
            .bind(&req.bundle_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            for item_row in item_rows {
                bundle.items.push(BundleItem {
                    product_id: item_row.get("product_id"),
                    tenant_id: item_row.get("tenant_id"),
                    split_cents: item_row.get("split_cents"),
                });
            }

            Ok(Response::new(bundle))
        } else {
            Err(Status::not_found("Bundle not found"))
        }
    }

    async fn create_cart(
        &self,
        request: Request<UnifiedCart>,
    ) -> Result<Response<UnifiedCart>, Status> {
        let mut cart = request.into_inner();
        cart.id = Uuid::new_v4().to_string();
        cart.status = "PENDING".to_string();

        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO unified_carts (id, customer_id, total_cents, status) VALUES ($1, $2, $3, $4)"
        )
        .bind(&cart.id)
        .bind(&cart.customer_id)
        .bind(cart.total_cents)
        .bind(&cart.status)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        for item in &cart.items {
            let item_id = if item.id.is_empty() { Uuid::new_v4().to_string() } else { item.id.clone() };
            sqlx::query(
                "INSERT INTO cart_items (id, cart_id, bundle_id, quantity) VALUES ($1, $2, $3, $4)"
            )
            .bind(&item_id)
            .bind(&cart.id)
            .bind(&item.bundle_id)
            .bind(item.quantity)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(cart))
    }

    async fn add_to_cart(
        &self,
        request: Request<AddToCartRequest>,
    ) -> Result<Response<UnifiedCart>, Status> {
        let req = request.into_inner();
        let cart_id = req.cart_id;

        if let Some(item) = req.item {
            let item_id = if item.id.is_empty() { Uuid::new_v4().to_string() } else { item.id.clone() };
            sqlx::query(
                "INSERT INTO cart_items (id, cart_id, bundle_id, quantity) VALUES ($1, $2, $3, $4)"
            )
            .bind(&item_id)
            .bind(&cart_id)
            .bind(&item.bundle_id)
            .bind(item.quantity)
            .execute(&self.db_pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        }

        let cart_row = sqlx::query(
            "SELECT id, customer_id, total_cents, status FROM unified_carts WHERE id = $1"
        )
        .bind(&cart_id)
        .fetch_optional(&self.db_pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = cart_row {
            let mut cart = UnifiedCart {
                id: row.get("id"),
                customer_id: row.get("customer_id"),
                total_cents: row.get("total_cents"),
                status: row.get("status"),
                items: vec![],
            };

            let item_rows = sqlx::query(
                "SELECT id, bundle_id, quantity FROM cart_items WHERE cart_id = $1"
            )
            .bind(&cart_id)
            .fetch_all(&self.db_pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            for item_row in item_rows {
                cart.items.push(CartItem {
                    id: item_row.get("id"),
                    bundle_id: item_row.get("bundle_id"),
                    quantity: item_row.get("quantity"),
                });
            }

            Ok(Response::new(cart))
        } else {
            Err(Status::not_found("Cart not found"))
        }
    }

    async fn checkout(
        &self,
        request: Request<CheckoutRequest>,
    ) -> Result<Response<CheckoutResponse>, Status> {
        let req = request.into_inner();

        // Distributed Saga pattern for checking out
        let mut tx = self.db_pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // 1. Mark cart as CHECKOUT_INITIATED
        sqlx::query("UPDATE unified_carts SET status = 'CHECKOUT_INITIATED' WHERE id = $1")
            .bind(&req.cart_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Fetch cart items
        let item_rows = sqlx::query("SELECT bundle_id, quantity FROM cart_items WHERE cart_id = $1")
            .bind(&req.cart_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // 2. Reserve inventory (Saga: Lock rows)
        // Here we simulate the reserve by looping through bundle items and taking lock
        for item_row in &item_rows {
            let bundle_id: String = item_row.get("bundle_id");
            let _quantity: i32 = item_row.get("quantity");

            let bundle_items = sqlx::query("SELECT product_id, tenant_id FROM bundle_items WHERE bundle_id = $1")
                .bind(&bundle_id)
                .fetch_all(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

            for bundle_item in bundle_items {
                let product_id: String = bundle_item.get("product_id");
                let tenant_id: String = bundle_item.get("tenant_id");

                // Assuming an inventory table exists, we use FOR UPDATE
                let reserve_result = sqlx::query(
                    "UPDATE inventory SET quantity = quantity - $1 WHERE product_id = $2 AND tenant_id = $3 AND quantity >= $1 RETURNING id"
                )
                .bind(_quantity)
                .bind(&product_id)
                .bind(&tenant_id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

                if reserve_result.is_none() {
                    // Saga: compensation / rollback happens automatically if we don't commit tx
                    return Ok(Response::new(CheckoutResponse {
                        success: false,
                        transaction_id: "".to_string(),
                        error_message: format!("Failed to reserve inventory for product {} (Tenant {})", product_id, tenant_id),
                    }));
                }
            }
        }

        // 3. Mark cart as RESERVED
        sqlx::query("UPDATE unified_carts SET status = 'RESERVED' WHERE id = $1")
            .bind(&req.cart_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // 4. Simulate Payment Split & Processing via Stripe Destination Charges
        // (In real implementation, call Stripe API here)

        // 5. Mark cart as PAID / COMPLETED
        sqlx::query("UPDATE unified_carts SET status = 'COMPLETED' WHERE id = $1")
            .bind(&req.cart_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(CheckoutResponse {
            success: true,
            transaction_id: Uuid::new_v4().to_string(),
            error_message: "".to_string(),
        }))
    }
}
