use crate::db::DB;
use std::sync::Arc;
use tokio::time::{interval, Duration, timeout};
use uuid::Uuid;
use chrono::Utc;

pub struct ChurnPredictionJob;

impl ChurnPredictionJob {
    pub fn start(db: Arc<DB>) {
        let mut interval = interval(Duration::from_secs(3600)); // Run every hour

        tokio::spawn(async move {
            loop {
                interval.tick().await;

                let tenants_query = "SELECT DISTINCT tenant_id FROM engagement_events";
                let tenants: Vec<String> = match &db.store {
                    crate::db::DbStore::Postgres => sqlx::query_scalar(tenants_query).fetch_all(&db.pool).await.unwrap_or_default(),
                    crate::db::DbStore::Sqlite(_) => sqlx::query_scalar(tenants_query).fetch_all(&db.pool).await.unwrap_or_default(),
                };

                for tenant_id in tenants {
                    // Find customers who haven't engaged recently
                    let at_risk_customers_query = "
                        SELECT customer_id, MAX(occurred_at) as last_engaged_at
                        FROM engagement_events
                        WHERE tenant_id = $1
                        GROUP BY customer_id
                        HAVING MAX(occurred_at) < CURRENT_TIMESTAMP - INTERVAL '30 days'";

                    // SQLite specific query logic
                    let sqlite_query = "
                        SELECT customer_id, MAX(occurred_at) as last_engaged_at
                        FROM engagement_events
                        WHERE tenant_id = ?
                        GROUP BY customer_id
                        HAVING MAX(occurred_at) < datetime('now', '-30 days')";

                    #[derive(sqlx::FromRow)]
                    struct AtRiskCustomer {
                        customer_id: String,
                    }

                    let at_risk_customers: Vec<AtRiskCustomer> = match &db.store {
                        crate::db::DbStore::Postgres => sqlx::query_as(at_risk_customers_query)
                            .bind(&tenant_id)
                            .fetch_all(&db.pool).await.unwrap_or_default(),
                        crate::db::DbStore::Sqlite(_) => sqlx::query_as(sqlite_query)
                            .bind(&tenant_id)
                            .fetch_all(&db.pool).await.unwrap_or_default(),
                    };

                    for customer in at_risk_customers {
                        let prediction_id = Uuid::new_v4().to_string();
                        let probability = 0.85; // High churn probability based on 30 day inactivity
                        let primary_factor = "Inactivity > 30 days";

                        // Insert prediction
                        let insert_prediction = "INSERT INTO churn_predictions (id, tenant_id, customer_id, probability, primary_factor) VALUES ($1, $2, $3, $4, $5)";
                        let sqlite_insert_pred = "INSERT INTO churn_predictions (id, tenant_id, customer_id, probability, primary_factor) VALUES (?, ?, ?, ?, ?)";

                        let _ = match &db.store {
                            crate::db::DbStore::Postgres => sqlx::query(insert_prediction)
                                .bind(&prediction_id)
                                .bind(&tenant_id)
                                .bind(&customer.customer_id)
                                .bind(probability)
                                .bind(primary_factor)
                                .execute(&db.pool).await,
                            crate::db::DbStore::Sqlite(_) => sqlx::query(sqlite_insert_pred)
                                .bind(&prediction_id)
                                .bind(&tenant_id)
                                .bind(&customer.customer_id)
                                .bind(probability)
                                .bind(primary_factor)
                                .execute(&db.pool).await,
                        };

                        // Trigger The Silent Ambassador (draft action)
                        let action_id = Uuid::new_v4().to_string();
                        let proposed_message = format!("Hi there, we noticed you haven't been around lately. We'd love to see you again! Here's a special offer to come back.");

                        let insert_action = "INSERT INTO retention_actions (id, tenant_id, prediction_id, status, proposed_message) VALUES ($1, $2, $3, $4, $5)";
                        let sqlite_insert_action = "INSERT INTO retention_actions (id, tenant_id, prediction_id, status, proposed_message) VALUES (?, ?, ?, ?, ?)";

                        let _ = match &db.store {
                            crate::db::DbStore::Postgres => sqlx::query(insert_action)
                                .bind(&action_id)
                                .bind(&tenant_id)
                                .bind(&prediction_id)
                                .bind("Draft")
                                .bind(&proposed_message)
                                .execute(&db.pool).await,
                            crate::db::DbStore::Sqlite(_) => sqlx::query(sqlite_insert_action)
                                .bind(&action_id)
                                .bind(&tenant_id)
                                .bind(&prediction_id)
                                .bind("Draft")
                                .bind(&proposed_message)
                                .execute(&db.pool).await,
                        };

                        // Surface in Triage Feed
                        let triage_item_id = Uuid::new_v4().to_string();
                        let context_message = format!("Retention Opportunity: Customer {} is slipping away. Reason: {}", customer.customer_id, primary_factor);

                        let insert_triage = "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES ($1, $2, $3, $4, $5, $6, $7)";
                        let sqlite_insert_triage = "INSERT INTO triage_items (id, tenant_id, customer_id, source, priority, context, status) VALUES (?, ?, ?, ?, ?, ?, ?)";

                        let _ = match &db.store {
                            crate::db::DbStore::Postgres => sqlx::query(insert_triage)
                                .bind(&triage_item_id)
                                .bind(&tenant_id)
                                .bind(&customer.customer_id)
                                .bind("The Silent Ambassador")
                                .bind("High")
                                .bind(&context_message)
                                .bind("pending")
                                .execute(&db.pool).await,
                            crate::db::DbStore::Sqlite(_) => sqlx::query(sqlite_insert_triage)
                                .bind(&triage_item_id)
                                .bind(&tenant_id)
                                .bind(&customer.customer_id)
                                .bind("The Silent Ambassador")
                                .bind("High")
                                .bind(&context_message)
                                .bind("pending")
                                .execute(&db.pool).await,
                        };

                        // Add triage action for 1-tap approval
                        let insert_triage_action = "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES ($1, $2, $3, $4, $5)";
                        let sqlite_insert_triage_action = "INSERT INTO triage_proposed_actions (id, triage_item_id, tenant_id, action_type, payload) VALUES (?, ?, ?, ?, ?)";

                        let _ = match &db.store {
                            crate::db::DbStore::Postgres => sqlx::query(insert_triage_action)
                                .bind(Uuid::new_v4().to_string())
                                .bind(&triage_item_id)
                                .bind(&tenant_id)
                                .bind("ApproveRetentionMessage")
                                .bind(&proposed_message)
                                .execute(&db.pool).await,
                            crate::db::DbStore::Sqlite(_) => sqlx::query(sqlite_insert_triage_action)
                                .bind(Uuid::new_v4().to_string())
                                .bind(&triage_item_id)
                                .bind(&tenant_id)
                                .bind("ApproveRetentionMessage")
                                .bind(&proposed_message)
                                .execute(&db.pool).await,
                        };

                        // Update risk score
                        let _ = sqlx::query("UPDATE customer_identities SET risk_score = $1 WHERE id = $2")
                                .bind(probability)
                                .bind(&customer.customer_id)
                                .execute(&db.pool).await;
                    }
                }
            }
        });
    }
}
