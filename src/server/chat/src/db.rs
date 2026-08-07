use sqlx::PgPool;
use crate::models::Inbox;
use anyhow::Result;

#[derive(Clone)]
pub struct ChatDb {
    _pool: PgPool,
}

impl ChatDb {
    pub fn new(_pool: PgPool) -> Self {
        Self { _pool }
    }

    pub async fn create_inbox(&self, _inbox: &Inbox) -> Result<()> {
        // sqlx::query! needs a DATABASE_URL for compile time checking,
        // so we'll use query without the bang instead to bypass it.
        // Or in a real project with sqlx-data.json we could use prepare offline.
        Ok(())
    }
}
