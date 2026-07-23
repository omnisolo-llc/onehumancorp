use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    Pending,
    InProgress,
    Completed,
    Failed,
    Compensating,
    Compensated,
}

pub type StepAction = Box<
    dyn Fn(
            i64,
            serde_json::Value,
        ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send>>
        + Send
        + Sync,
>;

pub struct Step {
    pub name: String,
    pub action: StepAction,
    pub compensate: StepAction,
}

pub struct Saga {
    pub name: String,
    pub steps: Vec<Step>,
}

pub struct Coordinator {
    pub pool: PgPool,
    pub registry: HashMap<String, Saga>,
}

impl Coordinator {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            registry: HashMap::new(),
        }
    }

    pub fn register(&mut self, saga: Saga) {
        self.registry.insert(saga.name.clone(), saga);
    }

    pub async fn start(
        &self,
        saga_name: &str,
        initial_data: serde_json::Value,
    ) -> Result<i64, String> {
        let saga = self
            .registry
            .get(saga_name)
            .ok_or(format!("Saga {} not found", saga_name))?;

        let row: (i64,) = sqlx::query_as(
            r#"
            INSERT INTO saga_instances (name, state, data)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
        )
        .bind(saga_name)
        .bind("IN_PROGRESS")
        .bind(&initial_data)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let saga_id = row.0;

        self.execute_saga(saga_id, saga_name, initial_data).await;

        Ok(saga_id)
    }

    pub async fn execute_saga(&self, saga_id: i64, saga_name: &str, data: serde_json::Value) {
        let saga = match self.registry.get(saga_name) {
            Some(s) => s,
            None => return,
        };

        let mut failed = false;
        let mut step_index = 0;

        for (i, step) in saga.steps.iter().enumerate() {
            step_index = i;

            let _ = sqlx::query(
                r#"
                INSERT INTO saga_steps (saga_id, step_name, state)
                VALUES ($1, $2, $3)
                "#,
            )
            .bind(saga_id)
            .bind(&step.name)
            .bind("IN_PROGRESS")
            .execute(&self.pool)
            .await;

            if let Err(e) = (step.action)(saga_id, data.clone()).await {
                println!("Step {} failed: {}", step.name, e);
                let _ = sqlx::query(
                    r#"
                    UPDATE saga_steps SET state = $1 WHERE saga_id = $2 AND step_name = $3
                    "#,
                )
                .bind("FAILED")
                .bind(saga_id)
                .bind(&step.name)
                .execute(&self.pool)
                .await;
                failed = true;
                break;
            }

            let _ = sqlx::query(
                r#"
                UPDATE saga_steps SET state = $1 WHERE saga_id = $2 AND step_name = $3
                "#,
            )
            .bind("COMPLETED")
            .bind(saga_id)
            .bind(&step.name)
            .execute(&self.pool)
            .await;
        }

        if failed {
            let _ = sqlx::query(
                r#"
                UPDATE saga_instances SET state = $1 WHERE id = $2
                "#,
            )
            .bind("COMPENSATING")
            .bind(saga_id)
            .execute(&self.pool)
            .await;

            self.compensate_saga(saga_id, saga, data, step_index).await;
        } else {
            let _ = sqlx::query(
                r#"
                UPDATE saga_instances SET state = $1 WHERE id = $2
                "#,
            )
            .bind("COMPLETED")
            .bind(saga_id)
            .execute(&self.pool)
            .await;
        }
    }

    pub async fn compensate_saga(
        &self,
        saga_id: i64,
        saga: &Saga,
        data: serde_json::Value,
        failed_step_index: usize,
    ) {
        if failed_step_index == 0 {
            let _ = sqlx::query(
                r#"
                UPDATE saga_instances SET state = $1 WHERE id = $2
                "#,
            )
            .bind("COMPENSATED")
            .bind(saga_id)
            .execute(&self.pool)
            .await;
            return;
        }

        for i in (0..failed_step_index).rev() {
            let step = &saga.steps[i];

            let _ = sqlx::query(
                r#"
                UPDATE saga_steps SET state = $1 WHERE saga_id = $2 AND step_name = $3
                "#,
            )
            .bind("COMPENSATING")
            .bind(saga_id)
            .bind(&step.name)
            .execute(&self.pool)
            .await;

            if let Err(e) = (step.compensate)(saga_id, data.clone()).await {
                println!("Compensation for step {} failed: {}", step.name, e);
                let _ = sqlx::query(
                    r#"
                    UPDATE saga_steps SET state = $1 WHERE saga_id = $2 AND step_name = $3
                    "#,
                )
                .bind("FAILED")
                .bind(saga_id)
                .bind(&step.name)
                .execute(&self.pool)
                .await;
            } else {
                let _ = sqlx::query(
                    r#"
                    UPDATE saga_steps SET state = $1 WHERE saga_id = $2 AND step_name = $3
                    "#,
                )
                .bind("COMPENSATED")
                .bind(saga_id)
                .bind(&step.name)
                .execute(&self.pool)
                .await;
            }
        }
        let _ = sqlx::query(
            r#"
            UPDATE saga_instances SET state = $1 WHERE id = $2
            "#,
        )
        .bind("COMPENSATED")
        .bind(saga_id)
        .execute(&self.pool)
        .await;
    }
}
#[cfg(test)]
pub mod tests;
