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
        if sqlcipher_key.is_some_and(|key| key.trim().is_empty() || !url.starts_with("sqlite:")) {
            return Err(sea_orm::DbErr::Custom(
                "SQLCipher requires a SQLite database and a nonempty configured key".into(),
            ));
        }
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
        if sqlcipher_key.is_some()
            && let Err(error) =
                require_sqlite_encryption(connection.get_sqlite_connection_pool()).await
        {
            let _ = connection.close().await;
            return Err(error);
        }
        Ok(Self::from_connection(connection))
    }

    /// Adopt the configured connection, preserving its pool, hooks and SQLite identity.
    pub fn from_connection(connection: DatabaseConnection) -> Self {
        let backend = match connection.get_database_backend() {
            sea_orm::DatabaseBackend::MySql => DatabaseBackend::MySql,
            sea_orm::DatabaseBackend::Postgres => DatabaseBackend::Postgres,
            sea_orm::DatabaseBackend::Sqlite => DatabaseBackend::Sqlite,
        };
        Self {
            connection,
            backend,
        }
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

/// Unknown SQLite pragmas are silently ignored. A configured key therefore is
/// not proof that encryption exists or that an existing database key is valid.
pub(crate) async fn require_sqlite_encryption(
    pool: &sqlx::SqlitePool,
) -> Result<(), sea_orm::DbErr> {
    let version: Option<String> = sqlx::query_scalar("PRAGMA cipher_version")
        .fetch_optional(pool)
        .await
        .map_err(|error| sea_orm::DbErr::Conn(sea_orm::RuntimeErr::SqlxError(error)))?;
    if version
        .as_deref()
        .is_none_or(|value| value.trim().is_empty())
    {
        return Err(sea_orm::DbErr::Custom(
            "Requested SQLite encryption requires a SQLCipher-enabled database engine".into(),
        ));
    }
    // Reading the actual schema forces key verification for an existing file;
    // reporting the cipher library version alone does not establish decryption.
    sqlx::query("SELECT count(*) FROM sqlite_schema")
        .execute(pool)
        .await
        .map_err(|error| sea_orm::DbErr::Conn(sea_orm::RuntimeErr::SqlxError(error)))?;
    Ok(())
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
