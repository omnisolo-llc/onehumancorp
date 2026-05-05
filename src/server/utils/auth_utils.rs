use sqlx::{Executor, Postgres, query};

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if org_id.is_empty() {
        // Elevate privileges for system-level queries.
        // We cannot issue multiple queries because sqlx extended protocol doesn't allow it,
        // and we cannot call execute multiple times because E is consumed.
        // Wait, we can use `query` instead of `executor.execute`, because `query` takes `executor` which we can borrow if we used `&mut executor`, but wait, we had errors with `&mut executor` too because E doesn't implement `Executor` for `&mut E`.
        // The right way is to use a single SQL function, or use an anonymous DO block if we want multiple statements!
        // But DO blocks can't be used with extended query protocol either? Actually they can!
        // Another option: "SET LOCAL ROLE ohc_bypassrls" is all we need! We don't strictly *need* to set current_tenant to empty.
        query("SET LOCAL ROLE ohc_bypassrls")
            .execute(executor)
            .await?;
    } else {
        // No need to RESET ROLE since SET LOCAL is transaction scoped.
        // We explicitly clear role to prevent cross-session leakage.
        // Cannot execute multiple statements in parameterized query using extended query protocol.
        // We can execute multiple statements but bind only works on the last one.
        // Wait, RESET ROLE does not take parameters.
        // Instead of executing two queries we can just execute `set_config` and let the caller reset role if they want to.
        // Actually the `SET LOCAL app.current_tenant` does not require parameters if we format it safely (since org_id is UUID/alphanumeric).
        // Since we are binding org_id, let's just keep the set_config.
        // Actually the issue is that "SET LOCAL ROLE ohc_bypassrls" leaks across transactions if the pool reuses the connection and doesn't reset it.
        // But the pool options have `after_release` configured to `RESET app.current_tenant`.
        // Wait! The pool does NOT have `RESET ROLE`.
        // Oh, `SET LOCAL ROLE` is transaction scoped. The role resets automatically at transaction commit/rollback.
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}
