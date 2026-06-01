pub enum DatabaseKind {
    Postgres,
    Sqlite,
}

pub fn get_database_kind(url: &str) -> DatabaseKind {
    if url.starts_with("postgres") {
        DatabaseKind::Postgres
    } else {
        DatabaseKind::Sqlite
    }
}

pub fn dialect_query(query: &str, kind: DatabaseKind) -> String {
    match kind {
        DatabaseKind::Postgres => query.to_string(),
        DatabaseKind::Sqlite => {
            let mut result = query.to_string();
            // Start from a high number and iterate downwards to avoid $1 matching $10
            let mut i = 99;
            while i >= 1 {
                let target = format!("${}", i);
                if result.contains(&target) {
                    result = result.replace(&target, "?");
                }
                i -= 1;
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_database_kind() {
        assert!(matches!(
            get_database_kind("postgres://user:pass@localhost/db"),
            DatabaseKind::Postgres
        ));
        assert!(matches!(
            get_database_kind("postgresql://user:pass@localhost/db"),
            DatabaseKind::Postgres
        ));
        assert!(matches!(
            get_database_kind("sqlite://my_db.sqlite"),
            DatabaseKind::Sqlite
        ));
        assert!(matches!(
            get_database_kind("file:my_db.sqlite"),
            DatabaseKind::Sqlite
        ));
    }

    #[test]
    fn test_dialect_query_postgres() {
        let query = "SELECT * FROM users WHERE id = $1 AND name = $2";
        let result = dialect_query(query, DatabaseKind::Postgres);
        assert_eq!(result, "SELECT * FROM users WHERE id = $1 AND name = $2");
    }

    #[test]
    fn test_dialect_query_sqlite() {
        let query = "SELECT * FROM users WHERE id = $1 AND name = $2";
        let result = dialect_query(query, DatabaseKind::Sqlite);
        assert_eq!(result, "SELECT * FROM users WHERE id = ? AND name = ?");
    }

    #[test]
    fn test_dialect_query_sqlite_double_digits() {
        // Ensure $1 doesn't incorrectly replace part of $10
        let query = "SELECT * FROM items WHERE a=$1 AND b=$2 AND c=$3 AND d=$4 AND e=$5 AND f=$6 AND g=$7 AND h=$8 AND i=$9 AND j=$10 AND k=$11";
        let result = dialect_query(query, DatabaseKind::Sqlite);
        assert_eq!(
            result,
            "SELECT * FROM items WHERE a=? AND b=? AND c=? AND d=? AND e=? AND f=? AND g=? AND h=? AND i=? AND j=? AND k=?"
        );
    }
}
