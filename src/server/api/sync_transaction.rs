//! A lost COMMIT reply is not proof of rollback.
#[derive(Debug)]
pub(super) enum SyncError {
    Database(sqlx::Error),
    Commit(sqlx::Error),
}
impl From<sqlx::Error> for SyncError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}
impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "transaction: {e}"),
            Self::Commit(e) => write!(f, "commit: {e}"),
        }
    }
}
impl SyncError {
    pub(super) fn status(&self) -> &'static str {
        match self {
            Self::Commit(sqlx::Error::Database(error))
                if error.code().is_some_and(|code| definite_rollback(&code)) =>
            {
                "blocked"
            }
            Self::Commit(_) => "reconciliation",
            Self::Database(_) => "blocked",
        }
    }
    pub(super) fn reason(&self) -> &'static str {
        if self.status() == "reconciliation" {
            "commit_outcome_unknown"
        } else {
            "transaction_not_committed"
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lost_commit_reply_requires_reconciliation() {
        let error = SyncError::Commit(sqlx::Error::Io(std::io::Error::from(
            std::io::ErrorKind::ConnectionReset,
        )));
        assert_eq!(error.status(), "reconciliation");
        assert_eq!(error.reason(), "commit_outcome_unknown");
    }
    #[test]
    fn failure_before_commit_cannot_be_acknowledged() {
        let error = SyncError::Database(sqlx::Error::Io(std::io::Error::from(
            std::io::ErrorKind::ConnectionReset,
        )));
        assert_eq!(error.status(), "blocked");
    }
}

fn definite_rollback(code: &str) -> bool {
    // Explicit constraint/serialization/deadlock/aborted-transaction failures
    // prove rejection. Unknown completion, shutdown and all other codes do not.
    code.starts_with("23") || matches!(code, "40001" | "40P01" | "25P02" | "P0001")
}
#[cfg(test)]
mod commit_code_tests {
    use super::*;
    #[test]
    fn statement_completion_unknown_and_shutdown_are_not_rollbacks() {
        assert!(!definite_rollback("40003"));
        assert!(!definite_rollback("57P01"));
        assert!(definite_rollback("23514"));
        assert!(definite_rollback("40001"));
        assert!(definite_rollback("P0001"));
    }
}
