use crate::db::DbStore;
use crate::integrations::quickbooks::provider::QuickBooksProvider;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, error};

pub async fn start_quickbooks_sync_worker(db: Arc<crate::db::DB>, registry: Arc<crate::integrations::registry::IntegrationsRegistry>) {
    let mut interval = interval(Duration::from_secs(60 * 5));

    tokio::spawn(async move {
        loop {
            interval.tick().await;
            info!("Running QuickBooks sync worker sweep");

            let mut qbo_tenants: Vec<(String, String, String, String)> = vec![]; // tenant_id, access_token, refresh_token, company_id

            match &db.store {
                DbStore::Postgres => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, serde_json::Value)>(
                        "SELECT tenant_id, credentials FROM tenant_integrations WHERE integration_id = 'quickbooks' AND status = 'connected'"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            if let (Some(access), Some(refresh), Some(company_id)) = (
                                row.1.get("access_token").and_then(|v| v.as_str()),
                                row.1.get("refresh_token").and_then(|v| v.as_str()),
                                row.1.get("company_id").and_then(|v| v.as_str())
                            ) {
                                qbo_tenants.push((row.0, access.to_string(), refresh.to_string(), company_id.to_string()));
                            }
                        }
                    }
                },
                DbStore::Sqlite(_) => {
                    if let Ok(rows) = sqlx::query_as::<_, (String, String)>(
                        "SELECT tenant_id, credentials FROM tenant_integrations WHERE integration_id = 'quickbooks' AND status = 'connected'"
                    )
                    .fetch_all(&db.pool).await {
                        for row in rows {
                            if let Ok(creds) = serde_json::from_str::<serde_json::Value>(&row.1) {
                                if let (Some(access), Some(refresh), Some(company_id)) = (
                                    creds.get("access_token").and_then(|v| v.as_str()),
                                    creds.get("refresh_token").and_then(|v| v.as_str()),
                                    creds.get("company_id").and_then(|v| v.as_str())
                                ) {
                                    qbo_tenants.push((row.0, access.to_string(), refresh.to_string(), company_id.to_string()));
                                }
                            }
                        }
                    }
                }
            }

            for (tenant_id, access_token, refresh_token, company_id) in qbo_tenants {
                let provider = if let Some(p) = registry.get_quickbooks(&tenant_id) {
                    p
                } else {
                    let p = Arc::new(QuickBooksProvider::new(access_token.clone(), refresh_token.clone()));
                    registry.quickbooks_clients.write().unwrap().insert(tenant_id.clone(), p.clone());
                    p
                };

                let mut invoices_to_sync: Vec<(String, f64, Option<String>)> = vec![];

                match &db.store {
                    DbStore::Postgres => {
                        let query = "SELECT id, total_amount, customer_id FROM invoices WHERE tenant_id = $1 AND payment_status = 'paid' AND (quickbooks_synced IS NULL OR quickbooks_synced = FALSE) LIMIT 10";
                        if let Ok(rows) = sqlx::query_as::<_, (String, f64, Option<String>)>(query)
                            .bind(&tenant_id)
                            .fetch_all(&db.pool)
                            .await {
                                for row in rows {
                                    invoices_to_sync.push((row.0, row.1, row.2));
                                }
                            }
                    },
                    DbStore::Sqlite(sqlite_pool) => {
                        let query = "SELECT id, total_amount, customer_id FROM invoices WHERE tenant_id = ? AND payment_status = 'paid' AND (quickbooks_synced IS NULL OR quickbooks_synced = FALSE) LIMIT 10";
                        if let Ok(rows) = sqlx::query_as::<_, (String, f64, Option<String>)>(query)
                            .bind(&tenant_id)
                            .fetch_all(sqlite_pool)
                            .await {
                                for row in rows {
                                    invoices_to_sync.push((row.0, row.1, row.2));
                                }
                            }
                    }
                }

                for (invoice_id, amount, customer_id) in invoices_to_sync {
                    let cust_id = match customer_id {
                        Some(id) => id,
                        None => {
                            error!("Cannot sync invoice {} because it has no customer_id", invoice_id);
                            match &db.store {
                                DbStore::Postgres => {
                                    let _ = sqlx::query("UPDATE invoices SET quickbooks_synced = TRUE WHERE id = $1 AND tenant_id = $2").bind(&invoice_id).bind(&tenant_id).execute(&db.pool).await;
                                },
                                DbStore::Sqlite(sqlite_pool) => {
                                    let _ = sqlx::query("UPDATE invoices SET quickbooks_synced = TRUE WHERE id = ? AND tenant_id = ?").bind(&invoice_id).bind(&tenant_id).execute(sqlite_pool).await;
                                }
                            };
                            continue;
                        }
                    };

                    let qbo_invoice = crate::integrations::quickbooks::client::QBOInvoice {
                        CustomerRef: crate::integrations::quickbooks::client::QBOCustomerRef {
                            value: cust_id,
                        },
                        Line: vec![crate::integrations::quickbooks::client::QBOLineAmount {
                            Amount: amount,
                            DetailType: "SalesItemLineDetail".to_string(),
                        }],
                    };

                    let mut success = false;
                    let mut transient_error = false;
                    match provider.sync_invoice(&company_id, qbo_invoice).await {
                        Ok(_) => {
                            info!("Successfully synced invoice {} to QuickBooks for tenant {}", invoice_id, tenant_id);
                            success = true;
                        },
                        Err(e) => {
                            error!("Failed to sync invoice {} to QuickBooks: {}", invoice_id, e);
                            if e.contains("401") || e.contains("429") || e.contains("timeout") || e.contains("50") {
                                transient_error = true;
                            } else {
                                // Bubble up the persistent error to the Finance Assistant by inserting an event into the DB
                                let _ = sqlx::query("INSERT INTO agent_memory (tenant_id, content) VALUES ($1, $2)")
                                    .bind(&tenant_id)
                                    .bind(&format!("URGENT FINANCE ALERT: QuickBooks Sync failed for invoice {}. Error: {}. Please investigate.", invoice_id, e))
                                    .execute(&db.pool).await;
                            }
                        }
                    }

                    if success || !transient_error {
                        match &db.store {
                            DbStore::Postgres => {
                                let _ = sqlx::query("UPDATE invoices SET quickbooks_synced = TRUE WHERE id = $1 AND tenant_id = $2").bind(&invoice_id).bind(&tenant_id).execute(&db.pool).await;
                            },
                            DbStore::Sqlite(sqlite_pool) => {
                                let _ = sqlx::query("UPDATE invoices SET quickbooks_synced = TRUE WHERE id = ? AND tenant_id = ?").bind(&invoice_id).bind(&tenant_id).execute(sqlite_pool).await;
                            }
                        };
                    }

                    // If token refreshed, save it to DB
                    let client_read = provider.client.read().await;
                    if client_read.access_token != access_token || client_read.refresh_token != refresh_token {
                        let creds = serde_json::json!({
                            "access_token": client_read.access_token,
                            "refresh_token": client_read.refresh_token,
                            "company_id": company_id
                        });

                        match &db.store {
                            DbStore::Postgres => {
                                let _ = sqlx::query("UPDATE tenant_integrations SET credentials = $1 WHERE tenant_id = $2 AND integration_id = 'quickbooks'").bind(&creds).bind(&tenant_id).execute(&db.pool).await;
                            },
                            DbStore::Sqlite(sqlite_pool) => {
                                let _ = sqlx::query("UPDATE tenant_integrations SET credentials = ? WHERE tenant_id = ? AND integration_id = 'quickbooks'").bind(&creds.to_string()).bind(&tenant_id).execute(sqlite_pool).await;
                            }
                        };
                    }

                }
            }
        }
    });
}
