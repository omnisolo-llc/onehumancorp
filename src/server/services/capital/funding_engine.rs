use sqlx::PgPool;

pub struct FundingEngine {
    _pool: sqlx::PgPool,
}

impl FundingEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { _pool: pool }
    }
}
