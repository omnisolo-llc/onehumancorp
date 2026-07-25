use sqlx::PgPool;
use std::sync::Arc;

pub struct NativeChatService {
    pub pool: Arc<PgPool>,
}

impl NativeChatService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}
