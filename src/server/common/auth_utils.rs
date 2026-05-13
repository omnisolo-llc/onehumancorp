use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    // Security Mandate: Ensure absolute isolation and explicit privilege elevation
    if org_id == "system" {
        // Elevate to bypass role for system-level operations across all tenants
        // SET LOCAL ROLE ensures this privilege is strictly scoped to the current transaction.
        // We also explicitly set the tenant to an empty/system state to prevent accidental leakage
        // if some queries don't check RLS but use the current_tenant config directly.
        query("DO $$ BEGIN EXECUTE 'SET LOCAL ROLE ohc_bypassrls'; PERFORM set_config('app.current_tenant', 'system', true); END $$")
            .execute(executor)
            .await?;
    } else {
        // Standard tenant isolation: enforce RLS via app.current_tenant
        // We use SELECT set_config to ensure the value is bound and sanitized correctly.
        // Transaction scoping (is_local = true) prevents cross-request pollution in pooled connections.
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
