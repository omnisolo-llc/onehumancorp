use sqlx::{Pool, Postgres};

pub struct ActionRequiredQueueRepo {
    pool: Pool<Postgres>,
}

impl ActionRequiredQueueRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
