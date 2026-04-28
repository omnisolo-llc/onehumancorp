use sqlx::{SqlitePool, Row};
use tokio::time::Duration;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use opentelemetry::{global, metrics::Counter};

#[derive(Clone)]
pub struct AutoDreamSyncWorker {
    pub pool: Option<SqlitePool>,
    pub sync_completed_count: Counter<u64>,
    pub sync_failed_count: Counter<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoDreamPayload {
    pub id: String,
    pub payload: String,
    pub updated_at: String,
    pub r#type: String, // "agent_mission" or "embedding_cache"
}

impl AutoDreamSyncWorker {
        pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let meter = global::meter("ohc_telemetry");
        let sync_completed_count = meter.u64_counter("sync_completed_count").with_description("Number of completed AutoDream sync items").init();
        let sync_failed_count = meter.u64_counter("sync_failed_count").with_description("Number of failed AutoDream sync items").init();

        let db_url = std::env::var("DATABASE_URL").unwrap_or_default();
        let pool = if db_url.starts_with("sqlite") {
            let sqlite_pool = sqlx::SqlitePool::connect(&db_url).await?;

            // Run SQLite fallback migrations dynamically from source tree
            // Since this runs in local desktop mode, the source tree is typically available.
            // If we want it bulletproof, we could embed them, but for now we read dir.
            // Better: parse and run them here, away from the global PgPool app setup.
            let mut entries = std::fs::read_dir("src/server/migrations")?
                .filter_map(|res| res.ok())
                .collect::<Vec<_>>();

            entries.sort_by_key(|e| e.path());

            sqlx::query("CREATE TABLE IF NOT EXISTS _sqlx_migrations (version BIGINT PRIMARY KEY, description TEXT NOT NULL, installed_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP, success BOOLEAN NOT NULL, checksum TEXT NOT NULL, execution_time BIGINT NOT NULL);")
                .execute(&sqlite_pool)
                .await?;

            for entry in entries {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("sql") {
                    let filename = path.file_name().unwrap().to_str().unwrap();
                    let version_str = filename.split('_').next().unwrap_or("0");
                    let version: i64 = version_str.parse().unwrap_or(0);

                    let check: (i64,) = sqlx::query_as("SELECT count(*) FROM _sqlx_migrations WHERE version = $1")
                        .bind(version)
                        .fetch_one(&sqlite_pool)
                        .await
                        .unwrap_or((0,));

                    if check.0 == 0 {
                        let mut sql = std::fs::read_to_string(&path)?;
                        // Parse PG-specific syntax
                        if sql.contains("DO $$") {
                            sql = sql.replace("DO $$", "");
                            sql = sql.replace("BEGIN", "");
                            sql = sql.replace("END $$;", "");
                            let re_if_not_exists = regex::Regex::new(r"(?is)IF NOT EXISTS.*?THEN(.*?)(?:END IF;|ELSE)").unwrap();
                            sql = re_if_not_exists.replace_all(&sql, "$1").to_string();
                        }

                        let re = regex::Regex::new(r"(?i)VECTOR\(\d+\)").unwrap();
                        sql = re.replace_all(&sql, "BLOB").to_string();
                        let re2 = regex::Regex::new(r"(?i)vector_cosine_ops").unwrap();
                        sql = re2.replace_all(&sql, "").to_string();
                        if sql.contains("USING hnsw") {
                            let lines: Vec<&str> = sql.lines().filter(|l| !l.contains("USING hnsw")).collect();
                            sql = lines.join("\n");
                        }

                        let statements: Vec<&str> = sql.split(';').filter(|s| !s.trim().is_empty()).collect();

                        for statement in statements {
                            let stmt = statement.trim();
                            if stmt.is_empty() { continue; }

                            match sqlx::query(stmt).execute(&sqlite_pool).await {
                                Ok(_) => {},
                                Err(e) => {
                                    if stmt.contains("ADD COLUMN") && e.to_string().contains("duplicate column name") {
                                        // Ignore duplicate column name
                                    } else {
                                        println!("Failed to run statement {}: {}", stmt, e);
                                        // On desktop we might want to continue or stop
                                        // for safety, don't hard crash the app
                                    }
                                }
                            }
                        }

                        let _ = sqlx::query("INSERT INTO _sqlx_migrations (version, description, success, checksum, execution_time) VALUES ($1, $2, true, '00', 0)")
                            .bind(version)
                            .bind(filename)
                            .execute(&sqlite_pool)
                            .await;
                    }
                }
            }

            Some(sqlite_pool)
        } else {
            None
        };

