use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineMutation {
    pub transaction_id: String,
    pub timestamp: Option<String>,
    pub product_id: String,
    pub quantity_deducted: i32,
    pub amount: Option<i64>,
    pub payment_method: Option<String>,
    pub payment_intent_id: Option<String>,
    pub currency: Option<String>,
    pub mutation_type: Option<String>,
    pub payload: Option<String>,
    pub client_mutation_id: Option<String>,
}

#[derive(Clone)]
pub struct LocalDb {
    pub pool: SqlitePool,
}

impl LocalDb {
    pub async fn init(db_path: &str) -> Result<Self, String> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite://{}?mode=rwc", db_path))
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS offline_mutations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                transaction_id TEXT NOT NULL UNIQUE,
                timestamp TEXT NOT NULL,
                product_id TEXT NOT NULL,
                quantity_deducted INTEGER NOT NULL,
                amount INTEGER,
                payment_method TEXT,
                payment_intent_id TEXT,
                currency TEXT,
                mutation_type TEXT,
                payload TEXT,
                client_mutation_id TEXT
            )"
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Self { pool })
    }

    pub async fn add_mutation(&self, muta: OfflineMutation) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO offline_mutations (transaction_id, timestamp, product_id, quantity_deducted, amount, payment_method, payment_intent_id, currency, mutation_type, payload, client_mutation_id)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&muta.transaction_id)
        .bind(&muta.timestamp.unwrap_or_default())
        .bind(&muta.product_id)
        .bind(muta.quantity_deducted)
        .bind(muta.amount)
        .bind(&muta.payment_method)
        .bind(&muta.payment_intent_id)
        .bind(&muta.currency)
        .bind(&muta.mutation_type)
        .bind(&muta.payload)
        .bind(&muta.client_mutation_id)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn get_pending_mutations(&self) -> Result<Vec<OfflineMutation>, String> {
        // Without query! to avoid sqlx prepare offline issues
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query("SELECT transaction_id, timestamp, product_id, quantity_deducted, amount, payment_method, payment_intent_id, currency, mutation_type, payload, client_mutation_id FROM offline_mutations")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        use sqlx::Row;
        let mutations = rows.into_iter().map(|row| OfflineMutation {
            transaction_id: row.get("transaction_id"),
            timestamp: Some(row.get("timestamp")),
            product_id: row.get("product_id"),
            quantity_deducted: row.get::<i32, _>("quantity_deducted"),
            amount: row.get("amount"),
            payment_method: row.get("payment_method"),
            payment_intent_id: row.get("payment_intent_id"),
            currency: row.get("currency"),
            mutation_type: row.get("mutation_type"),
            payload: row.get("payload"),
            client_mutation_id: row.get("client_mutation_id"),
        }).collect();

        Ok(mutations)
    }

    pub async fn remove_mutations(&self, transaction_ids: &[String]) -> Result<(), String> {
        if transaction_ids.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = transaction_ids.iter().map(|_| "?".to_string()).collect();
        let query_str = format!("DELETE FROM offline_mutations WHERE transaction_id IN ({})", placeholders.join(","));

        let mut query = sqlx::query(&query_str);
        for id in transaction_ids {
            query = query.bind(id);
        }

        query.execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
