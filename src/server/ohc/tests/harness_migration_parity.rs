//! Logical schema parity across the two supported durable-store dialects.
use std::collections::{BTreeMap, BTreeSet};

const POSTGRES: [&str; 2] = [
    include_str!("../../migrations/218_harness_middleware.sql"),
    include_str!("../../migrations/219_harness_middleware_records.sql"),
];
const MYSQL: [&str; 2] = [
    include_str!("../../db/migrations/218_harness_middleware_mysql.sql"),
    include_str!("../../db/migrations/219_harness_middleware_records_mysql.sql"),
];

fn columns(migrations: &[&str]) -> BTreeMap<String, BTreeSet<String>> {
    let mut tables = BTreeMap::<String, BTreeSet<String>>::new();
    let mut current = None;
    for line in migrations.iter().flat_map(|sql| sql.lines()).map(str::trim) {
        if let Some(create) = line.strip_prefix("CREATE TABLE IF NOT EXISTS ") {
            let table = create.split_whitespace().next().unwrap().to_owned();
            assert!(tables.insert(table.clone(), BTreeSet::new()).is_none());
            current = Some(table);
        } else if line.starts_with(')') {
            current = None;
        } else if let Some(table) = current.as_ref() {
            let mut fields = line.split_whitespace();
            let name = fields.next().unwrap_or("").trim_matches('`');
            let kind = fields.next().unwrap_or("");
            if !name.is_empty()
                && name.bytes().all(|c| c.is_ascii_lowercase() || c == b'_')
                && kind.bytes().next().is_some_and(|c| c.is_ascii_uppercase())
            {
                assert!(tables.get_mut(table).unwrap().insert(name.to_owned()));
            }
        }
    }
    tables
}

#[test]
fn postgres_and_mysql_preserve_every_canonical_table_and_column() {
    let postgres = columns(&POSTGRES);
    let mysql = columns(&MYSQL);
    assert_eq!(postgres.len(), 56);
    assert_eq!(
        postgres, mysql,
        "durable logical schemas must stay portable"
    );
    for (table, columns) in postgres {
        if !columns.contains("tenant_id") {
            let parent = match table.as_str() {
                "harness_event_parents" | "harness_event_branch_heads" => "session_id",
                "harness_capsule_loss_entries" => "capsule_id",
                _ => panic!("{table} lacks a tenant fence"),
            };
            assert!(columns.contains(parent));
            assert!(
                POSTGRES
                    .join("\n")
                    .contains(&format!("ALTER TABLE {table} FORCE ROW LEVEL SECURITY"))
            );
            let mysql = MYSQL.join("\n");
            let definition = mysql
                .split_once(&format!("CREATE TABLE IF NOT EXISTS {table} ("))
                .unwrap()
                .1
                .split_once(") ENGINE=InnoDB;")
                .unwrap()
                .0;
            assert!(
                definition.contains(&format!("FOREIGN KEY ({parent}")),
                "{table} lacks its parent fence"
            );
        }
        assert!(
            !columns
                .iter()
                .any(|name| name == "api_key" || name == "password" || name == "facade_token"),
            "{table} stores live authority"
        );
    }
}

#[test]
fn migrations_retain_dialect_specific_tenant_and_ordering_constraints() {
    let postgres = POSTGRES.join("\n");
    let mysql = MYSQL.join("\n");
    assert!(postgres.contains("ENABLE ROW LEVEL SECURITY"));
    assert!(postgres.contains("harness_assert_same_tenant"));
    assert!(mysql.contains("FOREIGN KEY (tenant_id, session_id)"));
    assert!(mysql.contains("FOREIGN KEY (tenant_id, attempt_id)"));
    for field in [
        "durable_sequence",
        "lease_generation",
        "fencing_token",
        "state_version",
    ] {
        assert!(postgres.contains(field), "PostgreSQL lacks {field}");
        assert!(mysql.contains(field), "MySQL lacks {field}");
    }
    assert!(!mysql.contains("DROP TABLE"));
    assert!(!postgres.contains("DROP TABLE"));
}
