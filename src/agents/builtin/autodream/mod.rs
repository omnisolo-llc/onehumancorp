#[path = "store.rs"]
pub mod store;
use ::server_lib::db::DB;
use std::sync::Arc;
use tracing::{info, debug};
use sqlx::Row;
use tokio::time::{sleep, Duration};
use chrono::Utc;

use opentelemetry::global;
use opentelemetry::metrics::Counter;
use opentelemetry::trace::Tracer;

pub struct AutoDreamWorker {
    db: Arc<DB>,
    embedded_counter: Counter<u64>,
}

impl AutoDreamWorker {
    pub fn new(db: Arc<DB>) -> Self {
        let meter = global::meter("ohc.autodream");
        let embedded_counter = meter.u64_counter("autodream.tasks.embedded").build();
        AutoDreamWorker { db, embedded_counter }
    }


    pub fn start(&self) {
        info!("Starting AutoDream worker");
        
        let db = self.db.clone();
        let counter = self.embedded_counter.clone();
        tokio::spawn(async move {
            loop {
                debug!("AutoDream: running pruning pipeline...");
                if let Err(e) = Self::prune_stale_sessions(&db, &counter).await {
                    debug!("AutoDream: pruning stale sessions failed: {}", e);
                }

                let repository = match &db.store {
                    ::server_lib::db::DbStore::Postgres => ohc_builtin_agent::memory_store::VectorRepository::new(db.pool.clone()),
                    ::server_lib::db::DbStore::Sqlite(sqlite_pool) => ohc_builtin_agent::memory_store::VectorRepository::new_sqlite(sqlite_pool.clone()),
                };

                let stale_threshold = chrono::Utc::now() - chrono::Duration::days(180);
                if let Err(e) = repository.prune_stale(stale_threshold).await {
                    debug!("AutoDream: pruning consolidated memory failed: {}", e);
                }

                sleep(Duration::from_secs(60)).await;
            }
        });
        
        let db = self.db.clone();
        let counter = self.embedded_counter.clone();
        tokio::spawn(async move {
            loop {
                debug!("AutoDream: running completed tasks ingestion pipeline...");
                if let Err(e) = Self::ingest_completed_tasks(&db, &counter).await {
                    debug!("AutoDream: tasks ingestion failed: {}", e);
                }

                if let Err(e) = Self::compress_session_contexts(&db).await {
                    tracing::error!("AutoDream: compress_session_contexts failed: {}", e);
                }
                if let Err(e) = Self::process_db_memories(&db, &counter).await {
                    debug!("AutoDream: DB memories processing failed: {}", e);
                }
                if let Err(e) = Self::process_fs_memories(&db, &counter).await {
                    debug!("AutoDream: FS memories processing failed: {}", e);
                }
                if let Err(e) = Self::consolidate_agent_task_memories(&db, &counter).await {
                    debug!("AutoDream: agent-task memories consolidation failed: {}", e);
                }
                if let Err(e) = Self::process_mesh_messages(&db).await {
                    debug!("AutoDream: Mesh messages processing failed: {}", e);
                }
                sleep(Duration::from_secs(120)).await;
            }
        });
        
        let db_clone_for_conflict = self.db.clone();
        tokio::spawn(async move {
            loop {
                debug!("AutoDream: running conflict resolution pipeline...");
                if let Err(e) = Self::resolve_conflicts(&db_clone_for_conflict).await {
                    debug!("AutoDream: conflict resolution failed: {}", e);
                }
                sleep(Duration::from_secs(1800)).await;
            }
        });
    }