        Ok(AutoDreamSyncWorker {
            pool,
            sync_completed_count,
            sync_failed_count,
        })
    }

    pub fn get_pool(&self) -> Option<&SqlitePool> {
        self.pool.as_ref()
    }

    pub fn start(&self) {
        if self.pool.is_none() {
            println!("AutoDreamSync worker disabled: not running on SQLite");
            return;
        }

        println!("Starting AutoDreamSync worker");
        let worker = self.clone();

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(60));
            loop {
                ticker.tick().await;
                if let Err(e) = worker.ProcessForecastTick().await {
                    println!("AutoDreamSync worker error: {}", e);
                }
            }
        });
    }

    #[allow(non_snake_case)]
    pub async fn ProcessForecastTick(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pool = match &self.pool {
            Some(p) => p,
            None => return Ok(()),
        };

        println!("AutoDreamSync: Processing forecast tick");

        let mut payloads = Vec::new();

        let missions_res = sqlx::query("SELECT id, payload, updated_at FROM agent_missions WHERE synced_to_cloud = false LIMIT 100")
            .fetch_all(pool)
            .await;

        match missions_res {
            Ok(rows) => {
                for row in rows {
                    let id: String = row.get("id");
                    let payload: String = row.get("payload");
                    let updated_at: String = match row.try_get::<chrono::DateTime<Utc>, _>("updated_at") {
                        Ok(dt) => dt.to_rfc3339(),
                        Err(_) => Utc::now().to_rfc3339(),
                    };
                    payloads.push(AutoDreamPayload {
                        id,
                        payload,
                        updated_at,
                        r#type: "agent_mission".to_string(),
                    });
                }
            }
            Err(e) => {
                println!("AutoDreamSync: error fetching missions: {}", e);
                return Err(Box::new(e));
            }
        }

        let embeddings_res = sqlx::query("SELECT cache_key as id, response_json as payload, created_at as updated_at FROM embedding_cache WHERE synced_to_cloud = false LIMIT 100")
            .fetch_all(pool)
            .await;

        match embeddings_res {
            Ok(rows) => {
                for row in rows {
                    let id: String = row.get("id");
                    let payload: String = row.get("payload");
                    let updated_at: String = match row.try_get::<chrono::DateTime<Utc>, _>("updated_at") {
                        Ok(dt) => dt.to_rfc3339(),
                        Err(_) => Utc::now().to_rfc3339(),
                    };
                    payloads.push(AutoDreamPayload {
                        id,
                        payload,
                        updated_at,
                        r#type: "embedding_cache".to_string(),
                    });
                }
            }
            Err(e) => {
                println!("AutoDreamSync: error fetching embeddings: {}", e);
                return Err(Box::new(e));
            }
        }

        if payloads.is_empty() {
            return Ok(());
        }

        println!("AutoDreamSync: Found {} items to sync", payloads.len());

        let cloud_url = std::env::var("OHC_CLOUD_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
        let sync_url = format!("{}/api/v1/sync/autodream", cloud_url);

        let client = reqwest::Client::new();
        match client.post(&sync_url).json(&payloads).send().await {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("AutoDreamSync: Successfully synced items");
                    self.sync_completed_count.add(payloads.len() as u64, &[]);

                    for p in payloads {
                        if p.r#type == "agent_mission" {
                            let _ = sqlx::query("UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1")
                                .bind(&p.id)
                                .execute(pool)
                                .await;
                        } else if p.r#type == "embedding_cache" {
                            let _ = sqlx::query("UPDATE embedding_cache SET synced_to_cloud = true WHERE cache_key = $1")
                                .bind(&p.id)
                                .execute(pool)
                                .await;
                        }
                    }
                } else {
                    println!("AutoDreamSync: Failed to sync, status: {}", resp.status());
                    self.sync_failed_count.add(payloads.len() as u64, &[]);
                }
            }
            Err(e) => {
                println!("AutoDreamSync: Failed to sync, error: {}", e);
                self.sync_failed_count.add(payloads.len() as u64, &[]);
            }
        }

        Ok(())
    }
}
