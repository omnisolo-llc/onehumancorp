use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id == "system" {
        // Elevate privileges for system-level queries.
        // In both Standalone and Cloud mode, background tasks (e.g. workers, mission cleanup)
        // require access to cross-tenant or system-only tables.
        // We use an anonymous DO block to set both the role and reset the tenant context
        // in a single round-trip, which is robust against RLS policies that check current_tenant.
        // We avoid bind parameters here to prevent runtime errors in DO blocks.
        query("DO $$ BEGIN PERFORM set_config('app.current_tenant', '', true); SET LOCAL ROLE ohc_bypassrls; END $$")
            .execute(executor)
            .await?;
    } else {
        // Enforce tenant isolation by setting the current_tenant context.
        // We also ensure we are using the default role to prevent privilege escalation.
        // SET LOCAL ROLE DEFAULT ensures we drop any previous elevation within the transaction.
        // We use a single query that calls set_config to allow for bind parameters.
        query("SELECT set_config('app.current_tenant', $1, true), set_config('role', 'default', true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
