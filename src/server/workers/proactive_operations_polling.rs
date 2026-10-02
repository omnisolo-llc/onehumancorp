use std::fmt;
use std::future::Future;
use std::time::Duration;

const TENANT_SCAN_BUDGET: Duration = Duration::from_secs(5);

#[derive(Debug)]
pub(crate) enum TenantScanError {
    Database(sqlx::Error),
    Deadline,
}
impl fmt::Display for TenantScanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(error) => write!(formatter, "{error}"),
            Self::Deadline => formatter.write_str("tenant operational scan exceeded five seconds"),
        }
    }
}

/// Only inside the caller's transaction: no timeout policy leaks to pool users.
/// Bound a blocked statement on the server as well as the client-side future.
pub(crate) async fn bound_postgres_transaction(
    connection: &mut sqlx::PgConnection,
) -> Result<(), sqlx::Error> {
    sqlx::raw_sql("SET LOCAL lock_timeout='1s'; SET LOCAL statement_timeout='4s'")
        .execute(connection)
        .await?;
    Ok(())
}

/// The cursor is progress-preserving in memory, not persisted. Advance before
/// awaiting any tenant: cancelling a partial batch cannot repeat an early slow
/// tenant forever. Failed tenants are retried after the ordered scan wraps.
pub(crate) async fn scan_tenants<F, Fut>(
    after: &mut String,
    tenants: &[String],
    page_limit: usize,
    mut scan: F,
) -> Vec<(String, Result<(), TenantScanError>)>
where
    F: FnMut(String) -> Fut,
    Fut: Future<Output = Result<(), sqlx::Error>>,
{
    let mut results = Vec::new();
    for tenant in tenants {
        after.clone_from(tenant);
        let result = match tokio::time::timeout(TENANT_SCAN_BUDGET, scan(tenant.clone())).await {
            Ok(result) => result.map_err(TenantScanError::Database),
            Err(_) => Err(TenantScanError::Deadline),
        };
        results.push((tenant.clone(), result));
    }
    // Reset only after this entire last page completes. Outer cancellation
    // drops this future before here, preserving the last attempted tenant.
    if tenants.len() < page_limit {
        after.clear();
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn cancelled_page_preserves_attempted_tenant_for_next_poll() {
        let mut after = String::new();
        let result = tokio::time::timeout(
            Duration::from_millis(10),
            scan_tenants(&mut after, &["a".into(), "b".into()], 100, |_| async {
                std::future::pending::<Result<(), sqlx::Error>>().await
            }),
        )
        .await;
        assert!(result.is_err());
        assert_eq!(after, "a", "a blocked tenant must not starve later tenants");
    }
    #[tokio::test]
    async fn a_slow_tenant_has_a_total_deadline_without_stopping_the_next_tenant() {
        let mut after = String::new();
        let results = tokio::time::timeout(
            Duration::from_secs(6),
            scan_tenants(
                &mut after,
                &["a".into(), "b".into()],
                100,
                |tenant| async move {
                    if tenant == "a" {
                        std::future::pending::<Result<(), sqlx::Error>>().await
                    } else {
                        Ok(())
                    }
                },
            ),
        )
        .await
        .expect("per-tenant work must finish before the outer deadline");
        assert!(results[0].1.is_err());
        assert!(results[1].1.is_ok());
        assert!(after.is_empty());
    }
}
