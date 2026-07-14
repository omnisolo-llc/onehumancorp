use sqlx::{Executor, Postgres, query};

pub async fn set_system_context<'a, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    query("SET LOCAL ROLE ohc_bypassrls")
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if ::server_config::get().multitenant {
        if org_id.trim().eq_ignore_ascii_case("system") {
            return Err(sqlx::Error::Configuration("tenant_id 'system' cannot be queried in multi-tenant mode".into()));
        }
        if org_id.trim().is_empty() {
            return Err(sqlx::Error::Configuration("empty tenant_id is not allowed in multi-tenant mode".into()));
        }
    }

    if org_id.trim().eq_ignore_ascii_case("system") {
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
        // We MUST use transaction scope (true) to prevent tenant leakage across queries on the same connection.
        // Pool hooks handle global resets, but transaction-level scope ensures that if a transaction commits/rolls back,
        // the tenant context is safely dropped, preventing IDOR and connection pooling leaks inside sequential flows.
        query("SELECT set_config('role', 'none', true), set_config('app.current_tenant', $1, true);")
            .bind(org_id)
            .execute(executor)
            .await?;
    }
    Ok(())
}


pub fn get_default_tenant() -> String {
    if ::server_config::get().multitenant {
        "".to_string()
    } else {
        "system".to_string()
    }
}

#[derive(serde::Deserialize)]
pub struct UiTenantQuery {
    pub tenant_id: Option<String>,
    pub tenant: Option<String>,
    pub mobile_optimized: Option<bool>,
    pub fields: Option<String>,
}

pub fn ui_tenant_id(query: &UiTenantQuery) -> String {
    query
        .tenant_id
        .as_deref()
        .or(query.tenant.as_deref())
        .map(str::trim)
        .unwrap_or("")
        .to_string()
}