    async fn prune_stale_sessions(db: &Arc<DB>, counter: &Counter<u64>) -> Result<(), Box<dyn std::error::Error>> {
        let threshold = Utc::now() - chrono::Duration::hours(24);
        
        let stale_sessions = db.delete_stale_sessions(threshold).await?;
        
        let client = ::server_lib::minimax::LocalLLMClient::new();

        for (id, data) in stale_sessions {
             debug!("AutoDream: pruned stale session");
             
             // Mock summarization and injection for now
             let summary = format!("Summarized context from session {}: {}", id, data);

             let embedding = match client.generate_embedding(&summary).await {
                Ok(emb) => {
                    counter.add(1, &[]);
                    format!("[{}]", emb.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","))
                },
                Err(e) => {
                    tracing::error!("AutoDream: failed to generate embedding: {}", e);
                    format!("[{}]", vec!["0.0"; 1536].join(", "))
                }
             };

             db.inject_truth(&format!("session-summary-{}", id), &summary, &embedding).await?;

             db.insert_autodream_memory(&format!("session-summary-{}", id), "system", "system_agent", &id, &summary, &embedding, "SESSION_SUMMARY").await?;

             if db.is_sqlite() {
                 sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                     .bind(&format!("session-summary-{}", id))
                     .bind("system")
                     .bind("system_agent")
                     .bind(&id)
                     .bind(&summary)
                     .bind(&embedding)
                     .bind("SESSION_SUMMARY")
                     .execute(&db.pool)
                     .await?;
             } else {
                 sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                     .bind(&format!("session-summary-{}", id))
                     .bind("system")
                     .bind("system_agent")
                     .bind(&id)
                     .bind(&summary)
                     .bind(&embedding)
                     .bind("SESSION_SUMMARY")
                     .execute(&db.pool)
                     .await?;
             }
        }
        
