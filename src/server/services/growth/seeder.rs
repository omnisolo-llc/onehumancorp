
use sqlx::PgPool;

pub struct GrowthSeeder {
    pool: PgPool,
}

impl GrowthSeeder {
    pub fn new(pool: PgPool) -> Self {
        GrowthSeeder { pool }
    }

    pub async fn seed_base_data(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let organizations = vec![
            ("org_1", "Acme Corp", "Pro"),
            ("org_2", "Globex", "Free"),
            ("org_3", "Initech", "Starter"),
            ("org_4", "Soylent", "Business"),
        ];

        for (id, name, tier) in organizations {
            sqlx::query(
                "INSERT INTO organizations (id, name, plan_tier) VALUES ($1, $2, $3) ON CONFLICT (id) DO NOTHING",
            )
            .bind(id)
            .bind(name)
            .bind(tier)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
