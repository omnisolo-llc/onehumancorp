#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize)]
pub enum DatabaseBackend {
    MySql,
    Postgres,
    Sqlite,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DatabaseCapabilities {
    pub transactions: bool,
    pub skip_locked: bool,
    pub pg_vector: bool,
    pub listen_notify: bool,
    pub row_level_security: bool,
    pub advisory_locks: bool,
    pub logical_replication: bool,
}

impl DatabaseCapabilities {
    pub const fn for_backend(backend: DatabaseBackend) -> Self {
        match backend {
            DatabaseBackend::Postgres => Self {
                transactions: true,
                skip_locked: true,
                pg_vector: true,
                listen_notify: true,
                row_level_security: true,
                advisory_locks: true,
                logical_replication: true,
            },
            DatabaseBackend::MySql => Self {
                transactions: true,
                skip_locked: true,
                pg_vector: false,
                listen_notify: false,
                row_level_security: false,
                advisory_locks: false,
                logical_replication: false,
            },
            DatabaseBackend::Sqlite => Self {
                transactions: true,
                skip_locked: false,
                pg_vector: false,
                listen_notify: false,
                row_level_security: false,
                advisory_locks: false,
                logical_replication: false,
            },
        }
    }
}
