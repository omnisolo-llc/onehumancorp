use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, sea_query::Expr,
};

use super::{capabilities::DatabaseBackend, connection::AppDatabase, entities};

#[derive(Clone, Debug, PartialEq)]
pub struct CatalogProduct {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub price_cents: i64,
    pub inventory_count: i32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone, Debug)]
pub struct NewProduct {
    pub tenant_id: String,
    pub title: String,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub price_cents: i64,
    pub inventory_count: i32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Clone)]
pub struct CatalogRepository {
    database: AppDatabase,
}

impl CatalogRepository {
    pub const fn new(database: AppDatabase) -> Self {
        Self { database }
    }

    pub const fn backend(&self) -> DatabaseBackend {
        self.database.backend()
    }

    pub async fn list_products(
        &self,
        tenant_id: &str,
    ) -> Result<Vec<CatalogProduct>, sea_orm::DbErr> {
        let rows = entities::product::Entity::find()
            .filter(entities::product::Column::TenantId.eq(tenant_id))
            .order_by_asc(entities::product::Column::Title)
            .all(self.database.connection())
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| CatalogProduct {
                id: row.id,
                title: row.title,
                description: row.description,
                item_type: row.item_type,
                price_cents: row.price_cents.unwrap_or_default(),
                inventory_count: row.inventory_count.unwrap_or_default(),
                metadata: row.metadata,
            })
            .collect())
    }

    pub async fn update_product(
        &self,
        tenant_id: &str,
        id: &str,
        title: &str,
        description: &str,
        price_cents: i64,
    ) -> Result<bool, sea_orm::DbErr> {
        let result = entities::product::Entity::update_many()
            .col_expr(
                entities::product::Column::Title,
                Expr::value(title.to_string()),
            )
            .col_expr(
                entities::product::Column::Description,
                Expr::value(description.to_string()),
            )
            .col_expr(
                entities::product::Column::PriceCents,
                Expr::value(price_cents),
            )
            .col_expr(
                entities::product::Column::UpdatedAt,
                Expr::value(Utc::now()),
            )
            .filter(entities::product::Column::Id.eq(id))
            .filter(entities::product::Column::TenantId.eq(tenant_id))
            .exec(self.database.connection())
            .await?;
        Ok(result.rows_affected == 1)
    }

    pub async fn create_product(
        &self,
        product: NewProduct,
    ) -> Result<CatalogProduct, sea_orm::DbErr> {
        let now = Utc::now();
        let row = entities::product::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            tenant_id: Set(product.tenant_id),
            title: Set(product.title),
            description: Set(product.description),
            item_type: Set(product.item_type),
            price_cents: Set(Some(product.price_cents)),
            inventory_count: Set(Some(product.inventory_count)),
            metadata: Set(product.metadata),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        }
        .insert(self.database.connection())
        .await?;
        Ok(CatalogProduct {
            id: row.id,
            title: row.title,
            description: row.description,
            item_type: row.item_type,
            price_cents: row.price_cents.unwrap_or_default(),
            inventory_count: row.inventory_count.unwrap_or_default(),
            metadata: row.metadata,
        })
    }
}
