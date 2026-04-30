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
            let mut i = 1;
            while result.contains(&format!("${}", i)) {
                result = result.replace(&format!("${}", i), "?");
                i += 1;
            }
            result
        }
    }
}
