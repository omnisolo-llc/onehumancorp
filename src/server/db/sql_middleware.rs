use sqlx::{MySqlPool, PgPool, Row};

const LEGACY_HARNESS_MIDDLEWARE_MIGRATION_VERSION: i64 = 218;
const MYSQL_MIGRATION_LOCK_NAME: &str = "omnisolo_harness_middleware_v219";
const POSTGRES_MIGRATION_LOCK_NAME: &str = "omnisolo_harness_middleware_v219";
pub const HARNESS_MIDDLEWARE_MIGRATION_VERSION: i64 = 219;
pub const HARNESS_MIDDLEWARE_MIGRATION_NAME: &str = "harness_middleware";

const POSTGRES_HARNESS_MIDDLEWARE_SQL: &str =
    include_str!("../migrations/218_harness_middleware.sql");
const MYSQL_HARNESS_MIDDLEWARE_SQL: &str =
    include_str!("migrations/218_harness_middleware_mysql.sql");
const POSTGRES_HARNESS_MIDDLEWARE_RECORDS_SQL: &str =
    include_str!("../migrations/219_harness_middleware_records.sql");
const MYSQL_HARNESS_MIDDLEWARE_RECORDS_SQL: &str =
    include_str!("migrations/219_harness_middleware_records_mysql.sql");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SqlDialect {
    Postgres,
    MySql,
}

impl SqlDialect {
    pub fn harness_middleware_schema(self) -> &'static str {
        match self {
            Self::Postgres => POSTGRES_HARNESS_MIDDLEWARE_SQL,
            Self::MySql => MYSQL_HARNESS_MIDDLEWARE_SQL,
        }
    }

    pub fn harness_middleware_record_extension(self) -> &'static str {
        match self {
            Self::Postgres => POSTGRES_HARNESS_MIDDLEWARE_RECORDS_SQL,
            Self::MySql => MYSQL_HARNESS_MIDDLEWARE_RECORDS_SQL,
        }
    }
}

/// Split DDL without breaking semicolons inside quoted values or comments.
/// The MySQL harness migration intentionally contains no stored programs, so
/// this parser does not need to interpret `BEGIN ... END` bodies.
pub fn split_sql_statements(sql: &str) -> Vec<String> {
    let chars: Vec<char> = sql.chars().collect();
    let mut statements = Vec::new();
    let mut statement = String::new();
    let mut index = 0;
    let mut quote = None;
    let mut line_comment = false;
    let mut block_comment = false;

    while index < chars.len() {
        let current = chars[index];
        let next = chars.get(index + 1).copied();

        if line_comment {
            if current == '\n' {
                line_comment = false;
                statement.push(current);
            }
            index += 1;
            continue;
        }

        if block_comment {
            if current == '*' && next == Some('/') {
                block_comment = false;
                index += 2;
            } else {
                index += 1;
            }
            continue;
        }

        if let Some(active_quote) = quote {
            statement.push(current);
            if current == '\\' {
                if let Some(escaped) = next {
                    statement.push(escaped);
                    index += 2;
                    continue;
                }
            } else if current == active_quote {
                if next == Some(active_quote) {
                    statement.push(active_quote);
                    index += 2;
                    continue;
                }
                quote = None;
            }
            index += 1;
            continue;
        }

        if current == '-' && next == Some('-') {
            line_comment = true;
            index += 2;
            continue;
        }
        if current == '#' {
            line_comment = true;
            index += 1;
            continue;
        }
        if current == '/' && next == Some('*') {
            block_comment = true;
            index += 2;
            continue;
        }
        if matches!(current, '\'' | '"' | '`') {
            quote = Some(current);
            statement.push(current);
            index += 1;
            continue;
        }
        if current == ';' {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                statements.push(trimmed.to_owned());
            }
            statement.clear();
        } else {
            statement.push(current);
        }
        index += 1;
    }

    let trimmed = statement.trim();
    if !trimmed.is_empty() {
        statements.push(trimmed.to_owned());
    }
    statements
}

pub async fn run_mysql_harness_middleware_migration(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let mut connection = pool.acquire().await?;
    let lock_acquired: Option<i64> = sqlx::query_scalar("SELECT GET_LOCK(?, 30)")
        .bind(MYSQL_MIGRATION_LOCK_NAME)
        .fetch_one(&mut *connection)
        .await?;

    if lock_acquired != Some(1) {
        return Err(sqlx::Error::Protocol(
            "could not acquire MySQL harness middleware migration lock".to_owned(),
        ));
    }

    let migration_result = run_locked_mysql_migration(&mut connection).await;
    let release_result: Result<Option<i64>, sqlx::Error> =
        sqlx::query_scalar("SELECT RELEASE_LOCK(?)")
            .bind(MYSQL_MIGRATION_LOCK_NAME)
            .fetch_one(&mut *connection)
            .await;

    match (migration_result, release_result) {
        (Err(error), _) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Ok(()), Ok(Some(1))) => Ok(()),
        (Ok(()), Ok(_)) => Err(sqlx::Error::Protocol(
            "MySQL harness middleware migration lock was not released".to_owned(),
        )),
    }
}

pub async fn run_postgres_harness_middleware_migration(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut connection = pool.acquire().await?;
    sqlx::query("SELECT pg_advisory_lock(hashtext($1))")
        .bind(POSTGRES_MIGRATION_LOCK_NAME)
        .execute(&mut *connection)
        .await?;

    let migration_result = run_locked_postgres_migration(&mut connection).await;
    let release_result = sqlx::query("SELECT pg_advisory_unlock(hashtext($1))")
        .bind(POSTGRES_MIGRATION_LOCK_NAME)
        .execute(&mut *connection)
        .await;

    match (migration_result, release_result) {
        (Err(error), _) => Err(error),
        (Ok(()), Err(error)) => Err(error),
        (Ok(()), Ok(_)) => Ok(()),
    }
}

