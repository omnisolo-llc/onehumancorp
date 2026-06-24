use sqlx::PgPool;

pub struct FundingEngine {
    _pool: PgPool,
}

impl FundingEngine {
    pub fn new(_pool: PgPool) -> Self {
        Self { _pool }
    }
}
