use std::fmt;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};

use super::capabilities::{DatabaseBackend, DatabaseCapabilities};

#[derive(Clone)]
pub struct AppDatabase {
    connection: DatabaseConnection,
    backend: DatabaseBackend,
}

impl AppDatabase {
    pub async fn connect(url: &str) -> Result<Self, sea_orm::DbErr> {
        let mut options = ConnectOptions::new(url.to_owned());
        options
            .max_connections(20)
            .min_connections(1)
            .sqlx_logging(false);
        let connection = Database::connect(options).await?;
        let backend = match connection.get_database_backend() {
            sea_orm::DatabaseBackend::MySql => DatabaseBackend::MySql,
            sea_orm::DatabaseBackend::Postgres => DatabaseBackend::Postgres,
            sea_orm::DatabaseBackend::Sqlite => DatabaseBackend::Sqlite,
        };
        Ok(Self {
            connection,
            backend,
        })
    }

    pub fn connection(&self) -> &DatabaseConnection {
        &self.connection
    }

    pub const fn backend(&self) -> DatabaseBackend {
        self.backend
    }

    pub const fn capabilities(&self) -> DatabaseCapabilities {
        DatabaseCapabilities::for_backend(self.backend)
    }
}
