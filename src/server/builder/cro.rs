use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use rand_distr::{Beta, Distribution};
use rand::{SeedableRng, rngs::StdRng};

#[derive(FromRow, Clone, Debug)]
pub struct CroExperiment {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub site_id: Uuid,
    pub title: String,
    pub target_element: String,
    pub status: String,
    pub winning_variant_id: Option<Uuid>,
}

#[derive(FromRow, Clone, Debug)]
pub struct CroVariant {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub experiment_id: Uuid,
    pub variant_name: String,
    pub content: serde_json::Value,
    pub traffic_weight: f64,
    pub views: i32,
    pub conversions: i32,
}

pub struct CroEngine {
    pub pool: PgPool,
}

impl CroEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_experiment(
        &self,
        tenant_id: Uuid,
        site_id: Uuid,
        title: &str,
        target_element: &str,
        variants: Vec<(String, serde_json::Value)>,
    ) -> Result<Uuid, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let exp_id: (Uuid,) = sqlx::query_as(
            "INSERT INTO cro_experiments (tenant_id, site_id, title, target_element) VALUES ($1, $2, $3, $4) RETURNING id"
        )
        .bind(tenant_id)
        .bind(site_id)
        .bind(title)
        .bind(target_element)
        .fetch_one(&mut *tx)
        .await?;

        for (name, content) in variants {
            sqlx::query(
                "INSERT INTO cro_variants (tenant_id, experiment_id, variant_name, content) VALUES ($1, $2, $3, $4)"
            )
            .bind(tenant_id)
            .bind(exp_id.0)
            .bind(name)
            .bind(content)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(exp_id.0)
    }

    pub async fn get_experiments_for_site(
        &self,
        tenant_id: Uuid,
        site_id: Uuid,
    ) -> Result<Vec<CroExperiment>, sqlx::Error> {
        sqlx::query_as::<_, CroExperiment>(
            "SELECT id, tenant_id, site_id, title, target_element, status, winning_variant_id FROM cro_experiments WHERE tenant_id = $1 AND site_id = $2 AND status = 'running'"
        )
        .bind(tenant_id)
        .bind(site_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_variants_for_experiment(
        &self,
        tenant_id: Uuid,
        experiment_id: Uuid,
    ) -> Result<Vec<CroVariant>, sqlx::Error> {
        sqlx::query_as::<_, CroVariant>(
            "SELECT id, tenant_id, experiment_id, variant_name, content, traffic_weight, views, conversions FROM cro_variants WHERE tenant_id = $1 AND experiment_id = $2"
        )
        .bind(tenant_id)
        .bind(experiment_id)
        .fetch_all(&self.pool)
        .await
    }

    pub fn select_variant_thompson(
        &self,
        variants: &[CroVariant],
        user_id: &str,
        experiment_id: Uuid,
    ) -> Option<CroVariant> {
        if variants.is_empty() {
            return None;
        }

        let mut rng = StdRng::from_entropy();

        let num_simulations = 1000;
        let mut wins = vec![0; variants.len()];

        let distributions: Vec<Beta<f64>> = variants.iter().map(|v| {
            let alpha = v.conversions as f64 + 1.0;
            let beta = (v.views - v.conversions) as f64 + 1.0;
            Beta::new(alpha, beta).unwrap()
        }).collect();

        for _ in 0..num_simulations {
            let mut trial_max = -1.0;
            let mut trial_best = 0;
            for (i, dist) in distributions.iter().enumerate() {
                let sample = dist.sample(&mut rng);
                if sample > trial_max {
                    trial_max = sample;
                    trial_best = i;
                }
            }
            wins[trial_best] += 1;
        }

        let probabilities: Vec<f64> = wins.iter().map(|&w| w as f64 / num_simulations as f64).collect();

        let mut hasher = Sha256::new();
        hasher.update(experiment_id.as_bytes());
        hasher.update(user_id.as_bytes());
        let hash = hasher.finalize();

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&hash[..8]);
        let val = u64::from_be_bytes(bytes) as f64 / (u64::MAX as f64);

        let mut cumulative = 0.0;
        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if val <= cumulative {
                return Some(variants[i].clone());
            }
        }

        Some(variants[variants.len() - 1].clone())
    }

    pub async fn record_view(&self, variant_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE cro_variants SET views = views + 1, updated_at = NOW() WHERE id = $1")
            .bind(variant_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn record_conversion(&self, variant_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE cro_variants SET conversions = conversions + 1, updated_at = NOW() WHERE id = $1")
            .bind(variant_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn evaluate_experiments(&self, tenant_id: Uuid) -> Result<(), sqlx::Error> {
        let experiments = sqlx::query_as::<_, CroExperiment>(
            "SELECT id, tenant_id, site_id, title, target_element, status, winning_variant_id FROM cro_experiments WHERE tenant_id = $1 AND status = 'running'"
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await?;

        for exp in experiments {
            let variants = self.get_variants_for_experiment(tenant_id, exp.id).await?;
            if variants.len() < 2 {
                continue;
            }

            let num_simulations = 10000;
            let mut wins = vec![0; variants.len()];
            let mut rng = StdRng::from_entropy();

            let distributions: Vec<Beta<f64>> = variants.iter().map(|v| {
                let alpha = v.conversions as f64 + 1.0;
                let beta = (v.views - v.conversions) as f64 + 1.0;
                Beta::new(alpha, beta).unwrap()
            }).collect();

            let total_views: i32 = variants.iter().map(|v| v.views).sum();

            if total_views < 100 {
                continue;
            }

            for _ in 0..num_simulations {
                let mut trial_max = -1.0;
                let mut trial_best = 0;
                for (i, dist) in distributions.iter().enumerate() {
                    let sample = dist.sample(&mut rng);
                    if sample > trial_max {
                        trial_max = sample;
                        trial_best = i;
                    }
                }
                wins[trial_best] += 1;
            }

            for (i, &win_count) in wins.iter().enumerate() {
                let confidence = win_count as f64 / num_simulations as f64;
                if confidence > 0.95 {
                    let winning_variant_id = variants[i].id;
                    sqlx::query(
                        "UPDATE cro_experiments SET status = 'completed', winning_variant_id = $1, updated_at = NOW() WHERE id = $2"
                    )
                    .bind(winning_variant_id)
                    .bind(exp.id)
                    .execute(&self.pool)
                    .await?;

                    tracing::info!("CRO Engine: Experiment {} completed. Winning variant: {}", exp.id, winning_variant_id);
                    break;
                }
            }
        }
        Ok(())
    }
}