async fn run_locked_postgres_migration(
    connection: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS harness_middleware_schema_migrations (\
            version BIGINT NOT NULL PRIMARY KEY,\
            migration_name TEXT NOT NULL,\
            applied_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP\
        )",
    )
    .execute(&mut **connection)
    .await?;

    let applied_legacy: Option<i64> = sqlx::query_scalar(
        "SELECT version FROM harness_middleware_schema_migrations WHERE version = $1",
    )
    .bind(LEGACY_HARNESS_MIDDLEWARE_MIGRATION_VERSION)
    .fetch_optional(&mut **connection)
    .await?;

    let applied_current: Option<i64> = sqlx::query_scalar(
        "SELECT version FROM harness_middleware_schema_migrations WHERE version = $1",
    )
    .bind(HARNESS_MIDDLEWARE_MIGRATION_VERSION)
    .fetch_optional(&mut **connection)
    .await?;
    if applied_current == Some(HARNESS_MIDDLEWARE_MIGRATION_VERSION) {
        return Ok(());
    }

    if applied_legacy.is_none() {
        sqlx::raw_sql(SqlDialect::Postgres.harness_middleware_schema())
            .execute(&mut **connection)
            .await?;
        sqlx::query(
            "INSERT INTO harness_middleware_schema_migrations (version, migration_name) \
             VALUES ($1, $2)",
        )
        .bind(LEGACY_HARNESS_MIDDLEWARE_MIGRATION_VERSION)
        .bind(HARNESS_MIDDLEWARE_MIGRATION_NAME)
        .execute(&mut **connection)
        .await?;
    }

    sqlx::raw_sql(SqlDialect::Postgres.harness_middleware_record_extension())
        .execute(&mut **connection)
        .await?;
    sqlx::query(
        "INSERT INTO harness_middleware_schema_migrations (version, migration_name) \
         VALUES ($1, $2)",
    )
    .bind(HARNESS_MIDDLEWARE_MIGRATION_VERSION)
    .bind(HARNESS_MIDDLEWARE_MIGRATION_NAME)
    .execute(&mut **connection)
    .await?;
    Ok(())
}

async fn run_locked_mysql_migration(
    connection: &mut sqlx::pool::PoolConnection<sqlx::MySql>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS harness_middleware_schema_migrations (\
            version BIGINT NOT NULL PRIMARY KEY,\
            migration_name VARCHAR(191) NOT NULL,\
            applied_at DATETIME(6) NOT NULL DEFAULT CURRENT_TIMESTAMP(6)\
        ) ENGINE=InnoDB",
    )
    .execute(&mut **connection)
    .await?;

    let applied_legacy: Option<i64> =
        sqlx::query("SELECT version FROM harness_middleware_schema_migrations WHERE version = ?")
            .bind(LEGACY_HARNESS_MIDDLEWARE_MIGRATION_VERSION)
            .fetch_optional(&mut **connection)
            .await?
            .map(|row| row.get("version"));

    let applied_current: Option<i64> =
        sqlx::query("SELECT version FROM harness_middleware_schema_migrations WHERE version = ?")
            .bind(HARNESS_MIDDLEWARE_MIGRATION_VERSION)
            .fetch_optional(&mut **connection)
            .await?
            .map(|row| row.get("version"));

    if applied_current == Some(HARNESS_MIDDLEWARE_MIGRATION_VERSION) {
        return Ok(());
    }

    if applied_legacy.is_none() {
        for statement in split_sql_statements(SqlDialect::MySql.harness_middleware_schema()) {
            sqlx::query(&statement).execute(&mut **connection).await?;
        }
        sqlx::query(
            "INSERT INTO harness_middleware_schema_migrations (version, migration_name) \
             VALUES (?, ?)",
        )
        .bind(LEGACY_HARNESS_MIDDLEWARE_MIGRATION_VERSION)
        .bind(HARNESS_MIDDLEWARE_MIGRATION_NAME)
        .execute(&mut **connection)
        .await?;
    }

    for statement in split_sql_statements(SqlDialect::MySql.harness_middleware_record_extension()) {
        sqlx::query(&statement).execute(&mut **connection).await?;
    }

    sqlx::query(
        "INSERT INTO harness_middleware_schema_migrations (version, migration_name) \
         VALUES (?, ?)",
    )
    .bind(HARNESS_MIDDLEWARE_MIGRATION_VERSION)
    .bind(HARNESS_MIDDLEWARE_MIGRATION_NAME)
    .execute(&mut **connection)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sql_splitter_preserves_quoted_semicolons_and_removes_comments() {
        let statements = split_sql_statements(
            "-- first\nCREATE TABLE example (value VARCHAR(20) DEFAULT 'a;b');\n/* ignored */\nCREATE TABLE `quoted;name` (id INT);",
        );

        assert_eq!(statements.len(), 2);
        assert!(statements[0].contains("'a;b'"));
        assert!(!statements[0].contains("first"));
        assert!(statements[1].contains("`quoted;name`"));
    }

    #[test]
    fn dialect_selects_backend_native_schema() {
        assert!(
            SqlDialect::Postgres
                .harness_middleware_schema()
                .contains("JSONB")
        );
        assert!(
            SqlDialect::MySql
                .harness_middleware_schema()
                .contains("ENGINE=InnoDB")
        );
    }
}
