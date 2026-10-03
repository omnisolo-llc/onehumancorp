use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ProductEdit {
    pub name: String,
    pub description: String,
    pub price: String,
}

pub(super) fn authorized_tenant(claims: &server_common::Claims) -> Result<&str, StatusCode> {
    let tenant = claims
        .organization_id
        .as_deref()
        .filter(|value| {
            !value.is_empty() && value.trim() == *value && !value.eq_ignore_ascii_case("system")
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;
    if !claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("admin") || role.eq_ignore_ascii_case("owner"))
    {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(tenant)
}

pub(super) fn price_cents(edit: &ProductEdit) -> Option<i64> {
    if edit.name.trim().is_empty()
        || edit.name.chars().count() > 200
        || edit.description.chars().count() > 10_000
    {
        return None;
    }
    let (whole, fraction) = edit.price.split_once('.').unwrap_or((&edit.price, ""));
    if whole.is_empty()
        || !whole.bytes().all(|c| c.is_ascii_digit())
        || fraction.len() > 2
        || !fraction.bytes().all(|c| c.is_ascii_digit())
        || edit.price.ends_with('.')
    {
        return None;
    }
    let whole: i64 = whole.parse().ok()?;
    let fractional: i64 = if fraction.is_empty() {
        0
    } else {
        fraction.parse::<i64>().ok()? * if fraction.len() == 1 { 10 } else { 1 }
    };
    let cents = whole.checked_mul(100)?.checked_add(fractional)?;
    (cents <= 1_000_000_000).then_some(cents)
}

pub(super) fn product_event_payload(
    tenant: &str,
    id: &str,
    name: &str,
    description: &str,
    item_type: &str,
    cents: i64,
) -> serde_json::Value {
    serde_json::json!({"product_id":id,"name":name,"description":description,"item_type":item_type,"price_cents":cents,"price":cents as f64/100.0,"organization_id":tenant,"tenant_id":tenant})
}

pub(super) async fn update_postgres(
    pool: &sqlx::PgPool,
    tenant: &str,
    id: &str,
    edit: &ProductEdit,
    cents: i64,
) -> Result<Option<String>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
    let item_type = sqlx::query_scalar::<_, String>(
        "UPDATE products SET title=$1,description=$2,price_cents=$3,price=$3::bigint::numeric/100,seo_title=NULL,seo_description=NULL,seo_schema_json=NULL,updated_at=clock_timestamp() WHERE id=$4 AND tenant_id=$5 RETURNING COALESCE(type,'Product')",
    ).bind(edit.name.trim()).bind(&edit.description).bind(cents).bind(id).bind(tenant).fetch_optional(&mut *tx).await?;
    tx.commit().await?;
    Ok(item_type)
}

#[cfg(test)]
#[path = "catalog_edit_test.rs"]
mod tests;
