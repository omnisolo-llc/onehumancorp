use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::fs;
use std::path::Path;

pub struct LocalSqliteWrapper {
    pub pool: SqlitePool,
}

impl LocalSqliteWrapper {
    pub async fn new(db_path: &str, encryption_key: &str) -> Result<Self, sqlx::Error> {
        // Ensure the SQLite file is created if it does not exist with secure permissions
        if !Path::new(db_path).exists() {
            if let Some(parent) = Path::new(db_path).parent() {
                fs::create_dir_all(parent).map_err(|e| sqlx::Error::Io(e))?;
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .mode(0o600)
                    .open(db_path)
                    .map_err(|e| sqlx::Error::Io(e))?;
            }
            #[cfg(not(unix))]
            {
                fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(db_path)
                    .map_err(|e| sqlx::Error::Io(e))?;
            }
        }

        // Initialize SqliteConnectOptions with encryption pragma
        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true)
            .pragma("key", encryption_key.to_string());

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await?;

        Ok(LocalSqliteWrapper { pool })
    }
}
