use sqlx::{Executor, Postgres, query};

pub async fn set_system_context<'a, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    tracing::warn!("Elevating database context to SYSTEM (bypass RLS)");
    query("SET LOCAL ROLE ohc_bypassrls")
        .execute(executor)
        .await?;
    Ok(())
}

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    let org_id_trimmed = org_id.trim();
    if ::server_config::get().multitenant {
        if org_id_trimmed.eq_ignore_ascii_case("system") {
            tracing::error!("CRITICAL SECURITY: Attempted to use 'system' tenant in multi-tenant mode");
            return Err(sqlx::Error::Configuration("tenant_id 'system' cannot be queried in multi-tenant mode".into()));
        }
        if org_id_trimmed.is_empty() {
            tracing::error!("CRITICAL SECURITY: Attempted to use empty tenant_id in multi-tenant mode");
            return Err(sqlx::Error::Configuration("empty tenant_id is not allowed in multi-tenant mode".into()));
        }
    }

    if org_id_trimmed.eq_ignore_ascii_case("system") {
        tracing::warn!("Elevating database context to SYSTEM for tenant 'system'");
        query("SET LOCAL ROLE ohc_bypassrls")
            .execute(executor)
            .await?;
    } else {
        query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(org_id_trimmed)
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
