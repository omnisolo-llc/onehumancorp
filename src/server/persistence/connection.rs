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
        Self::connect_with_optional_sqlcipher_key(url, None).await
    }

    pub async fn connect_with_sqlcipher_key(url: &str, key: &str) -> Result<Self, sea_orm::DbErr> {
        Self::connect_with_optional_sqlcipher_key(url, Some(key)).await
    }

    async fn connect_with_optional_sqlcipher_key(
        url: &str,
        sqlcipher_key: Option<&str>,
    ) -> Result<Self, sea_orm::DbErr> {
        let mut options = ConnectOptions::new(url.to_owned());
        options
            .max_connections(20)
            .min_connections(1)
            .sqlx_logging(false);
        if let Some(key) = sqlcipher_key {
            let pragma_key = format!("'{}'", key.replace('\'', "''"));
            options
                .sqlcipher_key(pragma_key)
                .map_sqlx_sqlite_opts(|options| {
                    options
                        .pragma("cipher", "'sqlcipher'")
                        .pragma("cipher_page_size", "4096")
                        .pragma("cipher_compatibility", "4")
                });
        }
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

#[derive(Clone)]
pub struct DatabaseUrl(String);

impl DatabaseUrl {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn expose_for_connection(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for DatabaseUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("DatabaseUrl(REDACTED)")
    }
}