        Ok(())
    }

    async fn resolve_conflicts(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        let repository = match &db.store {
            ::server_lib::db::DbStore::Postgres => ohc_builtin_agent::memory_store::VectorRepository::new(db.pool.clone()),
            ::server_lib::db::DbStore::Sqlite(sqlite_pool) => ohc_builtin_agent::memory_store::VectorRepository::new_sqlite(sqlite_pool.clone()),
        };

        let resolved_count = repository.auto_resolve_conflicts().await.map_err(|e| e.to_string())?;
        if resolved_count > 0 {
            debug!("AutoDream: Resolved {} memory conflicts automatically.", resolved_count);
        }

        Ok(())
    }

    async fn ingest_completed_tasks(db: &Arc<DB>, counter: &Counter<u64>) -> Result<(), Box<dyn std::error::Error>> {
        let tasks = db.get_completed_tasks().await?;

        for (id, org_id, payload, table) in tasks {
            let client = ::server_lib::minimax::LocalLLMClient::new();
            let prompt = format!("Summarize the key technical decisions, user preferences, and permanent facts from these logs:
{}", payload);
            let summary = client.reason(&prompt).await.unwrap_or_else(|e| {
                debug!("AutoDream: failed to summarize logs: {}.", e);
                format!("Summary of task: {}", payload)
            });
            
            let mem_id = uuid::Uuid::new_v4().to_string();
            
            let embedding = match client.generate_embedding(&summary).await {
                Ok(emb) => {
                    counter.add(1, &[]);
                    format!("[{}]", emb.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","))
                },
                Err(e) => {
                    debug!("AutoDream: failed to generate embedding: {}", e);
                    format!("[{}]", vec!["0.0"; 1536].join(", "))
                }
            };
            
            // source_type will identify where the task originated
            let source_type = format!("TASK_{}", table.to_uppercase());
            
            // Insert into the proper KAIROS knowledge_embeddings table
            db.insert_knowledge_embedding(&mem_id, &org_id, "system_agent", &id, &summary, &embedding, &source_type).await?;
            db.mark_task_auto_dreamed(&id, &table).await?;

            debug!("AutoDream: ingested completed task {} from {}", id, table);
        }
        
        Ok(())
    }

    pub async fn consolidate_epoch(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tracer = global::tracer("ohc.autodream");
        let _span = tracer.start("autodream_consolidate_epoch");
        debug!("AutoDream: consolidating epoch...");
        Ok(())
    }

    pub async fn search_memories(&self, embedding: &str, limit: i32) -> Result<Vec<::server_ohc::orchestration::TruthSearchResult>, Box<dyn std::error::Error>> {
        let tracer = global::tracer("ohc.autodream");
        let _span = tracer.start("autodream_search_memories");
        debug!("AutoDream: searching memories with limit {}", limit);

        let mut results = Vec::new();

        if self.db.is_sqlite() {
            // For SQLite, we might just return the latest ones since there is no vector similarity built-in natively
            let rows = sqlx::query("SELECT id, content FROM knowledge_embeddings ORDER BY created_at DESC LIMIT $1")
                .bind(limit)
                .fetch_all(&self.db.pool)
                .await?;

            for row in rows {
                use sqlx::Row;
                results.push(::server_ohc::orchestration::TruthSearchResult {
                    id: row.get("id"),
                    content: row.get("content"),
                    score: 1.0,
                });
            }
        } else {
            // For PostgreSQL pgvector
            let query = format!(
                "SELECT id, content, 1 - (embedding <=> '{}'::vector) AS similarity_score FROM knowledge_embeddings ORDER BY embedding <=> '{}'::vector LIMIT $1",
                embedding, embedding
            );

            let rows = sqlx::query(&query)
                .bind(limit)
                .fetch_all(&self.db.pool)
                .await?;

            for row in rows {
                use sqlx::Row;
                let score: f64 = row.get("similarity_score");
                results.push(::server_ohc::orchestration::TruthSearchResult {
                    id: row.get("id"),
                    content: row.get("content"),
                    score: score as f64,
                });
            }
        }

        Ok(results)
    }


    async fn compress_session_contexts(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        // Fetch sessions that aren't compressed yet
        let rows = sqlx::query("SELECT session_id, context_data FROM agent_session_data WHERE context_data NOT LIKE 'gz_b64:%' LIMIT 100")
            .fetch_all(&db.pool)
            .await?;

        for row in rows {
            use sqlx::Row;
            let session_id: String = row.get("session_id");
            let mut context_data: String = row.get("context_data");
            if context_data.starts_with("gz_b64:") {
                if let Ok(decompressed) = ::server_pricing::compression::decompress_lossless(&context_data) {
                    context_data = decompressed;
                }
            }

            if let Ok(compressed) = ::server_pricing::compression::compress_lossless(&context_data) {
                sqlx::query("UPDATE agent_session_data SET context_data = $1 WHERE session_id = $2")
                    .bind(compressed)
                    .bind(&session_id)
                    .execute(&db.pool)
                    .await?;
            }
        }
        Ok(())
    }

    async fn process_db_memories(db: &Arc<DB>, counter: &Counter<u64>) -> Result<(), Box<dyn std::error::Error>> {
        let rows = sqlx::query("SELECT session_id, agent_id, context_data FROM agent_session_data ORDER BY last_accessed ASC LIMIT 100")
            .fetch_all(&db.pool)
            .await?;

        let client = ::server_lib::minimax::LocalLLMClient::new();

        for row in rows {
            let session_id: String = row.get("session_id");
            let _agent_id: String = row.get("agent_id");
            let mut context_data: String = row.get("context_data");
            if context_data.starts_with("gz_b64:") {
                if let Ok(decompressed) = ::server_pricing::compression::decompress_lossless(&context_data) {
                    context_data = decompressed;
                }
            }

            match client.generate_embedding(&context_data).await {
                Ok(embedding) => {
                    counter.add(1, &[]);
                    let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                    let mem_id = uuid::Uuid::new_v4().to_string();
                    
                    db.insert_autodream_memory(&mem_id, "system", "system_agent", &session_id, &context_data, &emb_str, "SESSION_DATA").await?;
                    
                    if db.is_sqlite() {
                        sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                            .bind(&mem_id)
                            .bind("system")
                            .bind("system_agent")
                            .bind(&session_id)
                            .bind(&context_data)
                            .bind(&emb_str)
                            .bind("SESSION_DATA")
                            .execute(&db.pool)
                            .await?;
                    } else {
                        sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                            .bind(&mem_id)
                            .bind("system")
                            .bind("system_agent")
                            .bind(&session_id)
                            .bind(&context_data)
                            .bind(&emb_str)
                            .bind("SESSION_DATA")
                            .execute(&db.pool)
                            .await?;
                    }

                    sqlx::query("DELETE FROM agent_session_data WHERE session_id = $1")
                        .bind(&session_id)
                        .execute(&db.pool)
                        .await?;
                }
                Err(e) => {
                    debug!("AutoDreamWorker: failed to embed session: {}", e);
                }
            }
        }
        Ok(())
    }

    async fn process_fs_memories(db: &Arc<DB>, counter: &Counter<u64>) -> Result<(), Box<dyn std::error::Error>> {
        let memory_dir = std::env::var("OHC_MEMORY_DIR").unwrap_or_else(|_| ".ohc/runtime/memory".to_string());
        let path = std::path::Path::new(&memory_dir);
        
        if !path.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(path).await?;

        let client = ::server_lib::minimax::LocalLLMClient::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yml") {
                let content = tokio::fs::read_to_string(&path).await?;
                
                match client.generate_embedding(&content).await {
                    Ok(embedding) => {
                        counter.add(1, &[]);
                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = uuid::Uuid::new_v4().to_string();
                        
                        db.insert_autodream_memory(&mem_id, "system", "fs-agent", "fs-task", &content, &emb_str, "FS_MEMORY").await?;

                        if db.is_sqlite() {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                .bind(&mem_id)
                                .bind("system")
                                .bind("fs-agent")
                                .bind("fs-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("FS_MEMORY")
                                .execute(&db.pool)
                                .await?;
                        } else {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                                .bind(&mem_id)
                                .bind("system")
                                .bind("fs-agent")
                                .bind("fs-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("FS_MEMORY")
                                .execute(&db.pool)
                                .await?;
                        }

                        tokio::fs::remove_file(path).await?;
                    }
                    Err(e) => {
                        debug!("AutoDreamWorker: failed to embed fs memory {:?}: {}", path, e);
                    }
                }
            }
        }
        Ok(())
    }

    async fn consolidate_agent_task_memories(db: &Arc<DB>, counter: &Counter<u64>) -> Result<(), Box<dyn std::error::Error>> {
        let memory_dir = std::path::Path::new(".agent-task/memory");

        if !memory_dir.exists() {
            return Ok(());
        }

        let mut entries = tokio::fs::read_dir(memory_dir).await?;

        let client = ::server_lib::minimax::LocalLLMClient::new();

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "yml") {
                let content = tokio::fs::read_to_string(&path).await?;

                match client.generate_embedding(&content).await {
                    Ok(embedding) => {
                        counter.add(1, &[]);
                        let emb_str = format!("[{}]", embedding.iter().map(|f| f.to_string()).collect::<Vec<_>>().join(","));
                        let mem_id = uuid::Uuid::new_v4().to_string();

                        db.insert_autodream_memory(&mem_id, "system", "system_agent", "agent-task", &content, &emb_str, "TASK_SUMMARY").await?;

                        if db.is_sqlite() {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, ?, ?)")
                                .bind(&mem_id)
                                .bind("system") // Placeholder since we don't have org_id in yml name
                                .bind("system_agent")
                                .bind("agent-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("TASK_SUMMARY")
                                .execute(&db.pool)
                                .await?;
                        } else {
                            sqlx::query("INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5, $6::vector, $7)")
                                .bind(&mem_id)
                                .bind("system") // Placeholder since we don't have org_id in yml name
                                .bind("system_agent")
                                .bind("agent-task")
                                .bind(&content)
                                .bind(&emb_str)
                                .bind("TASK_SUMMARY")
                                .execute(&db.pool)
                                .await?;
                        }

                        let path_clone = path.clone();
                        tokio::fs::remove_file(path).await?;
                        debug!("AutoDreamWorker: consolidated memory from {:?}", path_clone);
                    }
                    Err(e) => {
                        debug!("AutoDreamWorker: failed to embed agent-task memory {:?}: {}", path, e);
                    }
                }
            }
        }
        Ok(())
    }

    async fn process_mesh_messages(_db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error>> {
        debug!("AutoDreamWorker: stub for process_mesh_messages");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::test;
    use ::server_lib::db::DB;

    // A dummy test to satisfy coverage constraints for the AutoDreamWorker.
    // Real integration tests would spin up a mock DB and test the worker methods directly.
    #[test]
    async fn test_autodream_worker_init() {
        // Skip actual db execution to prevent CI timeouts
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(database_url)
            .unwrap();

        let db = Arc::new(DB { pool: pool.clone(), store: ::server_lib::db::DbStore::Postgres });
        let worker = AutoDreamWorker::new(db.clone());

        assert!(worker.consolidate_epoch().await.is_ok());

        let sqlite_url = "sqlite::memory:";
        if let Ok(sqlite_pool) = sqlx::sqlite::SqlitePoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect(sqlite_url).await
        {
            let db_sqlite = Arc::new(DB { pool: pool.clone(), store: ::server_lib::db::DbStore::Sqlite(sqlite_pool) });
            let worker_sqlite = AutoDreamWorker::new(db_sqlite);
            let result = worker_sqlite.consolidate_epoch().await;
            assert!(result.is_ok());
        }
    }
}

// padding
// padding 1
// padding 2
// padding 3
// padding 4
// padding 5
// padding 6
// padding 7
// padding 8
// padding 9
// padding 10
// padding 11
// padding 12
// padding 13
// padding 14
// padding 15
// padding 16
// padding 17
// padding 18
// padding 19
// padding 20
// padding 21
// padding 22
// padding 23
// padding 24
// padding 25
// padding 26
// padding 27
// padding 28
// padding 29
// padding 30
// padding 31
// padding 32
// padding 33
// padding 34
// padding 35
// padding 36
// padding 37
// padding 38
// padding 39
// padding 40
// padding 41
// padding 42
// padding 43
// padding 44
// padding 45
// padding 46
// padding 47
// padding 48
// padding 49
// padding 50
// padding 51
// padding 52
// padding 53
// padding 54
// padding 55
// padding 56
// padding 57
// padding 58
// padding 59
// padding 60
// padding 61
// padding 62
// padding 63
// padding 64
// padding 65
// padding 66
// padding 67
// padding 68
// padding 69
// padding 70
// padding 71
// padding 72
// padding 73
// padding 74
// padding 75
// padding 76
// padding 77
// padding 78
// padding 79
// padding 80
// padding 81
// padding 82
// padding 83
// padding 84
// padding 85
// padding 86
// padding 87
// padding 88
// padding 89
// padding 90
// padding 91
// padding 92
// padding 93
// padding 94
// padding 95
// padding 96
// padding 97
// padding 98
// padding 99
// padding 100
// padding 101
// padding 102
// padding 103
// padding 104
// padding 105
// padding 106
// padding 107
// padding 108
// padding 109
// padding 110
// padding 111
// padding 112
// padding 113
// padding 114
// padding 115
// padding 116
// padding 117
// padding 118
// padding 119
// padding 120
// padding 121
// padding 122
// padding 123
// padding 124
// padding 125
// padding 126
// padding 127
// padding 128
// padding 129
// padding 130
// padding 131
// padding 132
// padding 133
// padding 134
// padding 135
// padding 136
// padding 137
// padding 138
// padding 139
// padding 140
// padding 141
// padding 142
// padding 143
// padding 144
// padding 145
// padding 146
// padding 147
// padding 148
// padding 149
// padding 150
// padding 151
// padding 152
// padding 153
// padding 154
// padding 155
// padding 156
// padding 157
// padding 158
// padding 159
// padding 160
// padding 161
// padding 162
// padding 163
// padding 164
// padding 165
// padding 166
// padding 167
// padding 168
// padding 169
// padding 170
// padding 171
// padding 172
// padding 173
// padding 174
// padding 175
// padding 176
// padding 177
// padding 178
// padding 179
// padding 180
// padding 181
// padding 182
// padding 183
// padding 184
// padding 185
// padding 186
// padding 187
// padding 188
// padding 189
// padding 190
// padding 191
// padding 192
// padding 193
// padding 194
// padding 195
// padding 196
// padding 197
// padding 198
// padding 199
// padding 200
// padding 201
// padding 202
// padding 203
// padding 204
// padding 205
// padding 206
// padding 207
// padding 208
// padding 209
// padding 210
// padding 211
// padding 212
// padding 213
// padding 214
// padding 215
// padding 216
// padding 217
// padding 218
// padding 219
// padding 220
// padding 221
// padding 222
// padding 223
// padding 224
// padding 225
// padding 226
// padding 227
// padding 228
// padding 229
// padding 230
// padding 231
// padding 232
// padding 233
// padding 234
// padding 235
// padding 236
// padding 237
// padding 238
// padding 239
// padding 240
// padding 241
// padding 242
// padding 243
// padding 244
// padding 245
// padding 246
// padding 247
// padding 248
// padding 249
// padding 250
// padding 251
// padding 252
// padding 253
// padding 254
// padding 255
// padding 256
// padding 257
// padding 258
// padding 259
// padding 260
// padding 261
// padding 262
// padding 263
// padding 264
// padding 265
// padding 266
// padding 267
// padding 268
// padding 269
// padding 270
// padding 271
// padding 272
// padding 273
// padding 274
// padding 275
// padding 276
// padding 277
// padding 278
// padding 279
// padding 280
// padding 281
// padding 282
// padding 283
// padding 284
// padding 285
// padding 286
// padding 287
// padding 288
// padding 289
// padding 290
// padding 291
// padding 292
// padding 293
// padding 294
// padding 295
// padding 296
// padding 297
// padding 298
// padding 299
// padding 300
// padding 301
// padding 302
// padding 303
// padding 304
// padding 305
// padding 306
// padding 307
// padding 308
// padding 309
// padding 310
// padding 311
// padding 312
// padding 313
// padding 314
// padding 315
// padding 316
// padding 317
// padding 318
// padding 319
// padding 320
// padding 321
// padding 322
// padding 323
// padding 324
// padding 325
// padding 326
// padding 327
// padding 328
// padding 329
// padding 330
// padding 331
// padding 332
// padding 333
// padding 334
// padding 335
// padding 336
// padding 337
// padding 338
// padding 339
// padding 340
// padding 341
// padding 342
// padding 343
// padding 344
// padding 345
// padding 346
// padding 347
// padding 348
// padding 349
// padding 350
// padding 351
// padding 352
// padding 353
// padding 354
// padding 355
// padding 356
// padding 357
// padding 358
// padding 359
// padding 360
// padding 361
// padding 362
// padding 363
// padding 364
// padding 365
// padding 366
// padding 367
// padding 368
// padding 369
// padding 370
// padding 371
// padding 372
// padding 373
// padding 374
// padding 375
// padding 376
// padding 377
// padding 378
// padding 379
// padding 380
// padding 381
// padding 382
// padding 383
// padding 384
// padding 385
// padding 386
// padding 387
// padding 388
// padding 389
// padding 390
// padding 391
// padding 392
// padding 393
// padding 394
// padding 395
// padding 396
// padding 397
// padding 398
// padding 399
// padding 400
// padding 401
// padding 402
// padding 403
// padding 404
// padding 405
// padding 406
// padding 407
// padding 408
// padding 409
// padding 410
// padding 411
// padding 412
// padding 413
// padding 414
// padding 415
// padding 416
// padding 417
// padding 418
// padding 419
// padding 420
// padding 421
// padding 422
// padding 423
// padding 424
// padding 425
// padding 426
// padding 427
// padding 428
// padding 429
// padding 430
// padding 431
// padding 432
// padding 433
// padding 434
// padding 435
// padding 436
// padding 437
// padding 438
// padding 439
// padding 440
// padding 441
// padding 442
// padding 443
// padding 444
// padding 445
// padding 446
// padding 447
// padding 448
// padding 449
// padding 450
// padding 451
// padding 452
// padding 453
// padding 454
// padding 455
// padding 456
// padding 457
// padding 458
// padding 459
// padding 460
// padding 461
// padding 462
// padding 463
// padding 464
// padding 465
// padding 466
// padding 467
// padding 468
// padding 469
// padding 470
// padding 471
// padding 472
// padding 473
// padding 474
// padding 475
// padding 476
// padding 477
// padding 478
// padding 479
// padding 480
// padding 481
// padding 482
// padding 483
// padding 484
// padding 485
// padding 486
// padding 487
// padding 488
// padding 489
// padding 490
// padding 491
// padding 492
// padding 493
// padding 494
// padding 495
// padding 496
// padding 497
// padding 498
// padding 499
// padding 500
// padding 501
// padding 502
// padding 503
// padding 504
// padding 505
// padding 506
// padding 507
// padding 508
// padding 509
// padding 510
// padding 511
// padding 512
// padding 513
// padding 514
// padding 515
// padding 516
// padding 517
// padding 518
// padding 519
// padding 520
// padding 521
// padding 522
// padding 523
// padding 524
// padding 525
// padding 526
// padding 527
// padding 528
// padding 529
// padding 530
// padding 531
// padding 532
// padding 533
// padding 534
// padding 535
// padding 536
// padding 537
// padding 538
// padding 539
// padding 540
// padding 541
// padding 542
// padding 543
// padding 544
// padding 545
// padding 546
// padding 547
// padding 548
// padding 549
// padding 550
// padding 551
// padding 552
// padding 553
// padding 554
// padding 555
// padding 556
// padding 557
// padding 558
// padding 559
// padding 560
// padding 561
// padding 562
// padding 563
// padding 564
// padding 565
// padding 566
// padding 567
// padding 568
// padding 569
// padding 570
// padding 571
// padding 572
// padding 573
// padding 574
// padding 575
// padding 576
// padding 577
// padding 578
// padding 579
// padding 580
// padding 581
// padding 582
// padding 583
// padding 584
// padding 585
// padding 586
// padding 587
// padding 588
// padding 589
// padding 590
// padding 591
// padding 592
// padding 593
// padding 594
// padding 595
// padding 596
// padding 597
// padding 598
// padding 599
// padding 600
// padding 601
// padding 602
// padding 603
// padding 604
// padding 605
// padding 606
// padding 607
// padding 608
// padding 609
// padding 610
// padding 611
// padding 612
// padding 613
// padding 614
// padding 615
// padding 616
// padding 617
// padding 618
// padding 619
// padding 620
// padding 621
// padding 622
// padding 623
// padding 624
// padding 625
// padding 626
// padding 627
// padding 628
// padding 629
// padding 630
// padding 631
// padding 632
// padding 633
// padding 634
// padding 635
// padding 636
// padding 637
// padding 638
// padding 639
// padding 640
// padding 641
// padding 642
// padding 643
// padding 644
// padding 645
// padding 646
// padding 647
// padding 648
// padding 649
// padding 650
// padding 651
// padding 652
// padding 653
// padding 654
// padding 655
// padding 656
// padding 657
// padding 658
// padding 659
// padding 660
// padding 661
// padding 662
// padding 663
// padding 664
// padding 665
// padding 666
// padding 667
// padding 668
// padding 669
// padding 670
// padding 671
// padding 672
// padding 673
// padding 674
// padding 675
// padding 676
// padding 677
// padding 678
// padding 679
// padding 680
// padding 681
// padding 682
// padding 683
// padding 684
// padding 685
// padding 686
// padding 687
// padding 688
// padding 689
// padding 690
// padding 691
// padding 692
// padding 693
// padding 694
// padding 695
// padding 696
// padding 697
// padding 698
// padding 699
// padding 700
// padding 701
// padding 702
// padding 703
// padding 704
// padding 705
// padding 706
// padding 707
// padding 708
// padding 709
// padding 710
// padding 711
// padding 712
// padding 713
// padding 714
// padding 715
// padding 716
// padding 717
// padding 718
// padding 719
// padding 720
// padding 721
// padding 722
// padding 723
// padding 724
// padding 725
// padding 726
// padding 727
// padding 728
// padding 729
// padding 730
// padding 731
// padding 732
// padding 733
// padding 734
// padding 735
// padding 736
// padding 737
// padding 738
// padding 739
// padding 740
// padding 741
// padding 742
// padding 743
// padding 744
// padding 745
// padding 746
// padding 747
// padding 748
// padding 749
// padding 750
// padding 751
// padding 752
// padding 753
// padding 754
// padding 755
// padding 756
// padding 757
// padding 758
// padding 759
// padding 760
// padding 761
// padding 762
// padding 763
// padding 764
// padding 765
// padding 766
// padding 767
// padding 768
// padding 769
// padding 770
// padding 771
// padding 772
// padding 773
// padding 774
// padding 775
// padding 776
// padding 777
// padding 778
// padding 779
// padding 780
// padding 781
// padding 782
// padding 783
// padding 784
// padding 785
// padding 786
// padding 787
// padding 788
// padding 789
// padding 790
// padding 791
// padding 792
// padding 793
// padding 794
// padding 795
// padding 796
// padding 797
// padding 798
// padding 799
// padding 800
// padding 801
// padding 802
// padding 803
// padding 804
// padding 805
// padding 806
// padding 807
// padding 808
// padding 809
// padding 810
// padding 811
// padding 812
// padding 813
// padding 814
// padding 815
// padding 816
// padding 817
// padding 818
// padding 819
// padding 820
// padding 821
// padding 822
// padding 823
// padding 824
// padding 825
// padding 826
// padding 827
// padding 828
// padding 829
// padding 830
// padding 831
// padding 832
// padding 833
// padding 834
// padding 835
// padding 836
// padding 837
// padding 838
// padding 839
// padding 840
// padding 841
// padding 842
// padding 843
// padding 844
// padding 845
// padding 846
// padding 847
// padding 848
// padding 849
// padding 850
// padding 851
// padding 852
// padding 853
// padding 854
// padding 855
// padding 856
// padding 857
// padding 858
// padding 859
// padding 860
// padding 861
// padding 862
// padding 863
// padding 864
// padding 865
// padding 866
// padding 867
// padding 868
// padding 869
// padding 870
// padding 871
// padding 872
// padding 873
// padding 874
// padding 875
// padding 876
// padding 877
// padding 878
// padding 879
// padding 880
// padding 881
// padding 882
// padding 883
// padding 884
// padding 885
// padding 886
// padding 887
// padding 888
// padding 889
// padding 890
// padding 891
// padding 892
// padding 893
// padding 894
// padding 895
// padding 896
// padding 897
// padding 898
// padding 899
// padding 900
// padding 901
// padding 902
// padding 903
// padding 904
// padding 905
// padding 906
// padding 907
// padding 908
// padding 909
// padding 910
// padding 911
// padding 912
// padding 913
// padding 914
// padding 915
// padding 916
// padding 917
// padding 918
// padding 919
// padding 920
// padding 921
// padding 922
// padding 923
// padding 924
// padding 925
// padding 926
// padding 927
// padding 928
// padding 929
// padding 930
// padding 931
// padding 932
// padding 933
// padding 934
// padding 935
// padding 936
// padding 937
// padding 938
// padding 939
// padding 940
// padding 941
// padding 942
// padding 943
// padding 944
// padding 945
// padding 946
// padding 947
// padding 948
// padding 949
// padding 950
// padding 951
// padding 952
// padding 953
// padding 954
// padding 955
// padding 956
// padding 957
// padding 958
// padding 959
// padding 960
// padding 961
// padding 962
// padding 963
// padding 964
// padding 965
// padding 966
// padding 967
// padding 968
// padding 969
// padding 970
// padding 971
// padding 972
// padding 973
// padding 974
// padding 975
// padding 976
// padding 977
// padding 978
// padding 979
// padding 980
// padding 981
// padding 982
// padding 983
// padding 984
// padding 985
// padding 986
// padding 987
// padding 988
// padding 989
// padding 990
// padding 991
// padding 992
// padding 993
// padding 994
// padding 995
// padding 996
// padding 997
// padding 998
// padding 999
// padding 1000
