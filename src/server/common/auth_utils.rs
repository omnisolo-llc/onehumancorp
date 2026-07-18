use sqlx::{Executor, Postgres, query};

pub async fn set_system_context<'a, E>(executor: E) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    query("SELECT set_config('role', 'ohc_bypassrls', true);").execute(executor).await?;
    Ok(())
}

pub async fn set_org_context<'a, E>(executor: E, org_id: &str) -> Result<(), sqlx::Error>
where
    E: Executor<'a, Database = Postgres>,
{
    if ::server_config::get().multitenant {
        if org_id.trim().eq_ignore_ascii_case("system") {
            return Err(sqlx::Error::Configuration(
                "tenant_id 'system' cannot be queried in multi-tenant mode".into(),
            ));
        }
        if org_id.trim().is_empty() {
            return Err(sqlx::Error::Configuration(
                "empty tenant_id is not allowed in multi-tenant mode".into(),
            ));
        }
    }

    if org_id.trim().eq_ignore_ascii_case("system") {
        query("SELECT set_config('role', 'ohc_bypassrls', true);").execute(executor).await?;
    } else {
        // We MUST use transaction scope (true) to prevent tenant leakage across queries on the same connection.
        // Pool hooks handle global resets, but transaction-level scope ensures that if a transaction commits/rolls back,
        // the tenant context is safely dropped, preventing IDOR and connection pooling leaks inside sequential flows.
        query(
            "SELECT set_config('role', 'none', true), set_config('app.current_tenant', $1, true);",
        )
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

pub fn signed_tenant_id(claims: &crate::Claims) -> Option<String> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty() && !tenant_id.eq_ignore_ascii_case("system"))
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::signed_tenant_id;

    #[test]
    fn signed_tenant_id_rejects_empty_and_system_claims() {
        let claims = |organization_id: Option<&str>| crate::Claims {
            sub: "user-1".to_string(),
            exp: 0,
            iat: 0,
            organization_id: organization_id.map(str::to_string),
            username: String::new(),
            email: String::new(),
            roles: vec![],
            session_id: None,
            jti: String::new(),
        };

        assert_eq!(
            signed_tenant_id(&claims(Some("tenant-a"))).as_deref(),
            Some("tenant-a")
        );
        assert_eq!(signed_tenant_id(&claims(Some(" system "))), None);
        assert_eq!(signed_tenant_id(&claims(Some(" "))), None);
        assert_eq!(signed_tenant_id(&claims(None)), None);
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
