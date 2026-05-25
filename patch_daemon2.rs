<<<<<<< SEARCH
            let queue_id = Uuid::new_v4().to_string();
            let now = Utc::now().naive_utc();

            // Enqueue task into SubAgentQueue in PostgreSQL
            // Note: Cloud Native Postgres db usage implies this queue handles `FOR UPDATE SKIP LOCKED`
            // when picking tasks, as required by the prompt. We just need to ensure we insert it properly
            // so the worker can pick it up with `FOR UPDATE SKIP LOCKED`. We'll just insert here.
            let mut tx = match self.pg_pool.begin().await {
                Ok(t) => t,
                Err(e) => {
                    warn!("Failed to begin pg transaction: {}, gracefully degrading (cloud unreachable).", e);
                    continue;
                }
            };

            let mission_res = sqlx::query("INSERT INTO agent_missions (id, status, payload, tenant_id) VALUES ($1, 'PENDING', $2, 'system')")
                .bind(&queue_id)
                .bind(payload.to_string())
                .execute(&mut *tx)
                .await;

            if let Err(e) = mission_res {
                warn!("Failed to insert pg agent_missions: {}, gracefully degrading (cloud unreachable).", e);
                let _ = tx.rollback().await;
                continue;
            }

            let res = sqlx::query("INSERT INTO sub_agent_queue (id, tenant_id, parent_task_id, payload, status, scheduled_at, created_at, updated_at) VALUES ($1, 'system', NULL, $2, 'QUEUED', $3, $3, $3)")
                .bind(&queue_id)
                .bind(payload.to_string())
                .bind(now)
                .execute(&mut *tx)
                .await;

            match res {
                Ok(_) => {
                    let commit_res = tx.commit().await;
                    if let Err(e) = commit_res {
                        warn!("Failed to commit pg transaction for memory_id: {}, gracefully degrading. Error: {}", id, e);
                        continue;
                    }

                    // Update SQLite sync status
                    sqlx::query("UPDATE swarm_truth_embeddings SET sync_status = 'SYNCED' WHERE memory_id = ?")
                        .bind(&id)
                        .execute(&self.sqlite_pool)
                        .await?;
                    info!("Successfully escalated memory_id: {} to cloud queue: {}", id, queue_id);
                    success_count += 1;

                    if let Err(e) = ::server_telemetry::record_rag_escalation(&self.pg_pool, "system", "").await {
                        warn!("Failed to record RAG escalation telemetry: {}", e);
                    }
                }
                Err(e) => {
                    let _ = tx.rollback().await;
                    warn!("Failed to escalate memory_id: {}, gracefully degrading (cloud unreachable). Error: {}", id, e);
                }
            }
=======
            let url = format!("{}/api/sync/missions", self.cloud_url);

            let req_payload = json!({
                "missions": [{
                    "memory_id": id,
                    "payload": payload
                }]
            });

            match self.client.post(&url)
                .header("Authorization", "Bearer system") // Or another token strategy as required by `RequireRole("system", ...)`
                .json(&req_payload)
                .send()
                .await
            {
                Ok(resp) if resp.status().is_success() => {
                    // Update SQLite sync status
                    if let Err(e) = sqlx::query("UPDATE swarm_truth_embeddings SET sync_status = 'SYNCED' WHERE memory_id = ?")
                        .bind(&id)
                        .execute(&self.sqlite_pool)
                        .await
                    {
                        warn!("Failed to update SQLite sync status for memory_id: {}. Error: {}", id, e);
                        continue;
                    }

                    info!("Successfully escalated memory_id: {} via cloud gateway", id);
                    success_count += 1;

                    if let Err(e) = ::server_telemetry::record_rag_escalation(&self.pg_pool, "system", "").await {
                        warn!("Failed to record RAG escalation telemetry: {}", e);
                    }
                }
                Ok(resp) => {
                    warn!("Cloud gateway returned error status {} for memory_id: {}, gracefully degrading.", resp.status(), id);
                }
                Err(e) => {
                    warn!("Failed to send HTTP request to escalate memory_id: {}, gracefully degrading. Error: {}", id, e);
                }
            }
>>>>>>> REPLACE
