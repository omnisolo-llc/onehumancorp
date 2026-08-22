use sqlx::{MySqlPool, Row};

const MYSQL_MIGRATION_LOCK_NAME: &str = "omnisolo_harness_middleware_v218";
pub const HARNESS_MIDDLEWARE_MIGRATION_VERSION: i64 = 218;
pub const HARNESS_MIDDLEWARE_MIGRATION_NAME: &str = "harness_middleware";

const POSTGRES_HARNESS_MIDDLEWARE_SQL: &str =
    include_str!("../migrations/218_harness_middleware.sql");
const MYSQL_HARNESS_MIDDLEWARE_SQL: &str =
    include_str!("migrations/218_harness_middleware_mysql.sql");

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

    let applied: Option<i64> =
        sqlx::query("SELECT version FROM harness_middleware_schema_migrations WHERE version = ?")
            .bind(HARNESS_MIDDLEWARE_MIGRATION_VERSION)
            .fetch_optional(&mut **connection)
            .await?
            .map(|row| row.get("version"));

    if applied == Some(HARNESS_MIDDLEWARE_MIGRATION_VERSION) {
        return Ok(());
    }

    for statement in split_sql_statements(SqlDialect::MySql.harness_middleware_schema()) {
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
