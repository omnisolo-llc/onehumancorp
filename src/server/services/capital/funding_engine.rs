use sqlx::PgPool;

pub struct FundingEngine {
    pool: PgPool,
}

impl FundingEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
